use std::rc::Rc;

struct TreeNode { 
    value: Rc<i32>,
    children: Vec<TreeNode>, 
}

impl TreeNode { 
    fn new(value: i32) -> TreeNode { 
        TreeNode {
            value: Rc::new(value),
            children: Vec::new(), 
        } 
    }

    fn add_child(&mut self, child_node: TreeNode) { 
        self.children.push(child_node); 
    }
 
}



fn main() {
    let mut root = TreeNode::new(5);
    let child1 = TreeNode::new(3); let child2 = TreeNode::new(7); 

    root.add_child(child1); 
    root.add_child(child2);

    let root_ref = Rc::new(root);

    let root_ref_clone1 = Rc::clone(&root_ref);

    let root_ref_clone2 = Rc::clone(&root_ref);

    println!("Root value: {}", root_ref.value);
    println!("Reference counts: {}", Rc::strong_count(&root_ref));

    println!("First child's value: {}", root_ref_clone1.children[0].value); 
    println!("Second child's value: {}", root_ref_clone2.children[1].value);
 


    // 
    let data = Rc::new(10);
    println!("The initial data: {}", data);

    let data_clone1 = Rc::clone(&data);
    println!("After clone 1: {}, strong_count: {}", data_clone1, Rc::strong_count(&data));

    let data_clone2 = Rc::clone(&data);
    println!("After clone 2: {}, strong count: {}", data_clone2, Rc::strong_count(&data));

    {
        let data_clone3 = Rc::clone(&data);
        println!("Inside scope: {}, strong count: {}", data_clone3, Rc::strong_count(&data));
    }

    println!("After scope: {}, strong count {}", data, Rc::strong_count(&data));

    drop(data_clone2);

    println!("After dropping clone 2: {}, strong count: {}", data, Rc::strong_count(&data));
}