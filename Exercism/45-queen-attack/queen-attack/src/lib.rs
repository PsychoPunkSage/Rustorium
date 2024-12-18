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
        self.rank == other.rank
            || self.file == other.file
            || (self.rank - other.rank).abs() == (self.file - other.file).abs()
    }
}
