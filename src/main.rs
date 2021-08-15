use std::io;

fn collatz(x: u64, v: &mut Vec<u64>) {
    v.push(x);
    match x {
        _ if x == 1 => return,
        x if x % 2 == 1 => {
            collatz((3 * x) + 1, v);
        },
        x if x % 2 == 0 => {
            collatz(x / 2, v);
        },
        _ => return,
    }
}

fn get_num_from_user() -> Result<u64, io::Error> {
    println!("Please enter a number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input.pop();
    let x = input.parse::<u64>().expect("Could not parse input");
    Ok(x)
}

fn main() {
    match get_num_from_user() {
        Ok(x) => {
            let mut vec = Vec::new();
            collatz(x, &mut vec);
            println!("Sequence: {:?}", vec);
        },
        Err(error) => {
            println!("error: {}", error);
        },
    }
        
}
