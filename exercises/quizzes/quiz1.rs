// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.
fn calculate_price_of_apples(quantity: i32) -> i32 {
    if quantity < 41 {
        quantity * 2
    } else {
        quantity
    }
}

fn main() {
    // You can optionally experiment here.
    let price1 = calculate_price_of_apples(20);
    let price2 = calculate_price_of_apples(49);
    let price3 = calculate_price_of_apples(10);

    println!("The price of 20 apples is {price1}");
    println!("The price of 20 apples is {price2}");
    println!("The price of 20 apples is {price3}");
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
