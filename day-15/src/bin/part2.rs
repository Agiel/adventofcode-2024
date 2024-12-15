use std::collections::BTreeMap;

use aocd::*;

#[aocd(2024, 15)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

#[derive(Clone)]
enum Object {
    Wall,
    Crate((i32, i32), (i32, i32)),
    Robot,
}

fn parse(input: &str) -> (BTreeMap<(i32, i32), Object>, (i32, i32), Vec<(i32, i32)>) {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let mut start = (0, 0);

    let mut tiles = BTreeMap::new();
    for (y, row) in map.lines().enumerate() {
        for (x, c) in row.char_indices() {
            let pos = ((x * 2) as i32, y as i32);
            let pos2 = (pos.0 + 1, pos.1);
            match c {
                '#' => {
                    tiles.insert(pos, Object::Wall);
                    tiles.insert(pos2, Object::Wall);
                }
                'O' => {
                    tiles.insert(pos, Object::Crate(pos, pos2));
                    tiles.insert(pos2, Object::Crate(pos, pos2));
                }
                '@' => {
                    tiles.insert(pos, Object::Robot);
                    start = pos;
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

fn can_move(pos: (i32, i32), dir: (i32, i32), map: &BTreeMap<(i32, i32), Object>) -> bool {
    if let Some(obj) = map.get(&pos) {
        let target = (pos.0 + dir.0, pos.1 + dir.1);
        match obj {
            Object::Robot => {
                return can_move(target, dir, map);
            }
            Object::Crate(pos1, pos2) => {
                let target1 = (pos1.0 + dir.0, pos1.1 + dir.1);
                let target2 = (pos2.0 + dir.0, pos2.1 + dir.1);
                if dir.1 != 0 {
                    return can_move(target1, dir, map) && can_move(target2, dir, map);
                } else if dir.0 < 0 {
                    return can_move(target1, dir, map);
                } else {
                    return can_move(target2, dir, map);
                };
            }
            Object::Wall => return false,
        }
    }
    return true;
}

fn do_move(pos: (i32, i32), dir: (i32, i32), map: &mut BTreeMap<(i32, i32), Object>) -> (i32, i32) {
    if let Some(obj) = map.get(&pos) {
        let obj = obj.clone();
        let target = (pos.0 + dir.0, pos.1 + dir.1);
        match obj {
            Object::Robot => {
                do_move(target, dir, map);
                if !map.contains_key(&target) {
                    let obj = map.remove(&pos).unwrap();
                    map.insert(target, obj);
                    return target;
                }
            }
            Object::Crate(pos1, pos2) => {
                let target1 = (pos1.0 + dir.0, pos1.1 + dir.1);
                let target2 = (pos2.0 + dir.0, pos2.1 + dir.1);
                let free =
                    if dir.1 != 0 && can_move(target1, dir, map) && can_move(target2, dir, map) {
                        do_move(target1, dir, map);
                        do_move(target2, dir, map);
                        (!map.contains_key(&target1), !map.contains_key(&target2))
                    } else if dir.0 < 0 {
                        do_move(target1, dir, map);
                        (!map.contains_key(&target1), true)
                    } else if dir.0 > 0 {
                        do_move(target2, dir, map);
                        (true, !map.contains_key(&target2))
                    } else {
                        (false, false)
                    };
                if free.0 && free.1 {
                    map.remove(&pos1);
                    map.remove(&pos2);
                    map.insert(target1, Object::Crate(target1, target2));
                    map.insert(target2, Object::Crate(target1, target2));
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
        .filter_map(|(_, v)| match v {
            Object::Crate(pos, _) => Some(pos.0 + pos.1 * 100),
            _ => None,
        })
        .sum::<i32>()
        / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 9021);
        let input = include_str!("./example2.txt");
        assert_eq!(solve(input), 618);
    }
}
