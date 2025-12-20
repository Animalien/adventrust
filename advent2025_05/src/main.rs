use std::collections::LinkedList;

use adventlib::*;

fn do_ranges_overlap(range0: &(usize, usize), range1: &(usize, usize)) -> bool {
    if range0.0 > range1.1 {
        return false;
    }

    if range0.1 < range1.0 {
        return false;
    }

    true
}

fn merge_ranges(range0: &(usize, usize), range1: &(usize, usize)) -> (usize, usize) {
    (range0.0.min(range1.0), range0.1.max(range1.1))
}

fn absorb_fresh_range(fresh_id_ranges: &mut LinkedList<(usize, usize)>, new_range: &(usize, usize), verbose: bool) {
    let mut new_ranges_list = LinkedList::<(usize, usize)>::new();

    let mut added_new_range = false;
    let mut check_for_overlaps_remaining = false;

    while !fresh_id_ranges.is_empty() {
        let top_old_range = fresh_id_ranges.pop_front().unwrap();

        if added_new_range {
            if check_for_overlaps_remaining && !fresh_id_ranges.is_empty() {
                let mut temp_iter = fresh_id_ranges.iter().peekable();
                let next_old_range = temp_iter.peek().unwrap();

                if verbose {
                    println!("  looking at {:?} vs {:?}", top_old_range, next_old_range);
                }

                if do_ranges_overlap(&top_old_range, &next_old_range) {
                    // they overlap, so merge them into a single one
                    let merged_range = merge_ranges(&top_old_range, &next_old_range);

                    // pop the next one as well, now that we have absorbed it
                    fresh_id_ranges.pop_front();

                    // but push new merged one back onto the original so we can keep checking overlaps later in the list
                    fresh_id_ranges.push_front(merged_range);
                } else {
                    // no overlap, so simply push the top range and stop checking for overlaps (as we did not allow any previously)
                    new_ranges_list.push_back(top_old_range);
                    check_for_overlaps_remaining = false;
                }
            } else {
                // already added new range, and have no overlap checking to do, so simply push remaining
                new_ranges_list.push_back(top_old_range);
                check_for_overlaps_remaining = false;
            }
        } else if new_range.1 < top_old_range.0 {
            // new range comes completely before original range
            new_ranges_list.push_back(*new_range);
            new_ranges_list.push_back(top_old_range);
            added_new_range = true;
        } else if do_ranges_overlap(&new_range, &top_old_range) {
            // new range overlaps with original range, so replace the current range with a merged range and continue
            let merged_range = merge_ranges(&new_range, &top_old_range);

            // but push back onto the original so we can keep checking overlaps
            fresh_id_ranges.push_front(merged_range);
            added_new_range = true;
            check_for_overlaps_remaining = true;
        } else {
            // simply progress past this top one, and transfer it over
            new_ranges_list.push_back(top_old_range);
        }
    }

    if !added_new_range {
        new_ranges_list.push_back(new_range.clone());
    }

    *fresh_id_ranges = new_ranges_list;

    if verbose {
        println!("  absorbed range \n{}-{}\n  into the list, leaving us with (list len = {}):", new_range.0, new_range.1, fresh_id_ranges.len());

        let mut prev: Option<(usize, usize)> = None;
        for r in fresh_id_ranges.iter() {
            if prev.is_some() {
                assert!(prev.unwrap().1 < r.0);
            }
            assert!(r.0 <= r.1);
            println!("    {}-{}", r.0, r.1);

            prev = Some(r.clone());
        }
    }
}

fn process(filename: &str, verbose1: bool, verbose2: bool) {
    let lines = match read_file_lines(filename) {
        Err(err) => {
            println!("process failed: {:?}", err);
            return;
        }
        Ok(val) => val
    };

    println!("Processing contents of {}", filename);

    let mut fresh_id_ranges = LinkedList::<(usize, usize)>::new();

    let mut lines_iter = lines.iter().peekable();

    while !lines_iter.peek().unwrap().is_empty() {
        let range_line = lines_iter.next().unwrap();
        let mut range_iter = range_line.split('-');
        let range_start = range_iter.next().unwrap().parse::<usize>().unwrap();
        let range_end = range_iter.next().unwrap().parse::<usize>().unwrap();
        let new_range = (range_start, range_end);

        absorb_fresh_range(&mut fresh_id_ranges, &new_range, verbose2);
    }
    lines_iter.next();

    let mut num_fresh_ingredients_provided = 0usize;
    while lines_iter.peek().is_some() {
        let line = lines_iter.next().unwrap();
        let id = line.parse::<usize>().unwrap();

        let mut is_fresh = false;
        for range in &fresh_id_ranges {
            if (id >= range.0) && (id <= range.1) {
                is_fresh = true;

                if verbose1 {
                    println!("  {id} is fresh because it is in the range {}-{}", range.0, range.1);
                }

                break;
            }
        }

        if is_fresh {
            num_fresh_ingredients_provided += 1;
        } else {
            if verbose1 {
                println!("  {id} is spoiled!");
            }
        }
    }

    println!("  Total number of fresh ingredients from provided list = {num_fresh_ingredients_provided}");

    let mut num_fresh_ingredients_possible = 0usize;

    if verbose2 {
        println!("  Calculating num of fresh ingredients possible:");
    }
    for range in &fresh_id_ranges {
        let num_in_range = range.1 - range.0 + 1;
        num_fresh_ingredients_possible += num_in_range;

        if verbose2 {
            println!("    {num_in_range} in range {}-{}, adding to running total of {num_fresh_ingredients_possible}", range.0, range.1);
        }
    }

    println!("  Total number of fresh ingredients possible = {num_fresh_ingredients_possible}");
}

fn main() {
    process("Example.txt", true, true);
    process("Input.txt", false, false);
}
