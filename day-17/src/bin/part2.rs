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
    a: u64,
    b: u64,
    c: u64,
}

fn parse(input: &str) -> (Vec<(Instr, u64)>, Registers, Vec<u64>) {
    let (registers, instructions) = input.split_once("\n\n").unwrap();
    let mut reg_iter = registers
        .lines()
        .map(|l| l.split_whitespace().last().unwrap().parse().unwrap());
    let a = reg_iter.next().unwrap();
    let b = reg_iter.next().unwrap();
    let c = reg_iter.next().unwrap();

    let (_, instructions) = instructions.split_once(" ").unwrap();
    let instructions = instructions.trim().split(",");
    let raw_instructions = instructions.map(|i| i.parse().unwrap());
    let instructions = raw_instructions
        .clone()
        .step_by(2)
        .zip(raw_instructions.clone().skip(1).step_by(2))
        .map(|(instr, operator)| {
            (
                match instr {
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
                operator,
            )
        })
        .collect();
    (
        instructions,
        Registers { a, b, c },
        raw_instructions.collect(),
    )
}

fn exec(
    instr: &Instr,
    operand: &u64,
    registers: &mut Registers,
    pointer: usize,
) -> (usize, Option<u64>) {
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

fn run_program(program: &Vec<(Instr, u64)>, registers: &mut Registers) -> Vec<u64> {
    let mut ptr = 0;
    let mut output = Vec::new();
    while let Some((instr, operand)) = program.get(ptr / 2) {
        let (new_ptr, new_output) = exec(instr, operand, registers, ptr);
        if let Some(o) = new_output {
            output.push(o);
        }
        ptr = new_ptr;
    }
    output
}

fn solve(input: &str) -> u64 {
    let (program, mut registers, raw_instructions) = parse(input);

    let mut num = vec![0; raw_instructions.len()];
    let mut n = num.len() - 1;
    let mut i = 0;
    loop {
        num[n] = i;

        registers.a = num
            .iter()
            .enumerate()
            .map(|(exp, n)| n * 8u64.pow(exp as u32))
            .sum();
        let output = run_program(&program, &mut registers);

        if raw_instructions == output {
            break;
        }
        if raw_instructions.len() == output.len() && raw_instructions[n..] == output[n..] {
            n -= 1;
            i = 0;
        } else if i >= 8 {
            num[n] = 0;
            n += 1;
            i = num[n] + 1;
        } else {
            i += 1;
        }
    }

    num.iter()
        .enumerate()
        .map(|(exp, n)| n * 8u64.pow(exp as u32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example2.txt");
        assert_eq!(solve(input), 117440);
    }
}
