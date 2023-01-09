use std::cmp;
use std::collections;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

const INPUT_FILEPATH: &str = "../input";
const HEAD: usize = 25;
const NUM_ROUNDS: usize = 1000000000000;

enum Action {
    Forward,
    Backward,
}

impl TryFrom<char> for Action {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '<' => Ok(Self::Backward),
            '>' => Ok(Self::Forward),
            _ => Err("Invalid character."),
        }
    }
}

fn rocks() -> Vec<Vec<Vec<bool>>> {
    vec![
        // Rock 1
        vec![vec![true, true, true, true]],
        // Rock 2
        vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ],
        // Rock 3
        vec![
            vec![true, true, true],
            vec![false, false, true],
            vec![false, false, true],
        ],
        // Rock 4
        vec![vec![true], vec![true], vec![true], vec![true]],
        // Rock 5
        vec![vec![true, true], vec![true, true]],
    ]
}

struct Well {
    well: Vec<[bool; 7]>,
    highest_westmost_solid: usize,
}

fn mul(a: &(i32, i32), c: i32) -> (i32, i32) {
    (a.0 * c, a.1 * c)
}

fn add(a: &(i32, i32), b: &(i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn sub(a: &(i32, i32), b: &(i32, i32)) -> (i32, i32) {
    add(a, &mul(b, -1))
}

fn right(a: &(i32, i32)) -> (i32, i32) {
    mul(&left(a), -1)
}

fn left(a: &(i32, i32)) -> (i32, i32) {
    // (di, dj) = (0, 1) must become (-1, 0)
    (-a.1, a.0)
}

impl Well {
    fn new() -> Self {
        Self {
            well: vec![],
            highest_westmost_solid: 0,
        }
    }
    fn conflicts(&self, rock: &Vec<Vec<bool>>, (i, j): (i32, i32)) -> bool {
        for (di, row) in rock.iter().enumerate() {
            for (dj, &is_solid) in row.iter().enumerate() {
                if self.is_solid((i + di as i32, j + dj as i32)) && is_solid {
                    return true;
                }
            }
        }
        return false;
    }

    fn is_solid(&self, (i, j): (i32, i32)) -> bool {
        if !(0 <= j && j < 7 && 0 <= i) {
            return true;
        }
        if i as usize >= self.well.len() {
            return false;
        }
        return self.well[i as usize][j as usize];
    }

    fn solidify(&mut self, rock: &Vec<Vec<bool>>, (i, j): (usize, usize)) {
        for (di, row) in rock.iter().enumerate() {
            let i = i + di;
            if i == self.well.len() {
                // Add extra row
                self.well.push([false; 7]);
            }
            for (dj, &is_solid) in row.iter().enumerate() {
                let j = j + dj;
                self.well[i][j] |= is_solid;
                if j == 0 && is_solid {
                    self.highest_westmost_solid = cmp::max(self.highest_westmost_solid, i + 1);
                }
            }
        }
    }

    fn shape(&self) -> Vec<(i32, i32)> {
        let mut pos = (self.highest_westmost_solid as i32, 0);
        let mut dir = (0, 1);
        // basically
        let mut path = vec![];
        while pos.1 != 6 {
            path.push(sub(&pos, &(self.well.len() as i32, 0)));
            // start by looking left
            dir = left(&dir);
            while self.is_solid(add(&pos, &dir)) {
                dir = right(&dir);
            }
            pos = add(&pos, &dir);
        }
        path.push(sub(&pos, &(self.well.len() as i32, 0)));
        path
    }
}

impl Display for Well {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.well.iter().rev().take(HEAD) {
            writeln!(
                f,
                "{}",
                row.map(|is_solid| if is_solid { "#" } else { "." })
                    .join("")
            )?;
        }
        if self.well.len() > HEAD {
            writeln!(f, "...")?;
        }
        Ok(())
    }
}

fn drop<I>(rock: &Vec<Vec<bool>>, well: &mut Well, actions: &mut I)
where
    I: Iterator<Item = Action>,
{
    let (mut i, mut j) = (well.well.len() as i32 + 3, 2);
    loop {
        let dj = match actions.next() {
            Some(Action::Forward) => 1,
            Some(Action::Backward) => -1,
            None => panic!("EOF reached."),
        };
        if !well.conflicts(rock, (i, j + dj)) {
            j += dj;
        }
        if well.conflicts(rock, (i - 1, j)) {
            break;
        }
        i -= 1;
    }
    well.solidify(rock, (i as usize, j as usize));
}

fn solve<T>(actions: &mut T) -> usize
where
    T: Iterator<Item = Action>,
{
    let rocks = rocks();
    let mut well = Well::new();
    let mut lines = io::stdin().lines();
    let mut seen_shapes = collections::HashSet::new();
    rocks.iter().cycle().take(NUM_ROUNDS).for_each(|rock| {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", well);
        println!("shape={:?}", well.shape());
        if !seen_shapes.insert(well.shape()) {
            println!("seen shape");
            lines.next();
        };
        drop(rock, &mut well, actions);
    });
    well.well.len()
}

fn main() -> io::Result<()> {
    let file = File::open(INPUT_FILEPATH)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!(
            "ans1 {}",
            solve(
                &mut line?
                    .chars()
                    .map(Action::try_from)
                    .map(Result::unwrap)
                    .cycle()
            )
        );
        return Ok(());
    }
    panic!("File is empty")
}
