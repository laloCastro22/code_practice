use std::collections::HashMap;

/// Adicion de logica para encontrar dichos indices intentando hacerlo en un solo bucle
/// Tiempo: O(n²)
/// Espacio: O(1)
#[allow(dead_code)]
pub fn two_sum1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut is_found = false;
    let mut i = 0;
    let mut j = i + 1;
    let len = nums.len();
    let mut result = vec![];
    if len == 1 {
        return result;
    } else if len == 2 {
        if nums[i] + nums[j] == target {
            result.push(i as i32);
            result.push(j as i32);
            return result;
        }
    }
    
    while !is_found {
        if j == len  {
            i += 1;
            j = 0;
        }
        if i != j {
            if nums[i] + nums[j] == target {
                result.push(i as i32);
                result.push(j as i32);
                is_found = true;
            }
        }
        j += 1;
    }
    result
}
/// Segunda version utulizando dos for
/// Tiempo: O(n²)
/// Espacio: O(1)
#[allow(dead_code)]
pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32,j as i32];
            }
        }
    }
    vec![]
}
///  Esta version funciona de la siguiente manera:
///  Si el complemento es igual al numero guardado en le map, es como hacer la suma implicitamente, 
///  ya se sabe que sumados nos dan el target
///  Tiempo: O(n)
/// Espacio: O(1)
#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return vec![j as i32, i as i32];
        }
        map.insert(num, i);
    }

    vec![]
}


