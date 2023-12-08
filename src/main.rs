use clap::{command, Arg, ArgMatches, ArgAction};
use std::path::Path;
mod compile;


pub struct ArgumentsStruct {
    run: bool,
    build: bool,
    source_path: String,
    destination_path: String,
    file_extension: String
}

fn get_matches() -> ArgMatches {
    return command!()
        .arg(
            Arg::new("build")
                .num_args(0)
                .short('b')
                .long("build")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("run")
                .num_args(0)
                .short('r')
                .long("run")
                .action(clap::ArgAction::SetTrue)
                // .requires("build")

        )
        .arg(
            Arg::new("source_path")
                .help("The path to your .cpp source file.")
                .default_value("./main.cpp")
        )
        .arg(
            Arg::new("destination_path")
                .help("The path for the compiled executable.")
                .default_value("./main")    
        )


        .get_matches();
}

fn main() {
        let command = get_matches();

        let settings = ArgumentsStruct {
            run: command.get_flag("run"),
            build: command.get_flag("build"),
            source_path: command.get_one::<String>("source_path").unwrap().to_string(),
            destination_path: command.get_one::<String>("destination_path").unwrap().to_string(),
            file_extension: Path::new(&command.get_one::<String>("source_path").unwrap().to_string()).extension().unwrap().to_string_lossy().to_string(),
        };
            

        compile::compile(settings);
}
