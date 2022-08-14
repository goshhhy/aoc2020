use std::{
    io::{self, Read},
    fmt::Debug,
    str::FromStr,
    path::Path
};

pub fn get_inputs<T: FromStr>(name: &'static str) -> Vec<T> where <T as FromStr>::Err: Debug  {
    let path = Path::new("inputs").join(name);

    let string = std::fs::read_to_string( path )
        .expect("couldn't read input file");

    string.lines()
        .map(T::from_str)
        .collect::<Result<Vec<T>, _>>()
        .expect("couldn't read input values as correct type")
}
