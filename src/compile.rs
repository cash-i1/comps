use std::{process::Command, process::Stdio, io::Write};
use crate::ArgumentsStruct;
use std::env::consts::OS;

fn get_os() -> &'static str {
    match OS {
        "macos" | "windows" | "linux" => return OS,
        _ => return "null",
    }
}

pub fn compile(settings: ArgumentsStruct) {
    std::io::stdout().flush().expect("Couldnt not");
    let current_os = get_os();
    println!("{}", settings.file_extension);
    if current_os == "linux" {
        if settings.file_extension == "cpp"{
            print!("\n");
            if settings.build == true {
                let gpp = Command::new("g++")
                    .arg(settings.source_path)
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("Failed to execute gcc");
            }
            if settings.run == true {
                std::io::stdout().flush().expect("Couldnt not");
                let run = Command::new("./a.out")
                    .stdout(Stdio::piped())
                    .spawn();
            }
            // println!("stdout: {}", String::from_utf8_lossy(&gcc.stderr))
        }

    }


}

