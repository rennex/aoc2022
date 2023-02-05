
// 11:22 start
// 12:41 solved part 1
// 13:07 solved part 2 (total: 1h 45min)

// #![allow(unused)]
use std::fmt::Display;

use input_downloader;

fn main() {
    let inputs = input_downloader::year(2022)
        .example(
"30373
25512
65332
33549
35390")
        .get_all();

    for input in inputs {
        solve(&input);
    }
}

fn solve(input: &String) {
    let mut grid = TreeGrid::from(input);
    println!("grid: {} * {}", grid.w, grid.h);
    grid.calc_visibility();
    let visible = grid.count_visible();
    println!("Found {visible} visible trees");

    // part 2
    grid.calc_scenic_scores();
    println!("Max scenic score = {}", grid.max_scenic_score);
}

struct TreeGrid {
    w: i32, h: i32,
    trees: Vec<Vec<Tree>>,
    max_scenic_score: i32
}

impl TreeGrid {
    fn from(input: &String) -> Self {
        let mut trees = vec![];
        let mut w = 0;
        for line in input.lines() {
            let mut row = vec![];
            w = line.len();
            for h in line.bytes() {
                row.push(Tree::new(h - ('0' as u8)));
            }
            trees.push(row);
        }
        if !trees.iter().all(|row| row.len() == w) {
            panic!("ffs, not all tree rows are {w} wide");
        }
        Self { w: w as i32, h: trees.len() as i32, trees, max_scenic_score: 0 }
    }

    fn get(&mut self, pos: Coord) -> &mut Tree {
        if !self.contains_coord(pos) {
            panic!("tree grid doesn't contain {pos}");
        }
        &mut self.trees[pos.y as usize][pos.x as usize]
    }

    fn contains_coord(&self, pos: Coord) -> bool {
        (0..self.w).contains(&pos.x) && (0..self.h).contains(&pos.y)
    }

    fn calc_visibility(&mut self) {
        let max_x = self.w - 1;
        let max_y = self.h - 1;
        for x in 0..self.w {
            self.calc_visibility_stretch(Coord::new(x, 0), Down);
            self.calc_visibility_stretch(Coord::new(x, self.h-1), Up);
        }
        for y in 0..self.h {
            self.calc_visibility_stretch(Coord::new(0, y), Right);
            self.calc_visibility_stretch(Coord::new(self.w-1, y), Left);
        }
        self.get(Coord::new(0, 0)).visible = true;
        self.get(Coord::new(max_x, 0)).visible = true;
        self.get(Coord::new(0, max_y)).visible = true;
        self.get(Coord::new(max_x, max_y)).visible = true;
    }

    fn calc_visibility_stretch(&mut self, mut pos: Coord, dir: Direction) {
        let along_edge = match dir {
            Up | Down => pos.x == 0 || pos.x == self.w - 1,
            Left | Right => pos.y == 0 || pos.y == self.h - 1,
        };
        if along_edge {
            // println!("Skipping scanning {dir:?} from {pos}");
            return;
        }
        // println!("calculating visibility from {pos} toward {dir:?}");
        let mut seen_h = {
            let mut tree = self.get(pos);
            tree.visible = true;
            tree.height
        };
        pos = pos.move_toward(dir);

        loop {
            let nextpos = pos.move_toward(dir);
            if !self.contains_coord(nextpos) {
                self.get(pos).visible = true;
                break;
            }
            let mut tree = self.get(pos);

            if tree.height > seen_h {
                tree.visible = true;
                seen_h = tree.height;
                // println!("Found visible tree at {pos}");
            }
            pos = nextpos;
        }
    }

    fn count_visible(&mut self) -> usize {
        let mut count = 0;
        for y in 0..self.h {
            for x in 0..self.w {
                let tree = self.get(Coord::new(x, y));
                if tree.visible {
                    count += 1;
                    // print!("#");
                } else {
                    // print!(".");
                }
            }
            // println!("");
        }
        count
    }

    fn calc_scenic_scores(&mut self) {
        for x in 1..self.w-1 {
            for y in 1..self.h-1 {
                let pos = Coord::new(x, y);
                let score_d = self.calc_visible_trees_toward(pos, Down);
                let score_u = self.calc_visible_trees_toward(pos, Up);
                let score_r = self.calc_visible_trees_toward(pos, Right);
                let score_l = self.calc_visible_trees_toward(pos, Left);
                let score = score_d * score_u * score_r * score_l;
                // println!("score at {pos} = {score} ({score_u} * {score_l} * {score_d} * {score_r})");
                if score > self.max_scenic_score {
                    self.max_scenic_score = score;
                }
            }
        }
    }

    fn calc_visible_trees_toward(&mut self, mut pos: Coord, dir: Direction) -> i32 {
        let mut count = 0;
        let height = self.get(pos).height;
        loop {
            let nextpos = pos.move_toward(dir);
            if !self.contains_coord(nextpos) {
                break;
            }
            count += 1;
            let tree = self.get(nextpos);
            if tree.height >= height {
                break;
            }
            pos = nextpos;
        }
        return count;
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up, Down, Left, Right
}
use Direction::*;

#[derive(Copy, Clone)]
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
}

#[derive(Debug, Clone)]
struct Tree {
    height: u8,
    visible: bool
}

impl Tree {
    fn new(height: u8) -> Tree {
        Tree { height, visible: false }
    }
}

