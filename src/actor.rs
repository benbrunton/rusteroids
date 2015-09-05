use std::fmt::Debug;
use std::cmp::PartialEq;
use messages::PlayerInstructions;
use messages::GameInstructions;

#[derive(Clone, Debug, PartialEq)]
pub enum CollisionType{
    Collide,
    Collect,
    Ignore
}

#[derive(Clone, Debug, PartialEq)]
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
    pub collision_type: CollisionType,
    pub show_secondary: bool,
    pub secondary_shape: Option<Vec<f32>>,
    pub secondary_color: Option<Vec<f32>>,
    pub meter: f32
}


pub trait Actor : Debug + PartialEq + Clone {
    fn update(&mut self, output_messages: &mut Vec<(GameInstructions, ActorView)>);
    fn get_view(&self) -> ActorView;
    fn execute(&mut self, message: &PlayerInstructions, output_messages:&mut Vec<(GameInstructions, ActorView)>);
    fn kill(&mut self);
    fn get_id(&self)->i32;
    fn is_alive(&self)->bool;
}
