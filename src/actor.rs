pub struct ActorPos{
    pub x: i32,
    pub y: i32
}

pub struct Actor{
    x: i32,
    y: i32,
    acc: i32,
    max_acc: i32,
    is_accelerating: bool,
    is_decelerating: bool
}

impl Actor{
    pub fn new(x: i32, y: i32) -> Actor { 
        Actor{
            x: x, y: y, acc: 0, max_acc: 10, is_accelerating: false, is_decelerating: false
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
    
    pub fn update(&mut self){
        if self.is_accelerating {
            self.accelerate();
        }

        if self.is_decelerating{
            self.decelerate();
        }

        self.x += self.acc;
    }

    pub fn get_pos(&self) -> ActorPos {
        ActorPos{ x: self.x, y: self.y }
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
}