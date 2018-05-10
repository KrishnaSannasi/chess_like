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

    fn handle_key(&mut self, _key: Key, _data: &Data) {}
    
    fn handle_mouse(&mut self, _mouse_button: MouseButton, _data: &Data) {}

    fn handle_controller(&mut self, _controller_button: ControllerButton, _data: &Data) {}
    
    fn handle_key_held(&mut self, _key: Key, _data: &Data) {}
    
    fn handle_mouse_held(&mut self, _mouse_button: MouseButton, _data: &Data) {}

    fn handle_controller_held(&mut self, _controller_button: ControllerButton, _data: &Data) {}

    // handle mouse movement
    fn mouse_moved(&mut self, _args: &Motion, _data: &Data) {}
    
    // handle cursor going on and off screen
    fn handle_cursor(&mut self, _cursor: bool, _data: &Data) {}

    // handle window focus going on and off
    fn handle_focus(&mut self, _focus: bool, _data: &Data) {}

    // handle window resizing
    fn handle_resize(&mut self, _width: u32, _height: u32, _data: &Data) {}

    fn on_close(&mut self, _args: &CloseArgs, _data: &Data) {}
}

pub struct Data {
    pub is_cursor_on: bool,
    pub is_window_focus: bool,
    pub screen_width: u32,
    pub screen_height: u32,
    pub mouse_x: f64,
    pub mouse_y: f64,
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
            mouse_x: 0.0,
            mouse_y: 0.0,
            button_held: Vec::new()
        }
    }
}

pub fn start(window: &mut GlutinWindow, app: &mut App, mut g: GlGraphics) {
    let mut events = Events::new(EventSettings::new());
    let mut _found = false;

    let mut data = Data::new(window);

    while let Some(e) = events.next(window) {
        match e {
            Event::Custom(a, _b) => {
                println!("custom = {:?}", a);
                /*
                if _found {
                    println!();
                }
                _found = false;
                */
            },
            Event::Loop(l) => {
                match l {
                    Loop::Render(r) => app.render(&r, &mut g, &data),
                    Loop::Update(u) => {
                        for button in &data.button_held {
                            match button {
                                &Button::Keyboard(key) => 
                                    app.handle_key_held(key, &data),
                                &Button::Mouse(mouse_button) => 
                                    app.handle_mouse_held(mouse_button, &data),
                                &Button::Controller(controller_button) => 
                                    app.handle_controller_held(controller_button, &data)
                            }
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
                /*
                if _found {
                    println!();
                }
                _found = false;
                */
            },
            Event::Input(i) => {
                match i {
                    // any button pressed or released
                    Input::Button(button) => {
                        // println!("button = {:?}", button);
                        if !data.button_held.contains(&button.button) {
                            match button.button {
                                Button::Keyboard(key) => 
                                    app.handle_key(key, &data),
                                Button::Mouse(mouse_button) => 
                                    app.handle_mouse(mouse_button, &data),
                                Button::Controller(controller_button) => 
                                    app.handle_controller(controller_button, &data)
                            }
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

                        if let Motion::MouseCursor(x, y) = motion {
                            data.mouse_x = x;
                            data.mouse_y = y;
                        }

                        app.mouse_moved(&motion, &data);
                    },
                    // keyboard text (with modifiers applied)
                    Input::Text(ref _text) => (), // println!("text = {:?}", _text),
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
                    // is screen closed
                    Input::Close(close) => {
                        // println!("close = {:?}", close);
                        app.on_close(&close, &data);
                    }
                }
                _found = true;
            }
        }
    }
}
