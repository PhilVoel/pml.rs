# PML - Philipp's Modern Language

Just another format to specify your configs in\
Currently under development, so don't expect full functionality until version 1.0.0\
Create Struct with ̀`let pml = pml::parse_file("file_name")` and get values with `pml.get_int("key")`, `pml.get_float("key")` or `pml.get_string("key")` respectively.\
Save the data in `file_name.pml` in the format

```
TestString=Hello World
testFloat=1.41
test_integer=69
```
