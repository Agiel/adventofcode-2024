use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};

use aocd::*;

#[aocd(2024, 12)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Region {
    plant: char,
    area: usize,
    perimeter: usize,
    tiles: BTreeSet<(i32, i32)>,
}

impl Region {
    fn price(&self) -> usize {
        self.area * self.count_edges()
    }
    fn count_edges(&self) -> usize {
        self.tiles
            .iter()
            .map(|tile| {
                let mut corners = 0;
                let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
                dirs.iter().enumerate().for_each(|(id, &forward)| {
                    let right = dirs[(id + 1) % 4];
                    let diagonal = (forward.0 + right.0, forward.1 + right.1);

                    if !self.tiles.contains(&(tile.0 + right.0, tile.1 + right.1))
                        && !self
                            .tiles
                            .contains(&(tile.0 + forward.0, tile.1 + forward.1))
                        || self.tiles.contains(&(tile.0 + right.0, tile.1 + right.1))
                            && !self
                                .tiles
                                .contains(&(tile.0 + diagonal.0, tile.1 + diagonal.1))
                            && self
                                .tiles
                                .contains(&(tile.0 + forward.0, tile.1 + forward.1))
                    {
                        corners += 1;
                    }
                });
                corners
            })
            .sum()
    }
}

fn solve(input: &str) -> usize {
    let garden = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let mut regions = BTreeMap::<(i32, i32), Rc<RefCell<Region>>>::new();
    for (y, row) in garden.iter().enumerate() {
        for (x, plant) in row.iter().enumerate() {
            let pos = (x as i32, y as i32);
            let region = if let Some(region) = regions.get(&pos) {
                if x as i32 - 1 < 0
                    || regions.get(&(pos.0 - 1, pos.1)).unwrap().borrow().plant != *plant
                {
                    region.borrow_mut().perimeter += 1;
                }
                if y as i32 - 1 < 0
                    || regions.get(&(pos.0, pos.1 - 1)).unwrap().borrow().plant != *plant
                {
                    region.borrow_mut().perimeter += 1;
                }
                region.borrow_mut().area += 1;
                region.clone()
            } else {
                let region = Rc::new(RefCell::new(Region {
                    plant: *plant,
                    area: 1,
                    perimeter: 2,
                    tiles: BTreeSet::new(),
                }));
                regions.insert(pos, region.clone());
                region
            };

            region.borrow_mut().tiles.insert(pos);

            if y + 1 >= garden.len() || garden[y + 1][x] != *plant {
                region.borrow_mut().perimeter += 1;
            } else {
                region.borrow_mut().tiles.insert((pos.0, pos.1 + 1));
                regions.insert((pos.0, pos.1 + 1), region.clone());
            }

            if x + 1 >= row.len() || row[x + 1] != *plant {
                region.borrow_mut().perimeter += 1;
            } else {
                if let Some(other) = regions.get(&(pos.0 + 1, pos.1)) {
                    let other = other.clone();
                    if !other.as_ptr().eq(&region.as_ptr())
                        && other.borrow().plant == region.borrow().plant
                    {
                        region.borrow().tiles.iter().for_each(|tile| {
                            regions.insert(*tile, other.clone());
                        });
                        other
                            .borrow_mut()
                            .tiles
                            .append(&mut region.borrow_mut().tiles);
                        other.borrow_mut().area += region.borrow().area;
                        other.borrow_mut().perimeter += region.borrow().perimeter;
                    }
                } else {
                    regions.insert((pos.0 + 1, pos.1), region.clone());
                }
            }
        }
    }

    let region_set = regions.into_values().collect::<BTreeSet<_>>();
    region_set.iter().map(|r| r.borrow().price()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 1206);
    }
}
