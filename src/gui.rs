use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use piston::window::Window;
use opengl_graphics::GlGraphics;

use std::collections::HashMap;

pub trait App {
    fn render(&self, args: &RenderArgs, gl: &mut GlGraphics, data: &Data);
    fn update(&mut self, args: &UpdateArgs, data: &Data);

    fn handle_button(&mut self, args: &ButtonArgs, data: &Data);        // if button is pressed or released (not held)
    fn button_held(&mut self, args: &Button, data: &Data);          // if button is held
    fn mouse_moved(&mut self, args: &Motion, data: &Data);              // handle mouse movement
    fn handle_cursor(&mut self, cursor: bool, data: &Data);             // handle cursor going on and off screen
    fn handle_focus(&mut self, focus: bool, data: &Data);               // handle window focus going on and off
    fn handle_resize(&mut self, width: u32, height: u32, data: &Data);  // handle window resizing
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
            Event::Custom(a, b) => {
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
                    Loop::AfterRender(ar) => (),
                    Loop::Idle(a) => () // println!("idle time {:?}ms", a.dt * 1000.0),
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
