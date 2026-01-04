```python
from algebraeon import *

+Nat(2)

Nat(3) + 5
3 + Nat(5)
Nat(3) + Nat(5)

Nat(3) * 5
3 * Nat(5)
Nat(3) * Nat(5)

Nat(3) ** 5
Nat(3) ** Nat(5)
```

```python
from algebraeon import *

+Int(2)

-Int(2)

Int(3) + 5
3 + Int(5)
Int(3) + Nat(5)
Nat(3) + Int(5)
Int(3) + Int(5)

Int(3) - 5
3 - Int(5)
Int(3) - Nat(5)
Nat(3) - Int(5)
Int(3) - Int(5)

Int(3) * 5
3 * Int(5)
Int(3) * Nat(5)
Nat(3) * Int(5)
Int(3) * Int(5)

Int(3) ** 5
Int(3) ** Nat(5)
```

```python
from algebraeon import *

+Rat(2)

-Rat(2)

Rat(3) + 5
3 + Rat(5)
Rat(3) + Nat(5)
Nat(3) + Rat(5)
Rat(3) + Int(5)
Int(3) + Rat(5)
Rat(3) + Rat(5)

Rat(3) - 5
3 - Rat(5)
Rat(3) - Nat(5)
Nat(3) - Rat(5)
Rat(3) - Int(5)
Int(3) - Rat(5)
Rat(3) - Rat(5)

Rat(3) * 5
3 * Rat(5)
Rat(3) * Nat(5)
Nat(3) * Rat(5)
Rat(3) * Int(5)
Int(3) * Rat(5)
Rat(3) * Rat(5)

Rat(3) ** 5
Rat(3) ** Nat(5)
```