#![allow(unused)]
use std::{
    future::Pending,
    io::{self, Write},
};

fn line_to_string(val: i32, fill: &str) -> String {
    let header = "*## ";
    let follower = " ##*";
    let amt_darkness = "        09 HOURS OF DARKNESS TODAY         ";

    fn create_stars(num: i32, fill: &str) -> String {
        fill.repeat(num.try_into().unwrap())
    }

    match val {
        0 | 20 => String::from("*** ### ### ***"),
        1 | 19 => format!("{}{}{}", header, create_stars(15, fill), follower),
        2 | 18 => format!("{}{}{}", header, create_stars(23, fill), follower),
        3 | 17 => format!("{}{}{}", header, create_stars(29, fill), follower),
        4 | 16 => format!("{}{}{}", header, create_stars(33, fill), follower),
        5 | 15 => format!("{}{}{}", header, create_stars(37, fill), follower),
        6 | 14 => format!("{}{}{}", header, create_stars(39, fill), follower),
        7 | 13 => format!("{}{}{}", header, create_stars(41, fill), follower),
        8 | 9 | 11 | 12 => format!("{}{}{}", header, create_stars(43, fill), follower),
        10 => format!("{}{}{}", header, amt_darkness, follower),
        _ => String::from(""),
    }
}

fn main() {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());

    for i in 0..21 {
        let main_string = line_to_string(i, "*");
        let main_string_length = main_string.chars().count();
        let num_spaces = (51 - main_string_length) / 2;
        let spaces = (0..num_spaces).map(|_| " ").collect::<String>();
        writeln!(handle, "{}", format!("{}{}", spaces, main_string));
    }
}
