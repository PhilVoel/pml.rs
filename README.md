# PML - Philipp's Modern Language

Just another format to specify your configs in\
Currently under development, so don't expect full functionality until version 1.0.0

## Code example

*.rs

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
```

testFile.pml

```pml
age= <u32> 420;
first_name = "Max";
"name and age" = |first_name, last_name| " "|age|;
last_name = "Mustermann";
stayAnonymous = true;
friends = [
    {
        name= "Person";
        past_ages= <u8> [
            0,1,2,
            3 , 4,5
        ]
    }
    {
        name= |..first_name|;
        past_ages = <u8> [
            0,1,2,3,4,
        ]
    }
]

```
