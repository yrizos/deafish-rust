use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut acc: i64 = 0;    
    
    for line in stdin.lock().lines() {
        for chr in line.unwrap().chars() {
        
            if chr == 'i' {
                acc+=1;
            } else if chr == 'd' {
                acc-=1;
            } else if chr == 'o' {
                println!("{}", acc);
            } else if chr == 's' {
                acc *= acc;
            }

            if acc == 256 || acc < 0 {
                acc = 0;
            }
        }
    }
}
