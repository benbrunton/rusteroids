use actor;
use messages::GameInstructions;
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

    pub fn process_messages(&mut self, messages: Vec<(GameInstructions, actor::ActorView)>){
        for &(ref msg, ref v) in messages.iter(){
            match msg{
                &GameInstructions::Collect  => {
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
