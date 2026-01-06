from algebraeon import *

assert(+Nat(2) == 2)

assert(Nat(3) + 5 == 8)
assert(3 + Nat(5) == 8)
assert(Nat(3) + Nat(5) == 8)

assert(Nat(5) - Nat(3) == 2)
assert(Nat(5) - 3 == 2)
assert(5 - Nat(3) == 2)
try:
    Nat(3) - Nat(5)
except ValueError:
    pass
else:
    raise Exception()

assert(-Nat(0) == 0)
try:
    -Nat(5)
except ValueError:
    pass
else:
    raise Exception()

assert(Nat(3) * 5 == 15)
assert(3 * Nat(5) == 15)
assert(Nat(3) * Nat(5) == 15)

assert(Nat(3) ** 5 == 243)
assert(Nat(3) ** Nat(5) == 243)