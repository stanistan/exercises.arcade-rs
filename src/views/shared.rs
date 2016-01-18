use ::phi::data::Rectangle;
use ::phi::gfx::{Sprite,CopySprite};
use ::sdl2::render::{Renderer};

#[derive(Clone)]
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

#[derive(Clone)]
pub struct BackgroundSet {
    pub back: Background,
    pub middle: Background,
    pub front: Background,
}

impl BackgroundSet {
    pub fn new(renderer: &mut Renderer) -> BackgroundSet {
        BackgroundSet {
            back: Background {
                pos: 0.0,
                vel: 20.0,
                sprite: Sprite::load(renderer, "assets/starBG.png").unwrap(),
            },
            middle: Background {
                pos: 0.0,
                vel: 40.0,
                sprite: Sprite::load(renderer, "assets/starMG.png").unwrap(),
            },
            front: Background {
                pos: 0.0,
                vel: 80.0,
                sprite: Sprite::load(renderer, "assets/starFG.png").unwrap(),
            },
        }
    }

    pub fn render(&mut self, renderer: &mut Renderer, elapsed: f64) {
        self.back.render(renderer, elapsed);
        self.middle.render(renderer, elapsed);
        self.front.render(renderer, elapsed);
    }

}
