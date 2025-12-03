

// get index of max value that is not the last element
fn get_index_of_max(list: &[u8], from: usize, to: usize, used_array: &[bool]) -> anyhow::Result<usize> {

    let mut val_index = from;
    let mut val = 0;

    for index in from..=to {

        if used_array[index] {
            continue;
        }

        if list[index] > val {
            val_index = index;
            val = list[index];
        }
    }
    Ok(val_index)
}

fn combine_to_num(list: Vec<u8>) -> u64 {
    let mut acc = 0;
    for (i, &value) in list.iter().enumerate() {
        acc += 10u64.pow((list.len() -1 - i) as u32) * value as u64;
    }
    acc
}

pub fn process(input: &str) -> anyhow::Result<String> {

    let mut results: Vec<u64> = vec![];

    // iterate over battery banks
    for bank in input.lines() {

        // parse input into a vec of u64
        let bats = bank.chars().map(|x| x.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();

        // save an array of indexes used as we can't use the same index twice, and pass it to the max_index function
        let mut index_track = vec![false; input.len()];
        let mut start_index = 0;

        for j in 0..12 {
            // find index of largest number that is 12-bat_num away from the end
            let max_search_index = 12-j;
            let max_index = get_index_of_max(&bats, start_index, bats.len()-max_search_index, &index_track)?;
            // update housekeeping
            start_index = max_index;
            index_track[max_index] = true;
        }

        // filter original bats by removing all that is false in index_track
        let res: Vec<u8> = bats
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| index_track[i])
            .map(|(_, x)| x)
            .collect();

        results.push(combine_to_num(res));
    }

    Ok(results.iter().sum::<u64>().to_string())
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
        assert_eq!("3121910778619", process(input)?);
        Ok(())
    }
}