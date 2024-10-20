pub fn actions(n: u8) -> Vec<&'static str> {
    // let vecc = format!("{:b}", n)
    //     .chars()
    //     .rev()
    //     .enumerate()
    //     .into_iter()
    //     .map(|(i, c)| {
    //         if c == '1' {
    //             match i {
    //                 0 => "wink",
    //                 1 => "double blink",
    //                 2 => "close your eyes",
    //                 3 => "jump",
    //                 _ => "reverse",
    //             }
    //         } else {
    //             ""
    //         }
    //     })
    //     .filter(|&c| !c.is_empty())
    //     .collect::<Vec<&str>>();

    // if vecc.clone().into_iter().any(|c| c == "reverse") {
    //     return vecc
    //         .into_iter()
    //         .rev()
    //         .filter(|&c| c != "reverse")
    //         .collect::<Vec<&str>>();
    // }

    // vecc

    let mut actions = Vec::new();

    if n & 1 != 0 {
        actions.push("wink");
    }
    if n & 2 != 0 {
        actions.push("double blink");
    }
    if n & 4 != 0 {
        actions.push("close your eyes");
    }
    if n & 8 != 0 {
        actions.push("jump");
    }

    if n & 16 != 0 {
        actions.reverse();
    }

    actions
}
