  

## Por qué ∣Sn∣=2n−1

Definimos Li=∣Si∣L_i = |S_i|.

Dado:

Si=Si−1+"1"+reverse(invert(Si−1))S_i = S_{i-1} + "1" + reverse(invert(S_{i-1}))Si=Si−1+"1"+reverse(invert(Si−1))

-   La parte Si−1S_{i-1} mide Li−1L_{i-1}.
    
-   El `"1"` mide 1.
    
-   invert(Si−1)invert(S_{i-1})
    
-   reverse(⋅)reverse(cdot)  **tampoco cambia el tamaño**, sigue midiendo Li−1L_{i-1}.
    

Entonces:

Li=Li−1+1+Li−1=2Li−1+1L_i = L_{i-1} + 1 + L_{i-1} = 2L_{i-1} + 1  

con base L1=∣"0"∣=1L_1 = |"0"| = 1.

Ahora resuelves la recurrencia:

Una forma rápida: prueba patrón.

-   L1=1=21−1L_1 = 1 = 2^1 - 1
    
-   L2=2(1)+1=3=22−1L_2 = 2(1)+1 = 3 = 2^2 - 1
    
-   L3=2(3)+1=7=23−1L_3 = 2(3)+1 = 7 = 2^3 - 1
    

Inducción:  
Si Li−1=2i−1−1L_{i-1} = 2^{i-1}-1, entonces

Li=2(2i−1−1)+1=2i−2+1=2i−1L_i = 2(2^{i-1}-1) + 1 = 2^i - 2 + 1 = 2^i - 1  

Listo.

## 2) Por qué el centro es mid=2n−1mid = 2^{n-1} y el bit ahí es `'1'`

Como Ln=2n−1L_n = 2^n - 1, el índice central (1-indexed) es:

mid=Ln+12=(2n−1)+12=2n2=2n−1mid = frac{L_n + 1}{2} = frac{(2^n - 1) + 1}{2} = frac{2^n}{2} = 2^{n-1}  

Y el valor en esa posición es `'1'` porque **la construcción lo pone explícitamente**:

Sn=Sn−1⏟izq+"1"⏟exactamente en mid+reverse(invert(Sn−1))⏟derS_n = underbrace{S_{n-1}}_{text{izq}} + underbrace{"1"}_{text{exactamente en } mid} + underbrace{reverse(invert(S_{n-1}))}_{text{der}}  

## 3) Por qué si k<midk < mid estás en Sn−1S_{n-1}  

Porque la mitad izquierda **es literalmente** Sn−1S_{n-1}Sn−1 y ocupa las posiciones:

1…Ln−11 ldots L_{n-1}  

y mid=Ln−1+1mid = L_{n-1}+1. (De hecho Ln−1=2n−1−1  
, entonces mid=2n−1=Ln−1+1mid = 2^{n-1} = L_{n-1}+1.)

Así que si k<midk < mid, no hay transformación: es el mismo índice en Sn−1S_{n-1}.

## 4) Por qué si k>midk > mid puedes “reflejar” y luego invertir

La derecha es:

R=reverse(invert(Sn−1))

y tiene longitud Ln−1L_{n-1}. Ocupa posiciones:

mid+1…Lnmid+1 ldots L_n  

Tomemos un ejemplo general con índices 1-based.

Sea jj la posición **dentro de la derecha** (también 1-based):

j=k−mid

(donde j=1j=1 es el primer carácter de la derecha).

Como es un **reverse**, el jj-ésimo carácter de RR corresponde al carácter Ln−1−j+1L_{n-1} - j + 1 de invert(Sn−1)invert(S_{n-1}).

Y como invertinvert solo cambia el bit, ese carácter viene del carácter Ln−1−j+1L_{n-1} - j + 1 de Sn−1S_{n-1}, pero invertido.

Entonces el índice equivalente en Sn−1S_{n-1} es:

k′=Ln−1−j+1k' = L_{n-1} - j + 1

Sustituyendo j=k−midj = k - mid:

k′=Ln−1−(k−mid)+1=(Ln−1+mid+1)−kk' = L_{n-1} - (k - mid) + 1 = (L_{n-1} + mid + 1) - k  

Ahora usa que:

-   mid=Ln−1+1mid = L_{n-1} + 1  
    Entonces:
    

k′=(Ln−1+(Ln−1+1)+1)−k=(2Ln−1+2)−kk' = (L_{n-1} + (L_{n-1}+1) + 1) - k = (2L_{n-1} + 2) - k  

y como Ln=2Ln−1+1L_n = 2L_{n-1} + 1:

2Ln−1+2=Ln+1

así que:

k′=(Ln+1)−k

Que es lo mismo que escribiste:

k′=Ln−k+1

Finalmente, como esa parte es invertida:

bit(n,k)=invert(bit(n−1,k′))

Y reemplazando Ln=2n−1L_n = 2^n - 1:

k′=(2n−1)−k+1=2n−kk' = (2^n - 1) - k + 1 = 2^n - k  

Eso justifica todo el razonamiento (tamaños, centro, espejo, e inversión) sin asumir nada “mágico”; solo conteo de longitudes + cómo funciona `reverse`.