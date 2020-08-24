pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows: Vec<Vec<u32>> = vec![];
        for i in 0..self.0 {
            let mut row: Vec<u32> = vec![];
            for j in 0..=i {
                if j == 0 || j == i {
                    row.push(1);
                } else {
                    row.push(
                        rows[i as usize - 1][j as usize - 1] + rows[i as usize - 1][j as usize],
                    );
                }
            }
            rows.push(row);
        }
        rows
    }
}
