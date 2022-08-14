
use std::{
    io::{self, Read},
    str::FromStr,
};


fn day1p1(inputs: Vec<i32>) -> Option<i32> {
    for m in 0..inputs.len() {
        let s = &inputs[m..inputs.len()];
        for n in s {
            if s[0] + n == 2020 {
                return Some(s[0] * n);
            }
        }
    }
    None
}

fn main() {
    let mut string = String::new();
    io::stdin().read_to_string(&mut string)
        .expect("couldn't read input values.");

    let inputs = string
        .lines()
        .map(i32::from_str)
        .collect::<Result<Vec<_>, _>>().unwrap();

    println!( "{}", day1p1(inputs).unwrap() );
}

#[cfg(test)]
mod tests {
    use crate::day1p1;
    #[test]
    fn day1() {
        let inputs = [1721, 979, 366, 299, 675, 1456].to_vec();
        assert_eq!(day1p1(inputs).unwrap(), 514579);
    }
}