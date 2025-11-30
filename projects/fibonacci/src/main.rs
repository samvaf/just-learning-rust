use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u32 = n.trim().parse().expect("Enter a valid number!");

    let mut f_n_i_0: u32 = 1;
    let mut f_n_i_1: u32 = 1;

    println!("Fibonnaci sequence for n={n}: ");
    if n >= 1 {
        println!("{f_n_i_0}");
    }
    if n >= 2 {
        println!("{f_n_i_1}");
    }

    for _ in 3..n {
        let temp: u32 = f_n_i_1;
        f_n_i_1 = f_n_i_1 + f_n_i_0;
        f_n_i_0 = temp;
        println!("{f_n_i_1}");
    }
}
