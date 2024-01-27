use base::prettyprint;
use toky::*;
use std::{io::{Read, Write}, process::Command, fs, path::Path};
pub mod base;

fn read(path: &str) -> String {
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

fn spawn_server() {
    let command = Command::new("python")
        .arg("main.py")
        .output();

    match command {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("{}", stdout);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Command failed with error: {}", stderr);
            }
        }
        Err(e) => {
            eprintln!("Error running command: {}", e);
        }
    }
}

fn main() {
    if !Path::new(".control").exists() {
        fs::create_dir(".control").unwrap();
    }
    loop {
        // question and getting tokens from the question
        let question = base::input("\u{001b}[31m>");

        // check for commands
        if question == "vasu" {
            let command = Command::new("viu")
                .arg("vasu.png")
                .output();

            match command {
                Ok(output) => {
                    if output.status.success() {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        println!("{}", stdout);
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        eprintln!("Command failed with error: {}", stderr);
                    }
                }
                Err(e) => {
                    eprintln!("Error running command: {}", e);
                }
            }

        } else if question == "exit" {
            prettyprint("See ya later, sucker!",10);
            break;
        } 
        // if the question is not a command then check for a response with the model        
        else {
            let tokens = get_tokens(question.as_str());
            let str_tokens = &tokens["str"];

            // reading the data file with all the keywords and responses
            let data = read("./datasets/general.txt");
            let lines: Vec<&str> = data.split("\n").collect();

            // getting response and displaying it
            let response = get_response_from_vec(&str_tokens,&lines,":?/:");
            let color_code = "\u{001b}[34m".to_owned();
            let new_string = color_code + response.as_str();


            // if the response is managable then just print the response
            if response != ":?/:" {
                prettyprint(&new_string, 10);
            }
            // else search the thing
            else {
                let mut file = std::fs::File::create(".control/query.tst").unwrap();
                file.write_all(question.as_str().as_bytes()).expect("write failed");
                spawn_server();

                let mut readfile = std::fs::File::open(".control/response.tst").unwrap();
                let mut response = String::new();
                readfile.read_to_string(&mut response).unwrap();

                prettyprint(&response, 10);
            }
        }
    }
}
