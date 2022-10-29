use chapter12_command_line_program::run;
use chapter12_command_line_program::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // run(config).unwrap_or_else(|err| {
    //     println!("Problem: {err}");
    //     process::exit(1);
    // });
    //
    //
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
