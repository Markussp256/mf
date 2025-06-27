fn backtrack(v: &Vec<Vec<usize>>, target: usize, index: usize, result: &mut Vec<usize>) -> bool {
    // Base case: if we reached the end of the vectors
    if index == v.len() {
        return target == 0; // If the target is exactly 0, we found a valid combination
    }

    // Try each element in the current Vec<usize>
    for &num in &v[index] {
        if num <= target { // Only proceed if the element can fit in the remaining target
            result.push(num);
            if backtrack(v, target - num, index + 1, result) {
                return true; // If we found a valid combination, return true
            }
            result.pop(); // Backtrack if no valid combination was found
        }
    }
    false // No valid combination found
}


pub fn find_combination(v: &Vec<Vec<usize>>, s: usize) -> Option<Vec<usize>> {
    let mut result = Vec::new();
    backtrack(&v, s, 0, &mut result)
        .then_some(result)
}