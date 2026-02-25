use std::cell::RefCell;
use std::rc::Rc;


    ///Se le proporciona la raíz de un árbol binario donde cada nodo tiene un valor de 0 o 1. 
    ///Cada ruta de raíz a hoja representa un número binario que comienza con el bit más significativo.
    ///
    ///Por ejemplo, si la ruta es 0 -> 1 -> 1 -> 0 -> 1, esto podría representar 01101 en binario, que es 13.
    ///
    ///Para todas las hojas del árbol, considere los números representados por la ruta desde la raíz hasta esa hoja. Devuelva la suma de estos números.
    ///
    ///Los casos de prueba se generan de modo que la respuesta se ajuste a un entero de 32 bits.

//Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(dead_code)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
   /// Este es un metodo recursivo y se debe recorrer el árbol:
   /// Un recorrido en un árbol binario es Una operación que consiste en visitar todos sus vértices o nodos, de tal manera que cada 
   /// vértice se visite una sola vez. Se distinguen tres tipos de recorrido: INORDEN, POSORDEN Y PREORDEN.
   /// En cada recorrido se tiene en cuenta la posición de la raíz (de ahí su nombre) y que siempre se debe ejecutar 
   /// primero el hijo izquierdo y luego el derecho.  
   /// 
   /// 
   /// Recorrido PREORDEN. Este recorrido se realiza así: primero visita la raíz; 
   /// segundo recorre el subárbol izquierdo y por último va a subárbol derecho. 
   /// En síntesis:raíz — hijo izquierdo — hijo derecho
   /// 
   /// El operador `<<` desplazacimiento a la izquierda de bits, como el nombre lo indica, este recorre a la izquierda n bits
   /// Ejemplo
   /// 2 en binario (8 bits): 0000 0010
   ///
   /// 2 << 1 → desplaza todos los bits una posición a la izquierda:
   /// 0000 0100 = 4
   ///
   /// 2 << 2 → dos posiciones:
   /// 0000 1000 = 8
   /// 
   /// Regla general
   /// *x<<n=x⋅2n*
#[allow(dead_code)]
pub fn sum_root_to_leaf_preorden(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn recorrido_pre_orden(node: Option<Rc<RefCell<TreeNode>>>, current: i32) -> i32 {
        if let Some(n) = node {
            let n = n.borrow();
            let new_val = (current << 1) | n.val;

            if n.left.is_none() && n.right.is_none() {
                return new_val;
            }

            return recorrido_pre_orden(n.left.clone(), new_val) + recorrido_pre_orden(n.right.clone(), new_val);
        }
        0
    }

    return recorrido_pre_orden(root, 0)
}