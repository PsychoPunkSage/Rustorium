use std::collections::HashMap;

pub fn lowest_price(books: &[u32]) -> u32 {
    let price_per_book = 800;
    let discounts = [0, 5, 10, 20, 25];
    let mut memo = HashMap::new();

    fn min_price_for_basket(
        books: Vec<u32>,
        price_per_book: u32,
        discounts: &[u32],
        memo: &mut HashMap<Vec<u32>, u32>,
    ) -> u32 {
        // If all books are 0, no cost for an empty set
        if books.iter().all(|&b| b == 0) {
            return 0;
        }

        // Use memoized result if available
        if let Some(&result) = memo.get(&books) {
            return result;
        }

        let mut min_cost = u32::MAX;

        // Try to form groups of different sizes (1 to 5 different books)
        for grp_size in 1..=5 {
            // Attempt to take `grp_size` unique books from the set
            let mut new_counts = books.clone();
            let mut selected = 0;

            for count in new_counts.iter_mut() {
                if *count > 0 {
                    *count -= 1;
                    selected += 1;
                }
                if selected == grp_size {
                    break;
                }
            }

            if selected == grp_size {
                // Calculate the group cost based on the discount
                let discount = discounts[grp_size] as f32 / 100.0;
                let grp_cost = (price_per_book * grp_size as u32) as f32 * (1.0 - discount);
                let total_cost = grp_cost as u32
                    + min_price_for_basket(new_counts, price_per_book, discounts, memo);

                min_cost = min_cost.min(total_cost);
            }
        }

        memo.insert(books, min_cost);
        min_cost
    }

    min_price_for_basket(books.to_vec(), price_per_book, &discounts, &mut memo)
}
