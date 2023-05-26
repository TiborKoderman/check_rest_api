use curl::easy::Easy;
use serde_json::Value;
use std::{env, io,process::ExitCode};

mod read_input;
use read_input::*;

enum STATUS {
    OK,
    WARNING,
    CRITICAL,
    UNKNOWN,
}

fn main() {
    let body: String;

    if !read_input::validate_arguments() {
        std::process::exit(STATUS::UNKNOWN as i32);
    }

    println!("Hello, world!");
}

