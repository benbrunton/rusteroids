pub struct ActorView{
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub rotation: i32
}

pub struct Actor{
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    acc: i32,
    max_acc: i32,
    rotation: i32,
    is_accelerating: bool,
    is_decelerating: bool,
    is_rotating_right: bool,
    is_rotating_left: bool
}

impl Actor{
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Actor { 
        Actor{
            x: x, y: y, width: w, height: h,
            acc: 0, max_acc: 10, rotation: 0,
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

        self.y += self.acc;
    }

    pub fn get_view(&self) -> ActorView {
        ActorView { 
            x: self.x, 
            y: self.y, 
            width: self.width, 
            height: self.height, 
            rotation: self.rotation 
        }
    }

    fn accelerate(&mut self){
        self.acc += 1;
        if self.acc > self.max_acc {
            self.acc = self.max_acc;
        }
        self.is_decelerating = false;
    }
    fn decelerate(&mut self){
        self.acc -= 1;
        if self.acc < - self.max_acc {
            self.acc = -self.max_acc;
        }
    }

    fn rotate(&mut self, direction : i32){
        self.rotation += direction;
    }
}