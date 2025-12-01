pub fn process(input: &str) -> anyhow::Result<String> {

    // parse input into addition and subtractions, aka signed ints

    // we start at pos 50
    let mut lock = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        // split into rotation and number
        let (rotation, num) = line.split_at(1);

        // cast input to number and use modulus 100, as it would take a rotation of 100 steps to go back to current position. So we only want the remainder in actual moves, the rest is just repeat cycles
        let number = match rotation {
            "L" => (num.parse::<i32>()? % 100)* -1,
            "R" => num.parse::<i32>()? % 100,
            _ => {0}
        };

        // count rotations pass 0
        zero_count += num.parse::<i32>()? / 100;

        // check if passing 0
        if (lock + number > 100 || lock + number < 0) && lock != 0 {
            zero_count += 1;
        }

        // add to lock
        lock += number;

        // handle 0->99 and 99->0
        if lock > 99 {
            // subtract 100 (we do one more to simulate the move from 99 -> 0 as that also costs one
            lock -= 100;
        } else if  lock < 0 {
            // add 99
            lock += 100;
        }

        // check if 0
        if lock == 0 {
            zero_count += 1;
        }
    }

    Ok(zero_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}