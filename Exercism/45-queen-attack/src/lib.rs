#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    rank: i32,
    file: i32,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            None
        } else {
            Some(Self { rank, file })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen {
            rank: position.rank,
            file: position.file,
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let rank_q1 = self.rank;
        let file_q1 = self.file;
        let rank_q2 = other.rank;
        let file_q2 = other.file;

        rank_q1 == rank_q2
            || file_q1 == file_q2
            || (rank_q1 - rank_q2).abs() == (file_q1 - file_q2).abs()
    }
}
