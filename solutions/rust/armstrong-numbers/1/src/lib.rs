pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }
    let mut numbers = Vec::new();
    let mut original = num;
    while original > 0 {
        numbers.push(original % 10);
        original /= 10;
    }
    let power = numbers.len() as u32;
    let mut sum = 0;

    for x in numbers {
        sum += x.pow(power);
    }

    num == sum
}
