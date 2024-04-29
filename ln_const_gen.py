import math

s = 16
u = 8

def m(x):
    return (x - 1) / (x + 1)

for i in range(1, s + 1):
    b = max(i*i, 1.000001)
    p_const = 1 / (2 * m(math.sqrt(b)))
    b_u = b ** (1 / u)
    ln_b = math.log(b)
    print(f"{p_const} {b_u} {ln_b}")
