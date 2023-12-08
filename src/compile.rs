use std::process::Command;
use crate::ArgumentsStruct;
use std::env::consts::OS;

fn get_os() -> &'static str {
    match OS {
        "macos" | "windows" | "linux" => return OS,
        _ => return "null",
    }
}

pub fn compile(settings: ArgumentsStruct) {
    let current_os = get_os();
    println!("{}", settings.file_extension);
    if current_os == "linux" {
        if settings.file_extension == "cpp"{
            let gpp = Command::new("g++")
                .arg(settings.source_path)
                .spawn()
                .expect("Failed to execute gcc");
            if settings.run == true {
                let run = Command::new("./a.out").spawn();
            }
            // println!("stdout: {}", String::from_utf8_lossy(&gcc.stderr))
        }

    }


}

