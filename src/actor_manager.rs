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

        for &mut actor in self.actors.iter(){
            for &(id, message) in messages.iter(){
                if id == actor.id {
                    actor.execute(message, output_messages);
                }
            }

            actor.update();

            ac.push(actor);
        }

        self.actors = ac;
    }
}