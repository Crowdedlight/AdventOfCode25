use std::iter::successors;

struct Range {
    start: u64,
    end: u64,
    start_str: String,
    end_str: String
}

impl Range {
    fn new_parse(input: &str) -> Range {
        let (start, end) = input.split_once("-").unwrap();

        Range {
            start: start.parse::<u64>().unwrap(),
            end: end.parse::<u64>().unwrap(),
            start_str: start.to_string(),
            end_str: end.to_string()
        }
    }
}

fn count_digits(num: u64) -> u64 {
    successors(Some(num), |&n| (n >= 10).then(|| n / 10)).count() as u64
}

fn is_even_num_digits(num: u64) -> bool {
    count_digits(num) % 2 == 0
}

pub fn process(input: &str) -> anyhow::Result<String> {

    // parse input, split into ranges
    let ranges: Vec<Range> = input.split(",").map(|x| Range::new_parse(x)).collect();

    let mut vec_false_ids: Vec<u64> = vec![];

    for range in ranges {

        // each number has to be even digits. Splitting it in the middle and subtracting each has to give 0
        // TODO there might be a better way to skip many numbers forward based on how out of whack it is...
        for n in range.start..=range.end {
            // check if even num of digits
            if !is_even_num_digits(n) {
                continue;
            }

            // split
            let divider = 10u64.pow((count_digits(n) / 2) as u32);
            let first_part = n/divider;
            let second_part = n%divider;

            if first_part as i64 - second_part as i64 == 0 {
                vec_false_ids.push(n);
            }
        }

    }

    Ok(vec_false_ids.iter().sum::<u64>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
                assert_eq!("1227775554", process(input)?);
                Ok(())
    }
}