use ::phi::data::Rectangle;
use ::std::cell::RefCell;
use ::std::path::Path;
use ::std::rc::Rc;
use ::sdl2::render::{Renderer, Texture};
use ::sdl2_image::LoadTexture;

#[derive(Clone)]
pub struct Sprite {
    tex: Rc<RefCell<Texture>>,
    src: Rectangle,
}

impl Sprite {
    /// Creates a new sprite by wrapping a `Texture`
    pub fn new(texture: Texture) -> Sprite {
        let q = texture.query();
        Sprite {
            tex: Rc::new(RefCell::new(texture)),
            src: Rectangle {
                w: q.width as f64,
                h: q.height as f64,
                x: 0.0,
                y: 0.0,
            }
        }
    }

    /// Creates a new sprite from an image file.
    pub fn load(renderer: &Renderer, path: &str) -> Option<Sprite> {
        renderer.load_texture(Path::new(path)).ok().map(Sprite::new)
    }

    /// Returns a new `Sprite` representing a subregion of the current one
    /// The provided `rect` is relative to the currently held region.
    /// Returns `Some` if the rect is valid.
    pub fn region(&self, rect: Rectangle) -> Option<Sprite> {
        let new_src = rect.moved_by(self.src.x, self.src.y);
        if self.src.contains(new_src) {
            Some(Sprite {
                tex: self.tex.clone(),
                src: new_src
            })
        } else {
            None
        }
    }

    pub fn size(&self) -> (f64, f64) {
        (self.src.w, self.src.h)
    }

    pub fn render(&self, renderer: &mut Renderer, dest: Rectangle) {
        renderer.copy(&mut self.tex.borrow_mut(), self.src.to_sdl(), dest.to_sdl());
    }
}

pub trait CopySprite {
    fn copy_sprite(&mut self, sprite: &Sprite, dest: Rectangle);
}

impl <'window> CopySprite for Renderer<'window> {
    fn copy_sprite(&mut self, sprite: &Sprite, dest: Rectangle) {
        sprite.render(self, dest);
    }
}

#[derive(Clone)]
pub struct AnimatedSprite {
    sprites: Vec<Sprite>,
    frame_delay: f64,
    current_time: f64,
}

impl AnimatedSprite {
    pub fn new(sprites: Vec<Sprite>, frame_delay: f64) -> AnimatedSprite {
        AnimatedSprite {
            sprites: sprites,
            frame_delay: frame_delay,
            current_time: 0.0,
        }
    }

    pub fn with_fps(sprites: Vec<Sprite>, fps: f64) -> AnimatedSprite {
        if fps == 0.0 {
            panic!("Passed 0 to AnimatedSprite::with_fps");
        }
        AnimatedSprite::new(sprites, 1.0 / fps)
    }

    pub fn frames(&self) -> usize {
        self.sprites.len()
    }

    pub fn set_frame_delay(&mut self, frame_delay: f64) {
        self.frame_delay = frame_delay;
    }

    pub fn set_fps(&mut self, fps: f64) {
        if fps == 0.0 {
            panic!("Passed 0 to AnimatedSprite::set_fps");
        }
        self.set_frame_delay(1.0 / fps);
    }

    pub fn add_time(&mut self, dt: f64) {
        self.current_time += dt;
        if self.current_time < 0.0 {
            self.current_time = (self.frames() - 1) as f64 * self.frame_delay;
        }
    }

    fn render(&self, renderer: &mut Renderer, dest: Rectangle) {
        let current_frame =
            (self.current_time / self.frame_delay) as usize % self.frames();
        let sprite = &self.sprites[current_frame];
        sprite.render(renderer, dest);
    }

}
