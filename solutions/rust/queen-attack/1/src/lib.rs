#[derive(Debug)]
pub struct ChessPosition {
    rank: u8, // Row from top-left 0 to bottom-right 7 (White's perspective)
    file: u8, // Column from top-left 0 to bottom-right 7
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (0..8).contains(&rank) && (0..8).contains(&file) {
            Some(ChessPosition {
                rank: rank as u8,
                file: file as u8,
            })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let same_rank = self.position.rank == other.position.rank;
        let same_file = self.position.file == other.position.file;
        let same_diagonal = self.position.rank.abs_diff(other.position.rank)
            == self.position.file.abs_diff(other.position.file);
        same_rank || same_file || same_diagonal
    }
}
