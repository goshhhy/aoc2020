use bencher::Bencher;
use bencher::{benchmark_group, benchmark_main};
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
        day1::day1_f(inputs.to_owned(), 2)
    });
}

fn day1p2b1(bench: &mut Bencher) {
    let inputs: Vec<i32> = get_inputs("day1");

    bench.iter(|
        | {
        day1::day1p2(inputs.to_owned()).unwrap()
    });
}

fn day1p2b2(bench: &mut Bencher) {
    let inputs: Vec<i32> = get_inputs("day1");

    bench.iter(|
        | {
            day1::day1_f(inputs.to_owned(), 3)
    });
}

benchmark_group!(benches, day1p1b1, day1p1b2, day1p2b1, day1p2b2);
benchmark_main!(benches);
