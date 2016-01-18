use ::sdl2::rect::Rect as SdlRect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    /// Generates an SDL-Compatible Rect equivalent to `self`.
    /// Panics if it could not be created, for example,
    /// if a coordinate to a corner overflows an `i32`.
    pub fn to_sdl(self) -> Option<SdlRect> {
        assert!(self.w >= 0.0 && self.h >= 0.0);
        SdlRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
            .unwrap()
    }

    pub fn position_right(self) -> f64 {
        self.x + self.w
    }

    pub fn position_bottom(self) -> f64 {
        self.y + self.h
    }

    pub fn moved(self, x: f64, y: f64) -> Rectangle {
        Rectangle {
            x: x, y: y, w: self.w, h: self.h
        }
    }

    pub fn moved_by(self, dx: f64, dy: f64) -> Rectangle {
        self.moved(self.x + dx, self.y + dy)
    }

    /// Return a (maybe moved) rectange which is contained by a `parent`
    /// rectangle. If it can indeed be moved to fit.
    pub fn move_inside(self, parent: Rectangle) -> Option<Rectangle> {
        if self.w > parent.w || self.h > parent.h {
            return None;
        }

        Some(self.moved(
            if self.x < parent.x { parent.x }
            else if self.position_right() >= parent.position_right() { parent.position_right() - self.w }
            else { self. x },
            if self.y < parent.y { parent.y }
            else if self.position_bottom() >= parent.position_bottom() { parent.position_bottom() - self.y }
            else { self.y }
        ))
    }
}
