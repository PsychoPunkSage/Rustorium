// <<<<<<<< NOOB WAY >>>>>>>>
// pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
//     let mut ans = vec![vec![0; size as usize]; size as usize];
//     let mut col = 0;
//     let mut row = 0;
//     let mut dir = 0;
//     /*
//     dir
//      0: Right ::> col++
//      1: Down  ::> row++
//      2: Left  ::> col--
//      3: Up    ::> row--
//      */
//     (1..=size * size).for_each(|ele| {
//         ans[row][col] = ele;
//         match dir {
//             0 => {
//                 if col + 1 == size as usize || ans[row][col + 1] != 0 {
//                     row += 1;
//                     dir = 1;
//                 } else {
//                     col += 1;
//                 }
//             }
//             1 => {
//                 if row + 1 == size as usize || ans[row + 1][col] != 0 {
//                     col -= 1;
//                     dir = 2;
//                 } else {
//                     row += 1;
//                 }
//             }
//             2 => {
//                 if col == 0 || ans[row][col - 1] != 0 {
//                     row -= 1;
//                     dir = 3;
//                 } else {
//                     col -= 1;
//                 }
//             }
//             3 => {
//                 if row == 0 || ans[row - 1][col] != 0 {
//                     col += 1;
//                     dir = 0;
//                 } else {
//                     row -= 1;
//                 }
//             }
//             _ => unreachable!(),
//         }
//     });
//     return ans;
// }

// <<<<<<<PRO Solution>>>>>>>>
fn spiral(width: u32, height: u32, x: u32, y: u32) -> u32 {
    if y == 0 {
        x + 1
    } else {
        width + spiral(height - 1, width, y - 1, width - x - 1)
    }
}
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    (0..size)
        .map(|y| (0..size).map(|x| spiral(size, size, x, y)).collect())
        .collect()
}
