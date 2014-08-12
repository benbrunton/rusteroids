use actor;
use spaceship;
use bullet;
use asteroid;
use kamikaze;
use explosion;

#[deriving(Clone, Show, PartialEq)]
pub struct ActorManager{
    spaceships: Vec<spaceship::Spaceship>,
    bullets: Vec<bullet::Bullet>,
    asteroids: Vec<asteroid::Asteroid>,
    kamikaze: Vec<kamikaze::Kamikaze>,
    explosions: Vec<explosion::Explosion>,
    count:i32
}

impl ActorManager {
    pub fn new() -> ActorManager {
        ActorManager { 
            spaceships : vec!(), 
            bullets: vec!(), 
            asteroids: vec!(),
            kamikaze: vec!(),
            explosions: vec!(),
            count: 1 
        }
    }

    

    pub fn get(&self) -> Vec<actor::ActorView> {
        let mut all_views = ActorManager::get_views(&self.spaceships.clone());
        all_views.push_all(ActorManager::get_views(&self.bullets.clone()).slice_from(0));
        all_views.push_all(ActorManager::get_views(&self.asteroids.clone()).slice_from(0));
        all_views.push_all(ActorManager::get_views(&self.kamikaze.clone()).slice_from(0));
        all_views.push_all(ActorManager::get_views(&self.explosions.clone()).slice_from(0));
        all_views
    }

    

    pub fn update(&mut self, messages:Vec<(i32, &str)>, output_messages:&mut Vec<(&str, actor::ActorView)>){
        let mut player_pos:actor::ActorView = actor::ActorView{id:0,parent:0, x:0.0, y:0.0, width:0, height:0, rotation:0.0, shape:vec!(), color:vec!()};

        for &mut actor in self.get().iter(){
            if actor.id == 1 {
                player_pos = actor;
                break;
            }
        }

        self.spaceships = ActorManager::update_actor_list(player_pos.clone(), &mut self.spaceships, messages.clone(), output_messages);
        self.bullets    = ActorManager::update_actor_list(player_pos.clone(), &mut self.bullets, messages.clone(), output_messages);
        self.asteroids  = ActorManager::update_actor_list(player_pos.clone(), &mut self.asteroids, messages.clone(), output_messages);
        self.kamikaze  = ActorManager::update_actor_list(player_pos.clone(), &mut self.kamikaze, messages.clone(), output_messages);
        self.explosions  = ActorManager::update_actor_list(player_pos.clone(), &mut self.explosions, messages.clone(), output_messages);
    }

    pub fn process_messages(&mut self, output_messages: &Vec<(&str, actor::ActorView)>){

        for &(msg, ref v) in output_messages.iter(){
            //println!("{} : {}", msg, v);
            match msg{
                "fire"  => self.add_bullet(v.id, v.x as i32, v.y as i32, v.rotation * 180.0 / 3.14159265359),
                //"enemy" => self.add(ActorManager::new_actor(0, 2, v.x as i32 - 2000, v.y as i32 - 2000, 2, 2, v.rotation * 180.0 / 3.14159265359, sh.clone(), 1.1)),
                "explode" => self.add_explosion(v.x as i32, v.y as i32),
                _       => ()
            }
        }

    }

    pub fn new_player(&mut self){
        let p = spaceship::Spaceship::new(1, 0, 0, 0.0);
        self.spaceships.push(p);
    }

    pub fn restart(&mut self){
        self.spaceships = vec!();
        self.bullets = vec!();
        self.asteroids = vec!();
        self.kamikaze = vec!();
        self.explosions = vec!();
        self.new_player();
    }

    pub fn new_spaceship(&mut self, x: i32, y:i32, r: f32){
        self.count += 1;
        let id = self.count;
        let ship = spaceship::Spaceship::new(id, x, y, r);
        self.spaceships.push(ship);
    }

    pub fn new_asteroid(&mut self, x: i32, y:i32){
        self.count += 1;
        let id = self.count;
        let ast = asteroid::Asteroid::new(id, x, y);
        self.asteroids.push(ast);
    }

    pub fn new_kamikaze(&mut self, x: i32, y:i32, target:(f32, f32)){
        self.count += 1;
        let id = self.count;
        let kam = kamikaze::Kamikaze::new(id, x, y, target);
        self.kamikaze.push(kam);
    }

    fn add_bullet(&mut self, parent:i32, x:i32, y:i32, r:f32){
        self.count += 1;
        let id = self.count;
        let bullet = bullet::Bullet::new(id, parent, x, y, r);
        self.bullets.push(bullet);
    }

    fn add_explosion(&mut self, x:i32, y:i32){
        let expl = explosion::Explosion::new(x, y);
        self.explosions.push(expl);
    }

    fn update_actor_list<T: actor::Actor>(player_pos:actor::ActorView, 
                                list:&mut Vec<T>, 
                                messages:Vec<(i32, &str)>, 
                                output_messages:&mut Vec<(&str, actor::ActorView)>) -> Vec<T>{
        let threshold = 4000.0;
        let mut new_list = vec!();

        for &mut actor in list.iter() {
            if actor.get_id() != 1{
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
                if id == actor.get_id() {
                    actor.execute(message, output_messages);
                }
            }

            actor.update();
            new_list.push(actor);
        }


        let mut ac = vec!();
        for &actor in new_list.iter(){
            if actor.is_alive() {
                ac.push(actor);
            }
        }

        ac

    }

    fn get_views<T: actor::Actor>(list: &Vec<T>) -> Vec<actor::ActorView>{
        let mut v: Vec< actor::ActorView > = vec!();
        for &actor in list.iter() {
            v.push(actor.get_view());
        }
        v
    }
}