use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("day1/input.txt").expect("file not found");
    let mut values_unedited = String::new();
    file.read_to_string(&mut values_unedited).expect("Cannot read the file");

    // Simplify string splitting
    let v_splitted: Vec<&str> = values_unedited.split_whitespace().collect();

    // Parse into usize
    let v_usize: Vec<usize> = v_splitted
        .iter()
        .map(|x| x.parse::<usize>().expect("Invalid number"))
        .collect();

    // Split into left and right
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();
    for (i, &val) in v_usize.iter().enumerate() {
        if i % 2 == 0 {
            left.push(val);
        } else {
            right.push(val);
        }
    }

    // Find differences
    let mut nums: Vec<isize> = Vec::new();
    while !left.is_empty() && !right.is_empty() {
        let lef = find_little(&left);
        left.remove(lef[1]); // Remove by index
        let rig = find_little(&right);
        right.remove(rig[1]);
        nums.push((lef[0] as isize - rig[0] as isize).abs());
    }

    // Calculate the result
    let res: isize = nums.iter().sum();
    println!("{res}");
}

// Helper function to find the smallest value and its index
fn find_little(vu: &Vec<usize>) -> Vec<usize> {
    let (index, &little) = vu.iter().enumerate().min_by_key(|&(_, &val)| val).unwrap();
    vec![little, index]
}
