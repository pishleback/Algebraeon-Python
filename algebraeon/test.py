from algebraeon import *




x = Int.polynomials().var()

f = -12 * (x ** 6 - 1)

print(f)

print(f.factor())

print(f.factor().powers())

print(f.factor().irreducibles())

print(f.factor().distinct_irreducibles())

print(f.factor().content())

print(f.factor().primitive())