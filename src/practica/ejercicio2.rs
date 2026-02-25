/// Esta es una implementación de ordenamiento por número de 1 que hay de una representacion binaria de un número n
/// Esta implementacion a mano es para probar la logica de ordenamiento 
pub fn sort_by_bits(
    arr: Vec<i32>
) -> Vec<i32> {
    let mut arr = arr.clone();
    let len = arr.len();
    let mut aux: i32;

    for i in 0..len {
        for j in i+1..len {
            let n_unos_i = arr[i].count_ones();
            let n_unos_j = arr[j].count_ones();

            if n_unos_i > n_unos_j || (n_unos_i == n_unos_j && arr[i] > arr[j]) {
                aux = arr[i];
                arr[i] = arr[j];
                arr[j] = aux;
            }
        }
    }

    arr
}
