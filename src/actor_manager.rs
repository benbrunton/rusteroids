use actor;

#[deriving(Clone, Show)]
pub struct ActorManager{
    actors: Vec<actor::Actor>
}

impl ActorManager {
    pub fn new() -> ActorManager {
        ActorManager { actors : vec!() }
    }

    pub fn add(&mut self, actor:actor::Actor){
        self.actors.push(actor);
    }

    pub fn get(&self) -> Vec<actor::Actor> {
        self.actors.clone()
    }

    pub fn update(&mut self, messages:Vec<(i32, &str)>, output_messages:&mut Vec<(&str, actor::ActorView)>){
        let mut ac = vec!();
        let threshold = 4000.0;
        let mut player_pos:actor::ActorView = actor::ActorView{id:0, x:0.0, y:0.0, width:0, height:0, rotation:0.0};

        for &mut actor in self.get().iter(){
            if actor.id == 1 {
                player_pos = actor.get_view();
                break;
            }
        }

        for &mut actor in self.actors.iter(){

            if actor.id != 1{
                let a_pos = actor.get_view();
                let x_distance = a_pos.x - player_pos.x;
                let y_distance = a_pos.y - player_pos.y; 
                let distance = (x_distance * x_distance + y_distance * y_distance).sqrt(); 
                if distance > threshold{
                    actor.kill();
                    continue;
                }
            }

            for &(id, message) in messages.iter(){
                if id == actor.id {
                    actor.execute(message, output_messages);
                }
            }

            actor.update();

            if actor.is_alive {
                ac.push(actor);
            }
        }

        self.actors = ac;
    }

    pub fn process_messages(&mut self, output_messages: &Vec<(&str, actor::ActorView)>){

        let sh: Vec<f32> = vec!(
                0.0,  0.05,
                0.025, -0.05,
                -0.025, -0.05,
            );

        for &(msg, v) in output_messages.iter(){
            match msg{
                "fire"  => self.add(ActorManager::new_bullet(v.id, v.x as i32, v.y as i32, v.rotation * 180.0 / 3.14159265359)),
                "enemy" => self.add(ActorManager::new_actor(0, 2, v.x as i32 - 2000, v.y as i32 - 2000, 2, 2, v.rotation * 180.0 / 3.14159265359, sh.clone(), 1.1)),
                _       => ()
            }
        }

    }

    fn new_bullet(parent: i32, x: i32, y:i32, r:f32) -> actor::Actor{
        let v: Vec<f32> = vec!(
            0.0,  0.005,
            0.005, -0.005,
            -0.005, -0.005,
        );
        ActorManager::new_actor(parent, 1, x, y, 2, 2, r, v, 1.8)
    }

    fn new_actor(parent: i32, t:i32, x: i32, y:i32, w: i32, h: i32, r: f32, v: Vec<f32>, acc:f32) -> actor::Actor{
        let id = actor::Actor::get_count();
        actor::Actor::new(id, parent, t, x, y, w, h, r, v, acc)
    }
}