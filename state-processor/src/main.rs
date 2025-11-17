use std::env;

enum TerrainArgs {
    BinaryName,
    BinaryMode,
    Argument1,
}

fn generate_terrain(args: &[String]) {
    //TODO
    println!("Generating terrain!");
    /*
     * Idea: Generate map from optional provided seed
     * Get noise seed from args 
     * Generate noise map for altitude/verticality
     * Generate secondary noise map for terrain 
     * Use both to generate overall map data 
    */


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
        "gen-map" => generate_terrain(&args),
        "process-state" => process_state()
        _ => println!("Error, invalid mode!")
    }

    println!("Hello, world!");
}
