use std::collections::{BTreeSet, BinaryHeap};

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

fn parse(input: &str) -> (Vec<Vec<Tile>>, (i32, i32)) {
    let mut map = Vec::new();
    let mut start = (0, 0);
    for (y, row) in input.lines().enumerate() {
        let mut r = Vec::new();
        for (x, c) in row.char_indices() {
            r.push(match c {
                '#' => Tile::Wall,
                'E' => Tile::End,
                'S' => {
                    start = (x as i32, y as i32);
                    Tile::Empty
                }
                _ => Tile::Empty,
            });
        }
        map.push(r);
    }
    (map, start)
}

fn solve(input: &str) -> i32 {
    let (map, start) = parse(&input);

    let mut heap = BinaryHeap::new();
    heap.push((0, start, (1, 0)));

    let mut seen = BTreeSet::new();

    while let Some((cost, pos, dir)) = heap.pop() {
        if matches!(map[pos.1 as usize][pos.0 as usize], Tile::End) {
            // BinaryHeap is a max heap so cost is inverted
            return -cost;
        }

        if seen.contains(&(pos, dir)) {
            continue;
        }

        seen.insert((pos, dir));

        let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        dirs.iter().for_each(|new_dir| {
            if dir.0 == -new_dir.0 && dir.1 == -new_dir.1 {
                return;
            }

            let next = (pos.0 + new_dir.0, pos.1 + new_dir.1);
            if matches!(map[next.1 as usize][next.0 as usize], Tile::Wall) {
                return;
            }

            if dir.0 == new_dir.0 && dir.1 == new_dir.1 {
                heap.push((cost - 1, next, *new_dir));
            } else {
                heap.push((cost - 1001, next, *new_dir));
            }
        });
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 7036);
        let input = include_str!("./example2.txt");
        assert_eq!(solve(input), 11048);
    }
}
