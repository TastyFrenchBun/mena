#!/bin/bash
sleep 1

cd /home/container

/usr/bin/chromedriver &
sleep 5
/usr/bin/mena-rust