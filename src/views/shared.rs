use ::phi::data::Rectangle;
use ::phi::gfx::{Sprite,CopySprite};
use ::sdl2::render::{Renderer};

pub struct Background {
    pub pos: f64,
    pub vel: f64,
    pub sprite: Sprite,
}

impl Background {
    pub fn render(&mut self, renderer: &mut Renderer, elapsed: f64) {
        let size = self.sprite.size();
        self.pos += self.vel * elapsed;
        if self.pos > size.0 {
            self.pos -= size.0;
        }

        let (win_w, win_h) = renderer.output_size().unwrap();
        let scale = win_h as f64 / size.1;

        let mut physicall_left = -self.pos * scale;
        while physicall_left < win_w as f64 {
            renderer.copy_sprite(&self.sprite, Rectangle {
                x: physicall_left,
                y: 0.0,
                w: size.0 * scale,
                h: win_h as f64,
            });
            physicall_left += size.0 * scale;
        }
    }
}
