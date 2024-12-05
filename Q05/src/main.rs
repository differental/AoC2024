use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn dfs(
    node_val: &i32,
    elements: &HashSet<i32>,
    data_map: &HashMap<i32, Vec<i32>>,
    visited: &mut HashSet<i32>,
    item_stack: &mut Vec<i32>,
) -> () {
    visited.insert(*node_val);

    if let Some(to_visit) = data_map.get(node_val) {
        for next_node_val in to_visit {
            if !visited.contains(next_node_val) && elements.contains(next_node_val) {
                dfs(next_node_val, elements, data_map, visited, item_stack);
            }
        }
    }

    item_stack.push(*node_val);
}

fn correct_array(elements: &HashSet<i32>, data_map: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::new();
    let mut visited: HashSet<i32> = HashSet::new();
    for item in elements {
        if !visited.contains(item) {
            dfs(item, elements, data_map, &mut visited, &mut stack);
        }
    }
    stack.reverse();
    stack
}

fn check_validity(arr: &Vec<i32>, data_map: &HashMap<i32, Vec<i32>>) -> bool {
    let mut invalids: HashSet<i32> = HashSet::new();

    for num in arr {
        if invalids.contains(&num) {
            //println!("Incorrect array: {:?}\nError at: {}", arr, num);
            return false;
        }

        match data_map.get(num) {
            Some(val) => {
                for item in val {
                    invalids.insert(*item);
                }
            }
            None => {}
        }
    }
    true
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Read error");

    let lines = contents.lines();
    let mut data_map: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut pair_input = true;

    let mut count = 0;
    let mut count2 = 0;

    for line in lines {
        if line.trim() == "" {
            pair_input = false;
            continue;
        }

        if pair_input {
            let nums: Vec<i32> = line
                .trim()
                .split('|')
                .map(|s| s.parse::<i32>().expect("Error when parsing"))
                .collect();
            if nums.len() != 2 {
                panic!();
            }

            let (l, r) = (nums[0], nums[1]);
            let arr = data_map.entry(r).or_insert(Vec::new());
            arr.push(l);
        } else {
            let nums: Vec<i32> = line
                .trim()
                .split(',')
                .map(|s| s.parse::<i32>().expect("Error when parsing"))
                .collect();

            let good_array = check_validity(&nums, &data_map);

            if good_array {
                count += nums[(nums.len() - 1) / 2];
            } else {
                let elements = HashSet::<i32>::from_iter(nums);

                let corrected_arr = correct_array(&elements, &data_map);
                //println!("Corrected: {:?}", corrected_arr);

                count2 += corrected_arr[(corrected_arr.len() - 1) / 2];
            }
        }
    }
    println!("{} {}", count, count2);
}
