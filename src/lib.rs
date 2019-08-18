mod render;
use render::Render;

mod ecs;
use ecs::ECS;

use std::{thread, time, mem};

pub struct Eng{
    m_render : Box<Render>,
    m_ecs : Box<ECS>,
}

impl Eng{
    pub fn new() -> Eng{
       println!("App created");
        let m_render = Box::new(Render::new());
        let m_ecs= Box::new(ECS::new());

        Eng{m_render, m_ecs}
    }

    pub fn run(&mut self){
        println!("App is running");

        let time = time::Duration::from_millis(10);
        loop {
            println!("frame");
            self.m_ecs.update();
            self.m_render.update();
            thread::sleep(time);
        }
    }
}