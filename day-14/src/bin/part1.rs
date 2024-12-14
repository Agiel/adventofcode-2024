use aocd::*;

#[aocd(2024, 14)]
fn main() {
    let input = input!();
    let sum = solve(&input, 101, 103);
    dbg!(sum);
}

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn tick(&mut self, width: usize, height: usize) {
        let (width, height) = (width as i32, height as i32);
        self.position = (
            (self.position.0 + self.velocity.0 + width) % width,
            (self.position.1 + self.velocity.1 + height) % height,
        );
    }

    fn get_quadrant(&self, width: usize, height: usize) -> Option<usize> {
        let (width, height) = (width as i32, height as i32);
        if self.position.0 < width / 2 {
            if self.position.1 < height / 2 {
                Some(0)
            } else if self.position.1 > height / 2 {
                Some(2)
            } else {
                None
            }
        } else if self.position.0 > width / 2 {
            if self.position.1 < height / 2 {
                Some(1)
            } else if self.position.1 > height / 2 {
                Some(3)
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" ").unwrap();
            let (_, p) = p.split_once("=").unwrap();
            let p = p
                .split_once(",")
                .map(|s| (s.0.parse::<i32>().unwrap(), s.1.parse::<i32>().unwrap()))
                .unwrap();
            let (_, v) = v.split_once("=").unwrap();
            let v = v
                .split_once(",")
                .map(|s| (s.0.parse::<i32>().unwrap(), s.1.parse::<i32>().unwrap()))
                .unwrap();
            Robot {
                position: p,
                velocity: v,
            }
        })
        .collect()
}

fn solve(input: &str, width: usize, height: usize) -> u32 {
    let mut robots = parse(&input);
    for _ in 0..100 {
        robots.iter_mut().for_each(|r| r.tick(width, height));
    }
    let mut quadrants = vec![0; 4];
    robots.iter().for_each(|r| {
        if let Some(q) = r.get_quadrant(width, height) {
            quadrants[q] += 1;
        }
    });

    quadrants.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input, 11, 7), 12);
    }
}
