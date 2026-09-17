use std::io::{self, Write};

fn main() {
    println!("🌴 JungleScript Terminal");
    println!("Type 'help' for commands.");

    loop {
        print!("JLS> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        match command {
            "help" => {
                println!("Available commands:");
                println!("  help");
                println!("  version");
                println!("  clear");
                println!("  exit");
            }

            "version" => {
                println!("JungleScript v1.0");
            }

            "clear" => {
                print!("\x1B[2J\x1B[1;1H");
            }

            "exit" => {
                println!("Goodbye! 🌴");
                break;
            }

            "" => {}

            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}