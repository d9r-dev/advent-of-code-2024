use std::fs;

#[derive(Debug, PartialEq)]
struct File {
    id: usize,
    size: usize,
}

#[derive(Debug, PartialEq)]
enum MemoryLocation {
    File(File),
    Free(usize),
}
pub fn run_day9() {
    let contents = fs::read_to_string("inputs/day9/input").expect("Could not read file");
    let mut memory = parse_input(&contents);
    let clean_memory = clean_memory(&mut memory);
    let result = check_sum(&clean_memory);
    println!("Result part 1: {:?}", result);
    let mut memory2 = parse_input(&contents);
    sort_memory(&mut memory2);
    let result2 = check_sum(&memory2);
    println!("Result part 2: {:?}", result2);
}

fn sort_memory(p0: &mut Vec<MemoryLocation>) {
    let mut i = p0.len() - 1;
    while i > 0 {
        if let MemoryLocation::File(File { id, size: filesize }) = p0[i] {
            let mut insertion_i = 0;

            loop {
                if let MemoryLocation::Free(freespace) = p0[insertion_i] {
                    if freespace > filesize {
                        p0[i] = MemoryLocation::Free(filesize);
                        p0[insertion_i] = MemoryLocation::File(File { id, size: filesize });
                        p0.insert(insertion_i + 1, MemoryLocation::Free(freespace - filesize));
                        break;
                    }

                    if freespace == filesize {
                        p0[i] = MemoryLocation::Free(filesize);
                        p0[insertion_i] = MemoryLocation::File(File { id, size: filesize });
                        break;
                    }
                }
                if insertion_i == i {
                    break;
                }
                insertion_i += 1;
            }
        }
        i -= 1;
    }
}

fn check_sum(input: &Vec<MemoryLocation>) -> usize {
    let mut position = 0;
    let mut result = 0;

    for block in input {
        match *block {
            MemoryLocation::Free(size) => position += size,
            MemoryLocation::File(File { id, size }) => {
                for _ in 0..size {
                    result += id * position;
                    position += 1;
                }
            }
        }
    }
    result
}

fn clean_memory(memory: &mut Vec<MemoryLocation>) -> Vec<MemoryLocation> {
    let mut cleaned_memory: Vec<MemoryLocation> = Vec::new();
    let mut write_i = 0;
    while write_i < memory.len() {
        let block = &memory[write_i];
        match *block {
            MemoryLocation::File(File { id, size }) => {
                cleaned_memory.push(MemoryLocation::File(File { id, size }));
            }
            MemoryLocation::Free(free) => {
                fill_freespace(&mut cleaned_memory, free, memory, write_i)
            }
        }
        write_i += 1;
    }

    cleaned_memory
}

fn fill_freespace(
    cleaned_memory: &mut Vec<MemoryLocation>,
    mut free_space: usize,
    memory: &mut Vec<MemoryLocation>,
    write_i: usize,
) {
    let mut read_i = memory.len() - 1;
    while free_space > 0 && read_i > write_i {
        if let MemoryLocation::File(File { id, size: filesize }) = memory[read_i] {
            if filesize <= free_space {
                cleaned_memory.push(MemoryLocation::File(File { id, size: filesize }));
                free_space -= filesize;
                memory.remove(read_i);
                read_i -= 1;
            } else {
                cleaned_memory.push(MemoryLocation::File(File {
                    id,
                    size: free_space,
                }));
                memory[read_i] = MemoryLocation::File(File {
                    id,
                    size: filesize - free_space,
                });
                free_space = 0;
            }
        } else {
            read_i -= 1;
        }
    }
}

fn parse_input(contents: &str) -> Vec<MemoryLocation> {
    let mut memory: Vec<MemoryLocation> = Vec::new();
    for (i, char) in contents.chars().enumerate() {
        let num = char.to_string().parse::<usize>().unwrap();
        if i == 0 || i % 2 == 0 {
            memory.push(MemoryLocation::File(File {
                id: i / 2,
                size: num,
            }));
        } else {
            memory.push(MemoryLocation::Free(num));
        }
    }
    memory
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse() {
        let input = "2332";
        let mut expected = vec![];
        expected.push(MemoryLocation::File(File { id: 0, size: 2 }));
        expected.push(MemoryLocation::Free(3));
        expected.push(MemoryLocation::File(File { id: 1, size: 3 }));
        expected.push(MemoryLocation::Free(2));

        let actual = parse_input(input);

        assert_eq!(expected, actual);
    }
}
