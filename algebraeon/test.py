from algebraeon import *



print(repr(Nat(1)))
print(repr(Nat(Nat(2))))

print(repr(Int(3)))
print(repr(Int(Nat(4))))
print(repr(Int(Int(5))))

print(repr(Rat(6)))
print(repr(Rat(Nat(7))))
print(repr(Rat(Int(8))))
print(repr(Rat(Rat(9))))

print(repr(Rat(2)))

# x = foo(168)
# print(repr(x))
# y = foo(x)
# print(repr(y))

