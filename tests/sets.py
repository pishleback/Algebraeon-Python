from algebraeon import *

assert(Nat(3).set() == Nat)
assert(Int(3).set() == Int)
assert(Rat(3).set() == Rat)

assert(str(Nat) == "ℕ")
assert(repr(Nat) == "Nat")
assert(str(Nat(3)) == "3")
assert(repr(Nat(3)) == "Nat(3)")

assert(str(Int) == "ℤ")
assert(repr(Int) == "Int")
assert(str(Int(3)) == "3")
assert(repr(Int(3)) == "Int(3)")

assert(str(Rat) == "ℚ")
assert(repr(Rat) == "Rat")
assert(str(Rat(3)) == "3")
assert(str(Rat(2, 3)) == "2/3")
assert(repr(Rat(3)) == "Rat(3)")