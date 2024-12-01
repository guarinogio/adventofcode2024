#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let (mut left, mut right) = parse(input);

    left.sort();
    right.sort();

    // Calculate the total distance between paired elements
    let total: u32 = left
        .iter() // Iterate over the left list
        .zip(right.iter()) // Pair each element with the corresponding one in the right list
        .map(|(left, right)| (left - right).unsigned_abs()) // Calculate the absolute difference
        .sum(); // Sum up all the differences
    total
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let (left, right) = parse(input);

    // Create a HashMap to count occurrences of each number in the right list
    let mut right_counts = std::collections::HashMap::new();

    // Count the occurrences of each number in the `right` list
    for &num in &right {
        *right_counts.entry(num).or_insert(0) += 1; // Increment the count for `num`, initializing to 0 if it doesn't exist
    }

    // Calculate the similarity score
    let similarity: u32 = left
        .iter() // Iterate over each number in the `left` list
        .map(|&num| {
            let count = *right_counts.get(&num).unwrap_or(&0); // Get the count of `num` from the `right_counts` map, or 0 if it doesn't exist
            num as u32 * count as u32 // Multiply `num` by its count and return the result
        })
        .sum(); // Sum up all the results to compute the total similarity score

    similarity
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        // Split the line into numbers and parse them into integers
        let parts: Vec<i32> = line
            .split_whitespace() // Split by whitespace
            .filter_map(|num| num.parse().ok()) // Try to parse each part as an integer
            .collect(); // Collect the parsed numbers into a vector

        if parts.len() == 2 {
            left.push(parts[0]);
            right.push(parts[1]);
        }
    }

    (left, right) // Return the two lists as a tuple
}
