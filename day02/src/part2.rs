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

fn frequency_analysis_vec(num: &str) -> Vec<u64> {

    let mut freq: Vec<u64> = vec![0; 10];

    // split num into digits, easiest to do it from the string value
    for char in num.chars() {
        let parsed: u32 = char.to_digit(10).unwrap();
        freq[parsed as usize] += 1;
    }

    freq
}

pub fn process(input: &str) -> anyhow::Result<String> {

    // parse input, split into ranges
    let ranges: Vec<Range> = input.split(",").map(|x| Range::new_parse(x)).collect();

    let mut vec_false_ids: Vec<u64> = vec![];

    for range in ranges {

        // each number has to be even digits. Splitting it in the middle and subtracting each has to give 0
        // TODO there might be a better way to skip many numbers forward based on how out of whack it is...
        for n in range.start..=range.end {

            // try with bruteforce
            let digit_count = count_digits(n);
            let n_string = n.to_string();

            // have to make windows of all possible sizes
            for i in 1..=digit_count/2 {

                let subs = n_string.as_bytes()
                    .chunks(i as usize)
                    .map(str::from_utf8)
                    .collect::<Result<Vec<&str>, _>>()?;

                if subs.iter().all(|&x| x == subs[0]) {
                    vec_false_ids.push(n);
                    break;
                }
            }

            // TODO this seemed like a much faster approach, but edge cases of : "1001" got added as they fit requirement of digits frequency, but was not in right order
            //  similarly, numbers with identical digits in the pattern would not count, as their frequency array would not all be identical... Couldn't figure out how to handle both edgecases
            // // get frequency analysis
            // let analysis = frequency_analysis_vec(&n.to_string());
            //
            // // get filtered analysis with all empties removed
            // let filtered_analysis: Vec<u64> = analysis.iter().filter(|x| **x != 0).cloned().collect();
            //
            // // if length of filtered analysis is equal to digits/2 and all elements are identical, then we got a invalid id
            // if filtered_analysis.len() <= (count_digits(n)/2) as usize && filtered_analysis.iter().all(|&x| x == filtered_analysis[0] || x == filtered_analysis[0]/2) {
            //     vec_false_ids.push(n);
            // }
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
        assert_eq!("4174379265", process(input)?);
        Ok(())
    }
}