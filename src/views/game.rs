use ::phi::{Phi, View, ViewAction};
use ::phi::data::Rectangle;
use ::phi::gfx::{AnimatedSprite, Sprite, CopySprite};
use ::views::shared::BackgroundSet;
use ::sdl2::pixels::Color;

// Constants
const DEBUG: bool = false;

const PLAYER_SPEED: f64 = 180.0;

const SHIP_W: f64 = 43.0;
const SHIP_H: f64 = 39.0;

const ASTEROID_PATH: &'static str = "assets/asteroid.png";
const ASTEROIDS_WIDE: usize = 21;
const ASTEROIDS_HIGH: usize = 7;
const ASTEROIDS_TOTAL: usize = ASTEROIDS_WIDE * ASTEROIDS_HIGH - 4;
const ASTEROID_SIDE: f64 = 96.0;

const BULLET_SPEED: f64 = 240.0;
const BULLET_W: f64 = 8.0;
const BULLET_H: f64 = 4.0;

trait Bullet {
    /// Update the bullet.
    /// If the bullet should be destroyed, e.g. because it has left the screen,
    /// then returns `None`.
    /// Otherwise, return `Some(update_bullet)`.
    fn update(self: Box<Self>, phi: &mut Phi, dt: f64) -> Option<Box<Bullet>>;

    /// Render the bullet to the screen.
    fn render(&self, phi: &mut Phi);

    /// Get the bounding box.
    fn rect(&self) -> Rectangle;
}

#[derive(Clone, Copy)]
enum CannonType {
    RectBullet,
}


#[derive(Clone, Copy)]
struct RectBullet {
    rect: Rectangle,
}

impl RectBullet {
    /// Create a new instance given
    /// the positoin of the rect.
    fn new(x: f64, y: f64) -> Self {
        RectBullet {
            rect: Rectangle {
                x: x,
                y: y,
                w: BULLET_W,
                h: BULLET_H,
            }
        }
    }
}

impl Bullet for RectBullet {

    fn update(mut self: Box<Self>, phi: &mut Phi, dt: f64) -> Option<Box<Bullet>> {
        let (w, _) = phi.output_size();
        self.rect.x += BULLET_SPEED * dt;

        if self.rect.x > w {
            None
        } else {
            Some(self)
        }
    }

    fn render(&self, phi: &mut Phi) {
        phi.renderer.set_draw_color(Color::RGB(230, 230, 30));
        phi.renderer.fill_rect(self.rect.to_sdl().unwrap());
    }

    fn rect(&self) -> Rectangle {
        self.rect
    }
}

// Data Types

struct Asteroid {
    sprite: AnimatedSprite,
    rect: Rectangle,
    vel: f64,
}

impl Asteroid {
    fn new(phi: &mut Phi) -> Asteroid {
        let mut asteroid =
            Asteroid {
                sprite: Asteroid::get_sprite(phi, 15.0),
                rect: Asteroid::asteroid_rect(128.0, 128.0),
                vel: 0.0,
            };

        asteroid.reset(phi);
        asteroid
    }

    fn asteroid_rect(x: f64, y: f64) -> Rectangle {
        Rectangle {
            w: ASTEROID_SIDE,
            h: ASTEROID_SIDE,
            x: x,
            y: y,
        }
    }

    fn rand() -> f64 {
        ::rand::random::<f64>().abs()
    }

    fn reset(&mut self, phi: &mut Phi) {
        let (w, h) = phi.output_size();
        self.sprite.set_fps(Self::rand() * 20.0 + 10.0);
        self.rect = Self::asteroid_rect(w, Self::rand() * (h - ASTEROID_SIDE));
        self.vel = Self::rand() * 100.0 + 50.0;
    }

    fn get_sprite(phi: &mut Phi, fps: f64) -> AnimatedSprite {
        let asteroid_spritesheet = Sprite::load(&mut phi.renderer, ASTEROID_PATH).unwrap();
        let mut asteroid_sprites = Vec::with_capacity(ASTEROIDS_TOTAL);

        for yth in 0..ASTEROIDS_HIGH {
            for xth in 0..ASTEROIDS_WIDE {
                if ASTEROIDS_WIDE * yth + xth >= ASTEROIDS_TOTAL {
                    break;
                }

                asteroid_sprites.push(
                    asteroid_spritesheet.region(Self::asteroid_rect(
                        ASTEROID_SIDE * xth as f64,
                        ASTEROID_SIDE * yth as f64
                    )).unwrap());
            }
        }

        AnimatedSprite::with_fps(asteroid_sprites, fps)
    }

    fn update(&mut self, phi: &mut Phi, dt: f64) {
        self.rect.x -= dt * self.vel;
        self.sprite.add_time(dt);
        if self.rect.x <= -ASTEROID_SIDE {
            self.reset(phi);
        }
    }

    fn render(&mut self, phi: &mut Phi) {
        phi.renderer.copy_sprite(&self.sprite, self.rect);
    }
}



#[derive(Clone, Copy)]
enum ShipFrame {
    UpNorm   = 0,
    UpFast   = 1,
    UpSlow   = 2,
    MidNorm  = 3,
    MidFast  = 4,
    MidSlow  = 5,
    DownNorm = 6,
    DownFast = 7,
    DownSlow = 8
}

