pub fn build_proverb(list: &[&str]) -> String {
    // NOOB WAY

    // match list.len() {
    //     0 => {
    //         return String::new();
    //     }

    //     1 => {
    //         return format!("And all for the want of a {}.", list[0]);
    //     }

    //     _ => {
    //         let mut ans = String::new();
    //         (0..=(list.len() - 2)).for_each(|i| {
    //             ans.push_str(&format!(
    //                 "For want of a {} the {} was lost.\n",
    //                 list[i],
    //                 list[i + 1]
    //             ));

    //             if i == list.len() - 2 {
    //                 ans.push_str(&format!("And all for the want of a {}.", list[0]));
    //             }
    //         });

    //         ans
    //     }
    // }

    // OOORRR
    list.windows(2)
        .map(|items| format!("For want of a {} the {} was lost.\n", items[0], items[1]))
        .collect::<String>()
        + &list
            .get(0)
            .map(|first| format!("And all for the want of a {}.", first))
            .unwrap_or("".to_string())
}
