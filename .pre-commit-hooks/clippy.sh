#!/bin/sh
cargo clippy -- -Dwarnings 2> /dev/null
clippy_exit_code=$?
if [ $clippy_exit_code != 0 ]; then
    echo -e "\e[31mYour code seems to contains style violations! Run cargo clippy before committing!\e[0m" ; exit 1
fi