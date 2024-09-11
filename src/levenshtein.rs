use std::cmp;
#[allow(dead_code)]
fn levenshtein(str1: &str, str2: &str) -> usize {
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
