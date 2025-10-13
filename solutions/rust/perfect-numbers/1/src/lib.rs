#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let factors_sum: u64 = (1..num).filter(|&i| num.is_multiple_of(i)).sum();
    if num == 0 {
        return None;
    }
    if factors_sum == 0 && num >= 1 {
        return Some(Classification::Deficient);
    }
    match factors_sum.cmp(&num) {
        std::cmp::Ordering::Less => Some(Classification::Deficient),
        std::cmp::Ordering::Equal => Some(Classification::Perfect),
        std::cmp::Ordering::Greater => Some(Classification::Abundant),
    }
}
