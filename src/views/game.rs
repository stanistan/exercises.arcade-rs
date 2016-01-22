use ::phi::{Phi, View, ViewAction};
use ::phi::data::{Rectangle, MaybeAlive};
use ::phi::gfx::{AnimatedSprite, AnimatedSpriteDescr, Sprite, CopySprite};
use ::views::shared::BackgroundSet;
use ::sdl2::pixels::Color;

// Constants
const DEBUG: bool = false;

const PLAYER_SPEED: f64 = 180.0;
const SHIP_PATH: &'static str = "assets/spaceship.png";
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

const EXPLOSION_PATH: &'static str = "assets/explosion.png";
const EXPLOSIONS_WIDE: usize = 5;
const EXPLOSIONS_HIGH: usize = 4;
const EXPLOSIONS_TOTAL: usize = 17;
const EXPLOSION_SIDE: f64 = 96.0;
const EXPLOSION_FPS: f64 = 16.0;
const EXPLOSION_DURATION: f64 = 1.0 / EXPLOSION_FPS * EXPLOSIONS_TOTAL as f64;


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
    SineBullet { amplitude: f64, angular_vel: f64 },
    DivergentBullet { a: f64, b: f64 },
}

fn bullet_sized_rectangle(x: f64, y: f64) -> Rectangle {
    Rectangle {
        x: x, y: y, w: BULLET_W, h: BULLET_H,
    }
}

struct DivergentBullet {
    pos_x: f64,
    origin_y: f64,
    a: f64,
    b: f64,
    total_time: f64,
}

impl Bullet for DivergentBullet {
    fn update(mut self: Box<Self>, phi: &mut Phi, dt: f64) -> Option<Box<Bullet>> {
        self.total_time += dt;
        self.pos_x += BULLET_SPEED * dt;

        let (w, h) = phi.output_size();
        let rect = self.rect();

        if rect.x > w || rect.x < 0.0 || rect.y > h || rect.y < 0.0 {
            None
        } else {
            Some(self)
        }
    }

    fn render(&self, phi: &mut Phi) {
         phi.renderer.set_draw_color(Color::RGB(230, 230, 30));
         phi.renderer.fill_rect(self.rect().to_sdl().unwrap());
    }

    fn rect(&self) -> Rectangle {
        let dy = self.a * (
            (self.total_time / self.b).powi(3) -
            (self.total_time / self.b).powi(2));

        bullet_sized_rectangle(self.pos_x, self.origin_y + dy)
    }
}

struct SineBullet {
    pos_x: f64,
    origin_y: f64,
    amplitude: f64,
    angular_vel: f64,
    total_time: f64,
}

impl Bullet for SineBullet {
    fn update(mut self: Box<Self>, phi: &mut Phi, dt: f64) -> Option<Box<Bullet>> {
        self.total_time += dt;
        self.pos_x += BULLET_SPEED * dt;
        let (w, _) = phi.output_size();
        if self.rect().x > w {
            None
        } else {
            Some(self)
        }
    }

    fn render(&self, phi: &mut Phi) {
        phi.renderer.set_draw_color(Color::RGB(230, 230, 30));
        phi.renderer.fill_rect(self.rect().to_sdl().unwrap());
    }

    fn rect(&self) -> Rectangle {
        let dy = self.amplitude * f64::sin(self.angular_vel * self.total_time);
        bullet_sized_rectangle(self.pos_x, self.origin_y + dy)
    }
}


#[derive(Clone, Copy)]
struct RectBullet {
    rect: Rectangle,
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

    fn asteroid_rect(x: f64, y: f64) -> Rectangle {
        Rectangle {
            w: ASTEROID_SIDE,
            h: ASTEROID_SIDE,
            x: x,
            y: y,
        }
    }

    fn update(mut self, dt: f64) -> Option<Asteroid> {
        self.rect.x -= dt * self.vel;
        self.sprite.add_time(dt);
        if self.rect.x <= -ASTEROID_SIDE {
            None
        } else {
            Some(self)
        }
    }

    fn render(&self, phi: &mut Phi) {
        if DEBUG {
            phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
            phi.renderer.fill_rect(self.rect().to_sdl().unwrap());
        }
        phi.renderer.copy_sprite(&self.sprite, self.rect);
    }

