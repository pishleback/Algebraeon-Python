# Polynomials

## Integer Polynomials

```python
from algebraeon import *

x = Int.Poly().var()

assert((x + 2) ** 2 == x**2 + 4*x + 4)

assert((x**2 + 3*x + 2) / (x + 1) == x + 2)

# not divisible
try:
    x / (2 * x)
except ValueError:
    pass
else:
    raise Exception()
```

## Rational Polynomials

```python
from algebraeon import *

x = Rat.Poly().var()

assert((x + 2) ** 2 == x**2 + 4*x + 4)

assert((x**2 + 3*x + 2) / (x + 1) == x + 2)

assert(x / (2 * x) == Rat(1, 2))
```

## Polynomials Over the Natural Numbers

```python
from algebraeon import *

x = Nat.Poly().var()

assert((x + 2) ** 2 == x**2 + 4*x + 4)
```

## Factoring Integer Polynomials
```python
from algebraeon import *

x = Int.Poly().var()

poly = -12 * x**2 + 60*x - 72
poly_factored = poly.factor()

print(f"poly                     =", poly)
print(f"poly_factored            =", poly_factored)
# A list of the irreducible factors with their multiplicity
print(f".powers()                =", poly_factored.powers())
# A list of the irreducible factors with repetitions
print(f".irreducibles()          =", poly_factored.irreducibles())
# A list of the irreducible factors without repetitions
print(f".distinct_irreducibles() =", poly_factored.distinct_irreducibles())
# The integer part of the factorisation
print(f".content()               =", poly_factored.content())
# The primitive polynomial part of the factorisation
print(f".primitive()             =", poly_factored.primitive())

"""
Output:
    poly                     = -12λ^2+60λ-72
    poly_factored            = -1 * (2)^2 * (3) * (λ-2) * (λ-3)
    .powers()                = [(Polynomial(2, Int), 2), (Polynomial(3, Int), 1), (Polynomial(λ-2, Int), 1), (Polynomial(λ-3, Int), 1)]
    .irreducibles()          = [Polynomial(2, Int), Polynomial(2, Int), Polynomial(3, Int), Polynomial(λ-2, Int), Polynomial(λ-3, Int)]
    .distinct_irreducibles() = [Polynomial(2, Int), Polynomial(3, Int), Polynomial(λ-2, Int), Polynomial(λ-3, Int)]
    .content()               = - 2^2 × 3
    .primitive()             = 1 * (λ-2) * (λ-3)
"""
```