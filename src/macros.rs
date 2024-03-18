// #[macro_export]
// macro_rules! printstyle {
//     ($style:expr, $($arg:tt)*) => {
//         let styles: Vec<&str> = $style.split(',').collect();
//         let mut combined_styles = String::new();
//         for s in styles {
//             match s.trim() {
//                 "red" => combined_styles.push_str("\x1b[31m"),
//                 "green" => combined_styles.push_str("\x1b[32m"),
//                 "blue" => combined_styles.push_str("\x1b[34m"),
//                 "magenta" => combined_styles.push_str("\x1b[35m"),
//                 "yellow" => combined_styles.push_str("\x1b[33m"),
//                 "cyan" => combined_styles.push_str("\x1b[36m"),
//                 "bold" => combined_styles.push_str("\x1b[1m"),
//                 "underline" => combined_styles.push_str("\x1b[4m"),
//                 "italic" => combined_styles.push_str("\x1b[3m"),
//                 _ => (),
//             }
//         }

//         print!("{}{}\x1b[0m", combined_styles, format_args!($($arg)*));
//     };
// }

// #[macro_export]
// macro_rules! printstyleln {
//     ($style:expr, $($arg:tt)*) => {
//         printstyle!($style, $($arg)*);
//         println!();
//     };
// }

#[macro_export]
macro_rules! style {
    ($style:expr, $($arg:tt)*) => {
        {
            let styles: Vec<&str> = $style.split(',').collect();
            let mut combined_styles = String::new();

            for s in styles {
                match s.trim() {
                    "black" => combined_styles.push_str("\x1b[30m"),
                    "red" => combined_styles.push_str("\x1b[31m"),
                    "green" => combined_styles.push_str("\x1b[32m"),
                    "yellow" => combined_styles.push_str("\x1b[33m"),
                    "blue" => combined_styles.push_str("\x1b[34m"),
                    "magenta" => combined_styles.push_str("\x1b[35m"),
                    "cyan" => combined_styles.push_str("\x1b[36m"),
                    "white" => combined_styles.push_str("\x1b[37m"),
                    "default" => combined_styles.push_str("\x1b[39m"),
                    "bold" => combined_styles.push_str("\x1b[1m"),
                    "dim" => combined_styles.push_str("\x1b[2m"),
                    "italic" => combined_styles.push_str("\x1b[3m"),
                    "underline" => combined_styles.push_str("\x1b[4m"),
                    "blink" => combined_styles.push_str("\x1b[5m"),
                    "reverse" => combined_styles.push_str("\x1b[7m"),
                    "hide" => combined_styles.push_str("\x1b[8m"),
                    _ => (),
                }
            }

            format!("{}{}\x1b[0m", combined_styles, format_args!($($arg)*))
        }
    };
}
