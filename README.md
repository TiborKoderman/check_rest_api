# check_rest_api
Rework of the check_rest_api Nagios plugin, to make it work on new systems, written in rust

## Dependancies
`rustc`, `rust` and `cargo`

### Debian systems
```sh
sudo apt install rust rustc cargo
```

### Other
[Rust installation documentation](https://forge.rust-lang.org/infra/other-installation-methods.html)

## Install program
Clone repository
```sh
git clone https://github.com/TiborKoderman/check_rest_api
```

CD into repository
```sh
cd check_rest_api
```

run `./install.sh`

```sh
chmod +x install.sh
./install.sh
```

## build program
```sh
cargo build --release
```

## Usage
copy to the `libexec` directory of the nagios instalation, usually in `/usr/local/nagios/libexec`

## Configuration example
```
define service{
        use                     generic-service
        host_name               name
        service_description     System Info
        check_command           check_rest_api!http://$HOSTADDRESS$/api/nagios/system/!cpu,ram!\~:80,\~:85!\~:100,\~:95
}
```

## Standalone command line
The below usage assumes you are running the program stand-alone on a command line.
Usage for a Nagios Command will be nearly identical.
| &nbsp; &nbsp;Name/Option &nbsp; &nbsp;  | Shorthand | Description  |
|---|:-:|--|
| `--hostname` | `-H` | Full address to the REST API endpoint |
| `--auth-basic` | `-b` | A string in the form `<username>:<password>` that is used for HTTP Basic Auth |
| `--auth-basic-file` | `-bf` | Filepath to a file that contains one line in the format `<username>:<password>` that is used for HTTP Basic Auth
| `--key` | `-K` | A comma-delimited list of JSON keys to check. More detail on accessing JSON keys are below |
| `--critical` | `-c` | A comma-delimited list of 'critical' value criteria. Each entry corresponds to a `--key` entry. See Nagios Plugin documentation on critical values |
| `--warning` | `-w` | A comma-delimited list of 'warning' value criteria. Each entry corresponds to a `--key` entry. See Nagios Plugin documentation on warning values |
| `--timeout` | `-t` | Sets a custom timeout, in seconds, to abort the HTTP request should the remote server not respond. Defaults to `10` seconds. |
| `--debug` | `-d` | Enable trace mode to see low level data flow in ASCII and HEXADECIMAL representation. |
| `--header` | `-D` | A string to be added into HTTP request header. |
| `--insecure` | `-k` | Disables checking peer's SSL certificate (if using SSL/HTTPS). Not recommended to use |
| `--http-method` | `-m` | Optional; the HTTP method for the API call. Only 'GET', 'POST', and 'PUT' are supported at this time. If omitted 'GET' is assumed. | 

### Example Usage
`Note`: Tildes (~) are escaped here as a normal BaSH will expand that to the users home directory. You needn't escape a tilde when writing a custom Nagios Command. 
```bash
# Check an API endpoint and send a 'critical' value if 
# the JSON key `cpu` is above `80`
./check_rest_api -H http://www.contoso.com/api/endpoint -K cpu -c \~:80

# Check an API endpoint and send a 'warning' value if the 
# JSON key `files` is less than `23`
./check_rest_api -H http://www.contoso.com/api/endpoint2 -K files 23:

# Check an API endpoint and send a 'warning' if the JSON key `ram` is above `75` 
# and a 'critical' if it is above `80`
./check_rest_api -H http://www.contoso.com/api/endpoint3 -K ram -w \~:75 -c \~:80
  
# Check an API endpoint. Send a 'warning' if `cpu` is above 
# 60 or `ram` is above `63`, and a 'critical' if `cpu` 
 #is above `70` or `ram` is above `83`
./check_rest_api -H http://www.contoso.com/api/endpoint4 -K cpu,ram -w \~:60,\~:63 -c \~:70,\~:83

# Check an API endpoint with HTTP Basic Auth (via CLI)
./check_rest_api -H http://www.contoso.com --auth-basic username:password

# Check an API endpoint with HTTP Basic Auth (with file). The file ./test has one line with the string "username:password" to use for HTTP Basic Auth
./check_rest_api -H http://www.contoso.com --auth-basic-file ./test
```  
