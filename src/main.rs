use clap::{command, Arg};
mod compile;

fn main() {
    let command = command!()
        .arg(
            Arg::new("build")
                .num_args(0)
                .short('b')
                .long("build")
        )
        .get_matches();

        if command.contains_id("build") {
            println!("Building");
            compile::compile();
        } else {
            println!("else.");
        }

}
