// use cli_clock::Digit::*;
use std::{thread, time::Duration};
use chrono::Local;
use cli_clock::BlockChar3By5;

fn main() {
    let filled_cell = ['█', '█'];
    let blank_cell = [' ', ' '];

    loop {
        let current_time = Local::now();
        let curr_h: [[[bool; 3]; 5]; 2] = current_time.format("%H").to_string()
            .chars().map(|c| c.char_grid())
            .collect::<Vec<_>>().try_into().unwrap();
        let curr_m: [[[bool; 3]; 5]; 2] = current_time.format("%M").to_string()
            .chars().map(|c| c.char_grid())
            .collect::<Vec<_>>().try_into().unwrap();
        let curr_s: [[[bool; 3]; 5]; 2] = current_time.format("%S").to_string()
            .chars().map(|c| c.char_grid())
            .collect::<Vec<_>>().try_into().unwrap();

        let mut output = String::new();

        for i in 0..5 {
            for digit in curr_h {
                for cell in digit[i] {
                    if cell {
                        output.extend(filled_cell);
                    } else {
                        output.extend(blank_cell);
                    }
                }
                output.extend(blank_cell);
            }
            output.extend(blank_cell);

            for digit in curr_m {
                for cell in digit[i] {
                    if cell {
                        output.extend(filled_cell);
                    } else {
                        output.extend(blank_cell);
                    }
                }
                output.extend(blank_cell);
            }
            output.extend(blank_cell);

            for digit in curr_s {
                for cell in digit[i] {
                    if cell {
                        output.extend(filled_cell);
                    } else {
                        output.extend(blank_cell);
                    }
                }
                output.extend(blank_cell);
            }
            output.push('\n');
        }

        print!("{esc}[2J{esc}[1;1H{output}", esc = 27 as char); // Clear screen and print clock
        thread::sleep(Duration::from_millis(50));
    }
}
