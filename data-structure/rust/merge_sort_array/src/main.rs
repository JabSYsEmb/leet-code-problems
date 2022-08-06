#![allow(unused)]
struct Solution {}

impl Solution {
    pub fn merge(
        nums1: &mut Vec<i32>,
        m: i32,
        nums2: &mut Vec<i32>,
        n: i32,
    ) {
        if nums1.len() + nums2.len() < (m + n) as usize {
            panic!();
        }

        if m == 0 {
            nums1.drain(0..);
            nums2.drain(n as usize..);
            nums1.append(nums2);
            return;
        }

        if n == 0 {
            nums1.drain(m as usize..);
            return;
        }

        nums1.drain(m as usize..);
        nums2.drain(n as usize..);

        if m + n == 1 {
            nums1.drain(1..);
            if m == 0 {
                nums1[0] = nums2[0];
            }
            return;
        }
        if nums1[m as usize - 1] < nums2[0] {
            nums1.append(nums2);
            return;
        }
        if nums1[0] > nums2[(n) as usize - 1] {
            for idx in 0..(n) as usize {
                nums1.insert(idx, nums2[idx]);
            }
            return;
        }
    }
}

fn main() {
    let mut example_4_nums1: Vec<i32> = vec![4, 5, 6];
    let mut example_4_nums2: Vec<i32> = vec![3, 5, 7, 40];
    Solution::leet_merge(&mut example_4_nums1, 2, &mut example_4_nums2, 3);
    println!("Hello, world!");
}

#[test]
fn it_works() {
    // Example 1
    let mut example_1_nums1: Vec<i32> = vec![0];
    let mut example_1_nums2: Vec<i32> = vec![1];
    Solution::leet_merge(&mut example_1_nums1, 0, &mut example_1_nums2, 1);
    assert_eq!(vec![1], example_1_nums1);

    // Example 2
    let mut example_2_nums1: Vec<i32> = vec![0];
    let mut example_2_nums2: Vec<i32> = vec![1];
    Solution::leet_merge(&mut example_2_nums1, 0, &mut example_2_nums2, 1);
    assert_eq!(vec![1], example_2_nums1);

    // Example 3
    let mut example_3_nums1: Vec<i32> = vec![2];
    let mut example_3_nums2: Vec<i32> = vec![0];
    Solution::leet_merge(&mut example_3_nums1, 1, &mut example_3_nums2, 0);
    assert_eq!(vec![2], example_3_nums1);

    // Example 4
    //nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
    let mut example_4_nums1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
    let mut example_4_nums2: Vec<i32> = vec![4, 5, 6];
    Solution::leet_merge(&mut example_4_nums1, 3, &mut example_4_nums2, 3);
    assert_eq!(vec![1, 2, 3, 4, 5, 6], example_4_nums1);

    // Example 4
    //nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
    let mut example_5_nums1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
    let mut example_5_nums2: Vec<i32> = vec![-6, -3, 0];
    Solution::leet_merge(&mut example_5_nums1, 3, &mut example_5_nums2, 3);
    assert_eq!(vec![-6, -3, 0, 1, 2, 3], example_5_nums1);

    // Example 4
    let mut example_6_nums1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
    let mut example_6_nums2: Vec<i32> = vec![2, 5, 6];
    Solution::leet_merge(&mut example_6_nums1, 3, &mut example_6_nums2, 3);
    assert_eq!(vec![1, 2, 2, 3, 5, 6], example_6_nums1);
}
