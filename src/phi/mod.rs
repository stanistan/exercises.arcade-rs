#[macro_use]
mod events;
pub mod data;

use ::sdl2::render::Renderer;

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_left: Left,
        key_right: Right,
        key_space: Space
    },
    else: {
        quit: Quit { .. }
    }
}

/// Bundles the Phi abstractions in a single strucuture.
pub struct Phi<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>,
}

impl <'window> Phi<'window> {

    fn new(events: Events, renderer: Renderer<'window>) -> Phi<'window> {
        ::sdl2_image::init(::sdl2_image::INIT_PNG);
        Phi {
            events: events,
            renderer: renderer,
        }
    }

    pub fn output_size(&self) -> (f64, f64) {
        let (w, h) = self.renderer.output_size().unwrap();
        (w as f64, h as f64)
    }

}

impl <'window> Drop for Phi<'window> {
    fn drop(&mut self) {
        ::sdl2_image::quit();
    }
}

/// A `ViewAction` is a way for the currently executed view
/// to communicate with the game loop. It specifies which action
/// should be executed before the next rendering.
pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>),
}

pub trait View {
    /// Called on every frame to take care of both the logic and the
    /// rendering of the current view.
    ///
    /// `elapsed is expressed in seconds.
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}

pub fn spawn<F>(title: &str, init: F)
where F: Fn(&mut Phi) -> Box<View> {
    // init
    let sdl_context = ::sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();

    // window
    let window = video.window(title, 800, 600)
        .position_centered().opengl().resizable()
        .build().unwrap();

    //
    let mut context = Phi::new(
        Events::new(sdl_context.event_pump().unwrap()),
        window.renderer().accelerated().build().unwrap());

    //
    let mut current_view = init(&mut context);

    // timing
    let interval = 1_000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;

    loop {

        let now = timer.ticks();
        let dt = now - before;
        let elapsed = dt as f64 / 1_000.0;

        if dt < interval {
            timer.delay(interval - dt);
            continue;
        }

        before = now;
        fps += 1;

        if now - last_second > 1_000 {
            println!("FPS: {}", fps);
            last_second = now;
            fps = 0;
        }

        // RENDERING

        context.events.pump(&mut context.renderer);
        match current_view.render(&mut context, elapsed) {
            ViewAction::None => context.renderer.present(),
            ViewAction::Quit => break,
            ViewAction::ChangeView(new_view) => {
                println!("changeView.before");
                current_view = new_view;
                println!("changeView.after");
            }
        }
    }
}
