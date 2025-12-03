

// get index of max value that is not the last element
fn get_index_of_max(list: &[u8]) -> anyhow::Result<usize> {

    let mut val_index = 0;

    for (index, &value) in list.iter().enumerate() {

        if value > list[val_index] {
            val_index = index;
        }
    }
    Ok(val_index)
}

fn combine_to_num(first: u8, second: u8) -> u32 {
    first as u32 * 10 + second as u32
}

pub fn process(input: &str) -> anyhow::Result<String> {

    let mut results: Vec<u32> = vec![];

    // iterate over battery banks
    for bank in input.lines() {

        // parse input into a vec of u64
        let bats = bank.chars().map(|x| x.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();

        // split at biggest number, as long as its not the last number
        // then in the subset that is to the right from this number, find the next biggest and that should be our answer

        // find index of largest number that is not last element
        let max_index = get_index_of_max(&bats[..bats.len() - 1])?;
        let value1 = bats[max_index];

        // split to get substring. Remember that first element here is our max value
        let (first, rest) = bats.split_at(max_index);
        // exclude first element as that is our max_index value
        let subset = &rest[1..];

        // find the next highest one in sub-set, excluding first element as that is our
        let second_index = get_index_of_max(&subset)?;
        let value2 = subset[second_index];

        results.push(combine_to_num(value1, value2));
    }

    Ok(results.iter().sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
                assert_eq!("357", process(input)?);
                Ok(())
    }
}