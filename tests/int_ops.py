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
except ValueError:
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