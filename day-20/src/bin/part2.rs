use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap},
    time::Instant,
};

use aocd::*;

#[aocd(2024, 20)]
fn main() {
    let input = input!();
    let start_time = Instant::now();
    let sum = solve(&input, 100);
    println!("Elapsed: {:?}", start_time.elapsed());
    dbg!(sum);
}

struct Track {
    map: BTreeSet<(i32, i32)>,
    start: (i32, i32),
    end: (i32, i32),
}

fn parse(input: &str) -> Track {
    let mut map = BTreeSet::new();
    let mut start = None;
    let mut end = None;

    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.char_indices() {
            let pos = (x as i32, y as i32);
            match c {
                'S' => {
                    start = Some(pos);
                }
                'E' => {
                    end = Some(pos);
                }
                '#' => {
                    map.insert(pos);
                }
                _ => (),
            }
        }
    }
    Track {
        map,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

fn find_distances(walls: &BTreeSet<(i32, i32)>, start: (i32, i32)) -> BTreeMap<(i32, i32), i32> {
    let mut heap = BinaryHeap::new();
    heap.push((0, start));

    let mut seen = BTreeMap::new();

    while let Some((cost, pos)) = heap.pop() {
        if seen.contains_key(&pos) {
            continue;
        }

        seen.insert(pos, -cost);

        let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        dirs.iter().for_each(|new_dir| {
            let next = (pos.0 + new_dir.0, pos.1 + new_dir.1);
            if seen.contains_key(&next) || walls.contains(&next) {
                return;
            }

            heap.push((cost - 1, next));
        });
    }

    return seen;
}

fn solve(input: &str, threshold: i32) -> i32 {
    let track = parse(&input);

    let from_start = find_distances(&track.map, track.start);
    let from_end = find_distances(&track.map, track.end);

    let legit = from_start.get(&track.end).unwrap();
    let mut count = 0;

    from_start.iter().for_each(|(from, start)| {
        from_end.iter().for_each(|(to, end)| {
            let distance = (from.0 - to.0).abs() + (from.1 - to.1).abs();
            if distance <= 20 {
                let length = start + distance + end;
                if legit - length >= threshold {
                    count += 1;
                }
            }
        });
    });

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input, 1), 44);
    }
}
