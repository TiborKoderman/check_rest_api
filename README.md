# check_rest_api
Rework of the check_rest_api Nagios plugin, to make it work on new systems, written in rust
## Build
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

