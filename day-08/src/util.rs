pub fn len_to_digit(len: usize) -> Option<usize> {
    let digit = match len {
        2 => 1,
        3 => 7,
        4 => 4,
        7 => 8,
        _ => return None
    };

    Some(digit)
}

static ZERO: [char; 6] = ['a', 'b', 'c', 'e', 'f', 'g'];
static ONE: [char; 2] = ['c', 'f'];
static TWO: [char; 5] = ['a', 'c', 'd', 'e', 'g']; // yes
static THREE: [char; 5] = ['a', 'c', 'd', 'f', 'g']; // yes
static FOUR: [char; 4] = ['b', 'c', 'd', 'f'];
static FIVE: [char; 5] = ['a', 'b', 'd', 'f', 'g']; //
static SIX: [char; 6] = ['a', 'b', 'd', 'e', 'f', 'g'];
static SEVEN: [char; 3] = ['a', 'c', 'f'];
static EIGHT: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
static NINE: [char; 6] = ['a', 'b', 'c', 'd', 'f', 'g'];

pub static DIGITS: [(&'static [char], usize); 10] = [
    (&ZERO, 0),
    (&ONE ,1),
    (&TWO, 2),
    (&THREE, 3),
    (&FOUR, 4),
    (&FIVE, 5),
    (&SIX, 6),
    (&SEVEN, 7),
    (&EIGHT, 8),
    (&NINE, 9)
];

pub fn digit_to_segments(digit: usize) -> (&'static [char], usize) {
    DIGITS[digit]
}