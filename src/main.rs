use clap::{command, Arg, ArgMatches, ArgAction};
mod compile;

struct ArgumentsStruct {
    run: bool,
    build: bool,
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


        .get_matches();
}

fn main() {
        let command = get_matches();

        let settings = ArgumentsStruct {
            run: command.get_flag("run"),
            build: command.get_flag("build")
        };
            

        if settings.run {
            println!("Runn is turee")
        }
        if settings.build {
            println!("build is turee")
        }
}
