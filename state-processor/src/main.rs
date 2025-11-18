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
        "process-state" => core::process_state(),
        _ => println!("Error, invalid mode!"),
    }

    println!("Hello, world!");
}
