//! `--pack`: build the packed sprite pages the game loads at startup.
//!
//! Loads both bodies the slow way (every sheet, keyed on the CPU, anchors
//! measured), asks each set for every cell it can resolve, crops each unique
//! source rectangle to its alpha bounds, packs the crops into a few pages,
//! and writes `assets/packed/<body>-<n>.png` plus a manifest of
//! `cell page x y w h anchor_x anchor_y height` lines. Nothing in the sim or
//! the source art changes; the game reads pages when they exist and falls
//! back to the sheets when they do not.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::path::Path;

use aeon_sim::CharacterId;
use macroquad::prelude::*;

use crate::sprites::{refit, retained_image, Cell, SpriteSet, KEEP_IMAGES};

/// Page side. 4096 is safe on every GPU this client targets.
pub const PAGE: u16 = 4096;
/// Transparent margin kept around every crop so linear filtering never
/// samples a neighbour.
const PAD: u32 = 2;

struct Item {
    /// Keyed source image and the crop within it (texture pixels).
    image_index: usize,
    crop: Rect,
    /// Where it landed: page and position.
    page: usize,
    x: u32,
    y: u32,
}

/// Alpha bounds of `source` within `image`, grown by one pixel and clamped
/// to the source rectangle. `None` when the region is fully transparent.
fn alpha_bounds(image: &Image, source: Rect) -> Option<Rect> {
    let w = image.width as usize;
    let (x0, y0) = (source.x.max(0.0) as usize, source.y.max(0.0) as usize);
    let x1 = ((source.x + source.w) as usize).min(w);
    let y1 = ((source.y + source.h) as usize).min(image.height as usize);
    let (mut l, mut t, mut r, mut b) = (usize::MAX, usize::MAX, 0usize, 0usize);
    for y in y0..y1 {
        let row = &image.bytes[(y * w + x0) * 4..(y * w + x1) * 4];
        for (i, px) in row.chunks_exact(4).enumerate() {
            if px[3] > 0 {
                let x = x0 + i;
                l = l.min(x);
                r = r.max(x + 1);
                t = t.min(y);
                b = b.max(y + 1);
            }
        }
    }
    if r == 0 {
        return None;
    }
    let l = l.saturating_sub(1).max(x0);
    let t = t.saturating_sub(1).max(y0);
    let r = (r + 1).min(x1);
    let b = (b + 1).min(y1);
    Some(Rect::new(l as f32, t as f32, (r - l) as f32, (b - t) as f32))
}

/// Shelf packing: tallest first, rows across a page, new page when full.
/// Returns the used height of every page, so a partial page is stored no
/// taller than it needs to be.
fn place(items: &mut [Item]) -> Vec<u32> {
    let mut order: Vec<usize> = (0..items.len()).collect();
    order.sort_by(|a, b| {
        let (ha, hb) = (items[*a].crop.h, items[*b].crop.h);
        hb.partial_cmp(&ha).unwrap().then(items[*b].crop.w.partial_cmp(&items[*a].crop.w).unwrap())
    });
    let side = u32::from(PAGE);
    let (mut page, mut x, mut y, mut shelf) = (0usize, PAD, PAD, 0u32);
    let mut used: Vec<u32> = vec![0];
    for i in order {
        let (w, h) = (items[i].crop.w as u32, items[i].crop.h as u32);
        assert!(w + 2 * PAD <= side && h + 2 * PAD <= side, "cell larger than a page");
        if x + w + PAD > side {
            x = PAD;
            y += shelf + PAD;
            shelf = 0;
        }
        if y + h + PAD > side {
            page += 1;
            used.push(0);
            x = PAD;
            y = PAD;
            shelf = 0;
        }
        items[i].page = page;
        items[i].x = x;
        items[i].y = y;
        x += w + PAD;
        shelf = shelf.max(h);
        used[page] = used[page].max(y + h + PAD);
    }
    // Round up so texture rows stay aligned.
    used.iter().map(|h| h.div_ceil(64) * 64).collect()
}

fn blit(page: &mut Image, image: &Image, crop: Rect, x: u32, y: u32) {
    let sw = image.width as usize;
    let pw = page.width as usize;
    let (cx, cy, cw, ch) = (crop.x as usize, crop.y as usize, crop.w as usize, crop.h as usize);
    for row in 0..ch {
        let src = ((cy + row) * sw + cx) * 4;
        let dst = ((y as usize + row) * pw + x as usize) * 4;
        page.bytes[dst..dst + cw * 4].copy_from_slice(&image.bytes[src..src + cw * 4]);
    }
}

