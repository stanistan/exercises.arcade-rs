use ::phi::{Phi, View, ViewAction};
use ::sdl2::pixels::Color;
use ::sdl2::rect::Rect as SdlRect;

// Constants

// Data Types

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

pub struct ShipView;

impl ShipView {
    pub fn new(phi: &mut Phi) -> ShipView {
        ShipView
    }
}

impl View for ShipView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        // View logic here

        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        // View rendering here

        ViewAction::None
    }
}

