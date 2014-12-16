use actor::Actor;
use actor::ActorView;
use std::rand;
use std::rand::Rng;
use actor;
use messages::PlayerInstructions;
use messages::GameInstructions;

static PI : f32 = 3.14159265359;


#[deriving(Show, Clone, PartialEq)]
pub struct Token{
    id: i32,
    x: f32,
    y: f32,
    rotation: f32,
    shape: Vec<f32>,
    is_alive:bool,
    color: Vec<f32>,
    r_speed: f32
}

impl Token{
    pub fn new(id: i32, x: i32, y: i32) -> Token {

        let r = rand::task_rng().gen_range(-5.0f32, 5.0);

        let shape = vec!(
            -0.02,  0.05,
            0.02,   0.05,
            0.05,   0.0,

            0.05,   0.00,
            0.02,  -0.05,
            -0.02, -0.05,

            -0.02, -0.05,
            -0.05,  0.00,
            -0.02,  0.05,

            -0.02,  0.05,
             0.05,  0.00,
             -0.02, -0.05


        );

        let color = vec!(0.9, 0.9, 0.4);
        Token{
            id: id, x: x as f32, y: y as f32,
            rotation: 0.0,
            shape: shape,
            is_alive: true,
            color: color,
            r_speed: r
        }
    }
}


impl Actor for Token{
    
    fn update(&mut self, _:&mut Vec<(GameInstructions, ActorView)>){
        self.rotation += self.r_speed;
    }

    fn get_view(&self) -> ActorView {
        ActorView {
            id: self.id,
            parent: 0,
            x: self.x, 
            y: self.y,
            width: 100.0, 
            height: 100.0, 
            rotation: (self.rotation * PI) / 180.0,
            shape: self.shape.clone(),
            color: self.color.clone(),
            collision_type: actor::CollisionType::Collect,
            show_secondary: false,
            secondary_shape: None,
            secondary_color: None,
            meter: 0.0
        }
    }

    fn execute(&mut self, message: &PlayerInstructions, _:&mut Vec<(GameInstructions, ActorView)>){
        match message {
            &PlayerInstructions::Collide                       => {
                                            //self.is_alive = false;
                                            //output_messages.push(("explode", self.get_view().clone()));
                                        },
            _                           => ()
        };
    }

    fn kill(&mut self){
        self.is_alive = false;
    }

    fn get_id(&self) -> i32{
        self.id
    }

    fn is_alive(&self) -> bool{
        self.is_alive
    }

}
