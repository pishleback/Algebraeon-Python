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