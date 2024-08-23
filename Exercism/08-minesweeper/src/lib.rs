pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let len_minefield = minefield.len();
    if len_minefield == 0 {
        return vec![];
    }

    let len_eachfield = minefield[0].len();
    let mut mine = minefield
        .iter()
        .map(|&row| row.chars().into_iter().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for i in 0..mine.len() {
        for j in 0..mine[i].len() {
            let mut count = 0;
            if mine[i][j] == '*' {
                continue;
            }
            for k in (i as i32 - 1)..=(i as i32 + 1) {
                for l in (j as i32 - 1)..=(j as i32 + 1) {
                    if k < 0 || k >= len_minefield as i32 || l < 0 || l >= len_eachfield as i32 {
                        continue;
                    }
                    if mine[k as usize][l as usize] == '*' {
                        count += 1;
                    }
                }
            }
            println!("{}", count);
            if count == 0 {
                mine[i][j] = ' ';
            } else {
                mine[i][j] = std::char::from_digit(count, 10).unwrap();
            }
        }
    }

    mine.iter()
        .map(|carr| carr.into_iter().collect::<String>())
        .collect::<Vec<String>>()
}
