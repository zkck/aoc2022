use std::collections;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::iter;

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
        Self { well: vec![] }
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
            }
        }
    }

    fn shape(&self) -> Vec<(i32, i32)> {
        let mut pos = (self.well.len() as i32, 0);
        let mut dir = (0, 1); // could actually be any direction

        let mut path = vec![];
        while pos != (self.well.len() as i32, 6) {
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

    fn drop<I>(&mut self, rock: &Vec<Vec<bool>>, actions: &mut I)
    where
        I: Iterator<Item = (usize, Action)>,
    {
        let (mut i, mut j) = (self.well.len() as i32 + 3, 2);
        loop {
            let dj = match actions.next() {
                Some((_, Action::Forward)) => 1,
                Some((_, Action::Backward)) => -1,
                None => panic!("EOF reached."),
            };
            if !self.conflicts(rock, (i, j + dj)) {
                j += dj;
            }
            if self.conflicts(rock, (i - 1, j)) {
                break;
            }
            i -= 1;
        }
        self.solidify(rock, (i as usize, j as usize));
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
        let diff: i32 = self.well.len() as i32 - HEAD as i32;
        if diff > 0 {
            writeln!(f, "({} more lines)", diff)?;
        }
        Ok(())
    }
}

/// Drops rocks until a cycle is found.
///
/// # Panics
///
/// Panics if .
fn find_cycle<I>(well: &mut Well, actions: &mut iter::Peekable<I>) -> (usize, (usize, usize))
where
    I: Iterator<Item = (usize, Action)>,
{
    let mut rocks = rocks().into_iter().enumerate().cycle().enumerate();
    let mut seen_states = collections::HashMap::new();

    while let Some((round_id, (rock_id, rock))) = rocks.next() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", well);
        well.drop(&rock, actions);

        let (action_id, _) = *actions.peek().unwrap();
        let state = (action_id, rock_id, well.shape());
        if seen_states.contains_key(&state) {
            return (round_id, seen_states[&state]);
        } else {
            seen_states.insert(state, (round_id, well.well.len()));
        };
    }
    panic!("unable to find cycle")
}

fn solve<T>(actions: &mut iter::Peekable<T>) -> usize
where
    T: Iterator<Item = (usize, Action)>,
{
    let mut well = Well::new();

    // find cycle in the rock-dropping process
    let (current_round, (cycle_round_id, height_at_cycle)) = find_cycle(&mut well, actions);

    // minus one at the end because one cycle is already in the well
    let num_cycles = (NUM_ROUNDS - (cycle_round_id + 1)) / (current_round - cycle_round_id) - 1;

    // simulate the remaining cycles
    let height_diff = num_cycles * (well.well.len() - height_at_cycle);

    let remaining_rounds = (NUM_ROUNDS - (cycle_round_id + 1)) % (current_round - cycle_round_id);
    for rock in rocks()
        .into_iter()
        .cycle()
        .skip(cycle_round_id + 1)
        .take(remaining_rounds)
    {
        well.drop(&rock, actions);
    }

    height_diff + well.well.len()
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
                    .enumerate()
                    .cycle()
                    .peekable()
            )
        );
        return Ok(());
    }
    panic!("File is empty")
}