impl ShipFrame {
    fn from_dx_dy(dx: f64, dy: f64) -> ShipFrame {
        if dx == 0.0 && dy < 0.0       { ShipFrame::UpNorm }
        else if dx > 0.0 && dy < 0.0   { ShipFrame::UpFast }
        else if dx < 0.0 && dy < 0.0   { ShipFrame::UpSlow }
        else if dx == 0.0 && dy == 0.0 { ShipFrame::MidNorm }
        else if dx > 0.0 && dy == 0.0  { ShipFrame::MidFast }
        else if dx < 0.0 && dy == 0.0  { ShipFrame::MidSlow }
        else if dx == 0.0 && dy > 0.0  { ShipFrame::DownNorm }
        else if dx > 0.0 && dy > 0.0   { ShipFrame::DownFast }
        else if dx < 0.0 && dy > 0.0   { ShipFrame::DownSlow }
        else { unreachable!() }
    }
}

struct Ship {
    rect: Rectangle,
    sprites: Vec<Sprite>,
    current: ShipFrame,
    cannon: CannonType,
}

impl Ship {
    fn spawn_bullets(&self) -> Vec<Box<Bullet>> {
        self.new_bullets(
            self.rect.x + 30.0,
            vec![self.rect.y + 6.0, self.rect.y + SHIP_H - 10.0])
    }

    fn new_bullets(&self, x: f64, ys: Vec<f64>) -> Vec<Box<Bullet>> {
        let mut bullets: Vec<Box<Bullet>> = Vec::with_capacity(ys.len());
        for y in ys.iter() {
            bullets.push(self.new_bullet(x, *y));
        }
        bullets
    }

    fn new_bullet(&self, x: f64, y: f64) -> Box<Bullet> {
        Box::new(match self.cannon {
            CannonType::RectBullet => RectBullet::new(x, y),
        })
    }

}


// View definition

pub struct ShipView {
    player: Ship,
    bullets: Vec<Box<Bullet>>,
    asteroid: Asteroid,
    bgs: BackgroundSet,
}

impl ShipView {
    pub fn new(phi: &mut Phi, bgs: BackgroundSet) -> ShipView {
        let spritesheet = Sprite::load(&mut phi.renderer, "assets/spaceship.png").unwrap();

        let mut sprites = Vec::with_capacity(9);

        for y in 0..3 {
            for x in 0..3 {
                sprites.push(spritesheet.region(Rectangle {
                    w: SHIP_W,
                    h: SHIP_H,
                    x: SHIP_W * x as f64,
                    y: SHIP_H * y as f64,
                }).unwrap());
            }
        }

        ShipView {
            player: Ship {
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: SHIP_W,
                    h: SHIP_H
                },
                sprites: sprites,
                current: ShipFrame::MidNorm,
                cannon: CannonType::RectBullet,
            },
            asteroid: Asteroid::new(phi),
            bullets: vec![],
            bgs: bgs,
        }
    }

    fn get_distance_moved(elapsed: f64, diagonal: bool) -> f64 {
        let moved = if diagonal { 1.0 / 2.0f64.sqrt() } else { 1.0 };
        moved * PLAYER_SPEED * elapsed
    }

    fn dx_dy(phi: &mut Phi, elapsed: f64) -> (f64, f64) {

        let moved = Self::get_distance_moved(
            elapsed,
            phi.events.is_key_diagonal());

        (
            Self::d_distance(phi.events.key_left, phi.events.key_right, moved),
            Self::d_distance(phi.events.key_up, phi.events.key_down, moved)
        )
    }

    /// Gets the delta distance given two keys being pressed, or not being pressed.
    fn d_distance(key1: bool, key2: bool, d: f64) -> f64 {
        match (key1, key2) {
            (true, false) => -d,
            (false, true) => d,
            _ => 0.0
        }
    }

    /// Gets the movable region for the window size.
    /// This is used for bounding the player's ship.
    fn movable_region(window: (f64, f64)) -> Rectangle {
        Rectangle { x: 0.0, y: 0.0, w: window.0 * 0.70, h: window.1 }
    }

    /// Gets the next position of the player given the bounding window size.
    fn next_player_rect(&self, dx: f64, dy: f64, window: (f64, f64)) -> Option<Rectangle> {
        self.player.rect.moved_by(dx, dy).move_inside(Self::movable_region(window))
    }

}

impl View for ShipView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        // quitting quits!
        if phi.events.now.quit {
            return ViewAction::Quit;
        }

        // esc toggles game
        if phi.events.now.key_escape == Some(true) {
            return ViewAction::ChangeView(
                Box::new(::views::main_menu::MainMenuView::with_backgrounds(phi, self.bgs.clone()))
            );
        }

        // Update all the current things
        let (dx, dy) = ShipView::dx_dy(phi, elapsed);

        // player position
        self.player.rect = self.next_player_rect(dx, dy, phi.output_size()).unwrap();

        // ship sprite
        self.player.current = ShipFrame::from_dx_dy(dx, dy);

        // bullets
        let old_bullets = ::std::mem::replace(&mut self.bullets, vec![]);
        self.bullets = old_bullets.into_iter()
            .filter_map(|bullet| bullet.update(phi, elapsed))
            .collect();

        // asteroid sprite
        self.asteroid.update(phi, elapsed);

        if phi.events.now.key_space == Some(true) {
            self.bullets.append(&mut self.player.spawn_bullets());
        }

        // Render all the things

        // clear
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        // bgs
        self.bgs.render(&mut phi.renderer, elapsed);

        if DEBUG {
            phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
            phi.renderer.fill_rect(self.player.rect.to_sdl().unwrap());
        }

        // the ship
        phi.renderer.copy_sprite(
            &self.player.sprites[self.player.current as usize],
            self.player.rect);

        // the asteroid
        self.asteroid.render(phi);

        for bullet in &self.bullets {
            bullet.render(phi);
        }

        ViewAction::None
    }
}