    fn rect(&self) -> Rectangle {
        self.rect
    }

    fn sprite_descr() -> AnimatedSpriteDescr<'static> {
        AnimatedSpriteDescr {
            image_path: ASTEROID_PATH,
            total_frames: ASTEROIDS_TOTAL,
            frames_high: ASTEROIDS_HIGH,
            frames_wide: ASTEROIDS_WIDE,
            frame_w: ASTEROID_SIDE,
            frame_h: ASTEROID_SIDE,
        }
    }

    fn factory(phi: &mut Phi) -> AsteroidFactory {
        AsteroidFactory {
            sprite: AnimatedSprite::load_frames_with_fps(phi, 1.0, Self::sprite_descr()),
        }
    }
}

struct Explosion {
    sprite: AnimatedSprite,
    rect: Rectangle,
    alive_since: f64,
}

impl Explosion {

    fn update(mut self, dt: f64) -> Option<Explosion> {
        self.alive_since += dt;
        self.sprite.add_time(dt);
        if self.alive_since >= EXPLOSION_DURATION {
            None
        } else {
            Some(self)
        }
    }

    fn render(&self, phi: &mut Phi) {
        phi.renderer.copy_sprite(&self.sprite, self.rect);
    }

    fn sprite_descr() -> AnimatedSpriteDescr<'static> {
        AnimatedSpriteDescr {
            image_path: EXPLOSION_PATH,
            total_frames: EXPLOSIONS_TOTAL,
            frames_high: EXPLOSIONS_HIGH,
            frames_wide: EXPLOSIONS_WIDE,
            frame_w: EXPLOSION_SIDE,
            frame_h: EXPLOSION_SIDE
        }
    }

    fn factory(phi: &mut Phi) -> ExplosionFactory {
        ExplosionFactory {
            sprite: AnimatedSprite::load_frames_with_fps(phi, EXPLOSION_FPS, Self::sprite_descr())
        }
    }
}

struct ExplosionFactory {
    sprite: AnimatedSprite,
}

impl ExplosionFactory {
    fn at_center(&self, center: (f64, f64)) -> Explosion {
        let mut sprite = self.sprite.clone();
        Explosion {
            sprite: sprite,
            rect: Rectangle::with_size(EXPLOSION_SIDE, EXPLOSION_SIDE).center_at(center),
            alive_since: 0.0
        }
    }
}

fn rand<T: ::rand::Rand>() -> T {
    ::rand::random::<T>()
}

fn randf64() -> f64 {
    rand::<f64>().abs()
}

struct AsteroidFactory {
    sprite: AnimatedSprite
}

impl AsteroidFactory {
    fn random(&self, phi: &mut Phi) -> Asteroid {
        let (w, h) = phi.output_size();
        let mut sprite = self.sprite.clone();
        sprite.set_fps(randf64() * 20.0 + 10.0);
        Asteroid {
            sprite: sprite,
            rect: Asteroid::asteroid_rect(w, randf64() * (h - ASTEROID_SIDE)),
            vel: randf64() * 100.0 + 50.0
        }
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
        let cannons_x = self.rect.x + 30.0;
        let cannon1_y = self.rect.y + 6.0;
        let cannon2_y = self.rect.y + SHIP_H - 10.0;

        match self.cannon {
            CannonType::RectBullet => vec![
                Box::new(RectBullet {
                    rect: bullet_sized_rectangle(cannons_x, cannon1_y)
                }),
                Box::new(RectBullet {
                    rect: bullet_sized_rectangle(cannons_x, cannon2_y)
                })
            ],
            CannonType::SineBullet { amplitude, angular_vel } => vec![
                Box::new(SineBullet {
                    amplitude: amplitude,
                    angular_vel: angular_vel,
                    pos_x: cannons_x,
                    origin_y: cannon1_y,
                    total_time: 0.0,
                }),
                Box::new(SineBullet {
                    amplitude: amplitude,
                    angular_vel: angular_vel,
                    pos_x: cannons_x,
                    origin_y: cannon2_y,
                    total_time: 0.0,
                })
            ],
            CannonType::DivergentBullet { a, b } => vec![
                Box::new(DivergentBullet {
                    a: -a,
                    b: b,
                    pos_x: cannons_x,
                    origin_y: cannon1_y,
                    total_time: 0.0,
                }),
                Box::new(DivergentBullet {
                    a: a,
                    b: b,
                    pos_x: cannons_x,
                    origin_y: cannon2_y,
                    total_time: 0.0,
                })
            ]
        }

    }


}


