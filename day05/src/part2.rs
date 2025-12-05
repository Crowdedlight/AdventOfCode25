use std::cmp::max;

#[derive(Clone, Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Range {
        Range { start, end }
    }
    fn new_from_str(input: &str) -> Range {
        let (start, end) = input.split_once("-").unwrap();

        Range::new(start.parse::<usize>().unwrap(), end.parse::<usize>().unwrap())
    }
}

fn merge_ranges(ranges: Vec<Range>) -> Vec<Range> {

    // as some ranges can overlap, we could remove ranges that is inside other ranges?
    // but how do we effectively figure out if a range is inside another range?
    let mut merged: Vec<Range> = vec![];
    let mut sorted_ranges = ranges.clone();

    // sort based on start number, then end number
    sorted_ranges.sort_by(|a, b| a.start.cmp(&b.start).then(a.end.cmp(&b.end)));

    // while loop through all ranges
    let mut merged_num = 10;

    // set start input
    merged.push(sorted_ranges.first().unwrap().clone());

    // loop through each sorted range
    for (i, range) in sorted_ranges.iter().enumerate() {

        // merge bool
        let mut did_merge = false;

        // check each element in merged, to see if we can merge into a range, as we are sorted, we should be able to always do it in on loop
        for (m_i, m_r) in merged.clone().iter().enumerate() {
            // if my start is less or equal to merged end, then we can expand the merge
            if range.start <= m_r.end {
                // get max end
                let end = max(range.end, m_r.end);

                // update existing range and skip adding ours to the list
                merged[m_i].end = end;

                // we found a match and we can't have multiple, so we break
                did_merge = true;
                break;
            }
        }

        // we no merge happened, then we can not merge this range
        if !did_merge {
            merged.push(range.clone());
        }
    }

    merged






    // while merged_num != 0 {
    //     // reset num
    //     merged_num = 0;
    //
    //     // set sorted_input_vec
    //     let sorted_input = merged.clone();
    //
    //     // for each range check if selected ranges end-number is higher than the start-number, then continue
    //     for (i, range) in sorted_ranges.iter().enumerate() {
    //         // check later ranges
    //         for j in i..sorted_ranges.len() {
    //             // If we have reached a range of start nums that is bigger than my end, then its impossible to be within the range
    //             if sorted_ranges[j].start > range.end {
    //                 break;
    //             }
    //
    //             // if start is less than end, we can merge
    //             if sorted_ranges[j].start < range.end {
    //                 // we can merge ranges and push, taking the biggest end as range-end
    //                 let end = max(sorted_ranges[j].end, range.end);
    //                 merged.push(Range::new(range.start, end));
    //                 merged_num += 1;
    //             }
    //         }
    //         // if no merges could happen, we just push current range
    //         if merged_num == 0 {
    //             merged.push(range.clone());
    //         }
    //     }
    //
    //     // cleanup,
    //
    // }

}

pub fn process(input: &str) -> anyhow::Result<String> {

    // parse input
    let mut range_input: Option<&str> = None;
    let mut ingredients_input: Option<&str> = None;
    if cfg!(test) {
        (range_input, ingredients_input) = input.split_once("\n\n").unzip();
    } else {
        (range_input, ingredients_input) = input.split_once("\r\n\r\n").unzip();
    }

    let ranges: Vec<Range> = range_input.unwrap().lines().map(|x| Range::new_from_str(x)).collect();
    let ingredients: Vec<usize> = ingredients_input.unwrap().lines().map(|x| x.parse::<usize>().unwrap()).collect();

    // merge ranges, should still be sorted
    let ranges = merge_ranges(ranges);

    // sum the length of ranges
    let sum_ranges: usize = ranges.iter().map(|x| (x.end - x.start)+1).sum();

    Ok(sum_ranges.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}