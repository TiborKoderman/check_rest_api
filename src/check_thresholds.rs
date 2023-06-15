use std::{env::args, fs::File, io::Read, process::ExitCode};

use serde_json::{json, Value};

use crate::read_input::{ArgValues, keyTreshCW};

pub enum Status {
    Ok,
    Warning,
    Critical,
    Unknown,
}

pub fn checkHTTPStatusCode(httpResponseCode: u32) -> Status {
    if httpResponseCode == 200 {
        format!("OK - Status Code = 200 | status_code=200\n");
        format!("HTTP Status Code: 200\n");
        return Status::Ok;
    } else if httpResponseCode == 201 {
        format!("OK - Status Code = 201 | status_code=201\n");
        format!("HTTP Status Code 201 - Created\n");
        return Status::Ok;
    }

    if httpResponseCode < 500 {
        format!(
            "WARNING - Status Code = {} | status_code={}\n",
            httpResponseCode, httpResponseCode
        );
        format!("Unexpected HTTP response code {}\n", httpResponseCode);
        return Status::Warning;
    }

    if httpResponseCode > 500 {
        format!(
            "CRITICAL - Status Code = {} | status_code={}\n",
            httpResponseCode, httpResponseCode
        );
        format!("Unexpected HTTP reponse code {}\n", httpResponseCode);
        return Status::Critical;
    }

    format!(
        "UNKNOWN - Status Code = {} | status_code={}\n",
        httpResponseCode, httpResponseCode
    );
    format!("Unknown HTTP status code {}\n", httpResponseCode);
    return Status::Unknown;
}

pub fn jsonKeyToPrefDataKey(key: &mut String) {
    *key = key.to_lowercase().replace(" ", "_");
}

pub fn checkHTTPBody(arg_vals: ArgValues, body: String) -> i32 {
    let numberOfKeys: i32 = arg_vals.number_of_keys;
    let mut keys: Vec<keyTreshCW> = arg_vals.keys;

    let mut severityLevel: Status = Status::Ok;
    let mut OKmessages: String = String::new();
    let mut WARNINGmessages: String = String::new();
    let mut CRITICALmessages: String = String::new();
    let mut UNKNOWNmessages: String = String::new();
    let mut LONGmessages: String = String::new();
    let mut FirstPrefMessage: String = String::new();
    let mut OtherPrefMessages: String = String::new();

    //check each key for validity
    


    return 0;
}
