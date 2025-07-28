pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.len();
    let n = target.len();
    
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for (i, ca) in source.chars().enumerate() {
        for (j, cb) in target.chars().enumerate() {
            let cost;
            if ca == cb {
                 cost = 0 
            } else {
              cost = 1 
            };

            let delete = dp[i][j + 1] + 1;
            let insert = dp[i + 1][j] + 1;
            let substitute = dp[i][j] + cost;

            let mut min_cost = delete;
            if insert < min_cost {
                min_cost = insert;
            }
            if substitute < min_cost {
                min_cost = substitute;
            }

            dp[i + 1][j + 1] = min_cost;
        }
    }

    dp[m][n]
}
