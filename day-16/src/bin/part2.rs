use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use aocd::*;

#[aocd(2024, 16)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

enum Tile {
    Wall,
    Empty,
    End,
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, (i32, i32), (i32, i32)) {
    let mut map = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, row) in input.lines().enumerate() {
        let mut r = Vec::new();
        for (x, c) in row.char_indices() {
            r.push(match c {
                '#' => Tile::Wall,
                'E' => {
                    end = (x as i32, y as i32);
                    Tile::End
                }
                'S' => {
                    start = (x as i32, y as i32);
                    Tile::Empty
                }
                _ => Tile::Empty,
            });
        }
        map.push(r);
    }
    (map, start, end)
}

fn solve(input: &str) -> usize {
    let (map, start, end) = parse(&input);

    let mut heap = BinaryHeap::new();
    heap.push((0, start, (1, 0)));

    let mut seen = BTreeSet::new();
    let mut nodes = BTreeMap::new();

    while let Some((cost, pos, dir)) = heap.pop() {
        if matches!(map[pos.1 as usize][pos.0 as usize], Tile::End) {
            nodes.insert((pos, dir), cost);
            break;
        }

        if seen.contains(&(pos, dir)) {
            continue;
        }

        seen.insert((pos, dir));
        nodes.insert((pos, dir), cost);

        let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        dirs.iter().for_each(|new_dir| {
            if dir.0 == -new_dir.0 && dir.1 == -new_dir.1 {
                return;
            }

            let next = (pos.0 + new_dir.0, pos.1 + new_dir.1);
            if matches!(map[next.1 as usize][next.0 as usize], Tile::Wall) {
                return;
            }

            let cost = if dir.0 == new_dir.0 && dir.1 == new_dir.1 {
                cost - 1
            } else {
                cost - 1001
            };

            heap.push((cost, next, *new_dir));
        });
    }

    let mut seats = BTreeSet::new();
    let mut heap = BinaryHeap::new();
    heap.push((end, (-1, 0)));
    heap.push((end, (0, -1)));

    while let Some((pos, dir)) = heap.pop() {
        if let Some(cost) = nodes.get(&(pos, dir)) {
            let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
            dirs.iter().for_each(|new_dir| {
                let next = (pos.0 - new_dir.0, pos.1 - new_dir.1);
                dirs.iter().for_each(|new_dir| {
                    if let Some(new_cost) = nodes.get(&(next, *new_dir)) {
                        if new_cost > cost {
                            if dir.0 == new_dir.0 && dir.1 == new_dir.1 && new_cost - cost == 1
                                || new_cost - cost == 1001
                            {
                                heap.push((next, *new_dir));
                            }
                        }
                    }
                })
            });
            seats.insert(pos);
        }
    }

    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if seats.contains(&(x as i32, y as i32)) {
                print!("O");
            } else {
                match tile {
                    Tile::Wall => print!("#"),
                    _ => print!("."),
                }
            }
        }
        println!();
    }

    return seats.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 45);
        let input = include_str!("./example2.txt");
        assert_eq!(solve(input), 64);
    }
}
