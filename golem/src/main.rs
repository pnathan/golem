extern crate argparse;
extern crate rustc_serialize;

use argparse::{ArgumentParser,
               Collect,
               Store,
               StoreTrue};
use rustc_serialize::json;
use std::env;
use std::io::{self, Read};
use std::io::Write;
use std::process::Command;

#[derive(RustcDecodable, RustcEncodable)]
pub struct GolemResult  {
    stdout: String,
    stderr: String,
    code: i32
}

fn main() {
    let mut commands : Vec<String>= Vec::new();
    let mut daemon = false;
    let mut filename = "".to_string();
    let mut golem_input = false;
    let mut posix_output = false;
    let mut verbose = 0;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("golem. a servant for the magician");

        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], Store,
            "Be verbose");
        ap.refer(&mut daemon)
            .add_option(&["-d", "--daemon"], StoreTrue,
                        "Spawn task as a daemon");
        ap.refer(&mut filename)
            .add_option(&["-f", "--filename"], Store,
                        "Tee output to a file");
        ap.refer(&mut golem_input)
            .add_option(&["-i"], StoreTrue,
                        "Take stdin as coming from a golem process");
        ap.refer(&mut posix_output)
            .add_option(&["-P", "--posix-output"], StoreTrue,
                        "Output stderr and stdout to their traditional pipes");

        // Take the positionals as the actual command
        ap.refer(&mut commands)
            .add_argument("commands", Collect,
                        "commands to the golem");
        ap.parse_args_or_exit();
    }


    if verbose >= 1 && filename != ""{
        println!("teeing output to {}", filename);
    }

    if verbose >= 1 && daemon {
        println!("Spawning tasks as daemon");
    }

    let commands = commands;
    let (first, args) = commands.split_first().unwrap();
    let mut cmd = "~/.golem/".to_string() + first;

    if cmd.contains("~") {
        let dir = match env::home_dir() {
            Some(result) => { result },
            None => panic!("Impossible to get your home dir!")
        };


        cmd = cmd.replace("~", dir.to_str().unwrap());

    }

    if verbose >= 1 {
        println!("running {} w/ args {:?}", cmd, args);
    }

    if golem_input {
        let mut buffer = String::new();
        match io::stdin().read_to_string(&mut buffer) {
            Ok(result) => result,
            Error => 10
        };
        // fuss with something something something.
        // rust process api not all there yet.
    }

    let output = Command::new(cmd)
        .args(&args)
        .output()
        .unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });


    let object = GolemResult {
        code : output.status.code().unwrap(),
        stdout : String::from_utf8_lossy(&output.stdout).to_string(),
        stderr : String::from_utf8_lossy(&output.stderr).to_string()
    };


    if posix_output {
        print!("{}", object.stdout);
        let mut stderr = std::io::stderr();
        write!(&mut stderr, "{}", object.stderr).unwrap();
    }
    else {
        let encoded = json::encode(&object).unwrap();
        println!("{}", encoded);
    }
}
