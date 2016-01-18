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
}

struct Ship {
    rect: Rectangle,
}

// View definition

pub struct DefaultView;

impl View for DefaultView {
    fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
        let renderer = &mut context.renderer;
        let events = &context.events;

        if events.now.quit || events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        ViewAction::None
    }
}

#[derive(Debug)]
pub struct RGBView(u16, u16, u16);

impl Drop for RGBView {
    fn drop(&mut self) {
        println!("Dropped view: {:?}", self);
    }
}

impl RGBView {

    pub fn red() -> RGBView {
        RGBView(255, 0, 0)
    }

    pub fn blue() -> RGBView {
        RGBView(0, 0, 255)
   }

    fn next_view(&self) -> RGBView {
        let RGBView(r, g, b) = *self;
        RGBView((r + 10) % 256, (g + 10) % 256, (b + 10) % 256)
    }

    fn color(&self) -> Color {
        let RGBView(r, g, b) = *self;
        Color::RGB(r as u8, g as u8, b as u8)
    }

}


impl View for RGBView {

    fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
        let renderer = &mut context.renderer;
        let events = &context.events;

        if events.now.quit || events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        if events.now.key_space == Some(true) {
            return ViewAction::ChangeView(Box::new(self.next_view()));
        }

        renderer.set_draw_color(self.color());
        renderer.clear();

        ViewAction::None
    }

}

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

        self.player.rect.x += dx;
        self.player.rect.y += dy;

        // clear
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        // render scene
        phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
        phi.renderer.fill_rect(self.player.rect.to_sdl().unwrap());

        ViewAction::None
    }
}

