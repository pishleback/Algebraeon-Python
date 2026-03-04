# Algebraic Numbers

## Real Algebraic Numbers

Algebraeon provides `RealAlg` for representing real algebraic numbers.

### Real Roots of Polynomials

Real algebraic numbers can be obtained from polynomials.

```python
from algebraeon import *

x = Int.polynomials().var()
p = (x ** 3 - 2 * x) ** 2 * (x ** 2 + 1)

print("all roots      : ", RealAlg.roots(p))
print("distinct roots : ", RealAlg.distinct_roots(p))

# Output:
"""
all roots      :  [RealAlg(0), RealAlg(0), RealAlg(-√2), RealAlg(-√2), RealAlg(√2), RealAlg(√2)]
distinct roots :  [RealAlg(0), RealAlg(-√2), RealAlg(√2)]
"""
```

### Arithmetic Operations

Standard arithmetic operations are implemented for real algebraic numbers.

```python
from algebraeon import *

x = Int.polynomials().var()
p = (x ** 3 - 2 * x) * (x ** 4 - 3)

for root in RealAlg.distinct_roots(p):
    print("root =", root)
    print("    root^2+1 =", root ** 2 + 1)

# Output:
"""
root = 0
    root^2+1 = 1
root = -√2
    root^2+1 = 3
root = √2
    root^2+1 = 3
root = ≈-1.315
    root^2+1 = 1+√3
root = ≈1.315
    root^2+1 = 1+√3
"""
```

### Minimal Polynomial and Isolating Interval

Real algebraic numbers are stored internally using their minimal polynomial \\(f(x)\\) and an isolating interval \\((a, b)\\) with \\(a, b \in \mathbb{Q}\\), \\(a < b\\).

```python
from algebraeon import *

x = Int.polynomials().var()
p = (x ** 3 - 2 * x) * (x ** 4 - 3)

for root in RealAlg.distinct_roots(p):
    print("root:", root)
    print(f"    is_rational:        {root.is_rational()}")
    print(f"    minimal_polynomial: {root.minimal_polynomial()}")
    print(f"    isolating_interval: {root.isolating_interval()}")

# Output:
"""
root: 0
    is_rational:        True
    minimal_polynomial: λ
    isolating_interval: (Rat(0), Rat(0))
root: -√2
    is_rational:        False
    minimal_polynomial: λ^2-2
    isolating_interval: (Rat(-4), Rat(0))
root: √2
    is_rational:        False
    minimal_polynomial: λ^2-2
    isolating_interval: (Rat(0), Rat(4))
root: ≈-1.315
    is_rational:        False
    minimal_polynomial: λ^4-3
    isolating_interval: (Rat(-5), Rat(0))
root: ≈1.315
    is_rational:        False
    minimal_polynomial: λ^4-3
    isolating_interval: (Rat(0), Rat(5))
"""
```