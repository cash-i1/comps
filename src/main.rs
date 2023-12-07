use std::{env, string};
use std::process::Command;
use std::process;
use std::env::consts::OS;

fn get_os() -> &'static str {

    match OS {
        "macos" | "windows" | "linux" => return OS,
        _ => return "null",
    }

    
}

fn compile(run: bool) {
    let os = get_os();

    if os == "linux" {
        println!("LINUX");
        Command::new("g++").arg("-o").arg("");
        

    }  

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage_message = "Usage: comps {action} {file} {compiler arguments}";

    let source: String;
    let compiled_file: String;
    let compiler_arguments: String;
    let action: String;

    if args.len() == 1 {
        Command::new("clear");
        println!(r#"
  ____  ____   _____ ______  ______
_/ ___\/  _ \ /     \\____ \/  ___/
\  \__(  <_> )  Y Y  \  |_> >___ \ 
 \___  >____/|__|_|  /   __/____  >
     \/            \/|__|       \/ 
     Welcome to comps!
"#);


    }
    
    else if args.len() >= 2 {
        match args[1].as_str() {
            "c" | "compile" | "build" => {
                println!("compiling");
                compile(false);
                //TODO: Make this not call fucntion and make the action var it and then have a differnt function that sees all the args and parses to the compile function

            },
            "r" | "run" => {
                println!("compiling + running");
                compile(true);
            }
            _ => {
                println!("{}", usage_message);
                process::exit(0);
            }
        }
        
    }

    println!("{:?}", args.len());
    println!("{:?}", args)
}
