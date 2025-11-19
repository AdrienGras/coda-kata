fn calculate_pricing(nb_items: u32, unit_price: f64, tax: f64) -> String {
    // calculate total price before tax
    let ht_price = nb_items as f64 * unit_price;

    // apply discount based on total price before tax
    let ht_price_after_discount = match ht_price {
        x if x > 5000.0 => x * 0.95, // 5% discount
        x if x > 1000.0 => x * 0.97, // 3% discount
        x => x,                      // no discount
    };

    // calculate total price including tax (after discount + tax)
    let total_price_ttc = ht_price_after_discount * (1.0 + tax / 100.0);

    // format unit price with comma as decimal separator
    let unit_price_display = format!("{:.2}", unit_price).replace('.', ",");

    format!(
        "{} articles à {} € et taxe {} % → \"{:.2} €\"",
        nb_items, unit_price_display, tax, total_price_ttc
    )
}

fn main() {
    println!("{}", calculate_pricing(3, 1.21, 0.0));
    println!("{}", calculate_pricing(3, 1.21, 5.0));
    println!("{}", calculate_pricing(3, 1.21, 20.0));
    println!("{}", calculate_pricing(5, 345.0, 10.0));
    println!("{}", calculate_pricing(5, 1299.0, 10.0));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_3_articles_1_21_sans_taxe() {
        assert_eq!(
            calculate_pricing(3, 1.21, 0.0),
            "3 articles à 1,21 € et taxe 0 % → \"3.63 €\""
        );
    }

    #[test]
    fn test_3_articles_1_21_taxe_5() {
        assert_eq!(
            calculate_pricing(3, 1.21, 5.0),
            "3 articles à 1,21 € et taxe 5 % → \"3.81 €\""
        );
    }

    #[test]
    fn test_3_articles_1_21_taxe_20() {
        assert_eq!(
            calculate_pricing(3, 1.21, 20.0),
            "3 articles à 1,21 € et taxe 20 % → \"4.36 €\""
        );
    }

    #[test]
    fn test_5_articles_345_taxe_10_remise_3() {
        assert_eq!(
            calculate_pricing(5, 345.0, 10.0),
            "5 articles à 345,00 € et taxe 10 % → \"1840.58 €\""
        );
    }

    #[test]
    fn test_5_articles_1299_taxe_10_remise_5() {
        assert_eq!(
            calculate_pricing(5, 1299.0, 10.0),
            "5 articles à 1299,00 € et taxe 10 % → \"6787.28 €\""
        );
    }
}
