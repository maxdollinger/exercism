pub fn squareOfSum(n: usize) usize {
    const sum: usize = (n * (n + 1)) / 2;
    return sum * sum;
}

pub fn sumOfSquares(n: usize) usize {
    return (n * (n + 1) * (2 * n + 1)) / 6;
}

pub fn differenceOfSquares(number: usize) usize {
    return squareOfSum(number) - sumOfSquares(number);
}
