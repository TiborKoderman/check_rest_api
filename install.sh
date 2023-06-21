#!/bin/bash

cargo build --release
cp target/release/check_rest_api /usr/local/nagios/libexec/check_rest_api