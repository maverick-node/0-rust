pub fn num_to_ordinal(x: u32) -> String {
    let mut res = String::new();
      if x % 100 == 11 || x % 100 == 12 || x % 100 == 13 {
        return x.to_string() + "th";
    }
    let suffix = if x % 10 == 1 {
        "st"
    } else if x % 10 == 2 {
        "nd"
    } else if x % 10 == 3 {
        "rd"
    } else {
        "th"
    };

    x.to_string() + suffix
}
