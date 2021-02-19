use std::{io, usize};
use std::{cmp::min, io::prelude::*, ops::Add};
use std::time::Instant;


fn main() {
    let timer = Instant::now();
    // allocate memory for the input buffer
    let mut buf: String = String::with_capacity(100000);

    // get input lines as strings
    io::stdin().read_to_string(&mut buf).expect("err");
    let lines = buf.split("\n");

    let mut dicts: Vec<Word> = Vec::with_capacity(500000);
    let mut inputs: Vec<Word>  = Vec::with_capacity(100);
    let mut dict_mode = true;
    for e in lines {
        if e != "" {
            if dict_mode {
            
                if e == "#" {
                    dict_mode = false;
                    continue;
                } else {
                    dicts.push(as_word(e));
                }
            } else {
                inputs.push(as_word(e));
            }
        }
    }

    //let input_end = timer.elapsed().as_millis();
    //println!("Input: {:?}ms", input_end);

    let mut distance: usize = 41;
    let mut shortest = vec![0;500_000];
    let mut current: usize = 0;
    let mut previous: [char; 40] = [' '; 40];
    for input in inputs {
        //let a: Vec<char> = input.chars().collect();
        let time = Instant::now();
        let mut matrix = [[0; 41]; 41];
        for col in 0..41 {
            matrix[col][0] = col;
        }
        for row in 0..41 {
            matrix[0][row] = row;
        }
        for i  in 0..dicts.len() {
            
            //let b: Vec<char> = word.chars().collect();
            
            let min = min_distance(&input, &dicts[i]) as usize;
            if min > distance {
                continue;
            }
            let result = compare(&input,&dicts[i] , &mut matrix, similarity(&dicts[i].array, &previous));
            //println!("result: {:?}", result);
            if result < distance {
                distance = result;
                current = 1;
                shortest[0] = i;
                
            } else if result == distance {
                shortest[current] = i;
                current += 1;
            }
            previous = dicts[i].array;
            //println!("{:?} ({:?}){:?}", input,distance , &shortest);
        }
        //println!("Algo: {}ms", time.elapsed().as_millis());
        //let time_print = Instant::now();
        let mut output = String::with_capacity(1000);
        for i  in 0..input.length {
            output.push(input.array[i]);
        }

        // output += input;
        //println!("Shortest at 0: {:?}", shortest[0]);
        output += &format!(" ({}) ", distance);
        for e in 0..current {
            for i in 0..dicts[shortest[e]].length {
                output.push(dicts[shortest[e]].array[i]);
            }
            output.push(' ');
        }
        
        println!("{}", output);
       // println!("Print: {}ms", time_print.elapsed().as_millis());
        //println!("{} ({}) {}", input, distance, output);
        current = 0;
        distance = 41;
        previous = [' '; 40];
        
    }
    println!("Total: {:?}ms", timer.elapsed().as_millis());
}

fn compare(input: &Word, dict: &Word, matrix: &mut [[usize; 41]; 41], offset: usize ) -> usize {
    //let a: Vec<char> = a.chars().collect();
    //let b: Vec<char> = b.chars().collect();
    
    //println!("{:?}", a);
    
    for row in 1..(input.length + 1) {
        for col in (offset + 1 )..(dict.length + 1) {

            let substitution;
            if input.array[row - 1] == dict.array[col - 1] {
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
    return matrix[dict.length][input.length];
}

fn min_distance(a: &Word, b: &Word) -> i32 {
    //let a: Vec<char> = a.chars().collect();
    //let b: Vec<char> = b.chars().collect();

    return (a.length as i32 - b.length as i32).abs()
}

fn similarity(a: &[char; 40], b: &[char; 40]) -> usize{
    
    for i in 0..40 {
        if a[i] != b[i] {
            
            return i;
        }
    }
    return 0;
}

struct Word {
    length: usize,
    array: [char; 40]
}

fn as_word(string: &str) -> Word {
    let mut chars = string.chars();
    let mut temp: [char; 40] = [' '; 40];
    let mut current: usize = 0;
    
    for c in chars {
        temp[current] = c;
        current += 1;
    }
    return Word{
        length: current + 1,
        array: temp
    };
}
// impl Word {
//     pub fn new
// }
#[cfg(test)]
mod test {
    use crate::as_word;
    use crate::*;
    #[test]
    fn simisdlarity() {
        let a = as_word("mask");
        let b = as_word("masken");
        let sim = similarity(&a.array, &b.array);
        println!("Sim: {}", sim );
    }
}

