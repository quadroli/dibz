use text_io::read;
mod analysis;
use analysis::analysis::{extreme_vals, fill_in, mean, median, sum};
mod internals;
use internals::internals::{bc_handler, capup, initdb, lookup, remove, update};

fn main() {
    println!("What would you love to do today?");
    prompt();
    loop {
        choices();
        println!();
        println!("More operations? (y/n)");
        let ans: String = read!();
        if ans == "n" {
            println!();
            println!("Goodbye, see you later i hope :)");
            break;
        } else if ans == "y" {
            println!();
            prompt();
            continue;
        } else {
            println!();
            println!("Back to the menu we go :)");
            println!();
            prompt();
        }
    }
}
fn prompt() {
    println!("(a)input/update records     (b)look up a record");
    println!("   (c)delete records           (d)analysis");
}

fn choices() {
    let mut mydb = initdb();
    let choice: String = read!();
    let name: String;
    if choice == "a" {
        update(&mut mydb);
    } else if choice == "b" {
        bc_handler("look up", &mydb);
        name = read!();
        println!();
        let named = capup(&name);
        lookup(&named[0..named.len()], &mydb);
    } else if choice == "c" {
        bc_handler("remove", &mydb);
        name = read!();
        let named = capup(&name);
        remove(&named[0..named.len()], &mut mydb);
        println!("Current number of records left: {}", mydb.total_keys());
    } else if choice == "d" && mydb.total_keys() > 1 {
        let total = sum(&mydb);
        println!();
        println!("Total number of records is {}", mydb.total_keys());
        println!();
        println!("The total amount is = {}", total);
        println!();
        println!("The mean is = {}", mean(&mydb, total));
        println!();
        median(fill_in(&mydb));
        println!();
        extreme_vals(fill_in(&mydb));
    } else if choice == "d" && mydb.total_keys() < 2 {
        println!(":( Not enough records");
    } else {
        println!("Invalid input!!!");
        println!();
        println!("heh, heh, back to prompt :)");
        println!();
    }
}
