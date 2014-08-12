use actor::Actor;
use actor::ActorView;

static PI : f32 = 3.14159265359;


#[deriving(Show, Clone, PartialEq)]
pub struct Asteroid{
    id: i32,
    x: f32,
    y: f32,
    rotation: f32,
    shape: Vec<f32>,
    is_alive:bool,
    color: Vec<f32>
}

impl Asteroid{
    pub fn new(id: i32, x: i32, y: i32) -> Asteroid { 
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

        let color = vec!(0.4, 0.4, 0.4);
        Asteroid{
            id: id, x: x as f32, y: y as f32,
            rotation: 0.0,
            shape: shape,
            is_alive: true,
            color: color
        }
    }
}


impl Actor for Asteroid{
    
    fn update(&mut self){
        
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

    fn execute(&mut self, message: &str, output_messages:&mut Vec<(&str, ActorView)>){
        match message {
            "die"                       => {
                                            self.is_alive = false;
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