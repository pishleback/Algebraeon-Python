

```python
from algebraeon import *

Nat(3)

Int(3)
Int(Nat(3))

Rat(3)
Rat(Nat(3))
Rat(Int(3))
Rat(3, 5)
Rat(Nat(3), Nat(5))
Rat(Int(3), Int(5))
```


```python
from algebraeon import *

assert(+Nat(2) == 2)

assert(Nat(3) + 5 == 8)
assert(3 + Nat(5) == 8)
assert(Nat(3) + Nat(5) == 8)

assert(Nat(3) * 5 == 15)
assert(3 * Nat(5) == 15)
assert(Nat(3) * Nat(5) == 15)

assert(Nat(3) ** 5 == 243)
assert(Nat(3) ** Nat(5) == 243)
```

```python
from algebraeon import *

assert(+Int(2) == 2)

assert(-Int(2) == -2)

assert(Int(3) + 5 == 8)
assert(3 + Int(5) == 8)
assert(Int(3) + Nat(5) == 8)
assert(Nat(3) + Int(5) == 8)
assert(Int(3) + Int(5) == 8)

assert(Int(3) - 5 == -2)
assert(3 - Int(5) == -2)
assert(Int(3) - Nat(5) == -2)
assert(Nat(3) - Int(5) == -2)
assert(Int(3) - Int(5) == -2)

assert(Int(3) * 5 == 15)
assert(3 * Int(5) == 15)
assert(Int(3) * Nat(5) == 15)
assert(Nat(3) * Int(5) == 15)
assert(Int(3) * Int(5) == 15)

assert(Int(12) / Int(4) == 3)
try:
    Int(12) / Int(0) == 3
except ZeroDivisionError:
    pass
except:
    raise Exception()
try:
    Int(12) / Int(5)
except ValueError:
    pass
except:
    raise Exception()

assert(Int(3) ** 5 == 243)
assert(Int(3) ** Nat(5) == 243)

assert(Int(0) == Int(0))
assert(Int(0) <= Int(0))
assert(not Int(0) < Int(0))
assert(not Int(0) == Int(1))
assert(Int(0) <= Int(1))
assert(Int(0) < Int(1))
assert(not Int(1) == Int(0))
assert(not Int(1) <= Int(0))
assert(not Int(1) < Int(0))
```

```python
from algebraeon import *
from fractions import Fraction as Frac

assert(Rat(Frac(3, 5)) == Rat(3, 5))

assert(+Rat(2) == 2)

assert(-Rat(2) == -2)

assert(Rat(3) + 5 == 8)
assert(3 + Rat(5) == 8)
assert(Rat(3) + Nat(5) == 8)
assert(Nat(3) + Rat(5) == 8)
assert(Rat(3) + Int(5) == 8)
assert(Int(3) + Rat(5) == 8)
assert(Rat(3) + Rat(5) == 8)

assert(Rat(3) - 5 == -2)
assert(3 - Rat(5) == -2)
assert(Rat(3) - Nat(5) == -2)
assert(Nat(3) - Rat(5) == -2)
assert(Rat(3) - Int(5) == -2)
assert(Int(3) - Rat(5) == -2)
assert(Rat(3) - Rat(5) == -2)

assert(Rat(3) * 5 == 15)
assert(3 * Rat(5) == 15)
assert(Rat(3) * Nat(5) == 15)
assert(Nat(3) * Rat(5) == 15)
assert(Rat(3) * Int(5) == 15)
assert(Int(3) * Rat(5) == 15)
assert(Rat(3) * Rat(5) == 15)

assert(Rat(12) / Rat(4) == 3)
try:
    Rat(12) / Rat(0) == 3
except ZeroDivisionError:
    pass
except:
    raise Exception()
assert(Rat(12) / Rat(5) == Rat(12, 5))

assert(Rat(3) ** 5 == 243)
assert(Rat(3) ** Nat(5) == 243)

assert(Rat(0) == Rat(0))
assert(Rat(0) <= Rat(0))
assert(not Rat(0) < Rat(0))
assert(not Rat(0) == Rat(1))
assert(Rat(0) <= Rat(1))
assert(Rat(0) < Rat(1))
assert(not Rat(1) == Rat(0))
assert(not Rat(1) <= Rat(0))
assert(not Rat(1) < Rat(0))
```


```python
from algebraeon import *

assert(Nat(0).factor().powers() is None)
assert(Nat(1).factor().powers() == {})
assert(Nat(2).factor().powers() == {2 : 1})
assert(Nat(12).factor().powers() == {2 : 2, 3 : 1})

assert(Nat(0).factor().primes() is None)
assert(Nat(1).factor().primes() == [])
assert(Nat(2).factor().primes() == [2])
assert(Nat(12).factor().primes() == [2, 2, 3])

assert(Nat(0).factor().distinct_primes() is None)
assert(Nat(1).factor().distinct_primes() == [])
assert(Nat(2).factor().distinct_primes() == [2])
assert(Nat(12).factor().distinct_primes() == [2, 3])

assert(Nat(0).is_prime() == False)
assert(Nat(1).is_prime() == False)
assert(Nat(2).is_prime() == True)
assert(Nat(3).is_prime() == True)
assert(Nat(4).is_prime() == False)
```