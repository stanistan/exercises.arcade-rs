use ::phi::{Phi, View, ViewAction};
use ::sdl2::pixels::Color;
use ::sdl2::rect::Rect as SdlRect;

// Constants

/// Pixels traveled by the player's shpi every second, when moving.
const PLAYER_SPEED: f64 = 180.0;

// Data Types

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


struct Ship {
    rect: Rectangle,
}

// View definition

pub struct ShipView {
    player: Ship,
}

impl ShipView {
    pub fn new(phi: &mut Phi) -> ShipView {
        ShipView {
            player: Ship {
                rect: Rectangle {
                    x: 64.0, y: 64.0, w: 32.0, h: 32.0,
                }
            }
        }
    }
}

impl View for ShipView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        // move the ship
        let diagonal =
            (phi.events.key_up ^ phi.events.key_down) &&
            (phi.events.key_left ^ phi.events.key_right);

        let moved =
            if diagonal { 1.0 / 2.0f64.sqrt() }
            else { 1.0 } * PLAYER_SPEED * elapsed;

        let dx = match (phi.events.key_left, phi.events.key_right) {
            (true, false) => -moved,
            (false, true) => moved,
            _ => 0.0
        };

        let dy = match (phi.events.key_up, phi.events.key_down) {
            (true, false) => -moved,
            (false, true) => moved,
            _ => 0.0
        };

        let moveable_region = Rectangle {
            x: 0.0, y: 0.0,
            w: phi.output_size().0 * 0.70,
            h: phi.output_size().1
        };

        self.player.rect = self.player.rect
            .moved_by(dx, dy)
            .move_inside(moveable_region)
            .unwrap();

        // clear
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        // render scene
        phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
        phi.renderer.fill_rect(self.player.rect.to_sdl().unwrap());

        ViewAction::None
    }
}

