#![allow(unused_imports, unused_variables)]

use rinha_compiladores::interpreter::interpreter;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() {
    println!(
        r"
     ____  _       _
    |  _ \(_)_ __ | |__   __ _
    | |_) | | '_ \| '_ \ / _` |
    |  _ <| | | | | | | | (_| |
    |_| \_\_|_| |_|_| |_|\__,_|
    
      ____                      _ _           _
     / ___|___  _ __ ___  _ __ (_) | __ _  __| | ___  _ __ ___  ___
    | |   / _ \| '_ ` _ \| '_ \| | |/ _` |/ _` |/ _ \| '__/ _ \/ __|
    | |__| (_) | | | | | | |_) | | | (_| | (_| | (_) | | |  __/\__ \
     \____\___/|_| |_| |_| .__/|_|_|\__,_|\__,_|\___/|_|  \___||___/
                         |_|
    
    "
    );
    println!("\t\t\t🐔 Welcome to the Rinha REPL!\n\n");
    let mut rl = DefaultEditor::new();
    loop {
        let readline = rl.as_mut().expect("Error").readline("👉  ");
        match readline {
            Ok(line) => {
                if line == "exit" || line == "quit" || line == "exit()" || line == "quit()" {
                    print!("👋 Bye!");
                    break;
                }

                if line == "clear" || line == "cls" {
                    rl.as_mut().expect("Error").clear_screen().expect("Error");
                    continue;
                }

                print!("🟰   ");
                match interpreter(&line) {
                    Ok(result) => {
                        println!("{}", result);
                        rl.as_mut()
                            .expect("Error")
                            .add_history_entry(line)
                            .expect("Error");
                    }
                    Err(e) => eprintln!("{}", e),
                };
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
