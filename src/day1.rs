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

// same as day1p1 but using itertools and being more functional
pub fn day1p1_2(mut inputs: Vec<i32>) -> i32 {
    inputs.drain(..).combinations_with_replacement(2)
        .find(|v| v.iter().sum::<i32>() == 2020 )
        .unwrap().iter().product::<i32>()
}

pub fn day1p2(inputs: Vec<i32>) -> Option<i32> {
    for m in 0..inputs.len() {
        let s = &inputs[m..inputs.len()];
        for n in 0..s.len() {
            let s2 = &s[n..s.len()];
            for o in s2 {
                if s2[0] + s[0] + o == 2020 {
                    return Some(s2[0] * s[0] * o);
                }
            }
        }
    }
    None
}

// same as day1p2 but using itertools and being more functional
pub fn day1p2_2(mut inputs: Vec<i32>) -> i32 {
    inputs.drain(..).combinations_with_replacement(3)
        .find(|v| v.iter().sum::<i32>() == 2020 )
        .unwrap().iter().product::<i32>()
}


#[cfg(test)]
mod tests {
    use crate::day1::{day1p1, day1p2, day1p1_2, day1p2_2};
    #[test]
    fn day1() {
        let inputs = [1721, 979, 366, 299, 675, 1456].to_vec();
        assert_eq!(day1p1(inputs.to_owned()).unwrap(), 514579);
        assert_eq!(day1p1_2(inputs.to_owned()), 514579);
        assert_eq!(day1p2(inputs.to_owned()).unwrap(), 241861950);
        assert_eq!(day1p2_2(inputs.to_owned()), 241861950);
    }
}

