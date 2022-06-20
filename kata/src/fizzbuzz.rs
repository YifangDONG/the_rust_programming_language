fn fizzbuzz(values: &[u32]) {
    let fizz = |x: u32| {
        if x % 3 == 0 {
            "fizz".to_string()
        } else {
            "".to_string()
        }
    };
    let buzz = |x: u32| {
        if x % 5 == 0 {
            "buzz".to_string()
        } else {
            "".to_string()
        }
    };
    let default = |x: u32| {
        if x % 3 != 0 && x % 5 != 0 {
            x.to_string()
        } else {
            String::new()
        }
    };

    for &i in values {
        let mut output = String::new();
        output.push_str(&fizz(i));
        output.push_str(&buzz(i));
        output.push_str(&default(i));
        println!("{}", output);
    }
}

#[test]
fn test() {
    let input = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    fizzbuzz(&input);
}
