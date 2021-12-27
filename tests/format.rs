
use fancy::*;

#[test]
fn format() {

    printcoln!("[bold|cyan]Hello world[magenta]!");
    printcoln!("[b|#7cd615]space[#c1973c]space!"); // purple/orange "spacespace"

    printcoln!("H[!]E[:]L[!]L[:]L[!]O[:]"); // black/white "HELLO"
    printcoln!("[b|u|#babaf1]rust[:] is [!]{}", "cool");
    // printcol!("{}I am not bold!\n", "[bold]");

}