// View definition

pub struct GameView {
    player: Ship,
    bullets: Vec<Box<Bullet>>,
    asteroid_factory: AsteroidFactory,
    asteroids: Vec<Asteroid>,
    explosion_factory: ExplosionFactory,
    explosions: Vec<Explosion>,
    bgs: BackgroundSet,
}

impl GameView {
    pub fn new(phi: &mut Phi, bgs: BackgroundSet) -> GameView {
        let spritesheet = Sprite::load(&mut phi.renderer, SHIP_PATH).unwrap();

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

        GameView {
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
            asteroid_factory: Asteroid::factory(phi),
            asteroids: vec![],
            explosion_factory: Explosion::factory(phi),
            explosions: vec![],
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

impl View for GameView {
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

        // change bullet type
        if phi.events.now.key_1 == Some(true) {
            self.player.cannon = CannonType::RectBullet;
        }

        if phi.events.now.key_2 == Some(true) {
            self.player.cannon = CannonType::SineBullet {
                amplitude: 10.0,
                angular_vel: 15.0
            };
        }

        if phi.events.now.key_3 == Some(true) {
            self.player.cannon = CannonType::DivergentBullet {
                a: 100.0,
                b: 1.2,
            };
        }

        // Update all the current things
        let (dx, dy) = Self::dx_dy(phi, elapsed);

        // player position
        self.player.rect = self.next_player_rect(dx, dy, phi.output_size()).unwrap();

        // ship sprite
        self.player.current = ShipFrame::from_dx_dy(dx, dy);

        // bullets
        let old_bullets = ::std::mem::replace(&mut self.bullets, vec![]);
        self.bullets = old_bullets.into_iter()
            .filter_map(|bullet| bullet.update(phi, elapsed))
            .collect();

        // asteroid sprites
        let old_asteroids = ::std::mem::replace(&mut self.asteroids, vec![]);
        self.asteroids = old_asteroids.into_iter()
            .filter_map(|asteroid| asteroid.update(elapsed))
            .collect();

        self.explosions = ::std::mem::replace(&mut self.explosions, vec![])
            .into_iter()
            .filter_map(|explosion| explosion.update(elapsed))
            .collect();

        let mut player_alive = true;
        let mut transition_bullets: Vec<_> =
            ::std::mem::replace(&mut self.bullets, vec![])
            .into_iter()
            .map(|bullet| MaybeAlive { alive: true, value: bullet })
            .collect();

        self.asteroids = ::std::mem::replace(&mut self.asteroids, vec![])
            .into_iter()
            .filter_map(|asteroid| {
                // start alive
                let mut asteroid_alive = true;

                // check if we're hitting bullets
                for bullet in &mut transition_bullets {
                    if asteroid.rect().overlaps(bullet.value.rect()) {
                        asteroid_alive = false;
                        bullet.alive = false;
                    }
                }

                if asteroid.rect().overlaps(self.player.rect) {
                    asteroid_alive = false;
                    player_alive = false;
                }

                if asteroid_alive {
                    Some(asteroid)
                } else {
                    self.explosions.push(self.explosion_factory.at_center(asteroid.rect().center()));
                    None
                }

            })
            .collect();

        self.bullets = transition_bullets.into_iter()
            .filter_map(MaybeAlive::as_option)
            .collect();

        if !player_alive {
            println!("GO HOMEEEEE");
        }

        if phi.events.now.key_space == Some(true) {
            self.bullets.append(&mut self.player.spawn_bullets());
        }

        if rand::<usize>() % 100 == 0 {
            self.asteroids.push(self.asteroid_factory.random(phi));
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

        for bullet in &self.bullets {
            bullet.render(phi);
        }

        for asteroid in &self.asteroids {
            asteroid.render(phi);
        }

        for explosion in &self.explosions {
            explosion.render(phi);
        }

        ViewAction::None
    }
}
