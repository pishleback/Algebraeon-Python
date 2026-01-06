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