use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use piston::window::Window;
use piston::window::Size;
use opengl_graphics::GlGraphics;

pub trait App {
    fn render(&self, args: &RenderArgs, mut gl: GlGraphics, size: Size) -> GlGraphics;
    fn update(&mut self, args: &UpdateArgs);

    // fn key_pressed(&mut self, key: Key);
    // fn mouse_clicked(&mut self);
}

pub fn start(window: &mut GlutinWindow, app: &mut App, mut g: GlGraphics) {
    let mut events = Events::new(EventSettings::new());
    let mut found = false;
    while let Some(e) = events.next(window) {
        match e {
            Event::Custom(a, b) => {
                println!("custom = {:?}", a);
            },
            Event::Loop(l) => {
                match l {
                    Loop::Render(r) => g = app.render(&r, g, window.draw_size()),
                    Loop::Update(u) => app.update(&u),
                    Loop::AfterRender(ar) => (),
                    Loop::Idle(a) => println!("idle time {:?}ms", a.dt * 1000.0),
                }
            },
            Event::Input(i) => {
                match i {
                    Input::Button(button) => println!("button = {:?}", button),
                    Input::Cursor(c) => println!("cursor = {:?}", c),
                    Input::Move(motion) => println!("move = {:?}", motion),
                    Input::Text(ref text) => println!("text = {:?}", text),
                    Input::Resize(w, h) => println!("resize = {}x{}", w, h),
                    Input::Focus(on) => println!("focus = {:?}", on),
                    Input::Close(b) => println!("close = {:?}", b),
                }
            }
        }
    }
}
