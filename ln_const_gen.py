import math
from fractions import Fraction

acc = 1e10

s = 65536
u = 128

def m(x):
    return (x - 1) / (x + 1)

def approx(x):
    return Fraction(x).limit_denominator(int(acc))

def inv(x):
    return Fraction(x.denominator, x.numerator)

print("// generated with ln_const_gen.py")
print(f"pub const S: u64 = {s};")
print(f"pub const U: u64 = {u};")
print(f"pub const LN_CONSTS: [[u64; 6]; {s}] = [")

for i in range(1, s + 1):
    b = i * i
    b_a = inv(approx(max(2 * m(i), 1 / acc)))
    b_b = approx(b ** (1 / u))
    b_c = approx(math.log(b))
    print(f"    [{b_a.numerator}, {b_a.denominator}, {b_b.numerator}, {b_b.denominator}, {b_c.numerator}, {b_c.denominator}], // {i}")

print("];")
