# PML - Philipp's Modern Language

Just another format to specify your configs in\
Currently under development, so don't expect full functionality until version 1.0.0

## Code example

*.js

```rust
use pml::parse::file as pml_parse;

fn main() {
	let pml_result = pml_parse("testFile.pml");
	match pml_result {
		Err(e) => println("{e:#?}"),
		Ok(result) => {
			if(result.get::<bool>("stayAnonymous").is_some()) {
				println!("I won't tell you anything about me.");
			}
			else {
				println!("Hi, my name is {} and I am {} years old.", result.get::<String>("name").unwrap(), result.get::<&u64>("age").unwrap());
			}
		}
	}
}
const pmlResult = pmlParser("testFile.pml");
if(!pmlResult.successful)
    console.log(pmlResult.error);
else {
    const result = pmlResult.result;
    if(result.stayAnonymous)
        console.log("I won't tell you anything about me.");
    else
        console.log("Hi, my name is " + result.name + " and I am " + result.age + " years old.");
}
```

testFile.pml

```pml
age=420
first_name = "Max"
name = {first_name} " "{last_name}
last_name = "Mustermann"
stayAnonymous = true

```

Note that you can have strings consist of other values (It wouldn't even have to have been other strings; I could have added the age as part of the name, too!).
Numbers and booleans are recognized as such and numbers are saved as the most memory efficient type.
Whitespaces between keys, values, the equal sign and the different parts of strings do not matter at all.
You can get the values either as a copy/clone or as reference. If you copy/clone them you can upcast numbers to a bigger type and everything can be converted to a String.
