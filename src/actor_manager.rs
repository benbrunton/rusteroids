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
        let threshold = 2000.0;
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
}