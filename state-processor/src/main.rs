use std::env;

fn generate_terrain() {
    //TODO
    println!("Generating terrain!");
}

fn process_state() {
    //TODO
    println!("Processing passed state");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mode = &args[1]; //the first element is binary name
    dbg!(mode);

    match mode.as_str() {
        "gen-map" => generate_terrain(),
        "process-state" => process_state()
        _ => println!("Error, invalid mode!")
    }

    println!("Hello, world!");
}
