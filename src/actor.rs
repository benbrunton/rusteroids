use std::fmt::Show;
use std::cmp::PartialEq;

#[deriving(Clone, Show, PartialEq)]
pub enum CollisionType{
    Collide,
    Collect,
    Ignore
}

#[deriving(Clone, Show, PartialEq)]
pub struct ActorView{
    pub id: i32,
    pub parent: i32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub shape: Vec<f32>,
    pub color: Vec<f32>,
    pub collision_type: CollisionType
}

impl ActorView{
    pub fn empty() -> ActorView {
        ActorView{
            id:0,
            parent:0, 
            x:0.0, 
            y:0.0, 
            width:0.0, 
            height:0.0, 
            rotation:0.0, 
            shape:vec!(), 
            color:vec!(),
            collision_type: Ignore
        }
    }
}


pub trait Actor : Show + PartialEq {
    fn update(&mut self);
    fn get_view(&self) -> ActorView;
    fn execute(&mut self, message: &str, output_messages:&mut Vec<(&str, ActorView)>);
    fn kill(&mut self);
    fn get_id(&self)->i32;
    fn is_alive(&self)->bool;
}