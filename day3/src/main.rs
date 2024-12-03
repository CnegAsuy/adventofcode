use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("day3/input.txt").expect("Cannot open file.");
    let mut corrupted_memory = String::new();
    file.read_to_string(&mut corrupted_memory).expect("Cannot read file.");

    let mut sum = 0;
    let mut doit = true;
    let mut i = 0;
    let chars: Vec<char> = corrupted_memory.chars().collect();

    while i < chars.len() {
        if doit && i + 3 <= chars.len() && &chars[i..i + 3].iter().collect::<String>() == "mul" {
            i += 3;
            if i < chars.len() && chars[i] == '(' {
                i += 1;

                let mut first_num = String::new();
                let mut second_num = String::new();
                let mut parsing_first = true;

                while i < chars.len() {
                    match chars[i] {
                        '0'..='9' => {
                            if parsing_first {
                                first_num.push(chars[i]);
                            } else {
                                second_num.push(chars[i]);
                            }
                        }
                        ',' => {
                            parsing_first = false;
                        }
                        ')' => {
                            if let (Ok(first), Ok(second)) = (first_num.parse::<u32>(), second_num.parse::<u32>()) {
                                sum += first * second;
                            }
                            i += 1;
                            break;
                        }
                        _ => break,
                    }
                    i += 1;
                }
            }
        } else if i + 2 <= chars.len() && &chars[i..i + 2].iter().collect::<String>() == "do" {
            if i + 7 <= chars.len() && &chars[i..i + 7].iter().collect::<String>() == "don't" {
                doit = false;
                i += 7;
            } else if i + 4 <= chars.len() && &chars[i..i + 4].iter().collect::<String>() == "(){" {
                doit = true;
                i += 4;
            } else {
                i += 3;
            }
        } else {
            i += 1;
        }
    }

    println!("Part 1: {}", sum);
}
