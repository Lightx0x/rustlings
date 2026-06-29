// This function returns how much ice cream there is left in the fridge.
// If it's before 22:00 (24-hour system), then 5 scoops are left. At 22:00,
// someone eats it all, so no ice cream is left (value 0). Return `None` if
// `hour_of_day` is higher than 23.
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    // TODO: Complete the function body.
    match hour_of_day {
        0..=21 => Some(5),
        22..=23 => Some(0),
        _ => None
    }
}

fn main() {
    // You can optionally experiment here.
    let time_now_22: u16 = 22;
    let time_now_23: u16 = 23;
    let time_now_24: u16 = 24;
    let time_now_14: u16 = 14;
    let time_now_04: u16 = 4;
    println!("How much icecream is left at {time_now_22}:00 => {:?} icecream is left"
        , maybe_ice_cream(time_now_22).unwrap());
    println!("How much icecream is left at {time_now_23}:00 => {:?} icecream is left"
        , maybe_ice_cream(time_now_23).unwrap());
    println!("How much icecream is left at {time_now_24}:00 => {:?} icecream is left"
        , maybe_ice_cream(time_now_24).unwrap_or_default());
    println!("How much icecream is left at {time_now_14}:00 => {:?} icecream is left"
        , maybe_ice_cream(time_now_14).unwrap());
    println!("How much icecream is left at {time_now_04}:00 => {:?} icecream is left"
        , maybe_ice_cream(time_now_04).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get the value contained in the
        // Option?
        let ice_creams = maybe_ice_cream(12).unwrap();

        assert_eq!(ice_creams, 5); // Don't change this line.
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), Some(5));
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(18), Some(5));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(24), None);
        assert_eq!(maybe_ice_cream(25), None);
    }
}
