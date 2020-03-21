use std::fs::File;
use std::env;
use std::io::Read;
use std::io;

fn interpret(code: &str, cells: &mut Vec<i32>, index: &mut usize) {
    for (i, elem) in code.split("").enumerate() {
        if elem == ">" {
            *index += 1;

            if cells.get(*index).is_none() {
                cells.push(0)
            }
        }

        if elem == "<" {
            *index -= 1
        }

        if elem == "+" {
            cells[*index] += 1;
        }

        if elem == "-" {
            cells[*index] -= 1;
        }

        if elem == "[" {
            return interpret(&code[i..], cells, index);
        }

        if elem == "]" {
            return if cells[*index] == 0 {
                interpret(&code[i..], cells, index)
            } else {
                interpret(&code, cells, index)
            }
        }

        if elem == "." {
            println!("[{}]",  cells[*index]);
        }
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let args = env::args().collect::<Vec<String>>();
     File::open(&args[1])?.read_to_string(&mut buffer)?;
    interpret(&buffer, &mut vec![0], &mut 0);
    Ok(())
}