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