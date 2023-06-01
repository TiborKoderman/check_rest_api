use curl::easy::Easy;
use serde_json::Value;
use std::{env, io::{self, Write},process::ExitCode, time::Duration,};

mod read_input;
use read_input::*;

enum Status {
    Ok,
    Warning,
    Critical,
    Unknown,
}



fn main() {
    let mut args  = read_input::ArgValues{
        hostname: None,
        username: None,
        password: None,
        keys: Vec::new(),
        number_of_keys: 0,
        warning_max: None,
        warning_min: None,
        warning_inclusive: None,
        critical_max: None,
        critical_min: 0.0,
        critical_inclusive: None,
        timeout: 10,
        insecure_ssl: 0,
        http_method: 0,
        debug: 0,
        header: None,
    };
    
    if !read_input::validate_arguments(&mut args) {
        std::process::exit(Status::Unknown as i32);
    }
    
    call_curl(args);
    println!("Hello, world!");
}

fn call_curl(args: ArgValues) {
    let mut easy = Easy::new();

    //set url
    if let Some(hostname) = args.hostname {
        easy.url(&hostname).unwrap();
    }
    //set timeout
    easy.timeout(Duration::new(args.timeout.try_into().unwrap(), 0)).unwrap();

    //if username and password are provided use basic auth
    if let Some(username) = args.username {
        if let Some(password) = args.password {
            easy.username(&username).unwrap();
            easy.password(&password).unwrap();
        }
    }
    


    let mut transfer = easy.transfer();


    transfer.write_function(|data| { // vrite function
        Ok(io::stdout().write(data).unwrap())
    }).unwrap();

    transfer.perform().unwrap();
    // Ok(());
}