
use aoc2020::get_inputs;
use aoc2020::day1::{day1p1, day1p2};

fn main() {
    let inputs = get_inputs("day1");
    println!( "day1p1 result: {}", day1p1(inputs.to_owned()).unwrap() );
    println!( "day1p2 result: {}", day1p2(inputs.to_owned()).unwrap() );
}