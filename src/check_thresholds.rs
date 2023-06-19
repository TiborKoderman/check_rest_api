// use std::{env::args, fs::File, io::Read, process::ExitCode, clone};

use std::fmt::format;

use serde_json::Value;

use crate::read_input::ArgValues;

#[repr(u32)]
#[derive(PartialEq)]
pub enum Status {
    Ok = 0,
    Warning = 1,
    Critical = 2,
    Unknown = 3,
}

//implement copy trait for Status
impl Copy for Status {}

//implement clone trait for Status
impl Clone for Status {
    fn clone(&self) -> Status {
        *self
    }
}

pub fn checkHTTPStatusCode(httpResponseCode: u32) -> u32 {
    if httpResponseCode == 200 {
        format!("OK - Status Code = 200 | status_code=200\n");
        format!("HTTP Status Code: 200\n");
        return Status::Ok as u32;
    } else if httpResponseCode == 201 {
        format!("OK - Status Code = 201 | status_code=201\n");
        format!("HTTP Status Code 201 - Created\n");
        return Status::Ok as u32;
    }

    if httpResponseCode < 500 {
        format!(
            "WARNING - Status Code = {} | status_code={}\n",
            httpResponseCode, httpResponseCode
        );
        format!("Unexpected HTTP response code {}\n", httpResponseCode);
        return Status::Warning as u32;
    }

    if httpResponseCode > 500 {
        format!(
            "CRITICAL - Status Code = {} | status_code={}\n",
            httpResponseCode, httpResponseCode
        );
        format!("Unexpected HTTP reponse code {}\n", httpResponseCode);
        return Status::Critical as u32; //as u32;
    }

    format!(
        "UNKNOWN - Status Code = {} 
        | status_code={}\n",
        httpResponseCode, httpResponseCode
    );
    format!("Unknown HTTP status code {}\n", httpResponseCode);
    return Status::Unknown as u32;
}

pub fn jsonKeyToPrefDataKey(key: &mut String) {
    *key = key.to_lowercase().replace(" ", "_");
}

