use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::iter::Enumerate;
use std::str::Chars;
use std::str::Lines;

fn main() {
    let input_file_location: String = String::from("C:\\Development\\aoc2023\\task1\\input.txt");
    let lines: Vec<String> = file_to_vector(input_file_location);
    let mut total: u32 = 0;
    for line in lines {
        total += number_from_line(line);
    }

    println!("{} is the result", total);
}

fn number_from_line(line: String) -> u32 {
    let values_to_replace = BTreeMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let values_lookup = [
        ("zero"),
        ("one"),
        ("two"),
        ("three"),
        ("four"),
        ("five"),
        ("six"),
        ("seven"),
        ("eight"),
        ("nine")
    ];

    let mut numbers: Vec<u32> = Vec::new();
    let char_vec: Enumerate<Chars<'_>> = line.chars().enumerate();
    for (pos, c) in char_vec {
        if c.is_numeric() {
            numbers.push(c.to_digit(10).unwrap());
        } else {
            for (key, value) in &values_to_replace {
                if (line[pos..].starts_with(key)) {
                    numbers.push(*value);
                }
            }
        }
    }
    /*
    The required format is interesting here. From the spec: 
    "On each line [of input], the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number."

    I didn't fancy going from u32 to String just to come back (which would've let me concat, which is somewhat easier) so instead the hack here is based on knowledge of the possible values.
    As only single digits are possible, we can:
    - Take the first seen value, multiply by 10 to put this into the first digit of a two digit combination.
    - We simply add the second digit to this to create a two digit number.

    Spec doesn't say what to do with the base case of "no digits on a line" so I've assumed this is a return of 0.
    Base case of only a single digit means we use it for both digits (this is clear from the spec examples).
    Inductive step is first + last and is covered off by the _ (default) match arm.
    */
    match numbers.len() {
        0 => return 0,
        1 => return numbers[0] * 10 + numbers[0],
        _ => numbers[0] * 10 + numbers[numbers.len() - 1],
    }
}

fn file_to_vector(file_path: String) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut line_vec: Vec<String> = Vec::new();
    let lines: Lines = contents.lines();
    for line in lines {
        line_vec.push(line.to_string());
    }

    return line_vec;
}
