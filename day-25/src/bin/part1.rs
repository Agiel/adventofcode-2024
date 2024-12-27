use aocd::*;

#[aocd(2024, 25)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

enum Schematic {
    Key(Vec<u32>),
    Lock(Vec<u32>),
}

fn parse(input: &str) -> Vec<Schematic> {
    input
        .split("\n\n")
        .map(|schematic| {
            schematic
                .lines()
                .map(|row| row.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .map(|schematic| {
            let is_lock = schematic[0].iter().all(|&c| c == '#');
            let mut shape = Vec::new();
            for x in 0..schematic[0].len() {
                let mut n = 0;
                for y in 0..schematic.len() {
                    let c = schematic[y][x];
                    if c == '#' {
                        n += 1;
                    }
                }
                shape.push(n - 1);
            }
            if is_lock {
                Schematic::Lock(shape)
            } else {
                Schematic::Key(shape)
            }
        })
        .collect::<Vec<_>>()
}

fn solve(input: &str) -> u32 {
    let schematics = parse(input);
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    schematics.iter().for_each(|s| match s {
        Schematic::Key(shape) => {
            keys.push(shape);
        }
        Schematic::Lock(shape) => {
            locks.push(shape);
        }
    });
    let mut n = 0;
    for key in &keys {
        for lock in &locks {
            if key.iter().zip(lock.iter()).all(|(k, l)| k + l < 6) {
                n += 1;
            }
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 3);
    }
}
