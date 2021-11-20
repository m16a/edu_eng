pub struct Render{
}

impl Render {
   pub fn new() -> Render {
      println!("Render creating");
      Render{}
   }

   pub fn update(&mut self){
      println!("Render updating");
   }

   pub fn test_method(&self){
      println!("Render test method");
   }
   
}