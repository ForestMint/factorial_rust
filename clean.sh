#!/bin/bash

# this file is used to remove the generated folders and files

rm report.html output.xml log.html Cargo.lock __pycache__/*
rm -rf target
rmdir __pycache__