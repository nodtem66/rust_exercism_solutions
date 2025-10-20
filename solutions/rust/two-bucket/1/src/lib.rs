#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

pub fn reach_goal(b1: u8, b2: u8, goal: u8) -> bool {
    b1 == goal || b2 == goal
}

pub fn last_action_was_fill(b1: u8, b2: u8, cap2: u8) -> bool {
    b1 == 0 && b2 == cap2
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut moves = 1;
    let mut b1 = match start_bucket {
        Bucket::One => capacity_1,
        Bucket::Two => capacity_2,
    };
    let mut b2 = 0;
    let (cap1, cap2) = match start_bucket {
        Bucket::One => (capacity_1, capacity_2),
        Bucket::Two => (capacity_2, capacity_1),
    };
    // Special cases that can be resolved in one or two moves
    if cap1 == goal {
        return Some(BucketStats {
            moves: 1,
            goal_bucket: start_bucket.clone(),
            other_bucket: 0,
        });
    }
    if cap2 == goal {
        return Some(BucketStats {
            moves: 2,
            goal_bucket: match start_bucket {
                Bucket::One => Bucket::Two,
                Bucket::Two => Bucket::One,
            },
            other_bucket: cap1,
        });
    }
    // General case
    while !reach_goal(b1, b2, goal) && !last_action_was_fill(b1, b2, cap2) {
        if b1 == 0 {
            b1 = cap1; // Fill bucket one
        } else if b2 == cap2 {
            b2 = 0; // Empty bucket two
        } else { // Pour from bucket one to bucket two
            let transfer = std::cmp::min(b1, cap2 - b2);
            b1 -= transfer;
            b2 += transfer;
        }
        moves += 1;
    }
    // Return result if goal was reached
    if reach_goal(b1, b2, goal) {
        let (goal_bucket, other_bucket) = match (b1 == goal, start_bucket) {
            (true, Bucket::One) => (Bucket::One, b2),
            (true, Bucket::Two) => (Bucket::Two, b2),
            (false, Bucket::One) => (Bucket::Two, b1),
            (false, Bucket::Two) => (Bucket::One, b1),
        };
        Some(BucketStats {
            moves,
            goal_bucket,
            other_bucket,
        })
    } else {
        None
    }
}
