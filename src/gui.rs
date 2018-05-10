use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use piston::window::Window;
use opengl_graphics::GlGraphics;

pub trait App {
    fn render(&self, _args: &RenderArgs, _gl: &mut GlGraphics, _data: &Data);
    fn update(&mut self, _args: &UpdateArgs, _data: &Data);

    // called after rendering
    fn post_render(&self, _args: &AfterRenderArgs, _data: &Data) {}

    /// if button is pressed or released (not held)
    fn handle_button(&mut self, _args: &ButtonArgs, _data: &Data) {}

    // if button is held
    fn button_held(&mut self, _args: &Button, _data: &Data) {}              

    // handle mouse movement
    fn mouse_moved(&mut self, _args: &Motion, _data: &Data) {}
    
    // handle cursor going on and off screen
    fn handle_cursor(&mut self, _cursor: bool, _data: &Data) {}

    // handle window focus going on and off
    fn handle_focus(&mut self, _focus: bool, _data: &Data) {}

    // handle window resizing
    fn handle_resize(&mut self, _width: u32, _height: u32, _data: &Data) {}
}

pub struct Data {
    pub is_cursor_on: bool,
    pub is_window_focus: bool,
    pub screen_width: u32,
    pub screen_height: u32,
    pub button_held: Vec<Button>
}

impl Data {
    fn new(window: &GlutinWindow) -> Data {
        let size = window.size();
        Data {
            is_cursor_on: false,
            is_window_focus: false,
            screen_width: size.width,
            screen_height: size.height,
            button_held: Vec::new()
        }
    }
}

pub fn start(window: &mut GlutinWindow, app: &mut App, mut g: GlGraphics) {
    let mut events = Events::new(EventSettings::new());
    let mut found = false;

    let mut data = Data::new(window);

    while let Some(e) = events.next(window) {
        match e {
            Event::Custom(a, _b) => {
                println!("custom = {:?}", a);
                if found {
                    println!();
                }
                found = false;
            },
            Event::Loop(l) => {
                match l {
                    Loop::Render(r) => app.render(&r, &mut g, &data),
                    Loop::Update(u) => {
                        for button in &data.button_held {
                            app.button_held(button, &data);
                        }

                        /*
                        if !data.button_held.is_empty() {
                            println!("{:?}", data.button_held);
                        }
                        */

                        app.update(&u, &data);
                    },
                    Loop::AfterRender(ar) => app.post_render(&ar, &data),
                    Loop::Idle(_a) => () // println!("idle time {:?}ms", _a.dt * 1000.0),
                }
                if found {
                    println!();
                }
                found = false;
            },
            Event::Input(i) => {
                match i {
                    // any button pressed or released
                    Input::Button(button) => {
                        // println!("button = {:?}", button);
                        if !data.button_held.contains(&button.button) {
                            app.handle_button(&button, &data);
                        }
                        match button.state {
                            ButtonState::Press => {
                                if !data.button_held.contains(&button.button) {
                                    data.button_held.push(button.button);
                                }
                            },
                            ButtonState::Release => {
                                let index = data.button_held.iter().position(|x| *x == button.button);
                                match index {
                                    Some(i) => { data.button_held.remove(i); },
                                    None => ()
                                }
                            }
                        };
                    }, 
                    // is cursor on screen
                    Input::Cursor(c) => {
                        // println!("cursor = {:?}", c);
                        data.is_cursor_on = c;
                        app.handle_cursor(c, &data);
                    },
                    // is mouse moved
                    Input::Move(motion) => {
                        // println!("move = {:?}", motion);
                        app.mouse_moved(&motion, &data);
                    },
                    Input::Text(ref text) => println!("text = {:?}", text),     // keyboard text (with modifiers applied)
                    Input::Resize(w, h) => {
                        // println!("resize = {}x{}", w, h);
                        data.screen_width = w;
                        data.screen_height = h;
                    },
                    // is screen showing
                    Input::Focus(focus) => {
                        // println!("focus = {:?}", focus);
                        data.is_window_focus = focus;
                        app.handle_focus(focus, &data);
                    },
                    Input::Close(b) => println!("close = {:?}", b),             // is screen closed
                }
                found = true;
            }
        }
    }
}
