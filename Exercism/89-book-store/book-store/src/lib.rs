pub fn lowest_price(books: &mut [u32]) -> u32 {
    let mut count = vec![0, 0, 0, 0, 0, 0];
    books.iter().for_each(|ele| count[*ele as usize] += 1);

    let mut dist = count.iter().filter(|ele| *ele != &0).collect::<Vec<&i32>>();
    dist.sort_unstable();

    if dist.len() == 0 {
        return 0;
    }

    // 1 type of book
    if dist.len() == 1 {
        return 800 * count.iter().sum::<i32>() as u32;
    }

    // 2 types of book
    if dist.len() == 2 {
        let (s, l) = (dist[0], dist[1]);
        return (800 * (l - s) + 95 * 8 * (s) * 2) as u32;
    }

    // 3 types of book
    if dist.len() == 3 {
        let (s, m, l) = (dist[0], dist[1], dist[2]);
        // let mut x = books;
        return min_price_for_basket(books);
    }

    // 3 types of book
    if dist.len() == 4 {
        let single_price = 800;
        let (s, m_s, m_l, l) = (dist[0], dist[1], dist[2], dist[3]);
        return std::cmp::min(
            single_price * 80 / 100 * 4 * s
                + single_price * 90 / 100 * 3 * (m_s - s)
                + single_price * 95 / 100 * 2 * (m_l - m_s)
                + single_price * (l - m_l),
            single_price * 95 / 100 * 2 * m_l
                + if l - m_l > *s {
                    single_price * (l - m_l - s) + single_price * 95 / 100 * 2 * s
                } else {
                    single_price * (s - (l - m_l)) + single_price * 95 / 100 * 2 * (l - m_l)
                },
        ) as u32;
    }

    println!("Dist {:?}", dist);
    return 800;
}

fn calculate_discounted_price(num_books: u32) -> u32 {
    let discount = match num_books {
        2 => 0.95, // 5% discount for 2 books
        3 => 0.90, // 10% discount for 3 books
        4 => 0.80, // 20% discount for 4 books
        5 => 0.75, // 25% discount for 5 books
        _ => 1.0,  // no discount for 1 book
    };
    (800.0 * num_books as f64 * discount) as u32
}

fn min_price_for_basket(book_counts: &mut [u32]) -> u32 {
    // Early return if there are no books to purchase
    if book_counts.iter().all(|&count| count == 0) {
        return 0;
    }

    // Sort the book counts in descending order
    book_counts.sort_by(|a, b| b.cmp(a));
    let mut min_price = u32::MAX;

    // Try different group sizes (from 1 to 5 distinct books in each group)
    for group_size in (1..=5).rev() {
        if book_counts.iter().filter(|&&count| count > 0).count() < group_size {
            continue; // Skip if we don't have enough distinct books for this group size
        }

        // Form a group of size `group_size` by decrementing the book counts
        for i in 0..group_size {
            book_counts[i] -= 1;
        }

        // Calculate price recursively for this group formation
        let price =
            calculate_discounted_price(group_size as u32) + min_price_for_basket(book_counts);
        min_price = min_price.min(price);

        // Restore book counts after the recursive call
        for i in 0..group_size {
            book_counts[i] += 1;
        }
    }

    min_price
}
