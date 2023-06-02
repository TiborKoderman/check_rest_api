use curl::easy::{Easy, List};
use serde_json::Value;
use std::{
    env,
    io::{self, stdout, Write},
    process::ExitCode,
    str,
    time::Duration,
};

mod read_input;
use read_input::*;

enum Status {
    Ok,
    Warning,
    Critical,
    Unknown,
}

fn main() {
    let mut args = read_input::ArgValues {
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

    let (curl_res, curl_res_code) = call_curl(args);

    //parse json
    let v: Value = serde_json::from_str(&curl_res).unwrap(); 



    println!("{curl_res}");

}

fn call_curl(args: ArgValues) -> (String, u32) {
    let mut easy = Easy::new();

    //set url
    if let Some(hostname) = args.hostname {
        easy.url(&hostname).unwrap();
    }
    //set timeout
    easy.timeout(Duration::new(args.timeout.try_into().unwrap(), 0))
        .unwrap();

    //if username and password are provided use basic auth
    if let Some(username) = args.username {
        if let Some(password) = args.password {
            easy.username(&username).unwrap();
            easy.password(&password).unwrap();
        }
    }

    //if header is set add it to the request
    let mut headers = List::new();
    if let Some(header) = args.header {
        headers.append(&header).unwrap();
        easy.http_headers(headers).unwrap();
    }

    //if insecure ssl is set
    if args.insecure_ssl == 1 {
        easy.ssl_verify_host(false).unwrap();
        easy.ssl_verify_peer(false).unwrap();
    }

    //if http method is set
    if args.http_method == 1 {
        easy.post(true).unwrap();
        //Http server will allow POST requests with no parameters
        easy.post_field_size(0).unwrap();
    } else if args.http_method == 2 {
        easy.custom_request("PUT").unwrap();
    }

    // let mut transfer = easy.transfer();

    // let mut dst = Vec::new();

    let mut response = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            response.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    let response_str = String::from_utf8(response).unwrap();
    // print!("{}", response_str);
    return (response_str, easy.response_code().unwrap());
}
