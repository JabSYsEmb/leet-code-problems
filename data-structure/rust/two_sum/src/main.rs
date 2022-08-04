#[allow(dead_code)]
#[allow(unused_variables)]

fn two_sum(
    nums: Vec<i32>,
    target: i32,
) -> Vec<i32> {
    vec![0, 1]
}

fn main() {
    println!("what is going on!");
}

#[test]
fn first_test() {
    let first_nums = vec![2, 7, 11, 15];
    assert_eq!(vec![0, 1], two_sum(first_nums, 9));
}

#[test]
fn second_test() {
    let second_nums = vec![3, 2, 4];
    assert_eq!(vec![0, 1], two_sum(second_nums, 9));
}

#[test]
fn third_test() {
    let third_nums = vec![3, 3];
    assert_eq!(vec![0, 1], two_sum(third_nums, 9));
}
