use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn main() -> Result<(), io::Error> {
  // Read from file to vec
  // Get inputs
  let path = "./inputs/day1.txt";
  let input = File::open(path)?;
  let buffered = BufReader::new(input);
  
  // Create vec
  let mut measurements: Vec<i32> = Vec::new();

  // Read file to vec 
  for line in buffered.lines() {
      measurements.push(line.unwrap().parse::<i32>().unwrap());
  }

  println!("Day 1\r\n{}", "-".repeat(5));
  // day 1, part 1
  println!("Part 1\r\n{}", "-".repeat(10));
  println!(
    "{}\r\n",
    measurements
      // create a two-measurement sliding window
      .windows(2)
      // collect all values where the measurement increased from one measurement to the next
      .map(|value| { value[1] > value[0] })
      .filter(|&increased| increased)
      // count those values
      .count()
  );

  // day 1, part 2
  println!("Part 2\r\n{}", "-".repeat(10));
  println!(
    "{}\r\n",
    measurements
      // create a three-measurement sliding window to collect sums of 3 subsequent measurements
      .windows(3)
      // accumulate the sum within that window
      .map(|sample| { sample.iter().sum() })
      .collect::<Vec<i32>>()
      // now create a two-measurement sliding window to compare sums
      .windows(2)
      // apply the same comparison and collection as part 1
      .map(|value| { value[1] > value[0] })
      .filter(|&increased| increased)
      .count()
  );

  // part 2, part 1
  let path = "./inputs/day2.txt";
  let input = File::open(path)?;
  let buffered = BufReader::new(input);
  let lines: Vec<String> = buffered.lines().flatten().collect();

  let mut h0: i32 = 0;
  let mut d0: i32 = 0;

  lines.iter().for_each(|line| {
      let mut command = line.split_whitespace();
      let direction = command.next().unwrap();
      let magnitude = command.next().unwrap().parse::<i32>().unwrap();
      match direction {
          "forward" => h0 += magnitude,
          "up" => d0 -= magnitude,
          "down" => d0 += magnitude,
          _ =>  println!("Throw the switch Vern, she's pumping mud")
      }
  });

  println!("Day 2\r\n{}", "-".repeat(5));
  println!("\rPart 1\r\n{}", "-".repeat(10));
  println!("Horizontal: {}\tDepth: {}\tProduct: {}\r\n", h0, d0, h0 * d0);


  let mut aim: i32 = 0;
  let mut h1: i32 = 0;
  let mut d1: i32 = 0;

  lines.iter().for_each(|line| {
      let mut command = line.split_whitespace();
      let direction = command.next().unwrap();
      let magnitude = command.next().unwrap().parse::<i32>().unwrap();
      match direction {
          "forward" => {h1 += magnitude;  d1 += aim * magnitude},
          "up" => aim -= magnitude,
          "down" => aim += magnitude,
          _ =>  println!("Throw the switch Vern, she's pumping mud")
      }
  });

  println!("Part 2\r\n{}", "-".repeat(10));
  println!("Horizontal: {}\tDepth: {}\tProduct: {}\r\n", h1, d1, h1 * d1);


  Ok(())

}