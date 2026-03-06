# Algebraic Numbers

## Real Algebraic

Algebraeon provides `RealAlg` for representing real algebraic numbers.

### Real Roots of Polynomials

Real algebraic numbers can be obtained from polynomials.

```python
from algebraeon import *

x = Int.Poly().var()
f = (x ** 3 - 2 * x) ** 2 * (x ** 2 + 1)

print("all roots      : ", RealAlg.roots(f))
print("distinct roots : ", RealAlg.distinct_roots(f))

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

x = Int.Poly().var()
f = (x ** 3 - 2 * x) * (x ** 4 - 3)

for root in RealAlg.distinct_roots(f):
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

x = Int.Poly().var()
f = (x ** 3 - 2 * x) * (x ** 4 - 3)

for root in RealAlg.distinct_roots(f):
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

### Rational Approximation

 - `x.approximate(r)` returns a rational number \\(y\\) such that \\(|x - y| < r\\).

```python
from algebraeon import *

x = Int.Poly().var()
f = x**5 - x + 1

for root in RealAlg.distinct_roots(f):
    print("root:", root)
    print(f"    approximate: {root.approximate(Rat(1, 10 ** 10))}")

# Output
"""
root: ≈-1.172
    approximate: -271383/232487
"""
```

## Complex Algebraic

Algebraeon provides `CpxAlg` for representing complex algebraic numbers.

### Complex Roots of Polynomials

Complex algebraic numbers can be obtained from polynomials.

```python
from algebraeon import *

x = Int.Poly().var()
f = (x ** 3 - 2 * x) ** 2 * (x ** 2 + 1)

print("all roots      : ", CpxAlg.roots(f))
print("distinct roots : ", CpxAlg.distinct_roots(f))

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

x = Int.Poly().var()
f = (x ** 3 - 2 * x) * (x ** 4 + 3)

for root in CpxAlg.distinct_roots(f):
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

x = Int.Poly().var()
f = (x ** 3 - 2 * x) * (x ** 4 + 3)

for root in CpxAlg.distinct_roots(f):
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

### Rational Approximation

 - `x.approximate(r)` returns a complex number \\(y\\) as a pair of rational numbers representing the real and imaginary parts such that \\(|x - y| < r\\).

```python
from algebraeon import *

x = Int.Poly().var()
f = x**5 - x + 1

for root in CpxAlg.distinct_roots(f):
    print("root:", root)
    print(f"    approximate: {root.approximate(Rat(1, 10 ** 10))}")

# Output
"""
root: ≈-1.172
    approximate: (Rat(-210940/180707), Rat(0))
root: ≈-0.179-1.087i
    approximate: (Rat(-13614/75119), Rat(-35714285333/32948152776))
root: ≈-0.179+1.087i
    approximate: (Rat(-13614/75119), Rat(35714285333/32948152776))
root: ≈0.767-0.352i
    approximate: (Rat(79125/103447), Rat(-19928216461/56538511220))
root: ≈0.767+0.352i
    approximate: (Rat(79125/103447), Rat(19928216461/56538511220))
"""
```

## \\(p\\)-Adic Algebraic

Algebraeon provides `PAdicAlg(p)` for representing \\(p\\)-adic algebraic numbers where \\(p\\) is a prime.


### \\(p\\)-Adic Roots of Polynomials

\\(p\\)-adic algebraic numbers can be obtained from polynomials.

```python
from algebraeon import *

x = Int.Poly().var()
f = (x - 3) * (4 * x ** 2 - 17) ** 2

print("all roots      : ", PAdicAlg(2).roots(f))
print("distinct roots : ", PAdicAlg(2).distinct_roots(f))

# Output:
"""
all roots      :  [PAdicAlg(2)(...000011), PAdicAlg(2)(...110100.1), PAdicAlg(2)(...110100.1), PAdicAlg(2)(...001011.1), PAdicAlg(2)(...001011.1)]
distinct roots :  [PAdicAlg(2)(...000011), PAdicAlg(2)(...110100.1), PAdicAlg(2)(...001011.1)]
"""
```

### Arithmetic Operations

Standard arithmetic operations are implemented for \\(p\\)-adic algebraic numbers.

```python
from algebraeon import *

x = Int.Poly().var()
f = (x - 3) * (4 * x ** 2 - 17)

for root in PAdicAlg(2).roots(f):
    print("root =", root)
    print("    root^2+1 =", root ** 2 + 1)

# Output:
"""
root = ...000011
    root^2 = ...001001
root = ...110100.1
    root^2 = ...000100.01
root = ...001011.1
    root^2 = ...000100.01
"""
```


### Minimal Polynomial and Isolating Ball

 - `.is_rational()` returns `True` or `False` depending upon whether it is a rational number.
 - `.minimal_polynomial()` returns the rational monic minimal polynomial.
 - `.isolate()` may return different things depending on the number:
   - If the number is rational, it returns the number as a `Rat`.
   - If the number is irrational and real, it returns \\((c, v)\\) where \\(c \in \mathbb{Q}\\) is the center of a \\(p\\)-adic ball and \\(v \in \mathbb{Z}\\) is the radius of the \\(p\\)-adic ball specified as a valuation. The ball is such that the irrational \\(p\\)-adic number is the unique root of its minimal polynomial inside the ball.
   
```python
from algebraeon import *

x = Int.Poly().var()
f = (16 * x ** 2 - 17) * (x - 3)

for root in PAdicAlg(2).roots(f):
    print("root:", root)
    print(f"    is_rational       : {root.is_rational()}")
    print(f"    minimal_polynomial: {root.minimal_polynomial()}")
    print(f"    isolate           : {repr(root.isolate())}")

# Output
"""
root: ...000011
    is_rational       : True
    minimal_polynomial: λ-3
    isolate           : Rat(3)
root: ...111010.01
    is_rational       : False
    minimal_polynomial: 16λ^2-17
    isolate           : (Rat(1/4), Int(0))
root: ...000101.11
    is_rational       : False
    minimal_polynomial: 16λ^2-17
    isolate           : (Rat(3/4), Int(0))
"""
```

### Digits and Approximation

There are a few ways to get an approximation of a \\(p\\)-adic value. There are methods to get arbitrarily many \\(p\\)-adic digits, and to find a rational number arbitrarily close.

 - `x.digits(v)` returns a pair `(digits, shift)` where `digits` is a list of natural numbers \\(0, \dots, p-1\\) making up the digits of the \\(p\\)-adic expansion of \\(x\\) up to valuation \\(v\\). `shift` is the valuation of the first digit in the list.
 - `x.approximate(v)` returns a rational number \\(y\\) approximating \\(x\\) in the sense that the valuation of the difference between \\(x\\) and \\(y\\) is at least \\(v\\).

```python
from algebraeon import *

x = Int.Poly().var()
f = (16 * x ** 2 - 17) * (x - 6) * x

for root in PAdicAlg(2).roots(f):
    print("root:", root)
    print("    digits     :", root.digits(3))
    print("    approximate:", root.approximate(3))

# Output
"""
root: ...000110
    digits     : ([Nat(1), Nat(1)], Int(1))
    approximate: 6
root: ...000000
    digits     : ([], Int(3))
    approximate: 0
root: ...111010.01
    digits     : ([Nat(1), Nat(0), Nat(0), Nat(1), Nat(0)], Int(-2))
    approximate: 9/4
root: ...000101.11
    digits     : ([Nat(1), Nat(1), Nat(1), Nat(0), Nat(1)], Int(-2))
    approximate: 23/4
"""
```