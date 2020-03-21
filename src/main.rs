use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

fn interpret(code: &str, cells: &mut Vec<i32>, index: &mut usize) {
    for (i, elem) in code.split("").enumerate() {
        match elem {
            ">" => {
                *index += 1;
                if cells.get(*index).is_none() {
                    cells.push(0)
                }
            }

            "<" => *index -= 1,
            "+" => {
                cells[*index] += 1;
            }
            "-" => {
                cells[*index] -= 1;
            }
            "[" => {
                return interpret(&code[i..], cells, index);
            }
            "]" => {
                return if cells[*index] == 0 {
                    interpret(&code[i..], cells, index)
                } else {
                    interpret(&code, cells, index)
                }
            }
            "." => {
                println!("[{}]", cells[*index]);
            }

            _ => (),
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
