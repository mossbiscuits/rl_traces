use std::fs::OpenOptions;
use std::io::Write;
use std::env;

mod traces;
mod model;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <number_of_traces>", args[0]);
        std::process::exit(1);
    }

    let num_traces: usize = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Error: <number_of_traces> must be a positive integer.");
        std::process::exit(1);
    });

    let transitions = model::make_8react_transitions();
    println!("Transitions: {:?}", transitions);
    let dep_graph = model::make_8react_graph(transitions.clone());
    println!("Dependency Graph: {:?}", dep_graph);

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("traces.txt")
        .unwrap_or_else(|e| panic!("Failed to open file: {}", e));
    let current_time = chrono::Local::now();
    writeln!(file, "EXPERIMENTS AT {}", current_time).unwrap();

    traces::make_traces(transitions.clone(), num_traces.try_into().unwrap());
}
