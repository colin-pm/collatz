use std::io;

fn collatz(x: u64, v: &mut Vec<u64>) {
    v.push(x);
    if x == 1 {
        return
    }
    match x {
        x if x % 2 == 1 => {
            collatz((3 * x) + 1, v);
        },
        x if x % 2 == 0 => {
            collatz(x / 2, v);
        },
        _ => return,
    }
}

fn main() {
    println!("Please enter a number: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.pop();
            match input.parse::<u64>() {
                Ok(x) => {
                    let mut vec = Vec::new();
                    collatz(x, &mut vec);
                    println!("Sequence: {:?}", vec);
                },
                Err(error) => {
                    println!("error: {}", error);
                },
            };
        }
        Err(error) => println!("error: {}", error),
    }
}
