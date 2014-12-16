use actor::Actor;
use actor::ActorView;
use actor;
use messages::PlayerInstructions;
use messages::GameInstructions;
use std::rand;
use std::rand::Rng;
use std::num::FloatMath;

static PI : f32 = 3.14159265359;
static SHIELD_TIME: uint = 180;
static SHOW_TRAILS: bool = false;

#[deriving(Show, Clone, PartialEq)]
pub struct Spaceship{
    id: i32,
    x: f32,
    y: f32,
    acc_x: f32,
    acc_y: f32,
    rotation: f32,
    is_accelerating: bool,
    is_decelerating: bool,
    is_rotating_right: bool,
    is_rotating_left: bool,
    shape: Vec<f32>,
    acc: f32,
    is_alive:bool,
    color: Vec<f32>,
    shield: bool,
    normal_color : Vec<f32>,
    normal_shape : Vec<f32>,
    fire_countdown: i32,
    shield_timer: uint,
    shield_max_time: uint,
    secondary_shape: Vec<f32>,
    secondary_shape_1: Vec<f32>,
    secondary_shape_2: Vec<f32>,
    secondary_color: Vec<f32>,
    thrust_timer: uint
}

impl Spaceship{
    pub fn new(id: i32, x: i32, y: i32, rotation: f32) -> Spaceship { 
        let shape = vec!(
            0.0, 0.05,
            0.025, -0.05,
            0.0, -0.025,

            0.0, -0.025,
            -0.025, -0.05,
            0.0, 0.05
        );

        let secondary_shape = vec!(
            0.0, -0.025,
            0.015, -0.04,
            0.0,  -0.07,

            0.0, -0.07,
            -0.015, -0.04,
            0.0, -0.025
        );

        let secondary_shape2 = vec!(
            0.0, -0.025,
            0.012, -0.03,
            0.0,  -0.04,

            0.0, -0.04,
            -0.012, -0.03,
            0.0, -0.025
        );

        let secondary_color = vec!(1.0, 1.0, 0.7);

        let acc = 1.05;

        let color = vec!(0.5, 0.2, 0.2);

        Spaceship{
            id: id, x: x as f32, y: y as f32,
            rotation: rotation, acc_x: 0.0, acc_y: 0.0,
            is_accelerating: false, is_decelerating: false,
            is_rotating_right: false, is_rotating_left: false,
            shape: shape.clone(),
            normal_shape: shape.clone(),
            acc: acc,
            is_alive: true,
            color: color.clone(),
            normal_color: color.clone(),
            shield: false,
            fire_countdown: 0,
            shield_timer: SHIELD_TIME,
            shield_max_time: SHIELD_TIME,
            secondary_color: secondary_color,
            secondary_shape: secondary_shape.clone(),
            secondary_shape_1: secondary_shape.clone(),
            secondary_shape_2: secondary_shape2,
            thrust_timer: 0
        }
    }

    pub fn set_color(&mut self, c: Vec<f32>){
        self.normal_color = c.clone();
        self.color = c;
    }

    fn begin_increase_throttle(&mut self){
        if !self.shield {
            self.is_accelerating = true;
        }
    }
    fn stop_increase_throttle(&mut self){
        self.is_accelerating = false;
        self.secondary_shape = self.secondary_shape_2.clone();
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

        self.secondary_shape = self.secondary_shape_1.clone();
        self.thrust_timer = 0;

        let (dirx, diry) = self.get_rotate_vec();
        self.acc_x += acc * dirx;
        self.acc_y += acc * diry;
        self.is_decelerating = false;
    }
    fn decelerate(&mut self){
        let acc = 0.8;

        let (dirx, diry) = self.get_rotate_vec();

        self.acc_x -= acc * dirx;
        self.acc_y -= acc * diry;
    }

