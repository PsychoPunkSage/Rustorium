pub struct RailFence(u32);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence(rails)
    }

    pub fn encode(&self, text: &str) -> String {
        let mut result = String::new();
        let mut rails = vec![Vec::new(); self.0 as usize];
        let mut direction: isize = -1;
        let mut row: isize = 0;

        for c in text.chars() {
            rails[row as usize].push(c);
            if row == 0 || row == self.0 as isize - 1 {
                direction *= -1;
            }
            row += direction;
        }

        rails.into_iter().for_each(|set| {
            set.iter().for_each(|&c| result.push(c));
        });

        result
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut text = cipher.chars().collect::<Vec<char>>();
        let mut result = String::new();
        let mut rails = vec![Vec::new(); self.0 as usize];

        let mut direction: isize = -1;
        let mut row: isize = 0;

        println!("len: {}", text.len());
        let n = (text.len() + 2 * self.0 as usize - 3) / (2 * (self.0 as usize - 1));

        for i in 0..rails.len() {
            if i == 0 {
                for _ in 0..n {
                    rails[i].push(text.remove(0));
                }
            } else if i == rails.len() - 1 {
                for _ in 0..n - 1 {
                    rails[i].push(text.remove(0));
                }
            } else {
                for _ in 0..(2 * (n - 1)) {
                    rails[i].push(text.remove(0));
                }
            }
        }

        println!("{:?}", rails);

        for _ in 0..cipher.len() {
            let c = rails[row as usize].remove(0);
            result.push(c);
            if row == 0 || row == self.0 as isize - 1 {
                direction *= -1;
            }
            row += direction;
        }

        return result;
    }
}
