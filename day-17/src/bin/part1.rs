use aocd::*;

#[aocd(2024, 17)]
fn main() {
    let input = input!();
    let ans = solve(&input);
    dbg!(ans);
}

enum Instr {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

struct Registers {
    a: u32,
    b: u32,
    c: u32,
}

fn parse(input: &str) -> (Vec<(Instr, u32)>, Registers) {
    let (registers, instructions) = input.split_once("\n\n").unwrap();
    let mut reg_iter = registers
        .lines()
        .map(|l| l.split_whitespace().last().unwrap().parse().unwrap());
    let a = reg_iter.next().unwrap();
    let b = reg_iter.next().unwrap();
    let c = reg_iter.next().unwrap();

    let (_, instructions) = instructions.split_once(" ").unwrap();
    let instructions = instructions.trim().split(",");
    let instructions = instructions
        .clone()
        .step_by(2)
        .zip(instructions.skip(1).step_by(2))
        .map(|(instr, combo)| {
            (
                match instr.parse().unwrap() {
                    0 => Instr::Adv,
                    1 => Instr::Bxl,
                    2 => Instr::Bst,
                    3 => Instr::Jnz,
                    4 => Instr::Bxc,
                    5 => Instr::Out,
                    6 => Instr::Bdv,
                    7 => Instr::Cdv,
                    _ => panic!(),
                },
                combo.parse().unwrap(),
            )
        })
        .collect();
    (instructions, Registers { a, b, c })
}

fn exec(
    instr: &Instr,
    operand: &u32,
    registers: &mut Registers,
    pointer: usize,
) -> (usize, Option<u32>) {
    let literal = *operand;
    let combo = match operand {
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        _ => literal,
    };

    let mut output = None;
    match instr {
        Instr::Adv => {
            registers.a = registers.a >> combo;
        }
        Instr::Bxl => {
            registers.b = registers.b ^ literal;
        }
        Instr::Bst => {
            registers.b = combo % 8;
        }
        Instr::Jnz => {
            if registers.a > 0 {
                return (literal as usize, None);
            }
        }
        Instr::Bxc => {
            registers.b = registers.b ^ registers.c;
        }
        Instr::Out => {
            output = Some(combo % 8);
        }
        Instr::Bdv => {
            registers.b = registers.a >> combo;
        }
        Instr::Cdv => {
            registers.c = registers.a >> combo;
        }
    }
    (pointer + 2, output)
}

fn solve(input: &str) -> String {
    let (program, mut registers) = parse(input);
    let mut ptr = 0;
    let mut output = "".to_string();
    while let Some((instr, operand)) = program.get(ptr / 2) {
        let (new_ptr, new_output) = exec(instr, operand, &mut registers, ptr);
        if let Some(o) = new_output {
            if output.len() == 0 {
                output = format!("{o}");
            } else {
                output.push_str(format!(",{o}").as_str());
            }
        }
        ptr = new_ptr;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), "4,6,3,5,6,3,5,2,1,0");
    }
}
