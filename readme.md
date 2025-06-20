# Easy Configuration Format

## A minimal settings format that strikes a great balance between simplicity for developers and end users

This crate lets you easily load, edit, and store settings with a format (Easy Configuration Format, or ECF) that is intuitive, error-resistant, very minimal, and still very powerful.

### Why use ECF?

- **More intuitive** (and readable) than other formats
  - No escape codes for strings
  - No whitespace shenanigans
  - Syntax is exactly what you'd expect
- **Gracefully handles errors** and continues to parse just fine
- **Preserves layout and comments** even after loading, modifying, then saving
- **Encourages good practices** through the api (but doesn't force anything on you)
- **Extremely fast**, approximately twice as fast as toml (see the ['benchmark' example](examples/benchmark.rs))
- **Extremely lightweight**, ~500 sloc and no dependencies outside std

<br>

## Example settings file:

```txt
format 1
# The first line defines the version number of your settings file.
# If you want to update your program's settings, this will allow
# you to update users' settings file to your newer version

example key: "example value"

example blank: empty
example string: "not empty"
example int: 3
example float: 3.5
example bool: true
example multiline: "
"first line (#0)
"also, because of how strings are stored, you can have " characters
"inside a string with no escape codes needed
"last line (#3)
example string 2: "you can also put " chars in single-line strings"

example namespace.example key: "example value 2"
# "namespaces" are entirely made up, they're just fancy names, but
# it's still the recommended way to structure settings

# example comment

##
example multiline comment
just like strings, you can have extra # chars anywhere you want
(expect for two # chars at the start of a line)
##

# again, this is nothing but a fancy name, so you can start arrays at 1 if you want
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

example duplicate key: "this setting will be kept"
example duplicate key: "this setting will be commented"

invalid key "doesn't have any colon"
invalid value 1: "missing an ending quote
invalid value 2: missing a starting quote"
invalid value 3: missing both quotes
# empty multiline strings aren't allowed:
invalid value 4: "

invalid value 6: .3

invalid entry: empty    # invalid because inline comments aren't allowed

##
invalid multiline comment (no end)

# single-line comments cannot be invalid!

working key: "and even after all that, it can still parse settings!"
```

### See the specification [Here](specification.txt)

<br>

## Example code (full walkthrough):

```rust
// load (and update) settings

pub struct UpdaterFunctionArgs {}
pub const UPDATER_FUNCTIONS: &[fn(&mut HashMap<String, ecf::Value>, &mut UpdaterFunctionArgs)] = &[
	update_1_to_2, // updates from format 1 to format 2
	// etc
]; // because there's 1 updater function, the crate will know that the newest format version is 2

pub fn update_1_to_2(settings: &mut HashMap<String, ecf::Value>, args: &mut UpdaterFunctionArgs) {
	println!("this example doesn't actually have a format 2, this is just to show how updates would be done");
}

let mut update_args = UpdaterFunctionArgs {};
let (mut ecf_file, did_run_updaters, errors) = ecf::File::from_str(include_str!("example_settings.ecf"), UPDATER_FUNCTIONS, &mut update_args); // NOTE: if you want to completely skip updater functions, you can replace `UPDATER_FUNCTIONS` with `&[]`

// if the user removes necessary settings, this can add them back
ecf_file.add_missing_values([
	("This key must exist, and the default (if missing) is Value::I64(64)", ecf::Value::I64(64)),
].into_iter());



// print parsed file data
println!("======== Layout: ========");
for layout_entry in &ecf_file.layout {println!("{layout_entry:?}");}

println!("\n\n\n======== Values: ========");
for (key, value) in &ecf_file.values {println!("{key}: {value:?}");}

println!("\n\n\n======== Parsing Errors: ========");
for error in errors {println!("{error:?}");}



// inspect and edit settings

println!("\n\n\n======== Editing Values: ========");

let example_value_str = ecf_file.get_str("example key");
println!("value in 'example key' as a str: {example_value_str:?}");
let example_value_int = ecf_file.get_int("example key");
println!("value in 'example key' as an int: {example_value_int:?}"); // this prints an `Err` variant

ecf_file.insert(String::from("example key"), ecf::empty());
ecf_file.insert(String::from("new key"), ecf::string("new value"));



// save settings

let (formatted_file, errors) = ecf_file.to_str();

println!("\n\n\n======== New Contents: ========");
println!("\"\"\"");
println!("{formatted_file}");
println!("\"\"\"");

println!("\n\n\n======== Formatting Errors: ========");
for error in errors {println!("{error:?}");}
```
