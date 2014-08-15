use actor::Actor;
use actor::ActorView;
use actor;

static PI : f32 = 3.14159265359;


#[deriving(Show, Clone, PartialEq)]
pub struct Explosion{
    id: i32,
    x: f32,
    y: f32,
    rotation: f32,
    is_alive:bool,
    color: Vec<f32>,
    shape: Vec<f32>,
    size: f32,
    age: i32
}

impl Explosion{
    pub fn new(x: i32, y: i32, d: i32, r: f32) -> Explosion {

        let size = d as f32 / 2000.0;
        let shape = vec!(
            0.0,  size,
            size, 0.0,
            0.0, -size,
            0.0, -size,
           -size, 0.0,
            0.0,  size
        );

        let color = vec!(0.9, 0.9, 0.9);
        Explosion{
            id: 0, x: x as f32, y: y as f32,
            rotation: r,
            is_alive: true,
            color: color,
            shape: shape,
            age: 0,
            size: size
        }
    }
}


impl Actor for Explosion{
    
    fn update(&mut self, _:&mut Vec<(&str, ActorView)>){
        self.age += 1;

        if self.age > 15 {
            let s = self.size * 1.5;
            self.color = vec!(0.7, 0.7, 0.7);
            self.shape = vec!(
                0.0,  s,
                s, 0.0,
                0.0,  -s,

                0.0,   -s,
                -s,    0.0,
                0.0,   s
            );

            
        }else if self.age > 10 {
            let s = self.size * 2.0;
            self.color = vec!(0.9, 0.9, 0.4);
            self.shape = vec!(
                -s,  s,
                -s,   -s,
                s,   -s,

                s,   -s,
                s,    s,
                -s,   s
            );
        }

        if self.age > 27{
            self.is_alive = false;
        }

        self.rotation += 1.0;
    }

    fn get_view(&self) -> ActorView {

        ActorView {
            id: self.id,
            parent: 0,
            x: self.x, 
            y: self.y,
            width: 0.0, 
            height: 0.0, 
            rotation: self.rotation * PI / 180.0,
            shape: self.shape.clone(),
            color: self.color.clone(),
            collision_type: actor::Ignore,
            show_secondary: false,
            secondary_shape: None,
            secondary_color: None,
            meter: 0.0
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