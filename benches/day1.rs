#[macro_use] extern crate bencher;

use bencher::Bencher;
use aoc2020::{get_inputs, day1};

fn day1p1b1(bench: &mut Bencher) {
    let inputs: Vec<i32> = get_inputs("day1");

    bench.iter(|
        | {
        day1::day1p1(inputs.to_owned()).unwrap()
    });
}

fn day1p1b2(bench: &mut Bencher) {
    let inputs: Vec<i32> = get_inputs("day1");

    bench.iter(|
        | {
        day1::day1p1_2(inputs.to_owned())
    });
}

benchmark_group!(benches, day1p1b1, day1p1b2);
benchmark_main!(benches);
