
//!
//! Easily print colored output using ANSI sequences.
//! 
//! This crate makes it easy to print colored text to the terminal.
//! There is no need to `use` anything then just the format macros themselfs.
//! 
//! Color formatting can be done using the [`colorize!`] macro.
//! It takes a color format strig and expands to the original text,
//! with the color modes replaced by an ANSI sequence. 
//! Just like for [`format!`] there is a shortcut macro [`printcol!`]
//! which prints the colored text directly to the console.
//! 
//! ```rust
//! use fancy::printcoln;
//! printcoln!("[bold|cyan]Hello world[magenta]!");
//! ```
//! 
//! All the color formating macros, do normal formatting too.
//! 
//! ```rust
//! use fancy::printcoln;
//! printcoln!("[bold]{:x} {:x} [red]world!", 104, 105);
//! ```
//! 
//! # Color modes
//! 
//! There are many different `color modes` (or `color format arguments`).
//! Color modes can be applied using square-brackets `[` `]`.
//! Multiple color modes must be seperated a pipe `|`.
//! The colon `:` is used to reset all modes.
//! It can just be prependet to any mode. (Eg. `[bold]Hello[:italic]world!`)
//! All color modes are reset at the end of the formatted string.
//! 
//! `[bold|underline|blue]` prints any text following it in bold, underlined and colored blue.
//! 
//! `[green]` prints any text following it colored green
//! 
//! # List of modes
//! 
//! This is a list of all available color modes.
//! 
//! ### Styles
//! 
//! | Name          | Short     | Description                    | Markdown |
//! |---------------|-----------|--------------------------------|----------|
//! | bold          | b         | Write text bold.               | **text** |
//! | italic        | i         | Write text in italics.         | *text*   |
//! | underline     | u         | Write text underlined.         |          |
//! | inverse       | !         | Inverse of the default colors. |          |
//! | strikethrough | s         | Write text strikethrough.      | ~~text~~ |
//! | :             |           | Reset all modes.               |          |
//!
//! ### Colors
//! 
//! | Name           |
//! |----------------|
//! | black          |
//! | red            |
//! | green          |
//! | yellow         |
//! | blue           |
//! | magenta        |
//! | cyan           |
//! | white          |
//! | default / def  |
//!
//! ### Background color
//! 
//! Prepent any color with a question mark to use it as a background color.
//! 
//! | Name             |
//! |------------------|
//! | ?black           |
//! | ?red             |
//! | ?green           |
//! | ?yellow          |
//! | ?blue            |
//! | ?magenta         |
//! | ?cyan            |
//! | ?white           |
//! | ?default / ?def  |
//! 
//! ### Custom colors
//! 
//! This crate supports "ANSI color codes" as well as raw RGB codes (Although
//! not every terminal supports truecolor).
//! You can prepend any of these color codes with a question mark `?` to set the background color.
//! 
//! `69`   - sets the foreground to an ansi code
//! `?187` - sets the background to an ansi code
//! `#ababd2` - sets the foreground to a hex rgb value
//! `?#ababd2` - sets the background to a hex rgb value
//! 
//! Ansi color code: `[bold|214]Hello world!` This is a light orange.
//! Hex color code:  `[#babaf1|u]Hi there!` This is a light blue/purple-ish color.
//!
//! # Escaping
//! 
//! Escaping is done like with normal formatting braces.
//! 
//! ```rust
//! use fancy::*;
//! eprintcoln!("[b|r]error[:] at [[{}:{}]][b]: {}", line, column, message);
//! ```
//! 
//! # Examples
//! 
//! ```rust
//! 
//! use fancy::*;
//! 
//! printcoln!("Hello {}", "world!"); // plain "Hello world!"
//! printcoln!("[bold]Hello world"); // bold "Hello world!"
//! printcoln!("[b|u]Hello world"); // bold and underline "Hello world!"
//! printcoln!("[b|#7cd615]space[#c1973c]space!"); // lime/orange "spacespace"
//! printcoln!("H[!]E[:]L[!]L[:]L[!]O[:]"); // black/white "HELLO"
//!
//! printcoln!("[b|u|#babaf1]rust[:] is [!]{}", "cool");
//! 
//! let error = "ANSI codes are not supportet for this terminal.";
//! eprintcoln!("[b|red]ERROR[:b]: [:]{}", error);
//! 
//! ```
//! 
//! # Comparison to other crates
//! 
//! This crate uses a different approach then most other color formatting crates.
//! The inline color formatting syntax is not as flexible as some other approaches but
//! in my opinion it is a bit easier to write and read.
//! 
//! Since this crate uses a procedural macro for generating the color sequences,
//! coloring shouldn't impact the runtime performance.
//! 

