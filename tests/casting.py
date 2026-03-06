from algebraeon import *

# Subtype casts
Nat(3)

Int(3)
Int(Nat(3))

Rat(3)
Rat(Nat(3))
Rat(Int(3))
Rat(3, 5)
Rat(Nat(3), Nat(5))
Rat(Int(3), Int(5))

# Supertype casts

Int(Rat(3))
Nat(Rat(3))
Nat(Int(3))

Int(Rat(-3))
try:
    Nat(-3)
except ValueError:
    pass
else:
    raise Exception()
try:
    Nat(Rat(-3))
except ValueError:
    pass
else:
    raise Exception()
try:
    Nat(Int(-3))
except ValueError:
    pass
else:
    raise Exception()
