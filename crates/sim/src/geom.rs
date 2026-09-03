//! Integer geometry. All match units are **subpixels** (256 per pixel).
//! The sim never touches floats so a later rollback layer can hash state.

pub const SUB: i32 = 256;
pub const TICK_HZ: u32 = 60;

/// Convert whole pixels to subpixels.
pub const fn px(pixels: i32) -> i32 {
    pixels * SUB
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Axis-aligned box in world space. `x,y` is the bottom-center (fighter origin
/// convention: feet, midline).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Aabb {
    pub left: i32,
    pub right: i32,
    pub bottom: i32,
    pub top: i32,
}

impl Aabb {
    pub fn from_center_size(cx: i32, bottom: i32, w: i32, h: i32) -> Self {
        let half = w / 2;
        Self {
            left: cx - half,
            right: cx + half,
            bottom,
            top: bottom + h,
        }
    }

    pub fn overlaps(self, other: Self) -> bool {
        self.left < other.right
            && self.right > other.left
            && self.bottom < other.top
            && self.top > other.bottom
    }

    pub fn center_x(self) -> i32 {
        (self.left + self.right) / 2
    }

    pub fn width(self) -> i32 {
        self.right - self.left
    }

    pub fn height(self) -> i32 {
        self.top - self.bottom
    }
}

/// Box defined in fighter-local space.
/// `x` is offset *toward facing* from the origin; `y` is up from the feet.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LocalBox {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl LocalBox {
    pub const fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    pub fn to_world(self, origin: Vec2i, facing_right: bool) -> Aabb {
        let signed_x = if facing_right {
            self.x
        } else {
            -self.x - self.w
        };
        let left = origin.x + signed_x;
        Aabb {
            left,
            right: left + self.w,
            bottom: origin.y + self.y,
            top: origin.y + self.y + self.h,
        }
    }
}
