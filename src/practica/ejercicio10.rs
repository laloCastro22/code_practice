    #[allow(dead_code)]
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        if m == 0 { return 0; }
        let n = mat[0].len();
        if n == 0 { return 0; }

        let mut row = vec![0_i32; m];
        let mut col = vec![0_i32; n];

        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 {
                    row[i] += 1;
                    col[j] += 1;
                }
            }
        }

        let mut count = 0;
        for i in 0..m {
            if row[i] != 1 { continue; }
            for j in 0..n {
                if mat[i][j] == 1 && col[j] == 1 {
                    count += 1;
                }
            }
        }
        count
    }