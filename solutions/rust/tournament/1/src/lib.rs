use std::collections::HashMap;
use std::io::Write;
struct TeamResult {
    wins: u32,
    draws: u32,
    losses: u32,
}

impl TeamResult {
    fn new() -> Self {
        TeamResult {
            wins: 0,
            draws: 0,
            losses: 0,
        }
    }

    fn points(&self) -> u32 {
        self.wins * 3 + self.draws
    }

    fn total_matches(&self) -> u32 {
        self.wins + self.draws + self.losses
    }

    fn print_score(&self, team_name: &str) -> String {
        format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            team_name,
            self.total_matches(),
            self.wins,
            self.draws,
            self.losses,
            self.points()
        )
    }
}

pub fn tally(match_results: &str) -> String {
    let mut match_result: HashMap<String, TeamResult> = HashMap::new();
    let mut output = Vec::new();
    // Parse each line of match results
    for line in match_results.lines() {
        let parts: Vec<&str> = line.split(';').collect();
        assert!(parts.len() == 3);
        let team1 = parts[0];
        let team2 = parts[1];
        let result = parts[2];
        match result {
            "win" => {
                // team1 wins, team2 loses
                match_result.entry(team1.to_string()).or_insert(TeamResult::new()).wins += 1;
                match_result.entry(team2.to_string()).or_insert(TeamResult::new()).losses += 1;
            },
            "loss" => {
                // team1 loses, team2 wins
                match_result.entry(team1.to_string()).or_insert(TeamResult::new()).losses += 1;
                match_result.entry(team2.to_string()).or_insert(TeamResult::new()).wins += 1;
            },
            "draw" => {
                // both teams draw
                match_result.entry(team1.to_string()).or_insert(TeamResult::new()).draws += 1;
                match_result.entry(team2.to_string()).or_insert(TeamResult::new()).draws += 1;
            },
            _ => unreachable!("result must be win, loss, or draw"),
        }
    }

    // Print header
    writeln!(output, "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}", "Team", "MP", "W", "D", "L", "P").unwrap();
    let mut sort_result: Vec<(&String, &TeamResult)> = match_result.iter().collect();
    sort_result.sort_by(|(team_a, result_a), (team_b, result_b)| {
        result_b.points().cmp(&result_a.points()).then_with(|| team_a.cmp(team_b))
    });
    for (team, result) in sort_result {
        writeln!(output, "{}", result.print_score(team)).unwrap();
    }
    String::from_utf8(output).unwrap().trim_end().to_string()
}