#!/bin/ash

echo "Starting mena-rust..."
/usr/bin/chromedriver &
sleep 5
/usr/sbin/mena-rust
