// Chess board
// Src and target (x, y)

fn main() {
    let (src_x, src_y) = (4, 4);
    let (dest_x, dest_y) = (4, 4);
    //  min must be 0

    // curr loc: 0,0
    // const KNIGHT_MOVES = [(2,1), (2, -1), (-2,1)] // Assume

    
}

fn minm_steps(src : (i32, i32), dest: (i32, i32)) -> i32 {
    struct Pos {
        x: i32
        y: i32
        steps: i32
    }
    if src == dest {
        return 0;
    }

    // queue
    let mut q = VecDeque::new();
    let mut visited = vec![vec![false; 8]; 8]; // ***

    q.push_back( Pos {
        x: src.0,
        y: src.1,
        steps: 0
    });

    visited[src.0 as i32][src.1 as i32] = true;

    while let Some(pos) = q.pop_front() {
        for &(x, y) in KNIGHT_MOVES {
            let newX = pos.x + x;
            let newY = pos.y + y;
        

        if (newX, newY) == dest {
            return pos.steps + 1;
        }

        visited[newX as i32][newY as i32] = true;
        q.push_back(Pos {
            x: newX,
            y: newY,
            steps: pos.steps + 1
        });
        }
    }
}:

