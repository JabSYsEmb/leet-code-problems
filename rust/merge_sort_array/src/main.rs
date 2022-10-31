#![allow(unused)]
struct Solution {}

impl Solution {
    pub fn merge(
        nums1: &mut Vec<i32>,
        m: i32,
        nums2: &mut Vec<i32>,
        n: i32,
    ) {
        let mut index_nums_1 = m - 1;
        let mut index_nums_2 = n - 1;
        for i in ((0..n + m).rev()) {
            if index_nums_2 == -1 {
                break;
            }
            if (index_nums_1 != -1
                && nums1[index_nums_1 as usize] > nums2[index_nums_2 as usize])
            {
                nums1[i as usize] = nums1[index_nums_1 as usize];
                index_nums_1 -= 1;
                continue;
            } else {
                nums1[i as usize] = nums2[index_nums_2 as usize];
                index_nums_2 -= 1;
                continue;
            }
        }
    }
}

fn main() {
    let mut example_4_nums1: Vec<i32> = vec![4, 5, 6, 0, 2];
    let mut example_4_nums2: Vec<i32> = vec![3, 5, 7, 40];
    Solution::merge(&mut example_4_nums1, 2, &mut example_4_nums2, 3);
    println!("Hello, world!");
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        // Example 1
        let mut example_1_nums1: Vec<i32> = vec![0];
        let mut example_1_nums2: Vec<i32> = vec![1];
        Solution::merge(&mut example_1_nums1, 0, &mut example_1_nums2, 1);
        assert_eq!(vec![1], example_1_nums1);

        // Example 2
        let mut example_2_nums1: Vec<i32> = vec![0];
        let mut example_2_nums2: Vec<i32> = vec![1];
        Solution::merge(&mut example_2_nums1, 0, &mut example_2_nums2, 1);
        assert_eq!(vec![1], example_2_nums1);

        // Example 3
        let mut example_3_nums1: Vec<i32> = vec![2];
        let mut example_3_nums2: Vec<i32> = vec![0];
        Solution::merge(&mut example_3_nums1, 1, &mut example_3_nums2, 0);
        assert_eq!(vec![2], example_3_nums1);

        // Example 4
        //nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
        let mut example_4_nums1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
        let mut example_4_nums2: Vec<i32> = vec![4, 5, 6];
        Solution::merge(&mut example_4_nums1, 3, &mut example_4_nums2, 3);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], example_4_nums1);

        // Example 4
        //nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
        let mut example_5_nums1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
        let mut example_5_nums2: Vec<i32> = vec![-6, -3, 0];
        Solution::merge(&mut example_5_nums1, 3, &mut example_5_nums2, 3);
        assert_eq!(vec![-6, -3, 0, 1, 2, 3], example_5_nums1);

        // Example 4
        let mut example_6_nums1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
        let mut example_6_nums2: Vec<i32> = vec![2, 5, 6];
        Solution::merge(&mut example_6_nums1, 3, &mut example_6_nums2, 3);
        assert_eq!(vec![1, 2, 2, 3, 5, 6], example_6_nums1);
    }
}
