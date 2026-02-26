#[allow(dead_code)]
pub fn num_steps(s: String) -> i32 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    if n == 1 {
        return 0; 
    }

    let mut carry: i32 = 0;
    let mut steps: i32 = 0;


    for i in (1..n).rev() {
        let bit = (bytes[i] - b'0') as i32; 
        let val = bit + carry;             

        if val % 2 == 0 {
            // Par -> solo "divide por 2"
            steps += 1;
            // carry se queda igual
        } else {
            // Impar -> "sumamos 1" y luego (en el siguiente paso) se podrá dividir
            steps += 2;
            carry = 1; // al sumar 1 a un impar, se genera/acumula acarreo hacia la izquierda
        }
    }

    steps + carry
}
#[allow(dead_code)]
pub fn num_steps2(s: String) -> i32 {
    let Ok(x) = u128::from_str_radix(s.as_str(), 2) else {
        return 0;
    };
    fn stpes(mut x: u128) -> i32 {
        let mut pasos = 0; 
        if x == 1 {
            return pasos;
        }
        if x % 2 != 0 {
            x+=1;
            pasos+=1;
        }
        x = x >> 1;
        pasos+=1;
        
        return pasos + stpes(x);
    }

    stpes(x)
}