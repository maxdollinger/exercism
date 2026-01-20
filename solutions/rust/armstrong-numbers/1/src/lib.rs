pub fn is_armstrong_number(num: u32) -> bool {
    let digit_cnt = if num == 0 {
        1
    } else {
        (num as f64).log10().floor() as u32 + 1
    };

    let mut n = num;
    let mut sum: u32 = 0;
    while n > 0 {
        let digit = n % 10;
        n /= 10;

        sum += digit.pow(digit_cnt);
    }

    sum == num
}
