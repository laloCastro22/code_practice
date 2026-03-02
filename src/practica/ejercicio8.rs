/// La variable `ultimo_uno` es para convertir el problema en:
/// Quiero que en la posición i haya un valor ≤ i, moviendo elementos con swaps adyacentes al costo mínimo.
/// Es como recorrerlo sin hacer el movimiento en si 
#[allow(dead_code)]
pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut ultimo_uno = Vec::with_capacity(n);

    
    for row in &grid {
        let mut pos = -1;
        for (j, &val) in row.iter().enumerate().rev() {
            if val == 1 {
                pos = j as i32;
                break;
            }
        }
        ultimo_uno.push(pos);
    }

    let mut swaps: i32 = 0;

    for i in 0..n {
        let mut j = i;
        while j < n && ultimo_uno[j] > i as i32 {
            j += 1;
        }
        if j == n {
            return -1;
        }

        swaps += (j - i) as i32;

        let val = ultimo_uno[j];
        while j > i {
            ultimo_uno[j] = ultimo_uno[j - 1];
            j -= 1;
        }
        ultimo_uno[i] = val;
    }

    swaps
}