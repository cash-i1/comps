use std::{process::{Command, exit}, process::Stdio, io::Write};
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
    if current_os == "linux" {
        if settings.file_extension == "cpp"{
            
            if settings.build == true {
                let mut gpp = Command::new("g++")
                    .arg(settings.source_path)
                    .spawn()
                    .expect("Failed to execute gcc");
                    
                
                let status = gpp.wait_with_output().unwrap();

                if !status.status.success() {
    
                    exit(1);
                }
                
            }

            if settings.run == true {
                std::io::stdout().flush().expect("Couldnt not");
                let mut run = Command::new("./a.out")

                    .spawn()
                    .unwrap();

                let _status = run.wait().unwrap();
                print!("\n");
            }


            else {
                println!("Format '{}' not supportted, exiting.", settings.file_extension);
                exit(1)
            }
            // println!("stdout: {}", String::from_utf8_lossy(&gcc.stderr))
        }

    }


}

