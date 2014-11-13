use actor;

static MAX_PLAYERS: uint = 5;

pub struct Game{
    pub score: uint,
    pub highscore: uint
}

impl Game{
    pub fn new() -> Game {
        Game {
            score : 0,
            highscore: 0
        }
    }
    pub fn max_players(&self)-> uint{
        MAX_PLAYERS + (self.score * 3)
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
                        self.score += 1;
                        if self.highscore < self.score {
                            self.highscore = self.score;
                        }
                    }
                },
                _       => ()
            }
        }
    }
}
