use console::Style;
use std::io::{self, Write};

pub struct Dark {
    pub value: u32,
    pub as_string: String,
}

fn line_to_string(darkness: &Dark, val: i32, fill: &str) -> String {
    let pre_repeat = match darkness.value {
        d if d >= 10 => 7,
        _ => 8,
    };

    let amt_darkness = format!(
        "{}{}{}{}",
        " ".repeat(pre_repeat),
        darkness.as_string,
        " HOURS OF DARKNESS TODAY",
        " ".repeat(8)
    );

    let create_stars = |num: i32| fill.repeat(num.try_into().unwrap());

    match val {
        0 | 20 => String::from("*** ***"),
        1 | 19 => create_stars(15),
        2 | 18 => create_stars(23),
        3 | 17 => create_stars(29),
        4 | 16 => create_stars(33),
        5 | 15 => create_stars(37),
        6 | 14 => create_stars(39),
        7 | 13 => create_stars(41),
        8 | 9 | 11 | 12 => create_stars(43),
        10 => amt_darkness,
        _ => String::from(""),
    }
}

pub fn draw_output(darkness: Dark) {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());
    // let hours_of_darkness = 16;

    let percent_darkness = darkness.value as f64 / 24.;
    let lines_darkness = (20. * percent_darkness).round() as i32;

    let dark = Style::new().color256(93);
    let text = Style::new().color256(204);
    let light = Style::new().color256(220);

    for i in 0..21 {
        let main_string = match i {
            i if i < lines_darkness => line_to_string(&darkness, i, "O"),
            _ => line_to_string(&darkness, i, "o"),
        };

        let main_string_length = main_string.chars().count();
        let num_spaces = (51 - main_string_length) / 2;
        let spaces = " ".repeat(num_spaces);

        match i {
            0 => writeln!(handle, "{}{}", spaces, dark.apply_to(main_string)),
            20 => writeln!(handle, "{}{}", spaces, light.apply_to(main_string)),
            10 => writeln!(
                handle,
                "{}",
                format!("{}{}", spaces, text.apply_to(main_string),)
            ),
            x if x > lines_darkness => writeln!(
                handle,
                "{}",
                format!("{}{}", spaces, light.apply_to(main_string),)
            ),
            _ => writeln!(
                handle,
                "{}",
                format!("{}{}", spaces, dark.apply_to(main_string),)
            ),
        };
    }
}
