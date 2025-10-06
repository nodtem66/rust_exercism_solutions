pub fn build_proverb(list: &[&str]) -> String {
    let mut result: Vec<String> = Vec::new();
    for i in 0..list.len() {
        if i == list.len() - 1 {
            result.push(format!("And all for the want of a {}.", list[0]));
        } else {
            result.push(format!(
                "For want of a {} the {} was lost.",
                list[i],
                list[i + 1]
            ));
        }
    }
    result.join("\n")
}
