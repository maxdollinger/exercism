use std::iter::successors;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

// normalize the intput to base10 and from there on calc the to_base
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    match number.iter().find(|&&n| n >= from_base) {
        None => {}
        Some(&n) => return Err(Error::InvalidDigit(n)),
    };

    let base10 = number.iter().fold(0u32, |dec, &d| dec * from_base + d);

    if number.is_empty() || base10 == 0 {
        return Ok(vec![0]);
    }

    let mut res = successors(Some(base10), |&d| Some(d / to_base))
        .take_while(|&d| d > 0)
        .map(|d| d % to_base)
        .collect::<Vec<u32>>();

    res.reverse();
    Ok(res)
}
