# Number Types

Algebraeon provides `Nat`, `Int`, and `Rat` types for representing natural numbers, integers, and rational numbers respectively. 

## Creating Numbers

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

## Operations with Numbers

The usual operations are defined for Algebraeon's number types. 

For operations involving more than one number, the type of the result is the largest of the input types. For example, adding a `Nat` to an `Int` produces an `Int`.

```python
from algebraeon import *

# The type of the result is the largest of the input types.
assert(type(Nat(2) + Int(3)) is Int)

assert(Int(4) + 5 == Rat(9))
```