pub fn reply(message: &str) -> &str {
    match message.trim().len() {
        0 => return "Fine. Be that way!",
        _ => {
            let c_alpha = message.chars().filter(|&c| c.is_alphabetic()).count();
            let c_upper = message.chars().filter(|&c| c.is_uppercase()).count();

            if message.trim().chars().last().unwrap() == '?' {
                if c_upper == c_alpha && c_alpha != 0 {
                    return "Calm down, I know what I'm doing!";
                } else {
                    return "Sure.";
                }
            } else {
                if message.chars().all(|c| !c.is_alphabetic()) {
                    return "Whatever.";
                } else {
                    if c_upper == c_alpha {
                        return "Whoa, chill out!";
                    } else {
                        return "Whatever.";
                    }
                }
            }
        }
    }
}
