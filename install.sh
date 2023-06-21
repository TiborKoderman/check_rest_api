#!/bin/bash

cargo build --release
rm -f /usr/local/nagios/libexec/check_rest_api
cp target/release/check_rest_api /usr/local/nagios/libexec/check_rest_api