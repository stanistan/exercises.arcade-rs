use ::phi::{Phi, View, ViewAction};
use ::phi::data::{Rectangle};
use ::phi::gfx::{Sprite, CopySprite};
use ::views::shared::BackgroundSet;
use ::sdl2::pixels::Color;

// Consts
const FONT: &'static str = "assets/belligerent.ttf";

// Types

type BoxAction = Box<Fn(&mut Phi, BackgroundSet) -> ViewAction>;
struct Action {
    func: BoxAction,
    idle_sprite: Sprite,
    hover_sprite: Sprite,
}

impl Action {
    fn new(phi: &mut Phi, label: &'static str, func: BoxAction) -> Action {
        Action {
            func: func,
            idle_sprite: Action::make_idle_sprite(phi, label),
            hover_sprite: Action::make_hover_sprite(phi, label),
        }
    }

    fn make_idle_sprite(phi: &mut Phi, label: &'static str) -> Sprite {
        phi.ttf_str_sprite(label, FONT, 32, Color::RGB(220, 220, 220)).unwrap()
    }

    fn make_hover_sprite(phi: &mut Phi, label: &'static str) -> Sprite {
        phi.ttf_str_sprite(label, FONT, 38, Color::RGB(255, 255, 255)).unwrap()
    }

}

pub struct MainMenuView {
    actions: Vec<Action>,
    selected: i8,
    bgs: BackgroundSet,
}

impl MainMenuView {
    pub fn new(phi: &mut Phi) -> MainMenuView {
        let bg = BackgroundSet::new(&mut phi.renderer);
        MainMenuView::with_backgrounds(phi, bg)
    }

    pub fn with_backgrounds(phi: &mut Phi, bgs: BackgroundSet) -> MainMenuView {
        MainMenuView {
            actions: vec![
                Action::new(phi, "New Game", Box::new(|phi, bgs| {
                    ViewAction::ChangeView(Box::new(::views::game::ShipView::new(phi, bgs)))
                })),
                Action::new(phi, "Quit", Box::new(|_, _| {
                    ViewAction::Quit
                })),
            ],
            selected: 0,
            bgs: bgs,
        }

    }
}

impl View for MainMenuView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {

        if phi.events.now.quit {
            return ViewAction::Quit;
        }

        if phi.events.now.key_escape == Some(true) {
            return ViewAction::ChangeView(Box::new(
                ::views::game::ShipView::new(phi, self.bgs.clone())
            ))
        }

        if phi.events.now.key_space == Some(true) ||
            phi.events.now.key_return == Some(true) {
            return (self.actions[self.selected as usize].func)(phi, self.bgs.clone())
        }

        if phi.events.now.key_up == Some(true) {
            self.selected -= 1;
            if self.selected < 0 {
                self.selected = self.actions.len() as i8 - 1;
            }
        }

        if phi.events.now.key_down == Some(true) {
            self.selected += 1;
            if self.selected >= self.actions.len() as i8 {
                self.selected = 0;
            }
        }

        // clear the screen
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        // bgs
        self.bgs.render(&mut phi.renderer, elapsed);

        let (win_w, win_h) = phi.output_size();
        let label_h = 50.0;
        let border_width = 3.0;
        let box_w = 360.0;
        let box_h = self.actions.len() as f64 * label_h;
        let margin_h = 10.0;

        phi.renderer.set_draw_color(Color::RGB(70, 15, 70));
        phi.renderer.fill_rect(Rectangle {
            w: box_w + border_width * 2.0,
            h: box_h + border_width * 2.0 + margin_h * 2.0,
            x: (win_w - box_w) / 2.0 - border_width,
            y: (win_h - box_h) / 2.0 - margin_h - border_width,
        }.to_sdl().unwrap());

        phi.renderer.set_draw_color(Color::RGB(140, 30, 140));
        phi.renderer.fill_rect(Rectangle {
            w: box_w,
            h: box_h + margin_h * 2.0,
            x: (win_w - box_w) / 2.0,
            y: (win_h - box_h) / 2.0 - margin_h,
        }.to_sdl().unwrap());

        for (i, action) in self.actions.iter().enumerate() {
            let sprite = if self.selected as usize == i {
                &action.hover_sprite
            } else {
                &action.idle_sprite
            };

            let (w, h) = sprite.size();
            phi.renderer.copy_sprite(sprite, Rectangle {
                x: (win_w - w) / 2.0,
                y: (win_h - box_h + label_h - h) / 2.0 + label_h * i as f64,
                w: w,
                h: h,
            });
        }

        ViewAction::None
    }
}
