# Numbers

## Naturals, Integers, and Rationals

Algebraeon provides `Nat`, `Int`, and `Rat` types for representing natural numbers, integers, and rational numbers respectively. 

### Constructing Numbers

The types can all be constructed using the primitive `int` type.

```python
from algebraeon import *

# Construct a `Nat` from non-negative primitive `int`s.
Nat(0)
Nat(7)

# ValueError because it is possible to construct a 
# `Nat` from a non-negative primitive `int`.
try:
    Nat(-5)
except ValueError:
    pass
else:
    raise Exception()

# Construct an `Int` from primitive `int`s.
Int(-7)
Int(0)
Int(7)

# Construct a `Rat` from primitive `int`s.
Rat(-7)
Rat(0)
Rat(7)
Rat(2, 3) # 2/3
```

Rational numbers can also be constructed from instaces of `Fraction` from the `fractions` module of the standard library.

```python
from algebraeon import *
import fractions

assert(Rat(fractions.Fraction(3, 5)) == Rat(3) / Rat(5))
```

They can also be implicitly casted to larger sets, but not to smaller ones.

```python
from algebraeon import *

# Creating numbers in a larger set from numbers in a smaller set.
Int(Nat(5))
Rat(Nat(5))
Rat(Int(-5))

# Numbers can be created from the same set too.
Nat(Nat(3))
Int(Int(-3))
Rat(Rat(3))

# TypeError because `Int` is a strictly larger set than 
# `Nat` so implicit conversion is not allowed.
try:
    Nat(Int(5))
except TypeError:
    pass
else:
    raise Exception()
```

### Operations

The usual operations are defined for Algebraeon's number types. 

For operations involving more than one number, the type of the result is the largest of the input types. For example, adding a `Nat` to an `Int` produces an `Int`.

```python
from algebraeon import *

# The type of the result is the largest of the input types.
assert((Nat(2) + Int(3)).set() == Int)

assert(Int(4) + 5 == Rat(9))

assert(Int(-3) ** 3 == -27)
```

Division exampes:

```python
from algebraeon import *

# Integer division is ok as long as the result is an integer.
assert(Int(6) / Int(2) == 3)
try:
    Int(7) / Int(3)
except ValueError:
    pass
else:
    raise Exception()

# Rational division is ok, as long as we're not dividing by 0.
assert(Rat(6) / Rat(2) == 3)
assert(Rat(7) / Rat(3) == Rat(7, 3))

# Division by 0 raises a `ValueError`.
try:
    Int(2) / Int(0)
except ValueError:
    pass
else:
    raise Exception()
```

## Modular Arithmetic

Operations modulo \\(10\\).

```python
from algebraeon import *

mod10 = Int.mod(10)

assert(mod10(7) + mod10(8) == mod10(5))
assert(mod10(3) - mod10(8) == mod10(5))
assert(mod10(8) * mod10(9) == mod10(2))
assert(mod10(3) ** 555     == mod10(7))
```

Modular inverses

```python
from algebraeon import *

# 3 * 21 = 1 mod 31
mod31 = Int.mod(31)
assert(mod31(3) ** -1 == mod31(21))

# 5 * 13 = 1 mod 16
mod16 = Int.mod(16)
assert(mod16(5) ** -1 == mod16(13))

# 4 has no inverse mod 12
mod12 = Int.mod(12)
try:
    mod12(4) ** -1
except ValueError:
    pass
else:
    raise Exception()
```

Automatic casting between moduli.

```python
from algebraeon import *

mod10 = Int.mod(10)
mod100 = Int.mod(100)

assert(mod100(71) + mod10(8) == 9)
```