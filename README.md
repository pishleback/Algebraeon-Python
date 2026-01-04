# Algebraeon (Python Module)

This is a Python wrapper around [the Rust library of the same name](https://github.com/pishleback/Algebraeon).

It currently supports only a _very_ small subset of the capabilities of the Rust library. Once it has been better fleshed out it will be the recommended way to use Algebraeon for people who care more about maths than software.

The latest release is published to PyPI [here](https://pypi.org/project/algebraeon/).

There is [![Discord](https://img.shields.io/badge/Discord-Join%20Chat-7289DA?logo=discord&logoColor=white)](https://discord.gg/DBqbPqPMKR) for informal discussions about Algebraeon.

See the [User Guide](https://pishleback.github.io/Algebraeon-Python/) to get started using Algebraeon with Python.

# Development

## Running Tests
The Python codeblocks in the User Guide can executed as tests by the following steps:

One time setup:
1. `cd` into `algebraeon`.
2. Run `python3 -m venv .env` to create a Python venv.
3. Run `source .env/bin/activate` to ender the venv.
4. Run `pip install maturin`. `maturin` is the tool used to build the Python module. Don't run `pip install algebraeon`; this venv is for installing the locally build `algebraeon` Python module into, which is handled by `maturin`.

To build the `algebraeon` Python module locally and run the tests in the User Guide against the local build:
1. `cd` into `examples`.
2. Run `cargo run`.

