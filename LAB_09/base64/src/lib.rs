pub fn base64(value: String) -> String {
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = base64("23");
        assert_eq!(result, "MjM=");
    }
}
