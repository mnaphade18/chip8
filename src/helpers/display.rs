pub fn as_binary(data: u8) -> String {
    let mut binary_string = String::new();
    let mut number = data;
    while number > 0 {
        if number % 2 == 0 {
            binary_string.push('0');
        } else {
            binary_string.push('1');
        }

        number /= 2;
    }
    while binary_string.len() != 8 {
        binary_string.push('0');
    }

    binary_string.chars().rev().collect()
}
