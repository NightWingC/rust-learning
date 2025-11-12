// enum Node {
//     Leaf(i32),
//     Branch(i32, Node, Node)
// }

enum TreeNode {
    Leaf(i32),
    Branch(i32, Box<TreeNode>, Box<TreeNode>)
}

#[derive(Debug)]
struct  LargeData {
    data: [u8; 1024 * 1024],
}

fn process_large_data_on_heap(data: LargeData){
    println!("Processing large data on the heap...");
    println!("Data size: {} bytes", std::mem::size_of_val(&data));
}

fn main(){
    let mut large_instance = LargeData {
        data: [0; 1024 * 1024]
    };

    let boxed_large_instance: Box<LargeData> = Box::new(large_instance);

    process_large_data_on_heap(*boxed_large_instance);

    
    // 
    let leaf_node = TreeNode::Leaf(10);
    let left_branch = Box::new(TreeNode::Leaf(5));
    let right_branch = Box::new(TreeNode::Leaf(15));

    let root = TreeNode::Branch(12, left_branch, right_branch);

    println!("Root value: ");

    if let TreeNode::Branch(value, _ , _ ) = root {
        println!("{}", value);
    }

    // 
    let s_boxed: Box<str> = Box::from("Hello");
    println!("Boxed string: {}", s_boxed);

    let s_ref: &str = &s_boxed;
    println!("Reference to boxed string: {}", s_ref);
}