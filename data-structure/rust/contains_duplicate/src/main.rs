/*
    Given an integer array nums, return true if any value appears at least 
    twice in the array, and return false if every element is distinct.

    Example 1:

    Input: nums = [1,2,3,1]
    Output: true
    Example 2:

    Input: nums = [1,2,3,4]
    Output: false
    Example 3:

    Input: nums = [1,1,1,3,3,4,3,2,4,2]
    Output: true
     

    Constraints:

    1 <= nums.length <= 105
    -109 <= nums[i] <= 109
*/

struct Node<'a>{
    data: &'a i32,
    next: Option<Box<Node<'a>>>,
}

impl<'a> Node<'a>{
    fn new(val: &'a i32) -> Self
    {
        return Node{
            data: val,
            next: None,
        }
    }

    fn insert(&mut self, val: &'a i32) 
    {
        let target_node = &mut self.next;
        match target_node {
            &mut Some(ref mut next_node) => next_node.insert(val),
            &mut None => *target_node = Some(Box::new(Node::new(val))),
        }
    }

    fn sort(&mut self) 
    {
        let temp = self.data;
        let mut next_node = &mut self.next;
    }

    fn insert_at_pos(&self, pos: &mut i32) 
    {
        let mut temp = &self.next;
        let mut current_node: &Option<Box<Node>>; 
        while *pos != 0{
            match temp.as_ref(){
                Some(node) => current_node = &node.next,
                _ => {},
            }
            println!("{pos}");
            *pos -= 1;
        }
    }

    fn to_print(&self) 
    {
        print!("{} ", self.data);
        match &self.next{
            Some(ref next_node) => {
                print!("-> ");
                next_node.to_print();
            },
            _ => println!(""),
        }
    }

}

struct Solution;
impl Solution{
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        return false;
    }
}

fn main() {
    let vec_arr: Vec<i32> = vec![1,234,34,2,45,1,2,3,23,23,1,2,3,5,2,3,1];
    println!("{:?}", vec_arr);
    let mut linked_list = Node::new(&vec_arr[0]); 
    for index in 1..vec_arr.len(){
        linked_list.insert(&vec_arr[index]);
    }

    linked_list.to_print();

    linked_list.insert_at_pos(&mut 7);

    if !Solution::contains_duplicate(vec_arr)
    {
        println!("testing");
    }
}
