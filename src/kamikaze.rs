use actor::Actor;
use actor::ActorView;

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
    is_alive:bool
}

impl Kamikaze{
    pub fn new(id: i32, x: i32, y: i32, (targetX, targetY): (f32, f32)) -> Kamikaze { 
        let shape = vec!(
            0.0,  0.06,
            0.024, -0.06,
            -0.024, -0.06
        );

        let acc = 1.04;
        let dx = targetX - x as f32;
        let dy = targetY - y as f32;
        let rotation = dx.atan2(dy) * 180.0 / PI;

        Kamikaze{
            id: id, x: x as f32, y: y as f32,
            rotation: rotation, accX: 0.0, accY: 0.0,
            shape: shape,
            acc: acc,
            is_alive: true
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
    
    fn update(&mut self){
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
            width: 1, 
            height: 1, 
            rotation: (self.rotation * PI) / 180.0,
            shape: self.shape.clone()
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