use std::{env::args, process::ExitCode, fs::File, io::Read};

struct ArgValues {
    hostname: Option<String>,
    username: Option<String>,
    password: Option<String>,
    keys: Vec<String>,
    number_of_keys: i32,
    warning_max: Option<f64>,
    warning_min: Option<f64>,
    warning_inclusive: Option<i32>,
    critical_max: Option<f64>,
    critical_min: f64,
    critical_inclusive: Option<i32>,
    timeout: i64,
    insecure_ssl: i32,
    header: Option<String>,
    debug: i32,
    http_method: i32,
}

pub fn validate_arguments() -> bool {
    let mut arg_vals = ArgValues {
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

    // require arguments
    if args().len() == 1 {
        print!("Arguments Required!\n\n {}", HELP);
        return false;
    }

    let mut lastArgHadNoInput = false;

    for mut i in (1..args().len()).step_by(2) {
        if i < args().len() || (i == args().len() && lastArgHadNoInput) {
            // For --insecure and the like that have no nextArg
            if (lastArgHadNoInput) {
                i = i - 1;
                lastArgHadNoInput = false;
            }

            let mut arg = args().nth(i).unwrap();
            let mut nextArg = args().nth(i + 1).unwrap();

            //remove '=' from argument if it exists
            arg = arg.replace("=", " ");

            //help message
            if arg == "--help" || arg == "-h" {
                format!("{}", VERSION);
                format!("{}", HELP);
                return false;
            }

            // Version 
            if arg == "--version" || arg == "-V" {
                format!("{}", VERSION);
                return false;
            }

            // Basic Auth - CLI
            if arg == "--auth-basic" || arg == "-b" {
              if nextArg.starts_with("-") {
                format!("Invalid value for -b, --auth-basic. Must be a string of <username>:<password>\n\n{}", HELP);
                return false;
              }

              let mut auth = nextArg.split(":");

              let username = auth.next().unwrap();
              let password = auth.next().unwrap();

              if password == "" {
                format!("Invalid value for -b, --auth-basic. Must be a string of <username>:<password>\n\n{}", HELP);
                return false;
              }

              arg_vals.username = Some(username.to_string());
              arg_vals.password = Some(password.to_string());

              continue;
            }

            //Basic Auth - File
            if arg == "--auth-basic-file" || arg == "-bf" {
              if nextArg.starts_with("-") {
                format!("Invalid value for -bf, --auth-basic-file. Must be a file path. \n\n{}", HELP);
                return false;
              }

              //open file if it exists
              
              let mut file = match File::open(nextArg.clone()) {
                Ok(file) => file,
                Err(_) => {
                  format!("Invalid value for -bf, --auth-basic-file. File does not exist. \n\n{}", HELP);
                  return false;
                }
              };

              //read the first line of the file
              let mut contents = String::new();
              file.read_to_string(&mut contents).unwrap();

              drop(file);
              //split the line into username and password
              let mut auth = contents.split(":");
              let username = auth.next().unwrap();
              let password = auth.next().unwrap();

              if password == "" {
                format!("Bad data in file '%s'. Verify the file has only one line and contains only '<username>:<password>'\n\n{}", HELP);
                return false;
              }
              
              arg_vals.username = Some(username.to_string());
              arg_vals.password = Some(password.to_string());        
        }

        // Hostname
        if arg == "--hostname" || arg == "-H" {
          if nextArg.starts_with("-") {
            format!("Invalid value for -H, --hostname. Must be an IP address or URL\n\n{}", HELP);
            return false;
          }

          arg_vals.hostname = Some(nextArg.to_string());
          continue;
        }

        // Optional HTTP header
        if arg == "--header" || arg == "-D" {
          arg_vals.header = Some(nextArg.to_string());
          continue;
        }

        // Optional Debug output
        if arg == "--debug" || arg == "-d" {
          arg_vals.debug = 1;
          continue;
        }

        // JSON Key
        if arg == "--key" || arg == "-K" {
          if nextArg.starts_with("-") {
            format!("Invalid value for -k, --key. This must be a comma-delimited list of strings.\n\n{}", HELP);
            return false;
          }
          for key in nextArg.split(",") {
            arg_vals.keys.push(key.to_string());
            arg_vals.number_of_keys += 1;
          }
        }

        // Warning threshold
        if arg == "--warning" || arg == "-w" {
          if nextArg.starts_with("-") {
            format!("Invalid value for -w, --warning. See Nagios Plugin documentation.\n\n{}", HELP);
            return false;
          }

          // if (!parseWarningOrCriticalValues(nextArg, 'w')) {
          //   return 0;
          //    }
          // return true;
        }

        // Timeout
        if arg == "--timeout" || arg == "-t" {
          if nextArg.starts_with("-") {
            format!("Invalid value for -t, --timeout. Must be an integer.\n\n{}", HELP);
            return false;
          }

          let timeout = nextArg.parse::<i64>().unwrap();

          if timeout < 1 {
            format!("Invalid value for -t, --timeout. Must be an integer greater than 0.\n\n{}", HELP);
            return false;
          }

          arg_vals.timeout = timeout;
          continue;
        }

        // Insecure SSL
        if arg == "--insecure" || arg == "-k" {
          arg_vals.insecure_ssl = 1;
          lastArgHadNoInput = true;
          continue;
        }

        // HTTP Method
        if arg == "--http-method" || arg == "-m" {
          if nextArg.starts_with("-") {
            format!("Invalid value for -m, --http-method. Must be a valid HTTP method.\n\n{}", HELP);
            return false;
          }

          nextArg = nextArg.to_uppercase();

          if nextArg == "GET" {
            arg_vals.http_method = 0;
          }
          else if nextArg == "POST" {
            arg_vals.http_method = 1;
          }
          else if nextArg == "put"{
            arg_vals.http_method = 2;
          }
          else {
            format!("Invalid value for -m, --http-method. Must be a valid HTTP method.\n\n{}", HELP);
            return false;
          }

          continue;
        }

        // Bad argument
        format!("Invalid argument: {}\n\n{}", arg, HELP);
        return false;
    }
  }

    return true;

  }
  
  //TODO: implementation of validateArguments function










const VERSION: &str = "0.1.0";

const HELP: &str = "Usage: check_rest_api [OPTIONS..]\n\nOptions:\n\
-h, --help\n\
  Print detailed help screen\n\
-V, --version\n\
  Print Version information\n\
-b, --auth-basic <username>:<password>\n\
  Uses HTTP Basic authentication with provided <username> and <password>\n\
-bf, --auth-basic-file <filename> \n\
  Uses HTTP Basic authentication and takes the required 'username:password' from the file provided\n\
  This file should only have one line that contains text in the format '<username>:<password>' (Excluding quotes)\n\
-H, --hostname HOSTNAME\n\
  The hostname, or IP address of the API you want to check\n\
-m, --http-method METHOD\n\
  Optional; the HTTP method for the API call. Only 'GET', 'POST', and 'PUT' are supported at this time.\n\
  If omitted 'GET' is assumed.\n\
-K, --key jsonKey\n\
  Optional; a comma delimited list of JSON keys to check. The value of this key must be a number\n\
   If not provided, check_rest_api will check the HTTP status code. Anything < 400 will return OK,\n\
   Anthing >=400 and < 500 will return WARNING, and >= 500 will return CRITICAL.\n\
-w, --warning warningThreshold\n\
  Optional; a comma delimited list of WARNING thresholds that map to the corresponding -K, --key (JSON key)\n\
    Returns WARNING if the corresponding --key is outside the defined -w range\n\
-c, --critical criticalThreshold\n\
  Optiona; a comma delimited list of CRITICAL thresholds that map to the corresponding -K, --key (JSON key)\n\
    Returns CRITICAL if the corresponding --key is outisde the defined -c range\n\
-t, --timeout timeoutValue\n\
  Optional, seconds before connection times out (default: 10 seconds)\n\
-D, --header\n\
  Optional HTTP Header(s)\n\
-d, --debug\n\
  Enable trace mode for CURL communication\n\
-k, --insecure\n\
  Disables checking peer's SSL certificate (if using SSL/HTTPS). Not recommended to use\n\
\nReport Bugs to: teeterwyatt@gmail.com\n";