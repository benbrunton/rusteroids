use actor::ActorView;
use std::rand;
use std::rand::Rng;

static PI : f32 = 3.14159265359;

enum Activity {
    Player(ActorView),
    Forward,
    Fire,
    Nothing
}

pub fn set_instructions(actor: ActorView, 
                        nearbys: Vec<ActorView>,
                        player_messages:&mut Vec<(i32, &str)>){


    // figure out priority

    // 1. look for player
    // 2. can go forward
    // 3. fire

    let mut priority = Nothing;
    
    for &enemy in nearbys.iter() {
        if enemy.id == 1 {
            priority = Player(enemy);
            break;
        }


    }

    match priority{
        Player(enemy)   => attack_player(actor, enemy, player_messages),
        Fire            => player_messages.push((actor.id, "fire")),
        _               => random_behaviour(actor.id, player_messages)
    }
}

fn random_behaviour(id: i32, player_messages: &mut Vec<(i32, &str)>){
    let rand = rand::task_rng().gen_range(0u32, 100);
    match rand {
        0..50  => {
            player_messages.push((id, "stop_rotate_right"));
            player_messages.push((id, "stop_rotate_left"));
            player_messages.push((id, "begin_increase_throttle"));
        },
        51..70 => {
            player_messages.push((id, "begin_rotate_left"));
            player_messages.push((id, "stop_rotate_right"));
            player_messages.push((id, "stop_increase_throttle"));
        },
        71..90 => {
            player_messages.push((id, "begin_rotate_right"));
            player_messages.push((id, "stop_rotate_left"));
            player_messages.push((id, "stop_increase_throttle"));
        }
        _      => {
            player_messages.push((id, "stop_rotate_right"));
            player_messages.push((id, "stop_rotate_left"));
            player_messages.push((id, "stop_increase_throttle"));
        }
    }
}

fn attack_player(player: ActorView, enemy: ActorView, player_messages: &mut Vec<(i32, &str)>){

    let dx = enemy.x - player.x;
    let dy = enemy.y - player.y;
    let mut ideal_rotation = dx.atan2(dy) * 180.0 / PI;
    let mut player_rotation = player.rotation * 180.0 / PI;
    

    while ideal_rotation > 360.0 {
        ideal_rotation -= 360.0;
    }

    while ideal_rotation < -360.0 {
        ideal_rotation += 360.0;
    }

    while player_rotation > 360.0 {
        player_rotation -= 360.0;
    }

    while player_rotation < -360.0 {
        player_rotation += 360.0;
    }

    let d_rotation = ideal_rotation - player_rotation;

    if d_rotation < 20.0 && d_rotation > -20.0 {
        player_messages.push((player.id, "begin_increase_throttle"));
        player_messages.push((player.id, "fire"));
        player_messages.push((player.id, "stop_rotate_left"));
        player_messages.push((player.id, "stop_rotate_right"));
    } else if d_rotation < 0.0 {
        player_messages.push((player.id, "begin_rotate_left"));
        player_messages.push((player.id, "stop_rotate_right"));
    } else {
        player_messages.push((player.id, "stop_rotate_left"));
        player_messages.push((player.id, "begin_rotate_right"));
    }
}

