mod actor;
mod spaceship;
mod actor_manager;


#[test]
fn actor_manager_get(){
    let mut actors = actor_manager::ActorManager::new();

    let p = spaceship::Spaceship::new(1, 0, 0, 0.0);
    actors.add_spaceship(p);

    assert!(actors.get() == actors.get());

}

#[test]
fn actor_get_view(){
    let p = spaceship::Spaceship::new(1, 0, 0, 0.0);
    view_test(p);
}

#[cfg(test)]
fn view_test<T:actor::Actor>(actor:T){
    assert!(actor.get_view() == actor.get_view());
}


fn main(){
    let p = spaceship::Spaceship::new(1, 0, 0, 0.0);
    loop_through_views(p);
}

fn loop_through_views<T: actor::Actor>(actor:T){
    for _ in range(0i32, 5i32){
        println!("{}", actor.get_view());
    }
}