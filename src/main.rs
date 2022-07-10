use tests::run;
use std::process;





fn main() {
    match run("input.txt") {
        Ok(val) => println!("Final value: {}", val),
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        },
    }
}