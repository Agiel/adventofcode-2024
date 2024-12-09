use aocd::*;

#[aocd(2024, 9)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

enum Chunk {
    File { id: u32, size: u32 },
    Free { size: u32 },
}

fn parse(input: &str) -> Vec<Chunk> {
    let mut disk = Vec::new();
    let mut free = false;
    let mut id = 0;
    for c in input.trim().chars() {
        let size = c.to_digit(10).unwrap() as u32;
        if !free {
            disk.push(Chunk::File { id, size });
            id += 1;
        } else {
            disk.push(Chunk::Free { size })
        }
        free = !free;
    }
    disk
}

fn solve(input: &str) -> i64 {
    let mut disk = parse(&input);
    // Reverse so we can work the array from left to right
    disk.reverse();

    let mut n = 0;
    while n < disk.len() {
        match disk[n] {
            Chunk::File {
                id: _,
                size: file_size,
            } => {
                for m in (n..disk.len()).rev() {
                    match disk[m] {
                        Chunk::Free { size: free_size } if free_size >= file_size => {
                            disk.swap(n, m);
                            if free_size > file_size {
                                let remaining = free_size - file_size;
                                disk[n] = Chunk::Free { size: file_size };
                                disk.insert(m, Chunk::Free { size: remaining });
                            }
                            break;
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        n += 1;
    }

    let mut sum = 0;
    let mut idx = 0;
    disk.iter().rev().for_each(|chunk| match chunk {
        Chunk::File { id, size } => {
            sum += (id * (idx + idx + size - 1)) as i64 * *size as i64 / 2;
            idx += size;
        }
        Chunk::Free { size } => {
            idx += size;
        }
    });

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 2858);
    }
}
