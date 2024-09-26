use std::vec;

pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        if row_count == 0 {
            return Self { rows: Vec::new() };
        }

        let mut rows = Vec::with_capacity(row_count as usize);
        (1..=row_count).for_each(|index| {
            if index == 1 {
                rows.push(vec![1]);
            } else {
                let mut temp = Vec::with_capacity((index + 1) as usize);
                (0..index).for_each(|inner| {
                    if inner == 0 || inner == index - 1 {
                        temp.push(1)
                    } else {
                        temp.push(
                            rows[index as usize - 2][inner as usize - 1]
                                + rows[index as usize - 2][inner as usize],
                        )
                    }
                });
                rows.push(temp.clone());
            }
        });

        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
