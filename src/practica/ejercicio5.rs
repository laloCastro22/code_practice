

/// El ejercicio nos pide que seleccionemos k indices diferentes para cambiar de 1 -> 0 o 0 -> 1
/// hastas que la cadena contenga solamente unos
/// Se deben contabilizar el minimo numero de cambios aplicados, si no se logra retornar un -1.
/// consideraciones:
///
///    1 <= s.length <= 10^​​​​​​​5
///    s[i] es '0' o '1'.
///    1 <= k <= s.length
/// 
/// 
/// 1) Como puedes elegir cualquier conjunto de índices, la posición de los bits no importa
/// 
/// En cada operación solo importa cuántos ceros y cuántos unos escoges para voltear.
/// 
/// Define:
/// 
/// n = s.len()
/// 
/// z = # de '0' (ceros)
/// 
/// o = n - z (unos)
/// 
/// En una operación eliges exactamente k índices:
/// 
/// eliges a ceros (0→1)  y b unos (1→0)
/// con a + b = k
/// 
/// Después de la operación:
/// 
/// los ceros bajan por los a que convertiste a 1 pero suben por los b unos que convertiste a 0
/// 
/// Entonces el nuevo número de ceros es:
/// 
/// z′=z−a+b=z−a+(k−a)=z+k−2a 
/// 
/// Ese es el corazón del problema.
/// No estás optimizando sobre strings, estás optimizando sobre el número z (0..n).
/// 
/// 2) ¿Qué valores de a (ceros elegidos) son posibles?
/// 
/// No puedes elegir más ceros de los que hay: a ≤ z
/// Ni más unos de los que hay: b = k-a ≤ o = n-z ⇒ a ≥ k-(n-z)
/// 
/// Así que:
/// 
/// a_min = max(0, k - (n - z))
/// 
/// a_max = min(k, z)
/// 
/// Y como z' = z + k - 2a, conforme a aumenta, z' disminuye.
/// 
/// Además, como estás restando 2a, solo puedes llegar a valores con el mismo “salto” de 2 (paridad).
/// 
/// 3) Restricción de paridad (clave para saber si es imposible)
/// 
/// De la fórmula:
/// 
/// z′≡z+k(mod2)
/// 
/// O sea, en cada operación la paridad de z:
/// 
/// si k es par, la paridad de z no cambia
/// 
/// si k es impar, la paridad de z se invierte en cada paso
/// 
/// Como quieres llegar a z = 0 (par), entonces:
/// 
/// si k es par y tu z inicial es impar ⇒ imposible (regresas -1)
/// 
/// si k es impar ⇒ por paridad, en principio no te bloquea (aunque aún puede haber otros límites)
/// 
/// (También: si k > n es imposible porque no puedes escoger k índices distintos.)
/// 
/// 4) ¿Por qué no es “paso a paso modificando el string”?
/// 
/// Porque tu operación no depende de “dónde están” los ceros.
/// Siempre puedes escoger los ceros que te convengan y los unos que te convengan, en cualquier parte. Eso convierte el problema en un “juego” sobre el conteo z.
/// 
/// 5) ¿Cómo se encuentra el mínimo de operaciones sin brute force gigante?
/// 
/// Como z solo puede estar entre 0 y n, tienes a lo mucho n+1 estados.
/// 
/// Desde un estado z, puedes ir a varios z' dependiendo de a en [a_min, a_max]:
/// 
/// z′=z+k−2apara a∈[amin,amax]
/// 
/// Eso describe un conjunto de siguientes estados (un intervalo con salto 2).
/// 
/// ✅ Entonces, “mínimo de operaciones” se vuelve:
/// camino más corto desde z_inicial hasta 0 en un grafo de 0..n.
/// 
/// La forma típica de hacerlo:
/// 
/// BFS (busqueda en amplitud) sobre z (cada arista cuesta 1 operación)
/// 
/// Y la optimización real:
/// 
/// no generas strings ni flips reales
/// 
/// solo generas transiciones entre números (0..n)
/// 
/// 6) Ejemplo visual (para que se vea el mecanismo)
/// 
/// Supón n = 5, k = 3.
/// Si ahora z = 2 (hay 2 ceros y 3 unos).
/// 
/// Posibles a (ceros que eliges):
/// 
/// a_max = min(k, z) = min(3,2)=2
/// 
/// a_min = max(0, k-(n-z)) = max(0, 3-3)=0
/// Así que a ∈ {0,1,2}
/// 
/// Calculas z' = z + k - 2a = 2 + 3 - 2a:
/// 
/// si a=0 ⇒ z' = 5
/// 
/// si a=1 ⇒ z' = 3
/// 
/// si a=2 ⇒ z' = 1
use std::cmp::{max, min};
use std::collections::{BTreeSet, VecDeque};
#[allow(dead_code)]
pub fn min_operations(s: String, k: i32) -> i32 {
    let n = s.len() as i32;
    if k > n {
        return -1;
    }

    let z0 = s.bytes().filter(|&b| b == b'0').count() as i32;
    if z0 == 0 {
        return 0;
    }

    // Paridad: z' = z + k - 2a  => z' ≡ z + k (mod 2)
    if k % 2 == 0 && z0 % 2 == 1 {
        return -1;
    }

    let mut dist = vec![-1i32; (n as usize) + 1];
    let mut q = VecDeque::<i32>::new();

    // sets de "no visitados" por paridad
    let mut even = BTreeSet::<i32>::new();
    let mut odd = BTreeSet::<i32>::new();
    for z in 0..=n {
        if z % 2 == 0 { even.insert(z); } else { odd.insert(z); }
    }

    dist[z0 as usize] = 0;
    q.push_back(z0);
    if z0 % 2 == 0 { even.remove(&z0); } else { odd.remove(&z0); }

    while let Some(z) = q.pop_front() {
        let d = dist[z as usize];

        let a_max = min(k, z);
        let a_min = max(0, k - (n - z));

        let mut low = z + k - 2 * a_max;
        let mut high = z + k - 2 * a_min;

        // clamp
        low = max(0, low);
        high = min(n, high);
        if low > high {
            continue;
        }

        let want_parity = (z + k) & 1;
        let set = if want_parity == 0 { &mut even } else { &mut odd };

        // Ajusta low para que tenga la paridad correcta
        if (low & 1) != want_parity {
            low += 1;
        }
        if low > high {
            continue;
        }

        // Consume todos los no visitados en [low, high] (paso 2 implícito por paridad)
        loop {
            let next = match set.range(low..=high).next().copied() {
                Some(v) => v,
                None => break,
            };

            set.remove(&next);
            dist[next as usize] = d + 1;
            if next == 0 {
                return d + 1;
            }
            q.push_back(next);

            // para evitar buscar siempre desde low si ya pasamos next
            low = next + 2;
            if low > high {
                break;
            }
        }
    }

    -1
}