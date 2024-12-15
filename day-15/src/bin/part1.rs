use std::collections::BTreeMap;

use aocd::*;

#[aocd(2024, 15)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

enum Object {
    Wall,
    Crate,
    Robot,
}

fn parse(input: &str) -> (BTreeMap<(i32, i32), Object>, (i32, i32), Vec<(i32, i32)>) {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let mut start = (0, 0);

    let mut tiles = BTreeMap::new();
    for (y, row) in map.lines().enumerate() {
        for (x, c) in row.char_indices() {
            let pos = (x as i32, y as i32);
            match c {
                '#' => {
                    tiles.insert(pos, Object::Wall);
                }
                'O' => {
                    tiles.insert(pos, Object::Crate);
                }
                '@' => {
                    start = pos;
                    tiles.insert(pos, Object::Robot);
                }
                _ => (),
            };
        }
    }
    let moves = moves
        .chars()
        .filter_map(|c| match c {
            '>' => Some((1, 0)),
            'v' => Some((0, 1)),
            '<' => Some((-1, 0)),
            '^' => Some((0, -1)),
            _ => None,
        })
        .collect();
    (tiles, start, moves)
}

fn do_move(pos: (i32, i32), dir: (i32, i32), map: &mut BTreeMap<(i32, i32), Object>) -> (i32, i32) {
    if let Some(obj) = map.get(&pos) {
        match obj {
            Object::Robot | Object::Crate => {
                let target = (pos.0 + dir.0, pos.1 + dir.1);
                do_move(target, dir, map);
                if !map.contains_key(&target) {
                    let obj = map.remove(&pos).unwrap();
                    map.insert(target, obj);
                    return target;
                }
            }
            _ => (),
        }
    }
    return pos;
}

fn solve(input: &str) -> i32 {
    let (mut map, start, moves) = parse(&input);
    let mut pos = start;
    for m in moves.iter() {
        pos = do_move(pos, *m, &mut map);
    }
    map.iter()
        .filter_map(|(k, v)| match v {
            Object::Crate => Some(k.0 + k.1 * 100),
            _ => None,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 10092);
    }
}
