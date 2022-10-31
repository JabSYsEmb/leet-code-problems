fn kadane_algorithm(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let mut max_till_now: i32 = std::i32::MIN;
    let mut cur_sum: i32 = 0;
    let mut _iter = nums.into_iter();

    while let Some(next_val) = _iter.next() {
        cur_sum = cur_sum + next_val;
        if max_till_now < cur_sum {
            max_till_now = cur_sum;
        }
        if cur_sum < 0 {
            cur_sum = 0;
        }
    }

    return max_till_now;
}

fn main() {
    println!("{}", kadane_algorithm(vec![2, 34, -2, 4, 1, 1]));
}

#[test]
fn kadane_algorithm_more_than_one_item() {
    let first_arr = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let first_ans = 6;
    assert_eq!(first_ans, kadane_algorithm(first_arr));
}

#[test]
fn kadane_algorithm_negative_sum() {
    let third_arr = vec![-5, -4, -1, -7, -8];
    let third_ans = -1;
    assert_eq!(third_ans, kadane_algorithm(third_arr));
}

#[test]
fn kadane_algorithm_one_item_arr() {
    let second_arr = vec![1];
    let second_ans = 1;
    assert_eq!(second_ans, kadane_algorithm(second_arr));
}
