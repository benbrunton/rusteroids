use actor::Actor;
use actor::ActorView;
use actor;

static PI : f32 = 3.14159265359;

#[deriving(Show, Clone, PartialEq)]
pub struct Kamikaze{
    id: i32,
    x: f32,
    y: f32,
    accX: f32,
    accY: f32,
    rotation: f32,
    shape: Vec<f32>,
    acc: f32,
    is_alive:bool,
    color: Vec<f32>
}

impl Kamikaze{
    pub fn new(id: i32, x: i32, y: i32, (targetX, targetY): (f32, f32)) -> Kamikaze { 
        let shape = vec!(
            0.0,  0.06,
            0.024, -0.06,
            -0.024, -0.06
        );

        let color = vec!(0.15, 0.15, 0.5);
        let acc = 1.01;
        let dx = targetX - x as f32;
        let dy = targetY - y as f32;
        let rotation = dx.atan2(dy) * 180.0 / PI;

        Kamikaze{
            id: id, x: x as f32, y: y as f32,
            rotation: rotation, accX: 0.0, accY: 0.0,
            shape: shape,
            acc: acc,
            is_alive: true,
            color: color
        }
    }


    fn accelerate(&mut self){
        let acc = self.acc;
        let (dirx, diry) = self.get_rotate_vec();
        self.accX += acc * dirx;
        self.accY += acc * diry;
    }

    fn get_rotate_vec(&mut self) -> (f32, f32){
        let r = (self.rotation * PI) / 180.0;
        (r.sin(), r.cos())
    }
}


impl Actor for Kamikaze{
    
    fn update(&mut self, _:&mut Vec<(&str, ActorView)>){
        self.accelerate();
        self.y += self.accY;
        self.x += self.accX;
    }

    fn get_view(&self) -> ActorView {
        ActorView {
            id: self.id,
            parent: 0,
            x: self.x, 
            y: self.y,
            width: 50.0, 
            height: 100.0, 
            rotation: (self.rotation * PI) / 180.0,
            shape: self.shape.clone(),
            color: self.color.clone(),
            collision_type: actor::Collide,
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