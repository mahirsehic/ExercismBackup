pub fn reverse(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    for x in input.chars() {
        output.insert(0, x);
    }
    output
}
