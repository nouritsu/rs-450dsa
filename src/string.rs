// Problem 0 from https://450dsa.com/string
pub fn reverse_string(s: &mut String) {
    if s.len() > 0 {
        *s = s.chars().rev().collect()
    }
}

// Problem 1 from https://450dsa.com/string
pub fn is_palindrome(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}

// Problem 15 from https://450dsa.com/string
pub fn is_balanced(s: &str) -> bool {
    let mut paren = 0usize;
    let mut square = 0usize;
    let mut brace = 0usize;

    s.chars().for_each(|c| match c {
        '(' => paren += 1,
        ')' => paren -= 1,
        '[' => square += 1,
        ']' => square -= 1,
        '{' => brace += 1,
        '}' => brace -= 1,
        _ => {}
    });

    (paren | square | brace) == 0
}

#[cfg(test)]
mod tests {
    #[test]
    fn reverse_string() {
        let mut s = "".to_string();
        super::reverse_string(&mut s);
        assert_eq!(s, "");

        let mut s = "Hello".to_string();
        super::reverse_string(&mut s);
        assert_eq!(s, "olleH");
    }

    #[test]
    fn is_palindrome() {
        let s = "racecar";
        assert!(super::is_palindrome(s));

        let s = "hey";
        assert!(!super::is_palindrome(s));

        let s = "";
        assert!(super::is_palindrome(s));
    }

    #[test]
    fn is_balanced() {
        let s = "{[()]}";
        assert!(super::is_balanced(s));

        let s = "[()";
        assert!(!super::is_balanced(s));

        let s = "";
        assert!(super::is_balanced(s))
    }
}
