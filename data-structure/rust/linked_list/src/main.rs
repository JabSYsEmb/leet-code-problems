mod data_struct;

fn main() {
    let mut linked_list = data_struct::Node::new(4);
    linked_list.insert(9);
    linked_list.insert(2);
    linked_list.insert(7);
    linked_list.print_whole_list();
}
