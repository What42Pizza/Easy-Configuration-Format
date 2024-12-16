# Easy Configuration Format

### A settings format that strikes a great balance between usage simplicity and parsing simplicity

<br>

Example settings file:

```
format 1

example key: "example value"

example blank: empty
example string: "not empty"
example int: 3
example float: 3.5
example bool: true
example multiline: "
"first line (#0)
"also, because of how strings are defined, you can have " characters inside a string with
"no escape codes needed
"last line (#3)

example namespace.example key: "example value 2"
# "namespaces" are entirely made up, there's no direct support for them and they're just
# a recommended way to structure settings

# example comment

##
example multiline comment
just like strings, you can have extra # chars anywhere you want (as long as you don't 
want one of the lines in a comment to just be "##")
##

example array.0: "value 0"
example array.1: "value 1"
example array.2: "value 2"
example array.3: "value 3"

example nested array.0.name: "person 0"
example nested array.0.age: "age 0"
example nested array.0.friends.0: "person 1"
example nested array.0.friends.1: "person 2"

example nested array.1.name: "person 1"
example nested array.1.age: "age 1"
example nested array.1.friends.0: "person 0"
example nested array.1.friends.1: "person 2"



# examples for error handling:

example duplicate key: "this key will be kept"
example duplicate key: "this key will be commented"

invalid key "doesn't have any colon"
invalid value 1: "missing an ending quote
invalid value 2: missing a starting quote"
invalid value 3: missing both quotes
# empty multiline strings aren't allowed:
invalid value 4: "

invalid value 6: .3

invalid entry: empty # inline comments aren't allowed

##
invalid multiline comment, only these two lines will be commented because of this

# single-line comments cannot be invalid!

working key: "and even after all that, it still parses (and reformats) continued data!"

```

<br>

Example code:

```rust
// load settings

pub const UPDATER_FUNCTIONS: &[fn(&mut HashMap<String, ecf::Value>, &())] = &[
	update_1_to_2, // updates from format 1 to format 2
	// etc
];

pub fn update_1_to_2(settings: &mut HashMap<String, ecf::Value>, args: &()) {
	println!("this example doesn't actually have a format 2, this is just to give an idea of how updates would be done");
}

let (mut ecf_file, errors) = ecf::parse_settings(include_str!("example_settings.txt"), UPDATER_FUNCTIONS, &());

// print file data
println!("======== Layout: ========");
for layout_entry in &ecf_file.layout {println!("{layout_entry:?}");}

println!("\n\n\n======== Values: ========");
for (key, value) in &ecf_file.values {println!("{key}: {value:?}");}

println!("\n\n\n======== Errors: ========");
for error in errors {println!("{error:?}");}



// alter settings

ecf_file.insert(String::from("example key"), ecf::Value::Empty);
ecf_file.insert(String::from("new key"), ecf::Value::String (String::from("new value")));



// save settings

let (contents, errors) = ecf::format_settings(&ecf_file);

println!("\n\n\n======== New Contents: ========");
println!("\"\"\"");
println!("{contents}");
println!("\"\"\"");

println!("\n\n\n======== Errors: ========");
for error in errors {println!("{error:?}");}
```
