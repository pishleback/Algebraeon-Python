from algebraeon import *

x = Int.polynomials().var()

poly = x**12 - 1
poly_factored = poly.factor()

print(f"poly          =", poly)
print(f"poly_factored =", poly_factored)