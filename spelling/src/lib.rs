pub fn spell(n: u64) -> String {
    let ones = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];

    let ten = [
        "",
        "",
        "twenty",
        "thirty",
        "forty",
        "fifty",
        "sixty",
        "seventy",
        "eighty",
        "ninety",
    ];

    if n <= 19 {
        return format!("{}", ones[n as usize]);
    }

    if n < 100 {
        let t = n / 10;
        let r = n % 10;
        if r == 0 {
            return format!("{}", ten[t as usize]);
        } else {
            return format!("{}-{}", ten[t as usize], ones[r as usize]);
        }
    }

    if n < 1000 {
        let t = spell(n / 100);
        let r = n % 100;
        if r == 0 {
            return format!("{} hundred", t);
        } else {
            return format!("{} hundred {}", t, spell(r));
        }
    }
    if n < 1_000_000 {
        let t = spell(n / 1000);
        let r = n % 1000;
        if r == 0 {
            return format!("{} thousand", t);
        } else {
            return format!("{} thousand {}",t, spell(r));
        }
    }
    if n == 1_000_000 {
        return "one million".to_string();
    }
    return format!("ff");
}
