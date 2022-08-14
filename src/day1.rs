use itertools::Itertools;

pub fn day1p1(inputs: Vec<i32>) -> Option<i32> {
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

pub fn day1p1_2(mut inputs: Vec<i32>) -> i32 {
    inputs.drain(..).combinations_with_replacement(2)
        .find(|v| v.iter().sum::<i32>() == 2020 )
        .unwrap().iter().product::<i32>()
}


#[cfg(test)]
mod tests {
    use crate::day1::{day1p1};
    #[test]
    fn day1() {
        let inputs = [1721, 979, 366, 299, 675, 1456].to_vec();
        assert_eq!(day1p1(inputs).unwrap(), 514579);
    }
}

