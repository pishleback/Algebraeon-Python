# Algebraeon (Python Module)

Algebraeon is a computational algebra system (CAS) written purely in Rust. It implements algorithms for working with matrices, polynomials, algebraic numbers, factorizations, etc. The focus is on exact algebraic computations over approximate numerical solutions. Algebraeon is in early stages of development and the API subject to change.

The Python library currently supports only a _very_ small subset of the capabilities of the Rust library. Once it has been better fleshed out it will be the recommended way to use Algebraeon for people who care more about maths than software.

 - See the [User Guide](https://pishleback.github.io/Algebraeon-Python/) to get started.
 - There is a [![Discord](https://img.shields.io/badge/Discord-Join%20Chat-7289DA?logo=discord&logoColor=white)](https://discord.gg/DBqbPqPMKR) server for informal discussions about Algebraeon.
 - [Published to PyPI](https://pypi.org/project/algebraeon/).
 - [GitHub for the Rust library](https://github.com/pishleback/Algebraeon).

# Examples

## Factoring

```python
from algebraeon import *

# Algebraeon can factor numbers with much 
# bigger prime factors than a naive algorithm is capable of.
assert(
    Nat(706000565581575429997696139445280900).factor().powers() 
    == {2: 2, 5: 2, 6988699669998001: 1, 1010203040506070809: 1}
)
```

# Development

## Running Tests
The Python codeblocks in the User Guide can executed as tests by the following steps:

One time setup:
1. `cd` into `algebraeon`.
2. Run `python3 -m venv .env` to create a Python venv.
3. Run `source .env/bin/activate` to ender the venv.
4. Run `pip install maturin`. `maturin` is the tool used to build the Python module. Don't run `pip install algebraeon`; this venv is for installing the _locally_ built version of `algebraeon`, which is handled by `maturin`.

To build the `algebraeon` Python module locally and run the tests in the User Guide against the local build:
1. `cd` into `examples`.
2. Run `cargo run`.
