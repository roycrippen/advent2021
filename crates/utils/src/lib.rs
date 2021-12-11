pub fn echo(s: &str) -> &str {
    s
}


#[cfg(test)]
mod tests {
    use crate::echo;

    #[test]
    fn test_echo() {
        assert_eq!("aaa", echo("aaa"));
    }
}
