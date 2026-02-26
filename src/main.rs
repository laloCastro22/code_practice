use std::{cell::RefCell, rc::Rc};

use crate::practica::{TreeNode, sum_root_to_leaf_preorden, sort_by_bits, two_sum, num_steps};

mod practica;

fn main() {
    
}
fn _conteo_binario() {
    let tree_derecho = TreeNode::new(1);
    let tree_izquierdo = TreeNode::new(0);
    let mut root = TreeNode::new(1);
    root.left = Some(Rc::new(RefCell::new(tree_izquierdo))); //izquierda
    root.right = Some(Rc::new(RefCell::new(tree_derecho))); //derecha
    let total_preorden = sum_root_to_leaf_preorden(Some(Rc::new(RefCell::new(root.clone()))));
    println!("El total del arbol en preorden es: {total_preorden}");
}
fn _ordenamiento_por_unos() {
    let arr =vec![1,3,4,8];
    let arr = sort_by_bits(arr);
    println!("Arreglo ordenado: {:?}", arr)
}
fn _target_mediante_indices() {
    let nums = vec![11,15, 2,7,];
    let target = 9;
    let indices = two_sum(nums, target);
    println!("Los indices que sumados me dan el target son: {:?}", indices)
}
fn _pasos_al_uno() {
    let n_steps = num_steps("1101".to_string());
    println!("Pasos: {n_steps}");
}

