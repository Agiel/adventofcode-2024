use std::collections::BTreeMap;

use aocd::*;

#[aocd(2024, 21)]
fn main() {
    let input = input!();
    let sum = solve(&input, 25);
    dbg!(sum);
}

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
fn get_code_key_coord(key: char) -> (i32, i32) {
    match key {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => panic!(),
    }
}

fn get_arrows_for_code(from: char, to: char) -> Vec<Vec<char>> {
    let from = get_code_key_coord(from);
    let to = get_code_key_coord(to);
    let hor = to.0 - from.0;
    let hor = if hor < 0 {
        vec!['<'; -hor as usize]
    } else if hor > 0 {
        vec!['>'; hor as usize]
    } else {
        vec![]
    };
    let ver = to.1 - from.1;
    let ver = if ver < 0 {
        vec!['^'; -ver as usize]
    } else if ver > 0 {
        vec!['v'; ver as usize]
    } else {
        vec![]
    };
    let mut ret = Vec::new();
    if hor.len() > 0 && !(from.1 == 3 && to.0 == 0) {
        ret.push(vec![hor.clone(), ver.clone(), vec!['A']].concat());
    }
    if ver.len() > 0 && !(from.0 == 0 && to.1 == 3) {
        ret.push(vec![ver.clone(), hor.clone(), vec!['A']].concat());
    }
    if hor.len() == 0 && ver.len() == 0 {
        ret.push(vec!['A']);
    }
    ret
}

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
fn get_arrow_key_coord(key: char) -> (i32, i32) {
    match key {
        '^' => (1, 0),
        'A' => (2, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => panic!(),
    }
}

fn get_arrows_for_arrows(from: char, to: char) -> Vec<Vec<char>> {
    let from = get_arrow_key_coord(from);
    let to = get_arrow_key_coord(to);
    let hor = to.0 - from.0;
    let hor = if hor < 0 {
        vec!['<'; -hor as usize]
    } else if hor > 0 {
        vec!['>'; hor as usize]
    } else {
        vec![]
    };
    let ver = to.1 - from.1;
    let ver = if ver < 0 {
        vec!['^'; -ver as usize]
    } else if ver > 0 {
        vec!['v'; ver as usize]
    } else {
        vec![]
    };
    let mut ret = Vec::new();
    if hor.len() > 0 && !(from.1 == 0 && to.0 == 0) {
        ret.push(vec![hor.clone(), ver.clone(), vec!['A']].concat());
    }
    if ver.len() > 0 && !(from.0 == 0 && to.1 == 0) {
        ret.push(vec![ver.clone(), hor.clone(), vec!['A']].concat());
    }
    if hor.len() == 0 && ver.len() == 0 {
        ret.push(vec!['A']);
    }
    ret
}

fn get_code_sequences(code: &str) -> Vec<Vec<char>> {
    let mut prev = 'A';
    let mut sequence = vec![Vec::new()];
    for c in code.chars() {
        let arrows = get_arrows_for_code(prev, c);
        sequence = sequence
            .into_iter()
            .flat_map(|s: Vec<_>| {
                arrows
                    .iter()
                    .map(|a| vec![s.clone(), a.clone()].concat())
                    .collect::<Vec<Vec<_>>>()
            })
            .collect();
        prev = c;
    }
    sequence
}

fn get_arrow_sequences(arrows: &Vec<char>) -> Vec<Vec<char>> {
    let mut prev = 'A';
    let mut sequence = vec![Vec::new()];
    for &a in arrows.iter() {
        let new_arrows = get_arrows_for_arrows(prev, a);
        sequence = sequence
            .into_iter()
            .flat_map(|s: Vec<_>| {
                new_arrows
                    .iter()
                    .map(|a| vec![s.clone(), a.clone()].concat())
                    .collect::<Vec<Vec<_>>>()
            })
            .collect();
        prev = a;
    }
    sequence
}

fn count_presses(
    keys: &Vec<char>,
    depth: usize,
    cache: &mut BTreeMap<(Vec<char>, usize), u64>,
) -> u64 {
    if let Some(&count) = cache.get(&(keys.clone(), depth)) {
        return count;
    }

    let count = if depth == 0 {
        keys.len() as u64
    } else {
        get_arrow_sequences(keys)
            .iter()
            .map(|s| {
                s.split_inclusive(|&c| c == 'A')
                    .map(|s| count_presses(&s.to_vec(), depth - 1, cache))
                    .sum()
            })
            .min()
            .unwrap()
    };

    cache.insert((keys.clone(), depth), count);
    count
}

fn solve(input: &str, depth: usize) -> u64 {
    let codes: Vec<&str> = input.lines().collect();

    let mut cache = BTreeMap::new();
    codes
        .iter()
        .map(|code| {
            let arrows = get_code_sequences(code);
            arrows
                .iter()
                .map(|s| {
                    s.split_inclusive(|&c| c == 'A')
                        .map(|s| count_presses(&s.to_vec(), depth, &mut cache))
                        .sum::<u64>()
                })
                .min()
                .unwrap()
                * code[..code.len() - 1].parse::<u64>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input, 2), 126384);
    }
}
