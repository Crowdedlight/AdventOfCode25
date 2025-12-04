

#[derive(Debug, Default, Clone)]
struct Matrix {
    pub rows: Vec<Vec<char>>
}
impl Matrix {
    pub fn get(&self, x: i32, y: i32) -> Option<char> {

        if x < 0 || y < 0 {
            return None;
        }

        Some(*self.rows.get(y as usize)?.get(x as usize)?)
    }
    pub fn get_with_pos(&self, pos: (i32, i32)) -> Option<char> {
        self.get(pos.0, pos.1)
    }
    pub fn add_row(&mut self, row: Vec<char>) {
        self.rows.push(row);
    }

    pub fn new(input: &str) -> Self {
        let mut matrix = Matrix::default();

        for line in input.lines() {
            matrix.add_row(line.chars().collect());
        }
        matrix
    }

    pub fn set(&mut self, x: usize, y: usize, c: char) {
        self.rows[y][x] = c;
    }

    pub fn set_all_to(&mut self, input: char) {
        self.rows.iter_mut().for_each(|i| {
            i.iter_mut().for_each(|c| *c = input);
        });
    }

    pub fn print(&self) {
        for row in self.rows.iter() {
            println!("{:?}", row.iter().collect::<String>());
        }
    }
}


fn count_adjacent(x: i32, y:i32, matrix: &Matrix) -> usize {
    // possible positions
    let positions: Vec<(i32, i32)> = vec![
        (x,y-1),    // up
        (x, y+1),   // down
        (x-1, y),   // left
        (x+1, y),   // right
        (x+1, y-1), // right-top-diag
        (x-1, y-1), // left-top-diag
        (x+1, y+1), // right-bottom-diag
        (x-1, y+1)  // left-bottom-diag
    ];

    let mut count = 0;

    for pos in positions {
        if let Some(c) = matrix.get_with_pos(pos) {
            if c == '@' {
                count += 1;
            }
        }
    }
    count
}

pub fn process(input: &str) -> anyhow::Result<String> {

    // make matrix of our map for processing
    let mut m = Matrix::new(input);
    let mut m_processed = m.clone();

    // before print
    println!("-------------------------------");
    println!("------------INPUT--------------");
    println!("-------------------------------");
    m.print();

    // while loop, we keep going until there is no change in removed_rolls
    let mut removed_rolls = 0;
    let mut removed_rolls_this_loop = 10; // had to set initial to get into first loop iteration

    while (removed_rolls_this_loop != 0) {

        removed_rolls_this_loop = 0;

        // for each position, count amount of rolls in the 8 adjecent positions
        for (y, row) in m.rows.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                // Got my x,y, val for each element
                if *val == '@' {
                    // count adjecent rolls
                    let count = count_adjacent(x as i32, y as i32, &m);

                    if count < 4 {
                        removed_rolls += 1;
                        removed_rolls_this_loop += 1;
                        m_processed.set(x, y, 'X');
                    }
                }
            }
        }
        println!("---------------------------------------");
        println!("---------REMOVED {} ROLLS--------------", removed_rolls_this_loop);
        println!("---------------------------------------");

        // processed print
        m_processed.print();

        // save input matrix as a clone of processed for new input, so we continue the work
        m = m_processed.clone();
    }

    Ok(removed_rolls.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!("43", process(input)?);
        Ok(())
    }
}