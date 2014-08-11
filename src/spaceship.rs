use actor::Actor;
use actor::ActorView;

static PI : f32 = 3.14159265359;

#[deriving(Show, Clone, PartialEq)]
pub struct Spaceship{
    id: i32,
    x: f32,
    y: f32,
    accX: f32,
    accY: f32,
    rotation: f32,
    is_accelerating: bool,
    is_decelerating: bool,
    is_rotating_right: bool,
    is_rotating_left: bool,
    shape: Vec<f32>,
    acc: f32,
    is_alive:bool
}

impl Spaceship{
    pub fn new(id: i32, x: i32, y: i32, rotation: f32) -> Spaceship { 
        let shape = vec!(
            0.0,  0.05,
            0.025, -0.05,
            -0.025, -0.05
        );

        let acc = 1.05;

        Spaceship{
            id: id, x: x as f32, y: y as f32,
            rotation: rotation, accX: 0.0, accY: 0.0,
            is_accelerating: false, is_decelerating: false,
            is_rotating_right: false, is_rotating_left: false,
            shape: shape,
            acc: acc,
            is_alive: true
        }
    }

    fn begin_increase_throttle(&mut self){
        self.is_accelerating = true;
    }
    fn stop_increase_throttle(&mut self){
        self.is_accelerating = false;
    }

    fn begin_decrease_throttle(&mut self){
        self.is_decelerating = true;
    }
    fn stop_decrease_throttle(&mut self){
        self.is_decelerating = false;
    }

    fn begin_rotate_right(&mut self){
        self.is_rotating_right = true;
    }

    fn stop_rotate_right(&mut self){
        self.is_rotating_right = false;
    }

    fn begin_rotate_left(&mut self){
        self.is_rotating_left = true;
    }
    fn stop_rotate_left(&mut self){
        self.is_rotating_left = false;
    }

    fn accelerate(&mut self){
        let acc = self.acc;

        let (dirx, diry) = self.get_rotate_vec();
        self.accX += acc * dirx;
        self.accY += acc * diry;
        self.is_decelerating = false;
    }
    fn decelerate(&mut self){
        let acc = 0.8;

        let (dirx, diry) = self.get_rotate_vec();

        self.accX -= acc * dirx;
        self.accY -= acc * diry;
    }

    fn slow_down(&mut self){

        self.accX *= 0.992;
        self.accY *= 0.992;

        if self.accX < 0.005 && self.accX > -0.005 {
            self.accX = 0.0;
        }

        if self.accY < 0.005 && self.accY > -0.005 {
            self.accY = 0.0;
        }
    }

    fn rotate(&mut self, direction : i32){
        self.rotation += (direction * 3) as f32;
    }

    fn get_rotate_vec(&mut self) -> (f32, f32){
        let r = (self.rotation * PI) / 180.0;
        (r.sin(), r.cos())
    }
}


impl Actor for Spaceship{
    
    fn update(&mut self){
        if self.is_accelerating {
            self.accelerate();
        }

        if self.is_decelerating{
            self.decelerate();
        }

        if self.is_rotating_left {
            self.rotate(-1);
        }

        if self.is_rotating_right {
            self.rotate(1);
        }

        self.y += self.accY;
        self.x += self.accX;

        self.slow_down();

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

    fn execute(&mut self, message: &str, output_messages:&mut Vec<(&str, ActorView)>){
        match message {
            "begin_increase_throttle"   => self.begin_increase_throttle(),
            "begin_decrease_throttle"   => self.begin_decrease_throttle(),
            "stop_increase_throttle"    => self.stop_increase_throttle(),
            "stop_decrease_throttle"    => self.stop_decrease_throttle(),
            "begin_rotate_right"        => self.begin_rotate_right(),
            "begin_rotate_left"         => self.begin_rotate_left(),
            "stop_rotate_right"         => self.stop_rotate_right(),
            "stop_rotate_left"          => self.stop_rotate_left(),
            "fire"                      => {
                                            output_messages.push(("fire", self.get_view().clone()));
                                        },
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