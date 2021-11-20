mod vulkan;


pub trait IBackend {
   fn init();
}

// pub struct Backend<T: IBackend>{
//    implementation: Box<T>
// }

pub struct Render{
   _backend : Box<vulkan::Vulkan>
}

impl Render {
   pub fn new() -> Render {
      println!("Render creating");
      let backend = Box::new(vulkan::Vulkan::new());
      Render{ _backend : backend}
   }

   pub fn update(&mut self){
      println!("Render updating");
   }

   pub fn test_method(&self){
      println!("Render test method");
   }
   
}