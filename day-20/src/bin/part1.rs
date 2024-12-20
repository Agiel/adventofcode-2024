use std::{
    collections::{BTreeSet, BinaryHeap},
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

fn find_path(track: &Track) -> i32 {
    let mut heap = BinaryHeap::new();
    heap.push((0, track.start));
    let size = track.map.last().unwrap();

    let mut seen = BTreeSet::new();

    while let Some((cost, pos)) = heap.pop() {
        if pos == track.end {
            return -cost;
        }

        if seen.contains(&pos) {
            continue;
        }

        seen.insert(pos);

        let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        dirs.iter().for_each(|new_dir| {
            let next = (pos.0 + new_dir.0, pos.1 + new_dir.1);
            if next.0 < 0 || next.1 < 0 || next.0 > size.0 || next.1 > size.1 {
                return;
            }
            if seen.contains(&next) || track.map.contains(&next) {
                return;
            }

            heap.push((cost - 1, next));
        });
    }

    return 0;
}

fn solve(input: &str, threshold: i32) -> i32 {
    let track = parse(&input);

    let mut heap = BinaryHeap::new();
    heap.push((0, 0, None, track.start));
    let size = track.map.last().unwrap();

    let mut seen = BTreeSet::new();
    let legit = find_path(&track);
    println!("{legit}");
    let mut count = 0;

    while let Some((_h, cost, cheat, pos)) = heap.pop() {
        if pos == track.end {
            if legit - cost >= threshold {
                count += 1;
                println!("{count}/{} saving {}", track.map.len(), legit - cost);
            } else {
                // BinaryHeap is a max heap so cost is inverted
                return count;
            }
        }

        if seen.contains(&(pos, cheat)) {
            continue;
        }

        seen.insert((pos, cheat));

        let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        dirs.iter().for_each(|new_dir| {
            let next = (pos.0 + new_dir.0, pos.1 + new_dir.1);
            if next.0 <= 0 || next.1 <= 0 || next.0 >= size.0 || next.1 >= size.1 {
                return;
            }
            if seen.contains(&(next, cheat)) {
                return;
            }

            let distance = (track.end.0 - next.0).abs() + (track.end.0 - next.1).abs();
            if track.map.contains(&next) {
                if cheat.is_none() {
                    heap.push((-(distance + cost + 1), cost + 1, Some(next), next))
                }
            } else {
                heap.push((-(distance + cost + 1), cost + 1, cheat, next));
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
        assert_eq!(solve(input, 1), 44);
    }
}