pub fn checkHTTPBody(arg_vals: ArgValues, json: Value) -> u32 {
    // let numberOfKeys: i32 = arg_vals.number_of_keys;
    // let mut keys: Vec<keyTreshCW> = arg_vals.keys;

    let mut severityLevel: Status = Status::Ok;
    let mut OKmessages: String = String::new();
    let mut WARNINGmessages: String = String::new();
    let mut CRITICALmessages: String = String::new();
    let mut UNKNOWNmessages: String = String::new();
    let mut LONGmessages: String = String::new();
    let mut FirstPrefMessage: String = String::new();
    let mut OtherPrefMessages: String = String::new();

    for key in arg_vals.keys.iter().clone() {
        //if key is not in json, return unknown
        if !json.is_object() {
            println!("UNKNOWN - JSON is not an object");
            return Status::Unknown as u32;
        }

        if !json.get(key.key.clone()).is_some() {
            println!("UNKNOWN - JSON does not contain key {}", key.key);
            return Status::Unknown as u32;
        }

        //Get the value of the key
        let value = json.get(key.key.clone()).unwrap().as_f64();

        // if value.is_none() {
        //     // Form the 'pretty' info string
        //     println!("UNKNOWN - Value of key {} is not a number", key.key);
        //     severityLevel = Status::Unknown;
        if !value.is_none() {
            let val = value.unwrap();
            let mut thisKeyStatus = Status::Ok;

            let message = format!("'{}'='{}', ", key.key, val.clone());

            //check if value is within thresholds
            if key.critical_min.is_some() {
                let (min, max, inclusive);

                min = key.critical_min.unwrap();
                max = key.critical_max.unwrap();
                inclusive = key.critical_inclusive;

                if inclusive {
                    if val <= min || val >= max {
                        // CRITICALmessages.push_str(&format!("{} is critical\n", key.key));
                        CRITICALmessages.push_str(&message);

                        if (severityLevel as u32) < (Status::Critical as u32) {
                            severityLevel = Status::Critical;
                        }
                        thisKeyStatus = Status::Critical;
                    }
                } else {
                    if val < min || val > max {
                        // CRITICALmessages.push_str(&format!("{} is critical\n", key.key));
                        CRITICALmessages.push_str(&message);

                        if (severityLevel as u32) < (Status::Critical as u32) {
                            severityLevel = Status::Critical;
                        }
                        thisKeyStatus = Status::Critical;
                    }
                }
            }

            if key.warning_min.is_some() && thisKeyStatus != Status::Critical {
                let (min, max, inclusive);
                min = key.warning_min.unwrap();
                max = key.warning_max.unwrap();
                inclusive = key.warning_inclusive;

                if inclusive {
                    if val <= min || val >= max {
                        // WARNINGmessages.push_str(&format!("{} is warning\n", key.key));
                        WARNINGmessages.push_str(&message);

                        if (severityLevel as u32) < (Status::Warning as u32) {
                            severityLevel = Status::Warning;
                        }
                        thisKeyStatus = Status::Warning;
                    }
                } else {
                    if val < min || val > max {
                        // WARNINGmessages.push_str(&format!("{} is warning\n", key.key));
                        WARNINGmessages.push_str(&message);

                        if (severityLevel as u32) < (Status::Warning as u32) {
                            severityLevel = Status::Warning;
                        }
                        thisKeyStatus = Status::Warning;
                    }
                }
            }

            //Set OK string if we never set WARNING or CRITICAL
            if thisKeyStatus == Status::Ok {
                // OKmessages.push_str(&format!("{} is ok\n", key.key));
                OKmessages.push_str(&message);
            }

            //Set long text description
            if thisKeyStatus == Status::Critical {
                LONGmessages.push_str(&format!("'{}' is '{}' (Critical)\n", key.key, val.clone()));
            } else if thisKeyStatus == Status::Warning {
                LONGmessages.push_str(&format!("'{}' is '{}' (Warning)\n", key.key, val.clone()));
            } else {
                LONGmessages.push_str(&format!("'{}' is '{}' (OK)\n", key.key, val.clone()));
            }

            //Set perf data
            //json key to pref data key
            let mut prefDataKey = key.key.clone();
            jsonKeyToPrefDataKey(&mut prefDataKey);

            let perfData = format!("'{}'={}\n", prefDataKey, val.clone());
            if FirstPrefMessage.is_empty() {
                FirstPrefMessage.push_str(&perfData);
            } else {
                OtherPrefMessages.push_str(&perfData);
            }
        } else {
            // object not found
            let message = format!("UNKNOWN - JSON key '{}' not found, ", key.key);
            UNKNOWNmessages.push_str(&message);

            LONGmessages.push_str(&message);

            severityLevel = Status::Unknown;
        }
    }
    //Print the overriding status
    match severityLevel {
        Status::Ok => print!("OK - "),
        Status::Warning => print!("WARNING - "),
        Status::Critical => print!("CRITICAL - "),
        Status::Unknown => print!("UNKNOWN - "),
    }

    //Print evrything in order of UNKNOWN, CRITICAL, WARNING, OK

    //unknown
    if UNKNOWNmessages.len() > 2
        && CRITICALmessages.len() == 0
        && WARNINGmessages.len() == 0
        && OKmessages.len() == 0
    {
        print!("{}", UNKNOWNmessages.split_off(UNKNOWNmessages.len() - 2));
    } else {
        print!("{}", UNKNOWNmessages);
    }

    //critical
    if CRITICALmessages.len() > 2 && WARNINGmessages.len() == 0 && OKmessages.len() == 0 {
        print!("{}", CRITICALmessages.split_off(CRITICALmessages.len() - 2));
    } else {
        print!("{}", CRITICALmessages);
    }

    //warning
    if WARNINGmessages.len() > 2 && OKmessages.len() == 0 {
        print!("{}", WARNINGmessages.split_off(WARNINGmessages.len() - 2));
    } else {
        print!("{}", WARNINGmessages);
    }

    //ok
    if OKmessages.len() > 2 {
        print!("{}", OKmessages.split_off(OKmessages.len() - 2));
    } else {
        print!("{}", OKmessages);
    }

    //Pref data - line 1
    if FirstPrefMessage.len() > 0 {
        print!(" | {}", FirstPrefMessage);
    } else {
        print!(" | \n");
    }

    //Long text descriptions
    if OtherPrefMessages.len() > 0 {
        print!("{} | ", LONGmessages);
    } else {
        print!("{}", LONGmessages);
    }

    print!("{}", OtherPrefMessages);
    return severityLevel as u32;
}
