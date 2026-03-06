use std::{cell::RefCell, rc::Rc};

use crate::practica::{
    ListNode, TreeNode, add_two_numbers, min_operations, num_steps, sort_by_bits, sum_root_to_leaf_preorden, two_sum,
    concatenated_binary, min_swaps, find_kth_bit, num_special, min_operations_11, check_ones_segment
};

mod practica;

fn main() {
    let s = "101".to_string();
    let is_segment = check_ones_segment(s);
    println!("Hay un unico segmento de unos: {is_segment}");
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
    let arr = vec![1, 3, 4, 8];
    let arr = sort_by_bits(arr);
    println!("Arreglo ordenado: {:?}", arr)
}
fn _target_mediante_indices() {
    let nums = vec![11, 15, 2, 7];
    let target = 9;
    let indices = two_sum(nums, target);
    println!(
        "Los indices que sumados me dan el target son: {:?}",
        indices
    )
}
fn _pasos_al_uno() {
    let n_steps = num_steps("1101".to_string());
    println!("Pasos: {n_steps}");
}
fn _operaciones_minimas() {
    let binari = String::from("0101");
    let operaciones = min_operations(binari, 3);
    println!("Numero minimo de operaciones: {operaciones}");
}
fn _suma_nodos_ligados_carry() {
    // 2 -> 4 -> 3
    let l1: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));

    // 5 -> 6 -> 4
    let l2: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    let value = add_two_numbers(l1, l2);
    println!("La operacion acumulativa es: {:?}", value);
}
fn _modulo() {
    let n = 12;
    let e = concatenated_binary(n);
    println!("El resto del numero despues de la aplicacion del moludo: {e}");
}
fn _numero_minimo_intercambios() {
    let matriz = vec![vec![0,0,1],vec![1,1,0],vec![1,0,0]];
    let min_swaps = min_swaps(matriz);
    println!("Numero minimo de intercambios de filas: {:?}", min_swaps);
}
fn _bit_esimo() {
    let bit = find_kth_bit(3, 1);
    println!("El k-esimo bit es: {bit}");
}
fn _numero_posiciones_especiales() {
    let mat = vec![vec![1,0,0],vec![0,1,0],vec![0,0,1]]; 
    let posiciones_especiales = num_special(mat);
    println!("Número de pocisiones especiales: {posiciones_especiales}")
}
fn _numero_minimo_operaciones() {
    let s = "111011".to_string();
    let total_minimo_movientos = min_operations_11(s);
    println!("El numero minimo de operaciones es: {total_minimo_movientos}");
}