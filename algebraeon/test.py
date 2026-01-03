from algebraeon import *

print(Nat(1))
print(Nat(Nat(2)))
print(Nat(Int(3)))
print(Nat(Rat(4)))

print(Int(-5))
print(Int(Nat(6)))
print(Int(Int(-7)))
print(Int(Rat(-8)))

print(Rat(-9))
print(Rat(Nat(10)))
print(Rat(Int(-11)))
print(Rat(Rat(-12)))



print(repr(Poly([1, 2, 3])))
print(repr(Poly(Int(6))))


print(Nat(Poly(7)))


# x = foo(168)
# print(repr(x))
# y = foo(x)
# print(repr(y))

