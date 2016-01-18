use ::phi::{Phi, View, ViewAction};
use ::phi::data::Rectangle;
use ::phi::gfx::Sprite;
use ::sdl2::pixels::Color;

// Constants

/// Pixels traveled by the player's shpi every second, when moving.
const PLAYER_SPEED: f64 = 180.0;

const SHIP_W: f64 = 43.0;
const SHIP_H: f64 = 39.9;

// Data Types

struct Ship {
    rect: Rectangle,
    sprite: Sprite,
}

// View definition

pub struct ShipView {
    player: Ship,
}

impl ShipView {
    pub fn new(phi: &mut Phi) -> ShipView {
        let sprite = Sprite::load(&mut phi.renderer, "assets/spaceship.png").unwrap();
        let (w, h) = sprite.size();
        ShipView {
            player: Ship {
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: w,
                    h: h
                },
                sprite: sprite,
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

        self.player.sprite.render(&mut phi.renderer, self.player.rect);

        ViewAction::None
    }
}

