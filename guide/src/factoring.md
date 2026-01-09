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

assert(Int(12).factor().sign()             == 1)
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