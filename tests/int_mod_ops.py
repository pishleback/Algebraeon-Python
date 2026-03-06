from algebraeon import *

mod10 = Int.mod(10)

assert(+mod10(7) == mod10(7))
assert(-mod10(7) == mod10(3))
assert(mod10(7) + mod10(8) == mod10(5))
assert(mod10(3) - mod10(8) == mod10(5))
assert(mod10(8) * mod10(9) == mod10(2))
assert(mod10(3) ** 555     == mod10(7))
