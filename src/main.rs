use std::{cell::RefCell, rc::Rc};

use crate::practica::{TreeNode, sum_root_to_leaf_preorden};

mod practica;

fn main() {
    let tree_derecho = TreeNode::new(1);
    let tree_izquierdo = TreeNode::new(0);
    let mut root = TreeNode::new(1);
    root.left = Some(Rc::new(RefCell::new(tree_izquierdo))); //izquierda
    root.right = Some(Rc::new(RefCell::new(tree_derecho))); //derecha
    let total_preorden = sum_root_to_leaf_preorden(Some(Rc::new(RefCell::new(root.clone()))));
    println!("El total del arbol en preorden es: {total_preorden}");
    let value = 5<<4;
    println!("Valores: {value}")
}
