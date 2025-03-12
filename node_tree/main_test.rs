fn main() {
    let root = new_node(1);
    let left_child = new_node("Hello");
    let right_child = new_node(3.65);
    
    // Same as both lines below
    root.borrow_mut().set_children(Some(left_child.clone()), Some(right_child.clone()));
    // root.borrow_mut().add_left(left_child.clone());
    // root.borrow_mut().add_right(right_child.clone());

    let left_left_child = new_node("Howdy");
    let left_right_child =new_node(155);
    
    left_child.borrow_mut().set_children(Some(left_left_child.clone()), Some(left_right_child.clone()));
    // left_child.borrow_mut().add_left(left_left_child.clone());
    // left_child.borrow_mut().add_right(left_right_child.clone());

    let right_left_child = new_node(200);
    let right_right_child = new_node("Hi pal !");
    
    right_child.borrow_mut().set_children(Some(right_left_child.clone()), Some(right_right_child.clone()));
    // right_child.borrow_mut().add_left(right_left_child.clone());
    // right_child.borrow_mut().add_right(right_right_child.clone());
    print_tree(root, 0);
}
