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

  // part 1
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

  // part 2
  println!("Part 2\r\n{}", "-".repeat(10));
  println!(
    "{}",
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

  Ok(())
}