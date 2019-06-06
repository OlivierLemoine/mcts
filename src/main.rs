const OFFSET: [usize; 9] = [0, 3, 6, 27, 30, 33, 54, 57, 60];

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

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
pub enum Piece {
    Empty,
    X,
    O,
}

impl Piece {
    #[inline]
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
    #[inline]
    fn position(self) -> Position {
        Position(self % 9, self / 9)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Position(usize, usize);

impl Position {
    #[inline]
    fn index(self) -> usize {
        let Position(x, y) = self;
        x + 9 * y
    }

    #[inline]
    fn get_first(self) -> Self {
        let Position(x, y) = self;
        Position((x / 3) * 3, (y / 3) * 3)
    }

    fn from_string(s: String) -> Result<Position> {
        let mut a = s.split(" ");
        let x = a.next().ok_or(position_error::Empty)?.parse::<usize>()?;
        let y = a.next().ok_or(position_error::Empty)?.parse::<usize>()?;
        Ok(Position(x, y))
    }

    fn format(self) -> String {
        let Position(x, y) = self;
        format!("{} {}", x, y)
    }
}

mod position_error {
    #[derive(Debug)]
    pub struct Empty;
    impl std::fmt::Display for Empty {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    impl std::error::Error for Empty {
        fn description(&self) -> &str {
            "String is empty"
        }

        fn cause(&self) -> Option<&std::error::Error> {
            None
        }
    }
}

mod tic_tac_toe_error {
    use crate::Position;

    #[derive(Debug)]
    pub struct PieceNotAllowd(pub crate::Piece);
    impl std::fmt::Display for PieceNotAllowd {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    impl std::error::Error for PieceNotAllowd {
        fn description(&self) -> &str {
            "You cant use this piece here"
        }

        fn cause(&self) -> Option<&std::error::Error> {
            None
        }
    }

    #[derive(Debug)]
    pub struct NotAValidMove(pub Position, pub Vec<Position>);
    impl std::fmt::Display for NotAValidMove {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    impl std::error::Error for NotAValidMove {
        fn description(&self) -> &str {
            "Not a valid move"
        }

        fn cause(&self) -> Option<&std::error::Error> {
            None
        }
    }

    #[derive(Debug)]
    pub struct OutOfBound;
    impl std::fmt::Display for OutOfBound {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    impl std::error::Error for OutOfBound {
        fn description(&self) -> &str {
            "The index is out of game bound"
        }

        fn cause(&self) -> Option<&std::error::Error> {
            None
        }
    }
}

#[derive(Clone)]
struct TicTacToe {
    board: [Piece; 81],
    next_piece: Piece,
    last_move: Option<Position>,
}

impl TicTacToe {
    fn new(starter: Piece) -> Result<TicTacToe> {
        if starter == Piece::Empty {
            Err(tic_tac_toe_error::PieceNotAllowd(starter).into())
        } else {
            Ok(TicTacToe {
                board: [Piece::Empty; 81],
                next_piece: starter,
                last_move: None,
            })
        }
    }

    fn place_next_piece(mut self, p: Position) -> Result<Self> {
        match p.index() {
            pos if pos < 81 => {
                if self.board[pos] != Piece::Empty {
                    Err(tic_tac_toe_error::PieceNotAllowd(self.board[pos]).into())
                } else if self.valid_moves().iter().find(|&&v| v == pos).is_none() {
                    let mut vm: Vec<usize> = self.valid_moves();
                    vm.sort();
                    Err(tic_tac_toe_error::NotAValidMove(
                        pos.position(),
                        vm.iter().map(|v| v.position()).collect(),
                    )
                    .into())
                } else {
                    let TicTacToe {
                        board: mut b,
                        next_piece: n,
                        last_move: _,
                    } = self;

                    b[pos] = n;
                    Ok(TicTacToe {
                        board: b,
                        next_piece: n.next(),
                        last_move: if self.check_square_full(pos.position().get_first()) {
                            None
                        } else {
                            Some(pos.position())
                        },
                    })
                }
            }
            _ => Err(tic_tac_toe_error::OutOfBound.into()),
        }
    }

    fn check_square_full(&self, p: Position) -> bool {
        match self.check_winner_local(p) {
            Piece::Empty => false,
            _ => true,
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

    fn valid_moves(&mut self) -> Vec<usize> {
        let res = self.moves();
        if res.len() == 0 {
            self.last_move = None;
            self.moves()
        } else {
            res
        }
    }

    fn moves(&self) -> Vec<usize> {
        if let Some(pos) = self.last_move.clone() {
            let i = pos.index();
            [
                i,
                i + 1,
                i + 2,
                i + 9,
                i + 10,
                i + 11,
                i + 18,
                i + 19,
                i + 20,
            ]
            .iter()
            .filter_map(|&v| {
                if self.board[v] == Piece::Empty {
                    Some(v)
                } else {
                    None
                }
            })
            .collect()
        } else {
            self.board
                .iter()
                .enumerate()
                .filter_map(|(i, &v)| if v == Piece::Empty { Some(i) } else { None })
                .collect()
        }
    }
}

#[inline]
fn tri_eq<T: PartialEq>(a: T, b: T, c: T) -> bool {
    a == b && a == c
}

fn main() -> Result<()> {
    let mut t = TicTacToe::new(Piece::O)?;
    t.valid_moves();
    t = t.place_next_piece(Position(0, 0))?;
    t.check_winner();
    Ok(())
}

#[test]
fn create_test() {
    assert_eq!(TicTacToe::new(Piece::Empty).is_err(), true);
    assert_eq!(TicTacToe::new(Piece::X).is_ok(), true);
    assert_eq!(TicTacToe::new(Piece::O).is_ok(), true);
}

#[test]
fn position_convertion() {
    for i in 0usize..81 {
        let p = i.position();
        assert_eq!(p.index(), i);
    }

    for i in 0usize..9 {
        for j in 0usize..9 {
            let p = Position(i, j);
            let v = i + 9 * j;
            assert_eq!(v.position(), p);
        }
    }
}

#[test]
fn play_out_of_bound() {
    assert_eq!(
        TicTacToe::new(Piece::X)
            .unwrap()
            .place_next_piece(81usize.position())
            .is_err(),
        true
    );
}

#[test]
fn play_same_place() {
    assert_eq!(
        TicTacToe::new(Piece::X)
            .unwrap()
            .place_next_piece(Position(0, 0))
            .unwrap()
            .place_next_piece(Position(1, 1))
            .unwrap()
            .place_next_piece(Position(0, 0))
            .is_err(),
        true
    );
    assert_eq!(
        TicTacToe::new(Piece::X)
            .unwrap()
            .place_next_piece(Position(0, 0))
            .unwrap()
            .place_next_piece(Position(0, 0))
            .is_err(),
        true
    );
}

#[test]
fn right_moves() {
    let mut t = TicTacToe::new(Piece::O).unwrap();
    assert_eq!(t.valid_moves().len(), 81);
    t = t.place_next_piece(Position(0, 0)).unwrap();
    assert_eq!(
        t.valid_moves().sort(),
        vec![
            Position(0, 1).index(),
            Position(0, 2).index(),
            Position(1, 0).index(),
            Position(1, 1).index(),
            Position(1, 2).index(),
            Position(2, 0).index(),
            Position(2, 1).index(),
            Position(2, 2).index(),
        ]
        .sort()
    );
    assert_eq!(t.clone().place_next_piece(Position(0, 3)).is_err(), true);

    t = t.place_next_piece(Position(1, 0)).unwrap();
    assert_eq!(
        t.valid_moves().sort(),
        vec![
            Position(3, 0).index(),
            Position(3, 1).index(),
            Position(3, 2).index(),
            Position(4, 0).index(),
            Position(4, 1).index(),
            Position(4, 2).index(),
            Position(5, 0).index(),
            Position(5, 1).index(),
            Position(5, 2).index(),
        ]
        .sort()
    );

    t = t.place_next_piece(Position(3, 0)).unwrap();
    assert_eq!(
        t.valid_moves().sort(),
        vec![
            Position(0, 1).index(),
            Position(0, 2).index(),
            Position(1, 1).index(),
            Position(1, 2).index(),
            Position(2, 0).index(),
            Position(2, 1).index(),
            Position(2, 2).index(),
        ]
        .sort()
    );

    t = t.place_next_piece(Position(1, 1)).unwrap();
    // assert_eq!(
    //     t.valid_moves().sort(),
    //     vec![
    //         Position(3, 3).index(),
    //         Position(3, 4).index(),
    //         Position(3, 5).index(),
    //         Position(4, 3).index(),
    //         Position(4, 4).index(),
    //         Position(4, 5).index(),
    //         Position(5, 3).index(),
    //         Position(5, 4).index(),
    //         Position(5, 5).index(),
    //     ]
    //     .sort()
    // );
}

#[test]
fn play_valid_but_not_right_move() {
    assert_eq!(
        TicTacToe::new(Piece::X)
            .unwrap()
            .place_next_piece(Position(0, 0))
            .unwrap()
            .place_next_piece(Position(0, 3))
            .is_err(),
        true
    );
}

// #[test]
fn basic_win() {
    let mut t = TicTacToe::new(Piece::X)
        .unwrap()
        .place_next_piece(Position(0, 0))
        .unwrap()
        .place_next_piece(Position(1, 0))
        .unwrap()
        .place_next_piece(Position(3, 0))
        .unwrap()
        .place_next_piece(Position(1, 1))
        .unwrap()
        .place_next_piece(Position(3, 3))
        .unwrap();
    assert_eq!(t.check_winner_local(Position(0, 0)), Piece::Empty);

    t = t.place_next_piece(Position(1, 2)).unwrap();

    assert_eq!(t.check_winner_local(Position(0, 0)), Piece::X);
}
