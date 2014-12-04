use actor::Actor;
use actor::ActorView;
use std::rand;
use std::rand::Rng;
use actor;

static PI : f32 = 3.14159265359;


#[deriving(Show, Clone, PartialEq)]
pub struct Asteroid{
    id: i32,
    x: f32,
    y: f32,
    rotation: f32,
    shape: Vec<f32>,
    is_alive:bool,
    color: Vec<f32>,
    r_speed: f32,
    vx: f32,
    vy: f32,
    width: f32,
    height: f32,
    parent: i32
}

impl Asteroid{
    pub fn new(id: i32, x: i32, y: i32) -> Asteroid{

        let d = rand::task_rng().gen_range(40.0f32, 180.0);
        Asteroid::new_with_d(id, x, y, d, 0)
    }
    pub fn new_with_d(id: i32, x: i32, y: i32, d: f32, parent: i32) -> Asteroid {

        let r = rand::task_rng().gen_range(-5.0f32, 5.0);
        let vx = rand::task_rng().gen_range(-30.0f32, 30.0);
        let vy = rand::task_rng().gen_range(-30.0f32, 30.0);

        let max = d / 2000.0;
        let min = max / 2.0;

        let shape = vec!(
            -min,  max,
            min,   max,
            max,   0.0,

            max,   0.00,
            min,  -max,
            -min, -max,

            -min, -max,
            -max,  0.00,
            -min,  max,

            -min,  max,
             max,  0.00,
             -min, -max


        );

        let color = vec!(0.4, 0.3, 0.3);
        Asteroid{
            id: id, x: x as f32, y: y as f32,
            rotation: 0.0,
            shape: shape,
            is_alive: true,
            color: color,
            r_speed: r,
            vx: vx,
            vy: vy,
            width: d,
            height: d,
            parent: parent
        }
    }
}


impl Actor for Asteroid{
    
    fn update(&mut self, _:&mut Vec<(&str, ActorView)>){
        self.x += self.vx;
        self.y += self.vy;
        self.rotation += self.r_speed;
    }

    fn get_view(&self) -> ActorView {
        ActorView {
            id: self.id,
            parent: self.parent,
            x: self.x, 
            y: self.y,
            width: self.width, 
            height: self.height, 
            rotation: (self.rotation * PI) / 180.0,
            shape: self.shape.clone(),
            color: self.color.clone(),
            collision_type: actor::CollisionType::Collide,
            show_secondary: false,
            secondary_shape: None,
            secondary_color: None,
            meter: 0.0
        }
    }

    fn execute(&mut self, message: &str, output_messages:&mut Vec<(&str, ActorView)>){
        match message {
            "collide"                       => {
                                            self.is_alive = false;
                                            if self.width > 100.0 {
                                                output_messages.push(("new_asteroid", self.get_view().clone()));
                                            }
                                            output_messages.push(("explode", self.get_view().clone()));
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
