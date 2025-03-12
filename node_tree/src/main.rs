#![allow(dead_code)]
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use std::any::Any;

trait Value: Any {
    fn display(&self);
    fn equals(&self, other: &dyn Value) -> bool;
    fn as_any(&self) -> &dyn Any;
}

impl Value for i32 {
    fn display(&self) {
        println!("i32 value: {}", self);
    }
    
    fn equals(&self, other: &dyn Value) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<i32>() {
            self == other
        } else {
            false
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Value for String {
    fn display(&self) {
        println!("String value: {}", self);
    }

    fn equals(&self, other: &dyn Value) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<String>() {
            self == other
        } else {
            false
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/*impl<'a> Value for &'a str {
    fn display(&self) {
        println!("&str value: {}", self);
    }

    fn equals(&self, other: &dyn Value) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<&str>() {
            self == other
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}*/

impl Value for f32 {
    fn display(&self) {
        println!("Float value: {}", self);
    }

    fn equals(&self, other: &dyn Value) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<f32>() {
            self == other
        } else {
            false
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Debug for dyn Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.display();
        write!(f, "")
    }
}

#[derive(Debug)]
struct Node {
    value: Box<dyn Value>,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: Box<dyn Value>) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn add_left(&mut self, value: Rc<RefCell<Node>>) {
        self.left = Some(value);
    }

    fn add_right(&mut self, value: Rc<RefCell<Node>>) {
        self.right = Some(value);
    }
    
    fn set_children(&mut self, left: Option<Rc<RefCell<Node>>>, right: Option<Rc<RefCell<Node>>>) {
        self.left = left;
        self.right = right;
    }
}

fn new_node<T: 'static + Value>(value: T) -> Rc<RefCell<Node>> {
    Rc::new(RefCell::new(Node::new(Box::new(value))))
}

fn new_node_from_str(value: &str) -> Rc<RefCell<Node>> {
    new_node(value.to_string())
}

fn print_tree(node: Rc<RefCell<Node>>, depth: usize) {
    let borrowed_node = node.borrow();
    let indent = "-".repeat(depth * 2);
    print!("{}Node: ", indent);
    borrowed_node.value.display();

    if let Some(ref left) = borrowed_node.left {
        print_tree(left.clone(), depth + 1);
    }

    if let Some(ref right) = borrowed_node.right {
        print_tree(right.clone(), depth + 1);
    }
}

fn build_tree_from_list<T: 'static + Value + Clone>(values: &[T]) -> Option<Rc<RefCell<Node>>> {
    fn helper<T: 'static + Value + Clone>(values: &[T], start: usize, end: usize) -> Option<Rc<RefCell<Node>>> {
        if start > end {
            return None;
        }

        let mid = (start + end) / 2;
        let node = new_node(values[mid].clone());

        if mid > start {
            let left = helper(values, start, mid - 1);
            node.borrow_mut().left = left;
        }

        if mid < end {
            let right = helper(values, mid + 1, end);
            node.borrow_mut().right = right;
        }

        Some(node)
    }

    if values.is_empty() {
        None
    } else {
        helper(values, 0, values.len() - 1)
    }
}

fn inorder_traversal(node: Option<Rc<RefCell<Node>>>) {
    if let Some(n) = node {
        let n = n.borrow();
        inorder_traversal(n.left.clone());
        n.value.display();
        inorder_traversal(n.right.clone());
    }
}

fn height(node: Option<Rc<RefCell<Node>>>) -> usize {
    if let Some(n) = node {
        let n = n.borrow();
        1 + usize::max(height(n.left.clone()), height(n.right.clone()))
    } else {
        0
    }
}

fn find_value(node: Option<Rc<RefCell<Node>>>, target: &dyn Value) -> bool {
    if let Some(n) = node {
        let n = n.borrow();

        if n.value.equals(target) {
            return true;
        }

        find_value(n.left.clone(), target) || find_value(n.right.clone(), target)
    } else {
        false
    }
}

fn main() {
    let root = new_node(1);
    let left_child = new_node_from_str("Hello");
    let right_child = new_node(3.65);
    
    root.borrow_mut().set_children(Some(left_child.clone()), Some(right_child.clone()));

    let left_left_child = new_node(String::from("Howdy"));
    let left_right_child =new_node(155);
    
    left_child.borrow_mut().set_children(Some(left_left_child.clone()), Some(left_right_child.clone()));

    let right_left_child = new_node(200);
    let right_right_child = new_node(String::from("Hi pal !"));
    
    right_child.borrow_mut().set_children(Some(right_left_child.clone()), Some(right_right_child.clone()));

    print_tree(root.clone(), 0);
    
    let target = String::from("Hello");

    if find_value(Some(root), &target) {
        println!("Value '{}' found in the tree!", target);
    } else {
        println!("Value '{}' not found in the tree.", target);
    }

    /*let values = vec![1, 2, 3, 4, 5, 6, 7, 0, 32, 2];
    if let Some(root) = build_tree_from_list(&values) {
        // print_tree(root.clone(), 0);
        inorder_traversal(Some(root.clone()));
        let tree_height = height(Some(root.clone())); 
        println!("Tree Height: {}", tree_height);
    }*/
}
