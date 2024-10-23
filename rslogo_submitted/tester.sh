#!/bin/bash

# Simple bash script to autotest 6991 rslogo based on given examples
# - Takes a couple seconds to run through all examples on cse
# - If program doesn't respond for longer than 10-20 sec, your solution
# might've run into an inf loop or something along those lines
# - Expects filenames to be unchanged in logo_examples for simplicity sake

# Change these paths as needed
# Source lg files
TEST_DIR="./logo_examples"

# Output svg files
OUTPUT_DIR="./test_output"

# Check if the target is not a directory
if [ ! -d "$TEST_DIR" ]; then
    exit 1
fi

# Check if a
if [ ! -d "$OUTPUT_DIR" ]; then
    mkdir test_output
else
    rm ./test_output/*
fi

>autotest_result.txt
>reference_output.log
>output.log
>diff_res.txt

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Loop through files in the target directory
for FILE in "$TEST_DIR"/*; do
    if [ -f "$FILE" ]; then
        EXPECTED=./test_output/${FILE:16:4}_expected.svg
        OUTPUT=./test_output/${FILE:16:4}_output.svg
        printf "\n========= Testing output for ${FILE} ========" >>reference_output.log
        printf "\n======== Testing output for ${FILE} ========" >>output.log
        # Run the reference solution, output any errors to log file
        6991 rslogo "$FILE" $EXPECTED 200 200 &>>reference_output.log # ALREADY DOWNLOADED RESULTS!!!!!

        # Run your solution, output any errors to log file
        6991 cargo run -- "$FILE" $OUTPUT 200 200 &>>output.log # DELETE 6991
        if [ -f "$OUTPUT" ] && [ -f "$EXPECTED" ]; then
            RES="$(diff $EXPECTED $OUTPUT)"
            if [ -z "$RES" ]; then
                printf "${GREEN}TEST PASSED${NC} $FILE\n"
            else
                printf "${RED}TEST FAILED${NC} $FILE: use 'diff $OUTPUT $EXPECTED' to see what differed\n"
            fi
        elif [ ! -f "$OUTPUT" ] && [ ! -f "$EXPECTED" ]; then
            printf "${GREEN}TEST PASSED${NC} $FILE no image file produced by both\n"
        elif [ -f "$OUTPUT" ] || [ -f "$EXPECTED" ]; then
            printf "${RED}TEST FAILED${NC} $FILE extra/no image file produced\n"
        fi
    fi
done
