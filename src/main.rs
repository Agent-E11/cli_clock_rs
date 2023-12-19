use std::{thread, time::Duration};
use chrono::Local;
use cli_clock::BlockChar3By5;

fn main() {
    let filled_cell = ['█', '█'];
    let blank_cell = [' ', ' '];
    const WAIT_TIME: Duration = Duration::from_millis(50);

    let mut rendered_time = Local::now();

    loop {

        let current_time = Local::now();
        if rendered_time.format("%H%M%S").to_string() == current_time.format("%H%M%S").to_string() {
            thread::sleep(WAIT_TIME);
            continue;
        }

        rendered_time = current_time;

        let curr_h: [[[bool; 3]; 5]; 2] = rendered_time.format("%H").to_string()
            .chars().map(|c| c.char_grid())
            .collect::<Vec<_>>().try_into().unwrap();
        let curr_m: [[[bool; 3]; 5]; 2] = rendered_time.format("%M").to_string()
            .chars().map(|c| c.char_grid())
            .collect::<Vec<_>>().try_into().unwrap();
        let curr_s: [[[bool; 3]; 5]; 2] = rendered_time.format("%S").to_string()
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
        thread::sleep(WAIT_TIME);
    }
}
