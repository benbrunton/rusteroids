static PI : f32 = 3.14159265359;

pub struct ActorView{
    pub x: f32,
    pub y: f32,
    pub width: i32,
    pub height: i32,
    pub rotation: f32
}

pub struct Actor{
    x: f32,
    y: f32,
    width: i32,
    height: i32,
    accX: f32,
    accY: f32,
    rotation: f32,
    is_accelerating: bool,
    is_decelerating: bool,
    is_rotating_right: bool,
    is_rotating_left: bool
}

impl Actor{
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Actor { 
        Actor{
            x: x as f32, y: y as f32, width: w, height: h,
            rotation: 0.0, accX: 0.0, accY: 0.0,
            is_accelerating: false, is_decelerating: false,
            is_rotating_right: false, is_rotating_left: false
        } 
    }
    pub fn begin_increase_throttle(&mut self){
        self.is_accelerating = true;
    }
    pub fn stop_increase_throttle(&mut self){
        self.is_accelerating = false;
    }

    pub fn begin_decrease_throttle(&mut self){
        self.is_decelerating = true;
    }
    pub fn stop_decrease_throttle(&mut self){
        self.is_decelerating = false;
    }

    pub fn begin_rotate_right(&mut self){
        self.is_rotating_right = true;
    }

    pub fn stop_rotate_right(&mut self){
        self.is_rotating_right = false;
    }

    pub fn begin_rotate_left(&mut self){
        self.is_rotating_left = true;
    }
    pub fn stop_rotate_left(&mut self){
        self.is_rotating_left = false;
    }
    
    pub fn update(&mut self){
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

    pub fn get_view(&self) -> ActorView {
        ActorView { 
            x: self.x, 
            y: self.y, 
            width: self.width, 
            height: self.height, 
            rotation: (self.rotation * PI) / 180.0
        }
    }

    fn accelerate(&mut self){
        let acc = 1.1;

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