use std::{fmt::Display, io};

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

impl Well {
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
                self.well[i][j] = is_solid;
            }
        }
    }
}

impl Display for Well {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.well.iter().rev() {
            writeln!(
                f,
                "{}",
                row.map(|is_solid| if is_solid { "#" } else { "." })
                    .join("")
            )?;
        }
        writeln!(f)
    }
}

fn drop<T>(rock: &Vec<Vec<bool>>, well: &mut Well, actions: &mut T)
where
    T: Iterator<Item = char>,
{
    let (mut i, mut j) = (well.well.len() as i32 + 3 + 1, 2);
    println!("  Starting pos {:?}", (i, j));
    while !well.conflicts(rock, (i - 1, j)) {
        println!("  Falls one unit");
        i = i - 1;
        let dj = match actions.next() {
            Some('<') => -1,
            Some('>') => 1,
            _ => panic!("Unexpected character or EOL"),
        };
        if !well.conflicts(rock, (i, j + dj)) {
            println!("  Pushed {}", if dj < 0 { "left" } else { "right" });
            j += dj;
        }
    }
    well.solidify(rock, (i as usize, j as usize));
}

fn solve<T>(actions: &mut T) -> usize
where
    T: Iterator<Item = char>,
{
    let rocks = rocks();
    let mut well = Well { well: vec![] };
    for i in 0..2002 {
        println!("Dropping rock {}", i);
        drop(&rocks[i % rocks.len()], &mut well, actions);
    }
    well.well.len()
}

fn main() {
    let line = String::from(io::stdin().lines().next().unwrap().unwrap());
    println!("ans1 {}", solve(&mut line.chars().cycle()));
}
