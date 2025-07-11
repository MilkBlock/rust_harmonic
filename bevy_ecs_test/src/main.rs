use bevy_ecs::prelude::*;

#[derive(Event)]
struct Explode{

}
#[derive(Event)]
struct MyEvent {
    message: String
}

#[derive(Component)]
struct S<T>{
    a:T
}

fn main(){
    let mut world = World::new();
    let entity = world.spawn_empty().id();
    // so you get a enitity 
    // should I insert the entity into world ?
    // 
    world.add_observer(|trigger: Trigger<MyEvent>| {
        println!("{}", trigger.event().message);
    });
    let system_id = world.register_system(system);
    world.run_system(system_id).unwrap();
    world.spawn(S::<i32>{a:3});

    world.flush();
    world.add_observer(|trigger: Trigger<Explode>, mut commands: Commands| {
        println!("Entity {} goes BOOM!", trigger.target());
        commands.entity(trigger.target()).despawn();
    });
    world.flush();

    world.trigger(MyEvent {
        message: "hello!".to_string(),
    });
    world.trigger_targets(Explode{}, entity);
}
fn system(){
    println!("hello world ")
}