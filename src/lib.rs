mod render;
use render::Render;

mod ecs;
use ecs::{ECS, Component};

use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};
//use std::{thread, time, mem};

const WINDOW_TITLE: &'static str = "Edu";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

pub struct Eng{
    pub m_render : Box<Render>,
    pub m_ecs : Box<ECS>,
}

struct CompA{}
struct CompB{}

impl Component for CompA{
   fn new() -> CompA {
       CompA{}
   }
}

impl Component for CompB{
    fn new() -> CompB {
        CompB{}
    }
}


impl Eng{
    pub fn new() -> Eng{
       println!("App created");
        let m_render = Box::new(Render::new());
        let m_ecs= Box::new(ECS::new());

        Eng{m_render, m_ecs}
    }

    fn init_window(event_loop: &EventLoop<()>) -> winit::window::Window {
        winit::window::WindowBuilder::new()
            .with_title(WINDOW_TITLE)
            .with_inner_size(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .build(event_loop)
            .expect("Failed to create window.")
    }

    pub fn main_loop(event_loop: EventLoop<()>, window: winit::window::Window) {

        event_loop.run(move |event, _, control_flow| {

            *control_flow = ControlFlow::Poll;
            match event {
                | Event::WindowEvent { event, .. } => {
                    match event {
                        | WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        },
                        | WindowEvent::KeyboardInput { input, .. } => {
                            match input {
                                | KeyboardInput { virtual_keycode, state, .. } => {
                                    match (virtual_keycode, state) {
                                        | (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                                            dbg!();
                                            *control_flow = ControlFlow::Exit;
                                        },
                                        | _ => {},
                                    }
                                },
                            }
                        },
                        | _ => {},
                    }
                },
                | Event::MainEventsCleared => {
                    window.request_redraw();
                },
                _ => (),
            }

        });
    }

    pub fn run(&mut self){
        println!("App is running");

        let event_loop = EventLoop::new();
        let window = Eng::init_window(&event_loop); 

        Eng::main_loop(event_loop, window);

        self.m_render.as_ref().test_method();
        self.m_ecs.register_component::<CompA>();
        self.m_ecs.register_component::<CompB>();
        self.m_ecs.register_component::<CompA>();

        /*
        let time = time::Duration::from_millis(10);
        loop {
            println!("frame");
            self.m_ecs.update();
            self.m_render.update();
            thread::sleep(time);
        }
        */
    }
}


