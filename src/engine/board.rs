const EMPTY_STATE: &[i8; 120] = &[
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1,  0,  0,  0,  0,  0,  0,  0,  0, -1,
    -1,  0,  0,  0,  0,  0,  0,  0,  0, -1,
    -1,  0,  0,  0,  0,  0,  0,  0,  0, -1,
    -1,  0,  0,  0,  0,  0,  0,  0,  0, -1,
    -1,  0,  0,  0,  0,  0,  0,  0,  0, -1,
    -1,  0,  0,  0,  0,  0,  0,  0,  0, -1,
    -1,  0,  0,  0,  0,  0,  0,  0,  0, -1,
    -1,  0,  0,  0,  0,  0,  0,  0,  0, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
];

const INITIAL_STATE: &[i8; 120] = &[
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, 21, 19, 20, 22, 17, 20, 19, 21, -1,
    -1, 18, 18, 18, 18, 18, 18, 18, 18, -1,
    -1,  0,  0,  0,  0,  0,  0,  0,  0, -1,
    -1,  0,  0,  0,  0,  0,  0,  0,  0, -1,
    -1,  0,  0,  0,  0,  0,  0,  0,  0, -1,
    -1,  0,  0,  0,  0,  0,  0,  0,  0, -1,
    -1, 10, 10, 10, 10, 10, 10, 10, 10, -1,
    -1, 13, 11, 12, 14,  9, 12, 11, 13, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
];

pub struct Board(Vec<i8>);

impl Board {
    pub fn new(state: &[i8]) -> Self {
        assert!(state.len() == 120, "Board must be an array of 120 8-bit integers");
        
        Self(state.to_owned())
    }

    pub fn empty() -> Self {
        Self(EMPTY_STATE.to_vec())
    }

    pub fn initial_state() -> Self {
        Self(INITIAL_STATE.to_vec())
    }

    pub fn is_square_empty(&self, index: usize) -> bool {
        self.0[index] == 0
    }
}
