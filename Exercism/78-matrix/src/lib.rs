pub struct Matrix {
    data: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let rows: Vec<&str> = input.split('\n').collect();
        Matrix {
            data: rows
                .iter()
                .map(|row| {
                    row.split(' ')
                        .map(|num_str| num_str.parse().unwrap())
                        .collect()
                })
                .collect(),
        }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        if row_no > self.data.len() {
            None
        } else {
            Some(self.data[row_no - 1].clone())
        }
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if col_no > 0 && col_no <= self.data[0].len() {
            Some(self.data.iter().map(|row| row[col_no - 1]).collect())
        } else {
            None
        }
    }
}
