pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.code.chars().rev().filter(|c| !c.is_whitespace())
            .try_fold((0, 0), |(sum, count), val| {
                val.to_digit(10)
                   .map(|num| if count % 2 == 1 {num*2} else {num})
                   .map(|num| if num > 9 {num - 9} else {num})
                   .map(|num| (sum + num, count + 1))
            }).is_some_and(|(sum, count)| sum % 10 == 0 && count > 1)
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn { code: input.to_string() }
    }
}

