use actor::Actor;
use actor::ActorView;

static PI : f32 = 3.14159265359;


#[deriving(Show, Clone, PartialEq)]
pub struct Explosion{
    id: i32,
    x: f32,
    y: f32,
    rotation: f32,
    shape: Vec<f32>,
    is_alive:bool,
    color: Vec<f32>,
    age: i32
}

impl Explosion{
    pub fn new(x: i32, y: i32) -> Explosion { 
        let shape = vec!(
            -0.05,  0.05,
            -0.05,   -0.05,
            0.05,   -0.05,

            0.05,   -0.05,
            0.05,    0.05,
            -0.05,   0.05
        );

        let color = vec!(0.9, 0.9, 0.9);
        Explosion{
            id: 0, x: x as f32, y: y as f32,
            rotation: 0.0,
            shape: shape,
            is_alive: true,
            color: color,
            age: 0
        }
    }
}


impl Actor for Explosion{
    
    fn update(&mut self){
        self.age += 1;

        if self.age > 30{
            self.is_alive = false;
        }
    }

    fn get_view(&self) -> ActorView {
        ActorView {
            id: self.id,
            parent: 0,
            x: self.x, 
            y: self.y,
            width: 1, 
            height: 1, 
            rotation: (self.rotation * PI) / 180.0,
            shape: self.shape.clone(),
            color: self.color.clone()
        }
    }

    fn execute(&mut self, message: &str, _:&mut Vec<(&str, ActorView)>){
        match message {
            "die"                       => self.is_alive = false,
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