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

 - `.is_rational()` returns `True` or `False` depending upon whether it is a rational number.
 - `.minimal_polynomial()` returns the rational monic minimal polynomial.
 - `.isolate()` may return different things depending on the number:
   - If the number is rational, it returns the number as a `Rat`.
   - If the number is irrational, it returns an open rational isolating interval \\((a, b)\\) such that the real algebraic number is the unique root of its minimal polynomial within the interval.

```python
from algebraeon import *

x = Int.polynomials().var()
p = (x ** 3 - 2 * x) * (x ** 4 - 3)

for root in RealAlg.distinct_roots(p):
    print("root:", root)
    print(f"    is_rational       : {root.is_rational()}")
    print(f"    minimal_polynomial: {root.minimal_polynomial()}")
    print(f"    isolate           : {repr(root.isolate())}")

# Output:
"""
root: 0
    is_rational       : True
    minimal_polynomial: λ
    isolate           : Rat(0)
root: -√2
    is_rational       : False
    minimal_polynomial: λ^2-2
    isolate           : (Rat(-4), Rat(0))
root: √2
    is_rational       : False
    minimal_polynomial: λ^2-2
    isolate           : (Rat(0), Rat(4))
root: ≈-1.315
    is_rational       : False
    minimal_polynomial: λ^4-3
    isolate           : (Rat(-5), Rat(0))
root: ≈1.315
    is_rational       : False
    minimal_polynomial: λ^4-3
    isolate           : (Rat(0), Rat(5))
"""
```

## Complex Algebraic Numbers

Algebraeon provides `CpxAlg` for representing complex algebraic numbers.

### Complex Roots of Polynomials

Complex algebraic numbers can be obtained from polynomials.

```python
from algebraeon import *

x = Int.polynomials().var()
p = (x ** 3 - 2 * x) ** 2 * (x ** 2 + 1)

print("all roots      : ", CpxAlg.roots(p))
print("distinct roots : ", CpxAlg.distinct_roots(p))

# Output:
"""
all roots      :  [RealAlg(-i), RealAlg(i), RealAlg(0), RealAlg(0), RealAlg(-√2), RealAlg(-√2), RealAlg(√2), RealAlg(√2)]
distinct roots :  [RealAlg(0), RealAlg(-√2), RealAlg(√2), RealAlg(-i), RealAlg(i)]
"""
```

### Arithmetic Operations

Standard arithmetic operations are implemented for complex algebraic numbers.

```python
from algebraeon import *

x = Int.polynomials().var()
p = (x ** 3 - 2 * x) * (x ** 4 + 3)

for root in CpxAlg.distinct_roots(p):
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
root = ≈-0.931-0.931i
    root^2+1 = 1+i√3
root = ≈-0.931+0.931i
    root^2+1 = 1-i√3
root = ≈0.931-0.931i
    root^2+1 = 1-i√3
root = ≈0.931+0.931i
    root^2+1 = 1+i√3
"""
```


### Minimal Polynomial and Isolating Box

 - `.is_rational()` returns `True` or `False` depending upon whether it is a rational number.
 - `.is_real()` returns `True` or `False` depending upon whether it is a real number.
 - `.minimal_polynomial()` returns the rational monic minimal polynomial.
 - `.isolate()` may return different things depending on the number:
   - If the number is rational, it returns the number as a `Rat`.
   - If the number is irrational and real, it returns an open rational isolating interval \\((a, b)\\) such that the real algebraic number is the unique root of its minimal polynomial within the interval.
   - If the number is irrational and complex, it returns a pair of open intervals \\((a, b)\\) and \\((c, d)\\) such that the real algebraic number \\(z\\) is the unique root of its minimal polynomial with \\(a < \operatorname{Re}(z) < b\\) and \\(c < \operatorname{Im}(z) < d\\). 

```python
from algebraeon import *

x = Int.polynomials().var()
p = (x ** 3 - 2 * x) * (x ** 4 + 3)

for root in CpxAlg.distinct_roots(p):
    print("root:", root)
    print(f"    is_rational       : {root.is_rational()}")
    print(f"    is_real           : {root.is_real()}")
    print(f"    minimal_polynomial: {root.minimal_polynomial()}")
    print(f"    isolate           : {repr(root.isolate())}")

# Output:
"""
root: 0
    is_rational       : True
    is_real           : True
    minimal_polynomial: λ
    isolate           : Rat(0)
root: -√2
    is_rational       : False
    is_real           : True
    minimal_polynomial: λ^2-2
    isolate           : (Rat(-4), Rat(0))
root: √2
    is_rational       : False
    is_real           : True
    minimal_polynomial: λ^2-2
    isolate           : (Rat(0), Rat(4))
root: ≈-0.931-0.931i
    is_rational       : False
    is_real           : False
    minimal_polynomial: λ^4+3
    isolate           : ((Rat(-1), Rat(0)), (Rat(-2), Rat(-1/2)))
root: ≈-0.931+0.931i
    is_rational       : False
    is_real           : False
    minimal_polynomial: λ^4+3
    isolate           : ((Rat(-1), Rat(0)), (Rat(1/2), Rat(2)))
root: ≈0.931-0.931i
    is_rational       : False
    is_real           : False
    minimal_polynomial: λ^4+3
    isolate           : ((Rat(0), Rat(1)), (Rat(-2), Rat(-1/2)))
root: ≈0.931+0.931i
    is_rational       : False
    is_real           : False
    minimal_polynomial: λ^4+3
    isolate           : ((Rat(0), Rat(1)), (Rat(1/2), Rat(2)))
"""
```