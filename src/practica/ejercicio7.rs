
#[allow(dead_code)]
/// Se optimiza para evitar construir una cadena binaria enorme y luego convertirla a decimal.
/// En su lugar, se construye el resultado en cada iteración con desplazamientos y módulo.
/// Ej: n=3 -> "1|10|11" = 11011₂ = 27.
pub fn concatenated_binary(n: i32) -> i32 {
    let n = n as u128;
    let modulo: u128 = u128::pow(10, 9) + 7;
    let mut ans: u128 = 0; // Resultado acumulado (decimal) de la concatenación, mod M
    let mut len: u32 = 0;  // Número de bits del valor actual i
    for i in 1..=n {
        if (i & (i - 1)) == 0 { // true cuando i es potencia de 2: 1,2,4,8,16...
            len += 1;           // en esos puntos aumenta la longitud en bits
        }
        ans = ((ans << len) + i) % modulo; // desplaza para "abrir espacio" y concatena i
    }
    ans as i32
}