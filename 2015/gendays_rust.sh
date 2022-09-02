#!/bin/bash
session=""

for i in {1..25}
do
    project=day$i
    cargo new $project
    curl -s "https://adventofcode.com/2015/day/${i}/input" --cookie "session=${session}" -o ${project}/input.txt
    echo -n 'fn main() {
    let data = include_str!("../input.txt");

    println!("Part 1: {}", "");
    println!("Part 2: {}", "");
}' > ${project}/src/main.rs
done