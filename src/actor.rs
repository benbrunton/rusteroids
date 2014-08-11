use std::fmt::Show;
use std::cmp::PartialEq;

#[deriving(Clone, Show, PartialEq)]
pub struct ActorView{
    pub id: i32,
    pub parent: i32,
    pub x: f32,
    pub y: f32,
    pub width: i32,
    pub height: i32,
    pub rotation: f32,
    pub shape: Vec<f32>
}


pub trait Actor : Show + PartialEq {
    fn update(&mut self);
    fn get_view(&self) -> ActorView;
    fn execute(&mut self, message: &str, output_messages:&mut Vec<(&str, ActorView)>);
    fn kill(&mut self);
    fn get_id(&self)->i32;
    fn is_alive(&self)->bool;
}