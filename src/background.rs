use rand;
use rand::Rng;

static RADIUS : f32 = 0.008;

pub struct BackgroundElement{
    pub shape: Vec<f32>,
    pub color: Vec<f32>,
    pub x: f32,
    pub y: f32
}

pub struct Background{
    num : usize,
    shape: Vec<f32>,
    stars: Vec<(f32, f32)>,
    color: Vec<f32>
}

impl Background{
    pub fn new() -> Background{
        Background {
            num: 20,
            shape: vec!(
                0.0, -RADIUS,
                RADIUS, 0.0,
                -RADIUS, 0.0,

                -RADIUS, 0.0,
                RADIUS, 0.0,
                0.0, RADIUS
            ),
            color: vec!(0.5, 0.5, 0.4),
            stars: vec!()
        }
    }

    pub fn get(&self) -> Vec<BackgroundElement>{
        let mut output = vec!();
        for &(x, y) in self.stars.iter(){
            output.push(BackgroundElement{
                x:x,
                y:y,
                color: self.color.clone(),
                shape: self.shape.clone()
            });
        }
        output
    }

    pub fn generate(&mut self, (cx, cy):(f32,f32)){

        let min_x = cx as i32 - 8000;
        let max_x = cx as i32 + 8000;
        let min_y = cy as i32 - 8000;
        let max_y = cy as i32 + 8000;

        while self.stars.len() < self.num {
            let x = rand::thread_rng().gen_range(min_x, max_x) as f32;
            let y = rand::thread_rng().gen_range(min_y, max_y) as f32;

            self.stars.push((x, y));
        }
    }

    pub fn offscreen_generate(&mut self, (cx, cy):(f32,f32)){
        let min_x = cx as i32 - 8000;
        let max_x = cx as i32 + 8000;
        let min_y = cy as i32 - 8000;
        let max_y = cy as i32 + 8000;
        let min_distance = 5500 * 5500; // square instead of sqrt on distance

        while self.stars.len() < self.num {
            let x = rand::thread_rng().gen_range(min_x, max_x) as f32;
            let y = rand::thread_rng().gen_range(min_y, max_y) as f32;
            
            let x_dis = (x - cx) as i32;
            let y_dis = (y - cy) as i32;
            let distance = x_dis * x_dis + y_dis * y_dis;

            if distance > min_distance {
                self.stars.push((x, y));
            }
        }
    }

    pub fn cleanup(&mut self, (cx, cy):(f32, f32)){

        let threshold = 8000.0 * 8000.0;
        let mut output = vec!();

        for &pos in self.stars.iter(){
            let (x, y) = pos;
            let x_distance = x - cx;
            let y_distance = y - cy; 
            let distance = x_distance * x_distance + y_distance * y_distance; 
            if distance <= threshold{
                output.push(pos);
            }
        }

        self.stars = output;
    }
}



