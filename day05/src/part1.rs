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
}

pub fn process(input: &str) -> anyhow::Result<String> {

    // parse input
    let (range_input, ingredients_input) = input.split_once("\r\n\r\n").unzip();

    let ranges: Vec<Range> = range_input.unwrap().lines().map(|x| Range::new_from_str(x)).collect();
    let ingredients: Vec<usize> = ingredients_input.unwrap().lines().map(|x| x.parse::<usize>().unwrap()).collect();

    // merge ranges, should still be sorted
    let ranges = merge_ranges(ranges);

    let mut fresh_count = 0;

    for ingredient in ingredients {
        // check if fresh
        for range in ranges.iter() {
            // if ingredient is smaller than range.start, its spoiled as no other ranges would include it as we have merged ranges increasingly
            if ingredient < range.start {
                break;
            } // if its within the range, its fresh
            else if ingredient >= range.start && ingredient <= range.end {
                fresh_count += 1;
                break;
            }
        }
    }

    Ok(fresh_count.to_string())
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
                assert_eq!("3", process(input)?);
                Ok(())
    }
}