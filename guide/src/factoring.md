# Factoring

Algebraeon implements algorithms for factoring elements belonging to various domains.

## Natural Numbers

Factoring natural numbers:

```python
from algebraeon import *

assert(Nat(12).factor().primes()          == [2, 2, 3])
assert(Nat(12).factor().distinct_primes() == [2, 3])
assert(Nat(12).factor().powers()          == {2 : 2, 3 : 1})

assert(Nat(0).factor().primes()          is None)
assert(Nat(0).factor().distinct_primes() is None)
assert(Nat(0).factor().powers()          is None)

# We can factor numbers much bigger than a naive algorithm is capable of
assert(
    Nat(706000565581575429997696139445280900).factor().powers() 
    == {2: 2, 5: 2, 6988699669998001: 1, 1010203040506070809: 1}
)
```

Checking if a natural number is prime

```python
from algebraeon import *

assert(not Nat(0).is_prime())
assert(not Nat(1).is_prime())
assert(Nat(2).is_prime())
assert(Nat(3).is_prime())
assert(not Nat(4).is_prime())

# The numbers from the factoring example above
assert(Nat(6988699669998001).is_prime())
assert(Nat(1010203040506070809).is_prime())
assert(not Nat(706000565581575429997696139445280900).is_prime())
```


## Integers

Factoring integers:

```python
from algebraeon import *

assert(Int(12).factor().sign()            == 1)
assert(Int(12).factor().primes()          == [2, 2, 3])
assert(Int(12).factor().distinct_primes() == [2, 3])
assert(Int(12).factor().powers()          == {2 : 2, 3 : 1})

assert(Int(-12).factor().sign()            == -1)
assert(Int(-12).factor().primes()          == [2, 2, 3])
assert(Int(-12).factor().distinct_primes() == [2, 3])
assert(Int(-12).factor().powers()          == {2 : 2, 3 : 1})

assert(Int(0).factor().sign()            == 0)
assert(Int(0).factor().primes()          is None)
assert(Int(0).factor().distinct_primes() is None)
assert(Int(0).factor().powers()          is None)
```

Checking if an integer is prime

```python
from algebraeon import *

assert(not Int(-4).is_prime())
assert(Int(-3).is_prime())
assert(Int(-2).is_prime())
assert(not Int(-1).is_prime())
assert(not Int(0).is_prime())
assert(not Int(1).is_prime())
assert(Int(2).is_prime())
assert(Int(3).is_prime())
assert(not Int(4).is_prime())
```

## Integer Polynomials
```python
from algebraeon import *

x = Int.polynomials().var()

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