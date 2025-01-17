fn is_valid_palindrome(input: &str) -> bool {
    let chars: Vec<char> = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let mut left = 0;
    let mut right = chars.len().saturating_sub(1);

    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}

fn main() {
    let test_cases = vec![
        "A man, a plan, a canal: Panama",
        "race a car",
        " ",
        "abba",
        "@bb@",
    ];

    for test in test_cases {
        println!(
            "\"{}\" is a palindrome: {}",
            test,
            is_valid_palindrome(test)
        );
    }
}
