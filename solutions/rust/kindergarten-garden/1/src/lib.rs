pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mut result = Vec::new();
    // Since we have only one student to search for, we can use a simple match statement.
    let i = match student {
        "Alice" => 0,
        "Bob" => 1,
        "Charlie" => 2,
        "David" => 3,
        "Eve" => 4,
        "Fred" => 5,
        "Ginny" => 6,
        "Harriet" => 7,
        "Ileana" => 8,
        "Joseph" => 9,
        "Kincaid" => 10,
        "Larry" => 11,
        _ => panic!("Unknown student"),
    };
    for line in diagram.lines() {
        let plants: Vec<char> = line.trim().chars().collect();
        for plant in plants.chunks(2).nth(i).unwrap() {
            match plant {
                'G' => result.push("grass"),
                'C' => result.push("clover"),
                'R' => result.push("radishes"),
                'V' => result.push("violets"),
                _ => panic!("Unknown plant"),
            }
        }
    }
    result
}
