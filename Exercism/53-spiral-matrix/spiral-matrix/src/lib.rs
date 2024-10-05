pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut ans = vec![vec![0; size as usize]; size as usize];
    let mut col = 0;
    let mut row = 0;
    let mut dir = 0;
    /*
    dir
     0: Right ::> col++
     1: Down  ::> row++
     2: Left  ::> col--
     3: Up    ::> row--
     */
    (1..=size * size).for_each(|ele| {
        // println!("{}:dir-{} ({}, {})", ele, dir, col, row);
        ans[row][col] = ele;
        // println!("{:?}", ans);
        match dir {
            0 => {
                if col + 1 == size as usize || ans[row][col + 1] != 0 {
                    row += 1;
                    dir = 1;
                } else {
                    col += 1;
                }
            }
            1 => {
                if row + 1 == size as usize || ans[row + 1][col] != 0 {
                    col -= 1;
                    dir = 2;
                } else {
                    row += 1;
                }
            }
            2 => {
                if col == 0 || ans[row][col - 1] != 0 {
                    row -= 1;
                    dir = 3;
                } else {
                    col -= 1;
                }
            }
            3 => {
                if row == 0 || ans[row - 1][col] != 0 {
                    col += 1;
                    dir = 0;
                } else {
                    row -= 1;
                }
            }
            _ => unreachable!(),
        }
    });
    return ans;
}
