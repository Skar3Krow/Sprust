// Function Imports
use std::cmp;
use std::cmp::min;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn levenshtein(str1: &str, str2: &str) -> usize {
    let l1 = str1.len();
    let l2 = str2.len();
    let mut grid = vec![vec![0; l2 + 1]; l1 + 1];
    for i in 0..l1 + 1 {
        grid[i][0] = i;
    }
    for j in 0..l2 + 1 {
        grid[0][j] = j;
    }

    for i in 1..l1 + 1 {
        for j in 1..l2 + 1 {
            if str1.chars().nth(i - 1) == str2.chars().nth(j - 1) {
                grid[i][j] = grid[i - 1][j - 1];
            } else {
                grid[i][j] =
                    1 + cmp::min(cmp::min(grid[i][j - 1], grid[i - 1][j]), grid[i - 1][j - 1])
            }
        }
    }
    grid[l1][l2]
}

#[allow(dead_code)]
pub fn damerau_levenshtein(a: &str, b: &str) -> usize {
    let len_a = a.len();
    let len_b = b.len();

    // Infinity value, greater than the maximum possible edit distance
    let inf = len_a + len_b;

    // Matrix: (M + 2) x (N + 2)
    let mut matrix: Vec<Vec<usize>> = vec![vec![inf; len_b + 2]];
    matrix.push({
        let mut row: Vec<usize> = vec![inf];
        row.extend(0..=len_b);
        row
    });
    for m in 1..=len_a {
        let mut row: Vec<usize> = vec![inf, m];
        row.extend(vec![0; len_b]);
        matrix.push(row);
    }

    // Holds last row each element was encountered
    let mut last_row: HashMap<char, usize> = HashMap::new();

    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    // Fill in costs
    for row in 1..=len_a {
        let ch_a = a_chars[row - 1];
        let mut last_match_col = 0;

        for col in 1..=len_b {
            let ch_b = b_chars[col - 1];
            let last_matching_row = *last_row.get(&ch_b).unwrap_or(&0);

            let cost = if ch_a == ch_b { 0 } else { 1 };

            matrix[row + 1][col + 1] = min(
                min(
                    matrix[row][col] + cost,  // Substitution
                    matrix[row + 1][col] + 1, // Addition
                ),
                min(
                    matrix[row][col + 1] + 1, // Deletion
                    matrix[last_matching_row][last_match_col]
                        + (row - last_matching_row - 1)  // Cost of letters between transposed letters
                        + (col - last_match_col - 1)
                        + 1, // Cost of the transposition itself
                ),
            );

            if cost == 0 {
                last_match_col = col;
            }
        }

        last_row.insert(ch_a, row);
    }

    // Return the last element
    matrix[len_a + 1][len_b + 1]
}

pub fn fuzzy_damerau_levenshtein(a: &str, b: &str) -> f64 {
    let len_a = a.len();
    let len_b = b.len();

    let inf = (len_a + len_b) as f64;

    // Initialize matrix (M+2) x (N+2)
    let mut matrix: Vec<Vec<f64>> = vec![vec![inf; len_b + 2]];
    matrix.push({
        let mut row: Vec<f64> = vec![inf];
        for j in 0..=len_b {
            row.push(j as f64);
        }
        row
    });
    for i in 1..=len_a {
        let mut row: Vec<f64> = vec![inf, i as f64];
        row.extend(vec![0.0; len_b]);
        matrix.push(row);
    }

    let mut last_row: HashMap<char, usize> = HashMap::new();

    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    for row in 1..=len_a {
        let ch_a = a_chars[row - 1];
        let mut last_match_col = 0;

        for col in 1..=len_b {
            let ch_b = b_chars[col - 1];
            let last_matching_row = *last_row.get(&ch_b).unwrap_or(&0);

            // fuzzy similarity (0 to 1)
            let sim = fuzzy_char_similarity(ch_a, ch_b);
            let cost = 1.0 - sim; // fuzzy cost

            // substitution, insertion, deletion
            let sub = matrix[row][col] + cost;
            let ins = matrix[row + 1][col] + 1.0;
            let del = matrix[row][col + 1] + 1.0;

            // transposition
            let trans = matrix[last_matching_row][last_match_col]
                + ((row - last_matching_row - 1) as f64)
                + ((col - last_match_col - 1) as f64)
                + 1.0;

            matrix[row + 1][col + 1] = sub.min(ins.min(del.min(trans)));

            if sim > 0.8 {
                last_match_col = col;
            }
        }

        last_row.insert(ch_a, row);
    }

    // Normalize distance into fuzzy similarity score (0–1)
    let distance = matrix[len_a + 1][len_b + 1];
    let max_len = len_a.max(len_b) as f64;
    let similarity = 1.0 - (distance / max_len);

    similarity.clamp(0.0, 1.0)
}

fn fuzzy_char_similarity(a: char, b: char) -> f64 {
    if a == b {
        1.0
    } else {
        // Define groups of similar-sounding letters
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        if vowels.contains(&a) && vowels.contains(&b) {
            0.7
        } else if (a.is_ascii_alphabetic() && b.is_ascii_alphabetic())
            && (a.to_ascii_lowercase() as i8 - b.to_ascii_lowercase() as i8).abs() == 1
        {
            0.6 // adjacent letters (keyboard proximity)
        } else {
            0.1 // very dissimilar
        }
    }
}
