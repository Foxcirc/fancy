
# Fancy

This crate makes it easy to print colored text to the terminal.

Color formatting can be done using the `colorize!` macro. It takes a color format strig and expands to the original text, with the color modes replaced by an ANSI sequence. 
Just like for `format!` there is a shortcut macro `printcol!` which prints the colored text directly to the console.

```rust
use fancy::printcoln;
printcoln!("[bold|cyan]Hello world[magenta]!");
```

All the color formating macros, do normal formatting too.

```rust
use fancy::printcoln;
printcoln!("[bold]{:x} {:x} [red]world!", 104, 105);
```

Results could look like this:
![Colored Text](https://imgur.com/a/GoPlhaR "Colored Text")

*Note:*
Currently escaping can only be done one level.
`[[escaped]]` will print the same as `[[[[escaped]]]]`
