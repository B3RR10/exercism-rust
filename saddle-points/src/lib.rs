pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points: Vec<(usize, usize)> = vec![];

    let row_max: Vec<u64> = input
        .iter()
        .map(|row| row.iter().map(|n| n.to_owned()).max().unwrap_or_default())
        .collect();

    let mut col_min: Vec<u64> = vec![];
    for i in 0..input[0].len() {
        col_min.push(input.iter().map(|row| row[i]).min().unwrap_or_default());
    }

    for i in 0..row_max.len() {
        for j in 0..col_min.len() {
            let point = input[i][j];
            if point == row_max[i] && point == col_min[j] {
                saddle_points.push((i, j));
            }
        }
    }

    saddle_points
}
