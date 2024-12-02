use std::{fs::File, io::{self, Read}};

fn main() -> io::Result<()> {
    // Read file content
    let mut file = File::open("day2/input.txt")?;
    let mut values_unedited = String::new();
    file.read_to_string(&mut values_unedited)?;
    let data = analyzer(&values_unedited);
    let mut res_p_1 = 0;
    let mut res_p_2 = 0;

    for line in data {
        if is_safe(&line) {
            res_p_1 += 1;
        } else {
            for j in 0..line.len() {
                let mut temp = line.clone();
                temp.remove(j);
                if is_safe(&temp) {
                    res_p_2 += 1;
                    break;
                }
            }
        }
    }

    // Output results
    println!("Part 1: {}", res_p_1);
    println!("Part 2: {}", res_p_1 + res_p_2);

    Ok(())
}
fn analyzer(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace() 
                .filter_map(|n| n.parse::<u32>().ok()) // Parse to u32 and ignore invalid
                .collect()
        })
        .collect()
}

// Check if a vector is "safe" based on the conditions
fn is_safe(nums: &[u32]) -> bool {
    if nums.len() < 2 {
        return true; // Single number is trivially "safe"
    }

    let monotonic_increasing = nums[1] > nums[0];
    for pair in nums.windows(2) {
        let diff = pair[1].abs_diff(pair[0]);

        // Check if difference is within bounds
        if diff < 1 || diff > 3 {
            return false;
        }

        // Check monotonicity
        let is_increasing = pair[1] > pair[0];
        if is_increasing != monotonic_increasing {
            return false;
        }
    }

    true
}
