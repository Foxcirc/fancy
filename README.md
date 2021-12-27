
# Fancy

Easily print colored output using ANSI sequences.

This crate makes it easy to print colored text to the terminal.
There is no need to `use` anything then just the format macros themselfs.

Color formatting can be done using the [`colorize!`] macro. It takes a color format strig and expands to the original text, with the color modes replaced by an ANSI sequence. 
Just like for [`format!`] there is a shortcut macro [`printcol!`] which prints the colored text directly to the console.

```rust
use fancy::printcoln;
printcoln!("[bold|cyan]Hello world[magenta]!");
```

All the color formating macros, do normal formatting too.

```rust
use fancy::printcoln;
printcoln!("[bold]{:x} {:x} [red]world!", 104, 105);
```
