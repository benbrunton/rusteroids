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
}