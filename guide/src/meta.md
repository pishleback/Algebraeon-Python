# Meta

## Checking Versions

To check which version of the python library `algebraeon` is installed:

```python
import algebraeon
assert(algebraeon.algebraeon_python_library_version() == "0.0.1")
```

To check which version of [the Rust library for algebraeon](https://github.com/pishleback/Algebraeon) is being used behind the scenes:

```python
import algebraeon
assert(algebraeon.algebraeon_rust_library_version() == "0.0.14")
```



