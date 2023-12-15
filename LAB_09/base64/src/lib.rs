pub fn encode(input: &mut [u8]) -> String {
    let mut input = input.iter().copied().collect::<Vec<u8>>();
    let mut encoded_str = String::new();
    let mut pad_string = String::new();
    let mut pad_count = input.len() % 3;

    let chars: Vec<char> =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
            .chars()
            .collect();

    if pad_count > 0 {
        while pad_count < 3 {
            pad_string.push('=');
            input.push(0);

            pad_count += 1;
        }
    }

    for chunk in input.chunks_exact(3) {
        let res: u32 = ((chunk[0] as u32) << 16) + ((chunk[1] as u32) << 8) + (chunk[2] as u32);
        let res: [u32; 4] = [
            (res >> 18) & 63,
            (res >> 12) & 63,
            (res >> 6) & 63,
            res & 63,
        ];

        for i in &res {
            encoded_str.push_str(&chars[*i as usize].to_string());
        }
    }

    let mut result_str = encoded_str[..encoded_str.len() - pad_string.len()].to_string();

    result_str.push_str(&pad_string);

    result_str
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

//=================================================
//source: https://dev.to/tiemen/implementing-base64-from-scratch-in-rust-kb1
//source: https://www.youtube.com/watch?v=aUdKd0IFl34
//=================================================