use std::io;

trait Matrix {
    fn height(&self) -> u32;
    fn width(&self) -> u32;

    fn value_at(&self, i: u32, j: u32) -> (u32, u32);
}

struct IndicesMatrix {
    width: u32,
    height: u32,
}

impl Matrix for IndicesMatrix {
    fn height(&self) -> u32 {
        self.height
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn value_at(&self, i: u32, j: u32) -> (u32, u32) {
        (i, j)
    }
}

struct RotatedMatrix<T> {
    base_matrix: Box<T>,
}

impl<T> Matrix for RotatedMatrix<T>
where
    T: Matrix,
{
    fn height(&self) -> u32 {
        self.base_matrix.width()
    }

    fn width(&self) -> u32 {
        self.base_matrix.height()
    }

    fn value_at(&self, i: u32, j: u32) -> (u32, u32) {
        // delegate to base matrix
        self.base_matrix.value_at(self.width() - 1 - j, i)
    }
}

fn mark_visible_trees<T>(matrix: &T, lines: &Vec<Vec<char>>, trees: &mut Vec<Vec<bool>>)
where
    T: Matrix,
{
    for i in 0..matrix.height() {
        let mut max: i32 = -1;
        for j in 0..matrix.width() {
            let (i, j) = matrix.value_at(i, j);
            let tree_height = lines[i as usize][j as usize].to_digit(10).unwrap() as i32;
            if tree_height > max {
                max = tree_height;
                trees[i as usize][j as usize] = true;
            }
        }
    }
}

fn main() {
    let lines: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let (width, height) = (lines.len(), lines[0].len());
    let matrix = IndicesMatrix {
        width: width as u32,
        height: height as u32,
    };
    let mut trees = vec![vec![false; width as usize]; height as usize];
    mark_visible_trees(&matrix, &lines, &mut trees);
    let matrix = RotatedMatrix {
        base_matrix: Box::new(matrix),
    };
    mark_visible_trees(&matrix, &lines, &mut trees);
    let matrix = RotatedMatrix {
        base_matrix: Box::new(matrix),
    };
    mark_visible_trees(&matrix, &lines, &mut trees);
    let matrix = RotatedMatrix {
        base_matrix: Box::new(matrix),
    };
    mark_visible_trees(&matrix, &lines, &mut trees);
    let count: u32 = trees.iter().flatten().map(|&b| b as u32).sum();
    println!("{}", count);
}
