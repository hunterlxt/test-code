mod result_option;
mod transmute;

use std::thread;

use clap::{App, Arg, SubCommand};

fn main() {
    macro_rules! veec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
    let v = veec![2, 3, 4];

    println!("{:?}", v);
}