use proc_macro::{TokenStream/* , Diagnostic, Level, Span */};
use std::{ops::Range, iter::repeat};

 /// Print a colorized string.
/// 
/// This macro can be used like [`print!`] and accepts format arguments.
/// Just like for [`print!`] there are versions for this macro that append a newline and/or print to stderr.
/// 
/// # Example
///
///  ```rust
/// use fancy::printcol;
/// printcol!("[bold|#babaf1]rust is [!]{}{}", "cool", "\n");
/// ```
/// 
/// # Note
///
/// Color format arguments parsed in as a format argument are ignored.
/// 
/// ```rust
/// use fancy::printcol;
/// printcol!("{}I am not bold!", "[bold]"); // "[bold]I am not bold!"
/// ```
/// 
/// For more information see the crate level documentation;
#[proc_macro]
pub fn printcol(grammar: TokenStream) -> TokenStream {
    format!(r#"::std::print!("{{}}", ::fancy::colorize!({}));"#, grammar.to_string()).parse().unwrap()
}

/// Print a colorized string to stderr.
/// For more information see [`printcol!`];
#[proc_macro]
pub fn eprintcol(grammar: TokenStream) -> TokenStream {
    format!(r#"::std::eprint!("{{}}", ::fancy::colorize!({}));"#, grammar.to_string()).parse().unwrap()
}

/// Print a colorized string followed by a newline.
/// This macro is to [`printcol!`] as [`println`] is to [`print!`].
/// For more information see [`printcol!`];
#[proc_macro]
pub fn printcoln(grammar: TokenStream) -> TokenStream {
    format!("::fancy::printcol!({}); ::std::println!()", grammar.to_string()).parse().unwrap()
}

/// Print a colorized string to stderr followed by a newline.
/// This macro is to [`eprintcol!`] as [`eprintln!`] is to [`eprint!`].
/// For more information see [`printcol!`];
#[proc_macro]
pub fn eprintcoln(grammar: TokenStream) -> TokenStream {
    format!("::fancy::eprintcol!({}); ::std::println!()", grammar.to_string()).parse().unwrap()
}

/// Colorize and format a string.
/// 
/// This macro interprets a color formatted string. It doesn't print anything,
/// but returns the formatted string.
/// It is to [`printcol!`] as [`format!`] is to [`print!`].
/// 
/// For more information see the crate level documentation.
#[proc_macro]
pub fn colorize(grammar: TokenStream) -> TokenStream {

    // get the input data
    let text = grammar.to_string();
    let mut colored = ColorString::new(text.capacity());
    let len = text.chars().count();

    // the start and end of the format args section of the data
    // eg. colorize!("{} world!", "hello")
    //                          ^--------^
    let mut formats = 0..len;

    // brace matching
    // += 1 for [
    // -= 1 for ]
    let mut braces: isize = 0;
    // incremented by one for each escaping brace
    // eg. [[[[ => escaped = 3
    let mut escaped = 0;

    let mut parsing = (false, 0..0);

    let mut prev = '\0';
    for (idx, chr) in text.chars().enumerate() {

        // update the variable
        parsing.0 = braces > 0 && escaped == 0;

        // handle the escaping
        if chr == '[' && prev == '[' { escaped += 1; braces = 0 }
        else if chr == ']' && prev == ']' { escaped += 1; braces = 0 }
        
        else if chr != '[' && prev == '[' && escaped > 0 {
            for br in repeat('[').take(escaped) { colored.push(br) };
            escaped = 0;
        }
        else if chr != ']' && prev == ']' && escaped > 0 {
            for br in repeat(']').take(escaped) { colored.push(br) };
            escaped = 0;
        }
        
        // handle the patterns (else-if is important here)
        else if chr == '[' {
            if parsing.0 { panic!("cannot have a opening square brace inside a pattern") }
            braces += 1;
            parsing.1.start = idx + 1;
        }
        else if chr == ']' && parsing.0 && braces == 1 {
            braces -= 1;
            // the pattern is closed with this char
            // parse now:
            parsing.1.end = idx;
            parse(&text[parsing.1.clone()], &mut colored);
        }
        else if chr == ']' {
            
        }

        // if this is the end of the string literal, we make the format argument section
        // start here
        else if chr == '"' && prev != '\\' && idx != 0 {
            formats.start = idx + 1;
            break
        }

        // ignore the first "
        else if chr == '"' && idx == 0 {}

        // push other char's onto the string
        else if !parsing.0 {
            colored.push(chr);
        }

        // else {
        //     panic!("unhandeled char case: chr = {chr:?}, prev = {prev:?}, pasing = {parsing:?}, braces = {braces:?}, escaped = {escaped:?}")
        // }

        prev = chr;

    }

    if braces != 0 {
        panic!("braces did not match, you either have to many `[` or to many `]`")
    }

    // append a reset character, so that all color styles are reset
    colored.raw("\x1b[0m");
    let output = colored.view();

    if formats.is_empty() {

        format!(
            "\"{}\"",
            output,
        ).parse().expect("Could not parse output.")
        
    } else {

        // formats eg. colorize!("{}[green]hey!", "John says ")
        //                                      ^------------^
        return format!(
            "format!(\"{}\"{})",
            output, &text[formats]
        ).parse().expect("Could not parse output.")
        
    }

}

fn parse(text: &str, buffer: &mut ColorString) -> bool {

    let mut view = Range { start: 0, end: text.len() };

    // reset
    if text.chars().nth(0) == Some(':') {
        buffer.raw("\x1b[0m");
        view.start = 1;
    };

    // parse modifiers
    let modifiers = (&text[view]).split('|');
    for modi in modifiers {

        match &modi[..] {

            // styles
            "bold"          | "b"     => buffer.add("1"),
            "dim"           | "faint" => buffer.add("2"),
            "italic"        | "i"     => buffer.add("3"),
            "underline"     | "u"     => buffer.add("4"),
            "inverse"       | "!"     => buffer.add("7"),
            "hidden"                  => buffer.add("8"),
            "strikethrough" | "s"     => buffer.add("9"),

            // "bright" | "br"   => buffer.add("1"),
            
            // foreground colors
            "black"                 => buffer.add("30"),
            "red"                   => buffer.add("31"),
            "green"                 => buffer.add("32"),
            "yellow"                => buffer.add("33"),
            "blue"                  => buffer.add("34"),
            "magenta"               => buffer.add("35"),
            "cyan"                  => buffer.add("36"),
            "white"                 => buffer.add("37"),
            "default" | "def" | "r" => buffer.add("39"),
            
            // background colors
            "?black"            => buffer.add("40"),
            "?red"              => buffer.add("41"),
            "?green"            => buffer.add("42"),
            "?yellow"           => buffer.add("43"),
            "?blue"             => buffer.add("44"),
            "?magenta"          => buffer.add("45"),
            "?cyan"             => buffer.add("46"),
            "?white"            => buffer.add("47"),
            "?default" | "?def" => buffer.add("49"),

            "visible" | "vis" => buffer.raw("\x1b[?25l"),
            "invisible" | "invis" => buffer.raw("\x1b[?25h"),

            "blink" => buffer.raw("\x1b[5m"),
            "noblink" => buffer.raw("\x1b[25m"),
            
            seq => {
                
                let isdec = |seq: &str| seq.chars().all(|v| matches!(v, '0'..='9'));
                let ishex = |seq: &str| seq.chars().all(|v| matches!(v, '0'..='9' | 'a'..='f' | 'A'..='B'));

                // ansi color id
                if (1..=3).contains(&seq.len()) && isdec(seq) {
                    buffer.add(&format!("38;5;{}", seq));
                }
                
                // ansi color id background
                else if (2..=4).contains(&seq.len()) && &seq[0..=1] == "?" && isdec(&seq[1..]) {
                    buffer.add(&format!("48;5;{}", seq));
                }
                
                // ansi color id background
                else if (4..).contains(&seq.len()) && (&seq[0..=1] == "?" && isdec(&seq[1..])) | isdec(&seq[..]) {
                                    
                    // Diagnostic::spanned(
                    //     Span::call_site(),
                    //     Level::Error,
                    //     format!("Invalid ansi color code: {}", seq))
                    //     .note("Ansi color codes must be in range 0..255.")
                    //     .emit();
                        // todo actually check for the ranges of the color code (0.255)
                    panic!("Invalid ansi color code: {}. Ansi color codes must be in range 0.155.", seq);

                }
                
                // hex color code
                else if seq.len() == 7 && &seq[0..1] == "#" && ishex(&seq[1..]) {
                    let red = u8::from_str_radix(&seq[1..=2], 16).unwrap();
                    let green = u8::from_str_radix(&seq[3..=4], 16).unwrap();
                    let blue = u8::from_str_radix(&seq[5..=6], 16).unwrap();
                    buffer.add(&format!("38;2;{};{};{}", red, green, blue));
                }
                
                // hex color code background
                else if seq.len() == 8 && &seq[0..=1] == "?#" && ishex(&seq[2..]) {
                    let red = u8::from_str_radix(&seq[2..=3], 16).unwrap();
                    let green = u8::from_str_radix(&seq[4..=5], 16).unwrap();
                    let blue = u8::from_str_radix(&seq[6..=7], 16).unwrap();
                    buffer.add(&format!("48;2;{};{};{}", red, green, blue));
                }
                
                // invalid hex color code 
                else if (2..).contains(&seq.len()) && (&seq[0..=1]).contains('#') {
                    
                    // Diagnostic::spanned(
                    //     Span::call_site(),
                    //     Level::Error,
                    //     format!("Invalid hex color code: {}", seq))
                    //     .note("Hex color codes must be a '#' or '?#' followed by exactly 6 hex-digits.")
                    //     .emit();
                    
                    panic!("Invalid hex color code: {}. Hex color codes must be a '#' or '?#' followed by exactly 6 hex-digits.", seq);
                    
                    // return false;

                }
                
                else if !seq.is_empty() {
                    panic!("{}", format!("Unknown modifier: {:?}", seq));
                }

            },

        }
        
    }

    buffer.next();
    true

}

#[derive(Debug)]
struct ColorString {
    text: String,
    next: String,
}

impl ColorString {

    pub(crate) fn new(cap: usize) -> Self {
        Self { text: String::with_capacity(cap), next: String::with_capacity(16) }
    }

    pub(crate) fn add(&mut self, style: &str) {
        if !self.next.is_empty() { self.next.push(';'); };
        self.next.push_str(&format!("{}", style));
    }

    pub(crate) fn raw(&mut self, style: &str) {
        self.next();
        self.text.push_str(style);
    }

    pub(crate) fn next(&mut self) {
        if !self.next.is_empty() {
            self.text.push_str(&format!("\x1b[{}m", self.next));
            self.next.clear();
        };
    }

    pub(crate) fn push(&mut self, chr: char) {
        self.text.push(chr);
    }

    pub(crate) fn view(self) -> String {
        self.text
    }

}
