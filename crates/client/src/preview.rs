//! Reproducible movement and contact review through real simulation inputs.
//! --polish-preview plays once; --capture also writes 30 fps PNGs and a trace.
use super::{draw_world, Assets};
use crate::render::{draw_hud, HudOpts, View, INK, LINEN, VW};
use crate::timing::FixedClock;
use aeon_sim::{px, Btn, CharacterId, InputFrame, MoveId, World};
use macroquad::prelude::*;
use std::io::Write;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Movement,
    Rekka,
    Whiff,
}

impl Kind {
    fn label(self) -> &'static str {
        match self {
            Self::Movement => "walk · run · hop · full jump",
            Self::Rekka => "rekka contact and recovery",
            Self::Whiff => "heavy whiff · approach · punish",
        }
    }
}

struct Scene {
    world: World,
    frame: u32,
    phase: u8,
    kind: Kind,
    run_steps: u8,
}

impl Scene {
    fn new(body: CharacterId, kind: Kind) -> Self {
        let opponent = if body == CharacterId::Kogan && kind != Kind::Whiff {
            CharacterId::Raya
        } else {
            CharacterId::Kogan
        };
        let mut world = World::new(body, opponent);
        world.fighters[0].pos.x = px(if kind == Kind::Rekka { 270 } else { 180 });
        world.fighters[1].pos.x = px(if kind == Kind::Rekka { 318 } else { 540 });
        if kind == Kind::Whiff {
            let hit = body.data().move_def(MoveId::StHS).unwrap().hitboxes[0].hit;
            world.fighters[0].pos.x = px(300);
            world.fighters[1].pos.x = px(326) + hit.x + hit.w;
        }
        Self {
            world,
            frame: 0,
            phase: 0,
            kind,
            run_steps: 0,
        }
    }

    fn tick(&mut self) {
        if self.kind == Kind::Whiff {
            let input = if self.frame == 24 {
                InputFrame::press(Btn::HS)
            } else {
                InputFrame::default()
            };
            let hs = self.world.fighters[0]
                .data()
                .move_def(MoveId::StHS)
                .unwrap();
            let whiffed = self.frame > 24
                && self.world.fighters[0]
                    .action
                    .attacking()
                    .map(|(_, frame, _)| frame >= hs.last_active())
                    .unwrap_or(true);
            let reply = if !whiffed || self.phase == 1 {
                InputFrame::default()
            } else if self.world.fighters[1].last_distance <= px(90) {
                self.phase = 1;
                InputFrame::dir_press(6, Btn::S)
            } else {
                self.run_steps += 1;
                InputFrame::dir(if self.run_steps == 2 { 5 } else { 6 })
            };
            self.world.tick(input, reply);
            self.frame += 1;
            return;
        }
        let input = if self.kind == Kind::Rekka {
            match self.phase {
                0 if self.frame >= 24 => {
                    self.phase = 1;
                    InputFrame::dir(2)
                }
                1 => {
                    self.phase = 2;
                    InputFrame::dir(3)
                }
                2 => {
                    self.phase = 3;
                    InputFrame::dir_press(6, Btn::S)
                }
                3 if self.world.fighters[1].combo == 1 => {
                    self.phase = 4;
                    InputFrame::press(Btn::S)
                }
                4 if self.world.fighters[1].combo == 2 => {
                    self.phase = 5;
                    InputFrame::press(Btn::S)
                }
                _ => InputFrame::default(),
            }
        } else {
            match self.frame {
                10..=49 => InputFrame::dir(6),
                50..=75 => InputFrame::dir(4),
                90 | 92..=111 => InputFrame::dir(6),
                130 => InputFrame::dir(9),
                138 | 190 => InputFrame::press(Btn::HS),
                175..=181 => InputFrame::dir(8),
                _ => InputFrame::default(),
            }
        };
        self.world.tick(input, InputFrame::default());
        self.frame += 1;
    }
}

pub async fn run(assets: &Assets) {
    let capture = std::env::args().any(|a| a == "--capture");
    let mut trace = if capture {
        std::fs::create_dir_all("shots/polish").expect("preview directory");
        let mut file = std::fs::File::create("shots/polish/trace.txt").expect("preview trace");
        writeln!(file, "scene,tick,hash,p1,p2,hitstop,combo").unwrap();
        Some(file)
    } else {
        None
    };
    let started = get_time();
    let mut rendered = 0_u32;
    let mut clock = FixedClock::default();
    let mut output_frame = 0;
    for (scene_index, (body, kind)) in [
        (CharacterId::Kogan, Kind::Movement),
        (CharacterId::Kogan, Kind::Rekka),
        (CharacterId::Kogan, Kind::Whiff),
        (CharacterId::Raya, Kind::Movement),
        (CharacterId::Raya, Kind::Rekka),
        (CharacterId::Raya, Kind::Whiff),
    ]
    .into_iter()
    .enumerate()
    {
        let mut scene = Scene::new(body, kind);
        clock.reset();
        while scene.frame < 240 {
            if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
                return;
            }
            let ticks = if capture {
                1
            } else {
                clock.advance(get_frame_time() as f64)
            };
            for _ in 0..ticks.min((240 - scene.frame) as usize) {
                scene.tick();
            }
            let w = &scene.world;
            let mut view = View::fit();
            view.follow(w);
            assets.stage.draw(&view, w.frame);
            draw_world(&view, assets, w, false, w.frame);
            draw_hud(
                &view,
                w,
                &HudOpts {
                    wins: None,
                    round: None,
                },
            );
            view.text_center(
                &format!("{} · {}", body.name(), kind.label()),
                VW / 2.0,
                146.0,
                24.0,
                LINEN,
            );
            view.text_center(
                "POLISH REVIEW · scripted inputs · ESC exits",
                VW / 2.0,
                696.0,
                17.0,
                INK,
            );
            if let Some(trace) = &mut trace {
                writeln!(
                    trace,
                    "{scene_index},{},{:016x},{:?},{:?},{},{}",
                    scene.frame,
                    w.state_hash(),
                    w.fighters[0].action,
                    w.fighters[1].action,
                    w.hitstop,
                    w.fighters[1].combo
                )
                .unwrap();
                if scene.frame.is_multiple_of(2) {
                    get_screen_data().export_png(&format!("shots/polish/{output_frame:04}.png"));
                    output_frame += 1;
                }
            }
            rendered += 1;
            next_frame().await;
        }
        eprintln!(
            "[aeon] preview {} {}: hash {:016x}, defender health {}",
            body.name(),
            kind.label(),
            scene.world.state_hash(),
            scene.world.fighters[1].health
        );
    }
    let elapsed = get_time() - started;
    eprintln!("[aeon] polish preview complete: {output_frame} captured frames; {rendered} draws in {elapsed:.2}s ({:.1} draws/s)", f64::from(rendered) / elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aeon_sim::EventKind;

    #[test]
    fn review_sequences_really_connect_and_punish() {
        for body in [CharacterId::Kogan, CharacterId::Raya] {
            for kind in [Kind::Rekka, Kind::Whiff] {
                let mut scene = Scene::new(body, kind);
                let mut combo = 0;
                let mut punished = false;
                for _ in 0..240 {
                    scene.tick();
                    combo = combo.max(scene.world.fighters[1].combo);
                    punished |= scene
                        .world
                        .events
                        .iter()
                        .any(|ev| ev.kind == EventKind::Punish);
                }
                if kind == Kind::Rekka {
                    assert_eq!(combo, 3, "{body:?} full rekka");
                } else {
                    assert!(punished, "{body:?} whiff punish");
                }
            }
        }
    }
}
