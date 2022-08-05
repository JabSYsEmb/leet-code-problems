#[allow(dead_code)]
#[allow(unused_variables)]

// Brute force solution ...
fn two_sum(
    nums: Vec<i32>,
    target: i32,
) -> Vec<i32> {
    if nums.len() == 2 {
        return vec![0, 1];
    }

    for index in 0..nums.len() {
        let mut result: i32 = target;
        result -= nums[index];
        for inner_idx in index + 1..nums.len() {
            let mut inner_result = result;
            inner_result -= nums[inner_idx];
            if inner_result == 0 {
                return vec![index as i32, inner_idx as i32];
            }
        }
    }

    return vec![];
}

// Optimal solution ... using HashMap

fn two_sum_optimal(
    nums: Vec<i32>,
    target: i32,
) -> Vec<i32> {
    if nums.len() == 2 {
        return vec![0, 1];
    }

    let mut hash_map: std::collections::hash_map::HashMap<i32, i32, _> =
        std::collections::HashMap::new();

    for index in 0..nums.len() {
        // if (nums[index] - target) > 0 && (target > 0) {
        //    continue;
        //}

        if hash_map.contains_key(&(target - nums[index])) {
            match hash_map.get_key_value(&(target - nums[index])) {
                Some((_, &n)) => {
                    return vec![n, index as i32];
                },
                _ => {},
            }
        }
        hash_map.insert(nums[index], index as i32);
    }

    return vec![];
}

fn main() {
    let fifth_nums = vec![-3, 4, 1, -5, 90, 3];
    println!("{:?}", two_sum_optimal(fifth_nums, 0));
}

#[test]
fn first_test() {
    let first_nums = vec![2, 7, 11, 15];
    assert_eq!(vec![0, 1], two_sum_optimal(first_nums, 9));
}

#[test]
fn second_test() {
    let second_nums = vec![3, 2, 4];
    assert_eq!(vec![1, 2], two_sum_optimal(second_nums, 6));
}

#[test]
fn third_test() {
    let third_nums = vec![3, 3];
    assert_eq!(vec![0, 1], two_sum_optimal(third_nums, 6));
}

#[test]
fn fourth_test() {
    let fourth_nums = vec![3, 2, 3];
    assert_eq!(vec![0, 2], two_sum_optimal(fourth_nums, 6));
}

#[test]
fn fifth_test() {
    let fifth_nums = vec![1, 1, 3, 3];
    assert_eq!(vec![2, 3], two_sum_optimal(fifth_nums, 6));
}

#[test]
fn sixth_test() {
    let fifth_nums = vec![-3, 4, 3, 90];
    assert_eq!(vec![0, 2], two_sum_optimal(fifth_nums, 0));
}

#[test]
fn seventh_test() {
    let fifth_nums = vec![-10, 7, 19, 15];
    assert_eq!(vec![0, 2], two_sum_optimal(fifth_nums, 9));
}
