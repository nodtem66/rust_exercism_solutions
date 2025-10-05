use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Problem {
    left_terms: Vec<Vec<char>>, // left_terms[i] contains all letters in the i-th column (0-indexed from the right)
    right_term: Vec<char>, // right_terms[i] is the letter in the i-th column of the result (0-indexed from the right)
    first_letters: HashSet<char>, // List of leading letters (these cannot be assigned to zero)
}

impl Problem {
    #[cfg(feature = "debug")]
    fn print(&self) {
        println!("Left terms:");
        for (i, col) in self.left_terms.iter().enumerate() {
            println!("  Column {}: {:?}", i, col);
        }
        println!("Right term: {:?}", self.right_term);
        println!("First letters: {:?}", self.first_letters);
    }

    fn solve_column(
        &self,
        column: usize, // Current column index (0 is the least significant column)
        available_digits: HashSet<u8>, // Digits that are not yet assigned to any letter
        solution: HashMap<char, u8>, // Current partial solution mapping letters to digits
        carry: u8,     // Carry from the previous column
    ) -> Option<HashMap<char, u8>> {
        if column >= self.right_term.len() {
            if carry == 0 {
                return Some(solution); // All columns processed and no carry left
            } else {
                return None; // All columns processed but carry left, invalid
            }
        }
        let unknown_letters_in_column: HashSet<char> = self.left_terms[column]
            .iter()
            .copied()
            .filter(|c| !solution.contains_key(c))
            .collect();
        let number_of_unknowns = unknown_letters_in_column.len();
        for digits in available_digits.iter().permutations(number_of_unknowns) {
            let mut tmp_solution = solution.clone();
            for (&i, d) in digits.iter().zip(unknown_letters_in_column.iter()) {
                tmp_solution.insert(*d, *i);
            }
            let left_term_sum: u64 = self.left_terms[column]
                .iter()
                .filter_map(|&c| tmp_solution.get(&c))
                .map(|&x| x as u64)
                .sum();
            let right_digit = ((left_term_sum + carry as u64) % 10) as u8;
            let new_carry = ((left_term_sum + carry as u64) / 10) as u8;
            let mut used_digits: HashSet<u8> = digits.iter().map(|&d| *d).collect();
            
            #[cfg(feature = "debug")]
            println!("  Trying digits {:?}, left sum: {}, right digit: {}, new carry: {}", digits, left_term_sum, right_digit, new_carry);
            
            match tmp_solution.get(&self.right_term[column]) {
                Some(&v) => {
                    // If letter in right Already assigned, check if it matches
                    if right_digit != v {
                        continue; // Not match, try next permutation
                    }
                }
                None => {
                    if used_digits.contains(&right_digit)
                        || !available_digits.contains(&right_digit)
                    {
                        continue; // Digit already used, try next permutation
                    }
                    // If letter in right is not assigned, assign it
                    tmp_solution.insert(self.right_term[column], right_digit);
                    used_digits.insert(right_digit);
                }
            }
            // Check leading zero constraint
            if self
                .first_letters
                .iter()
                .any(|&c| tmp_solution.get(&c) == Some(&0))
            {
                continue; // Leading letter cannot be zero, try next permutation
            }
            // Recurse to the next column
            if let Some(solution) = self.solve_column(
                column + 1,
                &available_digits - &used_digits,
                tmp_solution,
                new_carry,
            ) {
                return Some(solution);
            }
        }
        None
    }
}

// Alphametics is NP-complete. There are only 10! possible assignments of digits to letters,
// so a brute-force approach is feasible for typical puzzle sizes
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // Parse expression <expr> + <expr> + ... == <expr>
    let expressions: Vec<&str> = input
        .split(&['+', '='])
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    let max_num_digits = expressions.iter().map(|expr| expr.len()).max().unwrap();

    // Find all leading letters (these leading letter will be not allowed to be zero)
    let first_letters: HashSet<char> = expressions
        .iter()
        .filter_map(|expr| expr.chars().next())
        .filter(|c| c.is_ascii_alphabetic())
        .collect();
    // Get column-wise letters from the expressions
    // sum_terms[i] contains all letters in the i-th column (0-indexed from the right)
    let number_of_left_terms = expressions.len() - 1; // Last expression is the result
    let mut left_terms: Vec<Vec<char>> = vec![Vec::new(); max_num_digits];
    for expr in expressions.iter().take(number_of_left_terms) {
        for (i, c) in expr.chars().rev().enumerate() {
            left_terms[i].push(c);
        }
    }
    let right_term: Vec<char> = expressions.last().unwrap().chars().rev().collect();
    if right_term.len() < max_num_digits {
        return None; // Result must have at least as many digits as the longest term
    }
    let all_available_digits: HashSet<u8> = (0..=9).collect();
    let empty_solution: HashMap<char, u8> = HashMap::new();

    let problem = Problem {
        left_terms,
        right_term,
        first_letters,
    };
    
    #[cfg(feature = "debug")]
    problem.print();

    problem.solve_column(0, all_available_digits, empty_solution, 0)
}
