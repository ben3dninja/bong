#!/bin/bash

directory_name=${PWD##*/}
tmp_path="/mnt/c/Users/benja/rsync/$directory_name"

mkdir -p $tmp_path

rsync . $tmp_path -r --exclude-from=.gitignore

cd $tmp_path

powershell.exe -Command "cargo run $@"