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

    /// Update the bullet.
    /// If the bullet should be destroyed, e.g. because it has left the screen,
    /// then returns `None`.
    /// Otherwise, return `Some(update_bullet)`.
    fn update(mut self, phi: &mut Phi, dt: f64) -> Option<Self> {
        let (w, _) = phi.output_size();
        self.rect.x += BULLET_SPEED * dt;

        if self.rect.x > w {
            None
        } else {
            Some(self)
        }
    }

    /// Render the bullet to the screen
    fn render(self, phi: &mut Phi) {
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
}

impl Ship {
    fn spawn_bullets(&self) -> Vec<RectBullet> {
        let cannons_x = self.rect.x + 30.0;
        let cannon1_y = self.rect.y + 6.0;
        let cannon2_y = self.rect.y + SHIP_H - 10.0;
        vec![cannon1_y, cannon2_y].iter().map(|y| RectBullet::new(cannons_x, *y)).collect()
    }
}


// View definition

pub struct ShipView {
    player: Ship,
    bullets: Vec<RectBullet>,
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
            },
            asteroid: Asteroid::new(phi),
            bullets: vec![],
            bgs: bgs,
        }
    }
}

impl View for ShipView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit {
            return ViewAction::Quit;
        }

        if phi.events.now.key_escape == Some(true) {
            return ViewAction::ChangeView(
                Box::new(::views::main_menu::MainMenuView::with_backgrounds(phi, self.bgs.clone()))
            );
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

        // player position
        self.player.rect = self.player.rect
            .moved_by(dx, dy)
            .move_inside(moveable_region)
            .unwrap();

        // ship sprite
        self.player.current = ShipFrame::from_dx_dy(dx, dy);

        // bullets
        self.bullets = self.bullets.iter()
            .filter_map(|bullet| bullet.update(phi, elapsed))
            .collect();

        // asteroid sprite
        self.asteroid.update(phi, elapsed);

        if phi.events.now.key_space == Some(true) {
            self.bullets.append(&mut self.player.spawn_bullets());
        }

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
