#!/bin/bash

# Check if the number of arguments is correct
if [ $# -ne 1 ]; then
    echo "Usage: $0 <n>"
    exit 1
fi

# Run the command n times
for ((i = 1; i <= $1; i++))
do
    cargo run -- add "Todo $i"
done

cargo run -- list
