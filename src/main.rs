use std::io;
use std::{cmp::min, io::prelude::*, ops::Add};

fn main() {
    // allocate memory for the input buffer
    let mut buf: String = String::with_capacity(100);

    // get input lines as strings
    io::stdin().read_to_string(&mut buf).expect("err");
    let lines = buf.split("\n");

    let mut dicts: Vec<&str> = Vec::with_capacity(500000);
    let mut inputs: Vec<&str> = Vec::with_capacity(100);
    let mut dict_mode = true;
    for e in lines {
        if e != "" {
            if dict_mode {
            
                if e == "#" {
                    dict_mode = false;
                    continue;
                } else {
                    dicts.push(e);
                }
            } else {
                inputs.push(e);
            }
        }
    }
    let mut distance = 40;
    let mut shortest: Vec<&str> = Vec::with_capacity(dicts.len());
    for input in inputs {
        for word in &dicts {
            let result = compare(input, word);
            //println!("result: {:?}", result);
            if result < distance {
                distance = result;
                shortest.clear();
                shortest.push(word);
            } else if result == distance {
                shortest.push(word);
            }
            //println!("{:?} ({:?}){:?}", input,distance , &shortest);
        }
        let mut output = String::with_capacity(1000);
        // output += input;

        // output += &format!(" ({}) ", distance);
        for e in &shortest {
            output += &format!("{} ", e);
        }
        println!("{} ({}) {}", input, distance, output);
        shortest.clear();
        distance = 40;
    }
}

fn compare(a: &str, b: &str) -> usize {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let mut matrix = vec![vec![0; a.len() + 1]; b.len() + 1];
    for col in 0..(b.len() + 1) {
        matrix[col][0] = col;
    }
    for row in 0..(a.len() + 1) {
        matrix[0][row] = row;
    }
    for row in 1..(a.len() + 1) {
        for col in 1..(b.len() + 1) {
            let substitution;
            if a[row - 1] == b[col - 1] {
                substitution = 0;
            } else {
                substitution = 1;
            }
            matrix[col][row] = (matrix[col - 1][row - 1] + substitution)
                .min(matrix[col][row - 1] + 1)
                .min(matrix[col - 1][row] + 1);
            
        }
    }
    //println!("{:?}", matrix);
    return matrix[b.len()][a.len()];
}
