use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;

pub trait App {
    fn render(&self, args: &RenderArgs);
    fn update(&mut self, args: &UpdateArgs);
}

pub fn start(window: &mut Window, app: &mut App) {
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
