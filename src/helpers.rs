/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

pub fn isnumeric(s: &str) -> bool {
    s.chars().all(|c| c.is_numeric())
}
