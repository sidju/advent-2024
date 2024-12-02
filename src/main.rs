use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

fn day_one_part_one() {
  // Read the input
  let input = BufReader::new(File::open("input.txt").unwrap());
  // Parse it
  let mut first = Vec::new();
  let mut second = Vec::new();
  for line in input.lines().map(|x| x.unwrap()) {
    // Split by space, parse start and end as int
    let mut slices = line.split_whitespace();
    let first_num = slices.next().unwrap().parse::<isize>().unwrap();
    first.push(first_num);
    let second_num = slices.next().unwrap().parse::<isize>().unwrap();
    second.push(second_num);
  }
  first.sort();
  second.sort();
  println!("Parsed input lists:\n{first:?}\n{second:?}\n");
  // Do the calculation
  let mut distance = 0;
  for (one, other) in first.iter().zip(second.iter()) {
    let d = (one - other).abs();
    println!("Calculated difference between {one} and {other} is {d}");
    distance += d;
  }
  // Return the result
  println!("\nCalculated distance is: {distance}");
}

fn day_one_part_two() {
  // Read the input
  let input = BufReader::new(File::open("input.txt").unwrap());
  // Parse it
  let mut first = Vec::new();
  let mut second = HashMap::new();
  for line in input.lines().map(|x| x.unwrap()) {
    // Split by space, parse start and end as int
    let mut slices = line.split_whitespace();
    let first_num = slices.next().unwrap().parse::<isize>().unwrap();
    first.push(first_num);
    let second_num = slices.next().unwrap().parse::<isize>().unwrap();
    second.entry(second_num).and_modify(|counter| *counter += 1).or_insert(1);
  }
  println!("Parsed input:\n{first:?}\n{second:?}\n");
  // Do the calculation
  let mut score = 0;
  for num in first.iter() {
    let s = num * second.get(num).unwrap_or(&0);
    println!("Calculated for {num} is {s}");
    score += s;
  }
  // Return the result
  println!("\nCalculated score is: {score}");
}

fn main() {
  day_one_part_two()
}
