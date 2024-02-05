use std::collections::hash_map::DefaultHasher;
use std::env;
use std::env::args;
use std::fs;
use std::hash::{Hash, Hasher};
use std::process::Command;

fn show_help() {
    println!("USAGE:\n   rustcr INPUT_FILE : compile and run INPUT_FILE\n   rustcr --help/-h :  show this message");
}

fn compile(input_file: &str) -> Result<String, String> {
    let home_dir = env::var("HOME");
    match &home_dir {
        Ok(home) => {
            fs::create_dir_all(format!("{}/.cache/rustcr", { &home }))
                .expect("Could not create \"~/.cache/rustcr\" directory");
        }
        Err(err) => println!("Impossible to get your home dir! {}", err),
    }

    let mut hasher = DefaultHasher::new();
    input_file.hash(&mut hasher);
    let output_file = format!(
        "{}/.cache/rustcr/rustcr_{}",
        home_dir.unwrap(),
        hasher.finish()
    );

    match Command::new("rustc")
        .arg("-o")
        .arg(&output_file)
        .arg(input_file)
        .status()
    {
        Ok(compilation_status) => {
            if compilation_status.success() {
                return Ok(output_file);
            } else {
                return Err(String::from("Could not compile."));
            }
        }
        Err(err) => {
            return Err(format!("Could not compile. {}", err));
        }
    }
}

fn run(bin: String) -> Result<(), String> {
    if !Command::new(&bin)
        .status()
        .expect("Failed to execute binary")
        .success()
    {
        return Err(String::from("Could not execute  binary"));
    }

    match fs::remove_file(bin) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Could not clean cache. {}", err)),
    }
}

fn clean() -> Result<(), String> {
    println!("Cleanning cache directory ...");

    let home_dir = env::var("HOME");
    match &home_dir {
        Ok(home) => {
            let _ = fs::remove_dir_all(format!("{}/.cache/rustcr", { &home }));
            println!("... done.");
            Ok(())
        }
        Err(err) => Err(format!("Impossible to get your home dir! {}", err)),
    }
}

fn main() -> Result<(), String> {
    let arg_list: Vec<_> = args().collect();

    match arg_list.len() {
        1 => {
            show_help();
            Err(String::from("No input file was provided."))
        }
        _ => {
            if arg_list[1] == "--help" || arg_list[1] == "-h" {
                show_help();
                return Ok(());
            } else if arg_list[1] == "clean" {
                match clean() {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            } else {
                let bin = compile(&arg_list[1])?;
                run(bin)
            }
        }
    }
}
