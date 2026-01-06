# Polynomials

## Integer Polynomials

```python
from algebraeon import *

x = Int.polynomials().var()

assert((x + 2) ** 2 == x**2 + 4*x + 4)

assert((x**2 + 3*x + 2) / (x + 1) == x + 2)

# not divisible
try:
    x / (2 * x)
except ValueError:
    pass
else:
    raise Exception()
```

## Rational Polynomials

```python
from algebraeon import *

x = Rat.polynomials().var()

assert((x + 2) ** 2 == x**2 + 4*x + 4)

assert((x**2 + 3*x + 2) / (x + 1) == x + 2)

assert(x / (2 * x) == Rat(1, 2))
```

<!-- ## Natural Number Polynomials

```python
from algebraeon import *

x = Nat.polynomials().var()

assert((x + 2) ** 2 == x**2 + 4*x + 4)
``` -->