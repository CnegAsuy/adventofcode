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

    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();
    for (i, &val) in v_usize.iter().enumerate() {
        if i % 2 == 0 {
            left.push(val);
        } else {
            right.push(val);
        }
    }

    part_one(&mut left.clone(), &mut right.clone());
    part_two(left, right);
}

// Helper function to find the smallest value and its index
fn find_little(vu: &Vec<usize>) -> Vec<usize> {
    let (index, &little) = vu.iter().enumerate().min_by_key(|&(_, &val)| val).unwrap();
    vec![little, index]
}

fn part_one(left:&mut Vec<usize>, right:&mut Vec<usize>) {
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
    println!("Part 1: {res}");
}
use std::collections::HashMap;

fn part_two(left: Vec<usize>, right: Vec<usize>) {
    let mut right_counts: HashMap<usize, usize> = HashMap::new();
    for &num in &right {
        *right_counts.entry(num).or_insert(0) += 1;
    }
    let mut res = 0;
    for i in left {
        if let Some(&count) = right_counts.get(&i) {
            res += count * i;
        } else {
        }
    }

    println!("Part 2: {res}");
}
