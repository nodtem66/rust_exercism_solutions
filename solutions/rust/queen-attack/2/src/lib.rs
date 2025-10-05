#[derive(Debug)]
pub struct ChessPosition {
    rank: u8, // Row from top-left 0 to bottom-right 7 (White's perspective)
    file: u8, // Column from top-left 0 to bottom-right 7
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..8, 0..8) => Some( Self { rank: rank as u8, file: file as u8 }),
            _ => None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }
    pub fn can_attack(&self, other: &Queen) -> bool {
        let same_rank = self.0.rank == other.0.rank;
        let same_file = self.0.file == other.0.file;
        let same_diagonal = self.0.rank.abs_diff(other.0.rank)
            == self.0.file.abs_diff(other.0.file);
        same_rank || same_file || same_diagonal
    }
}
