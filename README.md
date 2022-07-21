
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

![Screenshot](doc/example.png?raw=true "FancyText")

Note that not every console supports all features this crate offers.
Some consoles like windows command prompt need you to enable support first and
even then some default colors might not render correctly, depending on the color
palette.

If you have any issues, you can tell me on github or via mail.
I Hope this crate is usefull and functional for you :)

# Changes

## 0.2.3 -> 0.3.0
- fixed a bug, where a string containing an escaped double quote would not parse correctly
- fixed escaping of square brackets, they can now be escaped correctly
- partially rewrote the parsing

## 0.1.2 -> 0.2.3
- minor bug, documentation and stability fixes