    fn control(&mut self){
        if self.is_accelerating {
            self.accelerate();
            let r1 = rand::task_rng().gen_range(0.8f32, 1.0);
            let r2 = rand::task_rng().gen_range(0.0f32, 1.0);
            self.secondary_color = vec!(r1, r1, r2);
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
    }

    fn slow_down(&mut self){

        self.acc_x *= 0.992;
        self.acc_y *= 0.992;

        if self.acc_x < 0.005 && self.acc_x > -0.005 {
            self.acc_x = 0.0;
        }

        if self.acc_y < 0.005 && self.acc_y > -0.005 {
            self.acc_y = 0.0;
        }
    }

    fn rotate(&mut self, direction : i32){
        self.rotation += (direction * 5) as f32;
    }

    fn get_rotate_vec(&mut self) -> (f32, f32){
        let r = (self.rotation * PI) / 180.0;
        (r.sin(), r.cos())
    }

    fn shield_up(&mut self){

        if self.shield_timer < 1 {
            return;
        }

        self.shield = true;
        self.color = vec!(0.75, 0.85, 0.5);
        self.shape = vec!(
            0.0, 0.06,
            0.04, -0.05,
            0.0, -0.03,

            0.0, -0.03,
            -0.04, -0.05,
            0.0, 0.06
        );

        self.is_accelerating = false;
        self.is_decelerating = false;
        self.is_rotating_left = false;
        self.is_rotating_right = false;;
    }

    fn shield_down(&mut self){
        self.shield = false;
        self.color = self.normal_color.clone();
        self.shape = self.normal_shape.clone();
    }
}


impl Actor for Spaceship{
    
    fn update(&mut self, output_messages: &mut Vec<(GameInstructions, ActorView)>){
        

        self.y += self.acc_y;
        self.x += self.acc_x;

        self.slow_down();

        if self.fire_countdown > 0 {
            self.fire_countdown -= 1;
        }

        if !self.is_accelerating {
            self.thrust_timer += 1;
        }

        if self.shield {
            if self.shield_timer > 0 {

                let r = rand::task_rng().gen_range(0.5f32, 1.0);
                let b = rand::task_rng().gen_range(0.2f32, 0.8);
                self.color = vec!(r, 0.85, b);
                self.shield_timer -= 1;
            } else {
                self.shield_down();
            }
            return;
        }

        if self.shield_timer < self.shield_max_time {
            self.shield_timer += 1;
        }
        self.control();

        if !SHOW_TRAILS{
            return;
        }

        if self.is_accelerating {
            if rand::task_rng().gen_range(0u32, 10) == 9 {
                output_messages.push((GameInstructions::Trail, self.get_view().clone()));
            }
        }

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
            collision_type: actor::CollisionType::Collide,
            show_secondary: self.is_accelerating || self.thrust_timer < 5,
            secondary_shape: Some(self.secondary_shape.clone()),
            secondary_color: Some(self.secondary_color.clone()),
            meter: self.shield_timer as f32 / self.shield_max_time as f32
        }
    }

    fn execute(&mut self, message: &PlayerInstructions, output_messages:&mut Vec<(GameInstructions, ActorView)>){

        match message {
            &PlayerInstructions::BeginIncreaseThrottle   => self.begin_increase_throttle(),
            &PlayerInstructions::BeginDecreaseThrottle   => self.begin_decrease_throttle(),
            &PlayerInstructions::StopIncreaseThrottle    => self.stop_increase_throttle(),
            &PlayerInstructions::StopDecreaseThrottle    => self.stop_decrease_throttle(),
            &PlayerInstructions::BeginRotateRight        => self.begin_rotate_right(),
            &PlayerInstructions::BeginRotateLeft         => self.begin_rotate_left(),
            &PlayerInstructions::StopRotateRight         => self.stop_rotate_right(),
            &PlayerInstructions::StopRotateLeft          => self.stop_rotate_left(),
            &PlayerInstructions::Fire                      => {
                                            if self.fire_countdown == 0 && !self.shield {
                                                output_messages.push((GameInstructions::Fire, self.get_view().clone()));
                                                self.fire_countdown = 20;
                                            }
                                        },
            &PlayerInstructions::Collide                   => {
                                            if !self.shield {
                                                self.is_alive = false;
                                                output_messages.push((GameInstructions::Explode, self.get_view().clone()));
                                            }
                                        },
            &PlayerInstructions::Collect                   => output_messages.push((GameInstructions::Collect, self.get_view().clone())),
            &PlayerInstructions::ShieldUp                 => self.shield_up(),
            &PlayerInstructions::ShieldDown               => self.shield_down()
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
