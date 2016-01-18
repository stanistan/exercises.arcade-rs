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