/// `export_png` flips rows for screenshots; flip first so the file is upright.
fn export_upright(page: &Image, path: &str) {
    let w = page.width as usize * 4;
    let mut flipped = page.clone();
    for (row, chunk) in flipped.bytes.chunks_exact_mut(w).enumerate() {
        let src = (page.height as usize - 1 - row) * w;
        chunk.copy_from_slice(&page.bytes[src..src + w]);
    }
    flipped.export_png(path);
}

pub async fn run(asset_root: &Path) {
    KEEP_IMAGES.store(true, std::sync::atomic::Ordering::Relaxed);
    let out_dir = asset_root.join("assets").join("packed");
    std::fs::create_dir_all(&out_dir).expect("packed directory");
    let mut summary = String::new();
    for body in [CharacterId::Kogan, CharacterId::Raya] {
        let set = SpriteSet::load_unpacked(body).await;
        let cells = set.available_cells();
        let mut images: Vec<Image> = Vec::new();
        let mut image_of: HashMap<miniquad::TextureId, usize> = HashMap::new();
        let mut items: Vec<Item> = Vec::new();
        let mut item_of: HashMap<(miniquad::TextureId, [i32; 4]), usize> = HashMap::new();
        // (cell, item index, original source rect, anchor, height)
        let mut entries: Vec<(Cell, usize, Rect, Vec2, f32)> = Vec::new();
        let mut skipped = Vec::new();
        for cell in &cells {
            let Some(frame) = set.frame(*cell) else { continue };
            let id = frame.texture.raw_miniquad_id();
            let source = frame.source.unwrap_or_else(|| {
                Rect::new(0.0, 0.0, frame.texture.width(), frame.texture.height())
            });
            let image_index = match image_of.get(&id) {
                Some(i) => *i,
                None => {
                    let Some(image) = retained_image(frame.texture) else {
                        skipped.push(cell.key());
                        continue;
                    };
                    images.push(image);
                    image_of.insert(id, images.len() - 1);
                    images.len() - 1
                }
            };
            let rect_key = [source.x as i32, source.y as i32, source.w as i32, source.h as i32];
            let item = match item_of.get(&(id, rect_key)) {
                Some(i) => *i,
                None => {
                    let Some(crop) = alpha_bounds(&images[image_index], source) else {
                        skipped.push(cell.key());
                        continue;
                    };
                    items.push(Item { image_index, crop, page: 0, x: 0, y: 0 });
                    item_of.insert((id, rect_key), items.len() - 1);
                    items.len() - 1
                }
            };
            entries.push((*cell, item, source, frame.anchor, frame.height));
        }
        let heights = place(&mut items);
        let pages = heights.len();
        let mut page_images: Vec<Image> = heights
            .iter()
            .map(|h| Image::gen_image_color(PAGE, *h as u16, Color::new(0.0, 0.0, 0.0, 0.0)))
            .collect();
        for item in &items {
            blit(&mut page_images[item.page], &images[item.image_index], item.crop, item.x, item.y);
        }
        let dir = match body {
            CharacterId::Kogan => "kogan",
            CharacterId::Raya => "raya",
        };
        for (n, page) in page_images.iter().enumerate() {
            export_upright(page, &out_dir.join(format!("{dir}-{n}.png")).to_string_lossy());
        }
        let mut manifest = String::new();
        for (cell, item, source, anchor, height) in &entries {
            let it = &items[*item];
            let (anchor, height) = refit(*source, *anchor, *height, it.crop);
            writeln!(
                manifest,
                "{} {} {} {} {} {} {:.6} {:.6} {:.6}",
                cell.key(),
                it.page,
                it.x,
                it.y,
                it.crop.w as u32,
                it.crop.h as u32,
                anchor.x,
                anchor.y,
                height
            )
            .unwrap();
        }
        std::fs::write(out_dir.join(format!("{dir}.manifest")), &manifest).expect("manifest");
        let source_px: f32 = images.iter().map(|i| f32::from(i.width) * f32::from(i.height)).sum();
        let packed_px: f32 = items.iter().map(|i| i.crop.w * i.crop.h).sum();
        let line = format!(
            "[aeon] packed {}: {} cells, {} unique crops from {} sheets, {} page(s); {:.0} MB of source pixels -> {:.0} MB of crops{}",
            body.name(),
            entries.len(),
            items.len(),
            images.len(),
            pages,
            source_px * 4.0 / 1_048_576.0,
            packed_px * 4.0 / 1_048_576.0,
            if skipped.is_empty() { String::new() } else { format!("; skipped {}", skipped.join(", ")) }
        );
        eprintln!("{line}");
        summary.push_str(&line);
        summary.push('\n');
    }
    std::fs::write(out_dir.join("PACK.log"), summary).ok();
    eprintln!("[aeon] pack complete: {}", out_dir.display());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn image(w: u16, h: u16, solid: Rect) -> Image {
        let mut img = Image::gen_image_color(w, h, Color::new(0.0, 0.0, 0.0, 0.0));
        for y in solid.y as u32..(solid.y + solid.h) as u32 {
            for x in solid.x as u32..(solid.x + solid.w) as u32 {
                img.set_pixel(x, y, WHITE);
            }
        }
        img
    }

    #[test]
    fn alpha_bounds_grow_by_one_pixel_and_stay_inside_the_source() {
        let img = image(64, 64, Rect::new(10.0, 20.0, 5.0, 6.0));
        let b = alpha_bounds(&img, Rect::new(0.0, 0.0, 64.0, 64.0)).unwrap();
        assert_eq!((b.x, b.y, b.w, b.h), (9.0, 19.0, 7.0, 8.0));
        let clipped = alpha_bounds(&img, Rect::new(10.0, 20.0, 3.0, 3.0)).unwrap();
        assert_eq!((clipped.x, clipped.y, clipped.w, clipped.h), (10.0, 20.0, 3.0, 3.0));
        assert!(alpha_bounds(&img, Rect::new(40.0, 40.0, 8.0, 8.0)).is_none());
    }

    #[test]
    fn refit_keeps_the_feet_and_the_drawn_size() {
        // A 100x200 source with feet at (50, 190) and body height 1.5 units.
        let source = Rect::new(300.0, 100.0, 100.0, 200.0);
        let crop = Rect::new(320.0, 140.0, 60.0, 60.0);
        let (anchor, height) = refit(source, vec2(0.5, 0.95), 1.5, crop);
        // Feet: 0.5*100 = x 50 in source = x 350 texture = (350-320)/60 in crop.
        assert!((anchor.x - 0.5).abs() < 1e-6);
        assert!((anchor.y - (190.0 - 40.0) / 60.0).abs() < 1e-6);
        assert!((height - 1.5 * 60.0 / 200.0).abs() < 1e-6);
    }

    #[test]
    fn shelf_packing_never_overlaps_and_fills_pages_in_order() {
        let mut items: Vec<Item> = (0..40)
            .map(|i| Item {
                image_index: 0,
                crop: Rect::new(0.0, 0.0, 700.0 + (i % 5) as f32 * 100.0, 900.0 - (i % 3) as f32 * 100.0),
                page: 0,
                x: 0,
                y: 0,
            })
            .collect();
        let heights = place(&mut items);
        let pages = heights.len();
        assert!(pages >= 2);
        assert!(heights.iter().all(|h| *h <= u32::from(PAGE) && h % 64 == 0));
        for a in 0..items.len() {
            for b in (a + 1)..items.len() {
                let (p, q) = (&items[a], &items[b]);
                if p.page != q.page {
                    continue;
                }
                let apart = p.x + p.crop.w as u32 + PAD <= q.x
                    || q.x + q.crop.w as u32 + PAD <= p.x
                    || p.y + p.crop.h as u32 + PAD <= q.y
                    || q.y + q.crop.h as u32 + PAD <= p.y;
                assert!(apart, "items {a} and {b} overlap");
            }
            assert!(items[a].x + items[a].crop.w as u32 + PAD <= u32::from(PAGE));
            assert!(items[a].y + items[a].crop.h as u32 + PAD <= heights[items[a].page]);
        }
    }

    #[test]
    fn manifest_round_trips_through_the_loader_parser() {
        let mut manifest = String::new();
        for cell in Cell::candidates().into_iter().take(60) {
            writeln!(manifest, "{} 1 10 20 30 40 0.5 0.9 1.25", cell.key()).unwrap();
        }
        let frames = crate::sprites::Packed::parse(&manifest);
        assert_eq!(frames.len(), 60);
        for cell in Cell::candidates().into_iter().take(60) {
            let f = frames[&cell];
            assert_eq!(f.page, 1);
            assert_eq!((f.source.x, f.source.w), (10.0, 30.0));
        }
    }
}
