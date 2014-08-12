use actor;

static max_players: uint = 15;

pub struct Game{
    pub score: uint
}

impl Game{
    pub fn new() -> Game {
        Game {
            score : 0
        }
    }
    pub fn max_players(&self)-> uint{
        max_players + (self.score * 4) 
    }

    pub fn restart(&mut self){
        self.score = 0;
    }

    pub fn process_messages(&mut self, messages: &Vec<(&str, actor::ActorView)>){
        for &(msg, ref v) in messages.iter(){
            //println!("{} : {}", msg, v);
            match msg{
                "collect"  => {
                    if v.id == 1 {
                        self.score += 1
                    }
                },
                _       => ()
            }
        }
    }
}