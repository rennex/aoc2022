
// 15:30 start
// 15:50 part 1 done
// 16:10 part 2 done

// #![allow(unused)]
use std::fmt::Display;
use std::ops::{Add, Sub};
use std::collections::HashSet;

use input_downloader;

fn main() {
    let inputs = input_downloader::year(2022)
.example(
"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2").example(
"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20")
        .get_all();

    for input in inputs {
        solve(&input);
    }
}

fn solve(input: &String) {
    let mut head = Coord::new(0, 0);
    let mut tail_coords = HashSet::new();
    tail_coords.insert(Coord::new(0, 0));

    let mut knots = vec![];
    for _ in 0..9 {
        knots.push(Coord::new(0, 0));
    }

    for line in input.lines() {
        let dir = match line.chars().nth(0).unwrap() {
            'U' => Up,
            'D' => Down,
            'L' => Left,
            'R' => Right,
            _ => panic!("wtf")
        };
        let steps: i32 = line[2..].parse().unwrap();
        // println!("{steps} steps {dir:?}");

        for _ in 0..steps {
            head = head.move_toward(dir);
            move_knots(&mut knots, head, &mut tail_coords);
        }
    }

    println!("tail_coords has {} items", tail_coords.len());
}

fn move_knots(knots: &mut [Coord], head: Coord, tail_coords: &mut HashSet<Coord>) {
    let knot_ref = &mut knots[0];
    let knot = *knot_ref;
    let diff = head - knot;
    // println!("head={head}, knot={knot}, diff={diff}");
    if diff.max_abs() > 1 {
        // knot needs to move too
        let diff_step = diff.limit_to_1();
        let new_knot = knot + diff_step;
        *knot_ref = new_knot;
        // is this the tail, i.e. the last knot?
        if knots.len() == 1 {
            tail_coords.insert(new_knot);
        } else {
            move_knots(&mut knots[1..], new_knot, tail_coords);
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up, Down, Left, Right
}
use Direction::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }

    fn move_toward(&self, direction: Direction) -> Coord {
        let (dx, dy) = match direction {
            Up => (0,-1),
            Down => (0,1),
            Left => (-1,0),
            Right => (1,0)
        };
        Coord {
            x: self.x + dx,
            y: self.y + dy
        }
    }

    fn max_abs(&self) -> i32 {
        Ord::max(self.x.abs(), self.y.abs())
    }

    fn limit_to_1(&self) -> Self {
        let x_abs = Ord::max(self.x.abs(), 1);
        let y_abs = Ord::max(self.y.abs(), 1);
        Coord { x: self.x / x_abs, y: self.y / y_abs }
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
