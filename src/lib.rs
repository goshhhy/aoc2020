use std::{
    io::{self, Read},
    fmt::Debug,
    str::FromStr,
};

pub fn get_inputs<T: FromStr>() -> Vec<T> where <T as FromStr>::Err: Debug  {
    let mut string = String::new();
    io::stdin().read_to_string(&mut string)
        .expect("couldn't read input values.");

    string.lines()
        .map(T::from_str)
        .collect::<Result<Vec<T>, _>>()
        .expect("couldn't read input values as correct type")
}
