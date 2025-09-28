fn work_routing_mdp() -> MDP {
    let mut builder = MDP::builder();

    // states
    let home = builder.new_state("home");
    let waiting_room = builder.new_state("waiting_room");
    let train = builder.new_state("train");
    let light_traffic = builder.new_state("light_traffic");
    let medium_traffic = builder.new_state("medium_traffic");
    let heavy_traffic = builder.new_state("heavy_traffic");
    let work = builder.terminal_state("work");

    // actions
    let bike_action = builder.new_action("bike_action", 45);
    let car_action = builder.new_action("car_action", 1);
    let drive_light_traffic_action = builder.new_action("drive_light_traffic_action", 20);
    let drive_medium_traffic_action = builder.new_action("drive_medium_traffic_action", 30);
    let drive_heavy_traffic_action = builder.new_action("drive_heavy_traffic_action", 70);
    let train_action = builder.new_action("train_action", 2);
    let wait_action = builder.new_action("wait_action", 3);
    let relax_action = builder.new_action("relax_action", 35);
    let go_home_action = builder.new_action("go_home_action", 2);

    // transitions
    
    // transitions from home
    builder.add_transition(&home, &bike_action, &work, 1);
    builder.add_transition(&home, &car_action, &light_traffic, 0.2);
    builder.add_transition(&home, &car_action, &medium_traffic, 0.7);
    builder.add_transition(&home, &car_action, &heavy_traffic, 0.1);
    builder.add_transition(&home, &train_action, &waiting_room, 0.1);
    builder.add_transition(&home, &train_action, &train, 0.9);

    // transitions from traffic
    builder.add_transition(&light_traffic, &drive_light_traffic_action, &work, 1);
    builder.add_transition(&medium_traffic, &drive_medium_traffic_action, &work, 1);
    builder.add_transition(&heavy_traffic, &drive_heavy_traffic_action, &work, 1);

    // transitions from train
    builder.add_transition(&train, &relax_action, &work, 1);

    // transitions from waiting room
    builder.add_transition(&waiting_room, &wait_action, &waiting_room, 0.1);
    builder.add_transition(&waiting_room, &wait_action, &train, 0.9);
    builder.add_transition(&waiting_room, &go_home_action, &home, 1);

    builder.build()
}

// constraint I need to uphold. 
// - differentiate between action and state (cannot use one in place for another)
// - all probabilities for s' from (s, a) should add up to 1

// batch transitions to improve redability
// deterministic shorthand
