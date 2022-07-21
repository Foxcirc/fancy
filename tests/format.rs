
use fancy::*;

#[test]
fn format() {

    let msg = colorize!("[b]Hi [u]there!");
    println!("{}", msg);

    printcoln!("[bold|magenta]\"Hello world!\"");
    printcoln!("[bold|cyan]Hello world[magenta]!");
    printcoln!("[b|#7cd615]space[#c1973c]space!"); // purple/orange "spacespace"

    printcoln!("H[!]E[:]L[!]L[:]L[!]O[:]"); // black/white "HELLO"
    printcoln!("[b|u|#babaf1]rust[:] is [!]{}", "cool");
    printcol!("{}I am not bold!\n", "[bold]");
    printcol!("{}, i am {:?} years old!\n", "Hello", 16);
    printcol!("I am [[ -- [[escaped]] -- ]]!\n");
    printcol!("I am triple [[[[ -- [[escaped]] -- ]]]]!\n");

    println!("\x1b[1mHi!\x1b[0m");
    printcoln!("[b|blue]Hi!");

    printcoln!("[u|red]error[:]: invalid keyword \"data\"");

}
