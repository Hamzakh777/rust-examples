use super::Draw;

pub struct Button {
    pub height: u32,
    pub width: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // Code that draw the button
        println!("drawring the button");
    }    
}