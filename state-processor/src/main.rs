// Contains binary arg handling to allow testing of rust lib
mod core;

use more_asserts::assert_lt;
use std::env;


fn validate_and_run_terrain_gen(args: &[String]) {
    assert_eq!(args[1], "gen-map"); // technically impossible to be wrong,
                                                          // but just for sanity
    assert_lt!(args.len(), 4);                            // Shouldn't have nonsense
    println!("{}", args.len());
    if args.len() == 2 {
        core::generate_terrain((100, 100, 10), None);

    }
}

fn run_entity_gen(_args: &[String]) {
    let mut mgmt = core::EntityMgmt::new((0,0,100,100), (800, 800));
    mgmt.generate_random_entities(5);
    let entity_locs = mgmt.get_all_entity_locs();
    for (id, loc) in &entity_locs {
        println!("Entity ID: {}, is at {:#?}", id, loc);
    }

}

fn advance_game_state(args: &[String]) {
    let mut gs = core::generate_game_state((800, 800, 10), (200, 200, 400, 400), None);
    let iterations: u16 = args[2].parse().expect("Failed to parse string to integer");
    for i in 0..iterations {
        println!("Iteration number: {:#?}", i);
        gs.advance_state();
        let entity_locs = gs.entity_mgmt.get_all_entity_locs();
        for (id, loc) in entity_locs {
            println!("Entity ID: {}, is at {:#?}", id, loc);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Invalid number of arguments");
        return;
    }

    let mode = &args[1]; //the first element is binary name
    dbg!(mode);

    match mode.as_str() {
        "gen-map" => validate_and_run_terrain_gen(&args),
        "gen-entities" => run_entity_gen(&args),
        "gen-state" => advance_game_state(&args),
        _ => println!("Error, invalid mode!"),
    }

    println!("Hello, world!");
}
