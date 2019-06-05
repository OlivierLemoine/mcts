const OFFSET: [usize; 9] = [0, 3, 6, 27, 30, 33, 54, 57, 60];

macro_rules! check_line {
    ($b:expr, $i:expr, $i1:expr, $i2:expr, $i3:expr) => {
        tri_eq($b[$i + $i1], $b[$i + $i2], $b[$i + $i3])
    };
}

macro_rules! returner {
    ( $b:expr, $fb:expr, $i:expr => $([$i1:expr, $i2:expr, $i3:expr]),* ) => {
        $(
            if check_line!($b, $i, $i1, $i2, $i3) {
                return $b[$i + $i1]
            }
        )*
        return $fb
    };
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Piece {
    Empty,
    X,
    O,
}

impl Piece {
    fn next(self) -> Piece {
        match self {
            Piece::Empty => Piece::Empty,
            Piece::X => Piece::O,
            Piece::O => Piece::X,
        }
    }
}

trait Index {
    fn position(self) -> Position;
}

impl Index for usize {
    fn position(self) -> Position {
        Position(self % 9, self / 9)
    }
}

struct Position(usize, usize);

impl Position {
    fn index(self) -> usize {
        let Position(x, y) = self;
        x + 9 * y
    }

    fn format(self) -> String {
        let Position(x, y) = self;
        format!("{} {}", x, y)
    }
}

enum TicTacToeErr {
    PieceNotAllowd,
    OutOfBound,
}

struct TicTacToe {
    board: [Piece; 81],
    next_piece: Piece,
}

impl TicTacToe {
    fn new(starter: Piece) -> Result<TicTacToe, TicTacToeErr> {
        if starter == Piece::Empty {
            Err(TicTacToeErr::PieceNotAllowd)
        } else {
            Ok(TicTacToe {
                board: [Piece::Empty; 81],
                next_piece: starter,
            })
        }
    }

    fn place_next_piece(mut self, p: Position) -> Result<Self, TicTacToeErr> {
        match p.index() {
            pos if pos < 81 => {
                let TicTacToe {
                    board: mut b,
                    next_piece: n,
                } = self;
                b[pos] = n;
                Ok(TicTacToe {
                    board: b,
                    next_piece: n.next(),
                })
            }
            _ => Err(TicTacToeErr::OutOfBound),
        }
    }

    fn check_winner_local(&self, p: Position) -> Piece {
        let i = p.index();
        returner!(self.board, Piece::Empty, i => [0,1,2],[9,10,11],[18,19,20],[0,9,18],[1,10,19],[2,11,20],[0,10,20],[2,10,18]);
    }

    fn check_winner(&self) -> Piece {
        let pieces: Vec<Piece> = OFFSET
            .iter()
            .map(|v| self.check_winner_local(v.position()))
            .collect();

        returner!(pieces, Piece::Empty, 0 => [0,1,2],[3,4,5],[6,7,8],[0,3,6],[1,4,7],[2,5,8],[0,4,8],[2,4,6]);
    }
}

#[inline]
fn tri_eq<T: PartialEq>(a: T, b: T, c: T) -> bool {
    a == b && a == c
}

fn main() {
    
}
