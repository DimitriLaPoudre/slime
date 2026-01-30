#[macro_export]
macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        ($r << 16) | ($g << 8) | $b
    };
}

///for know if two direction vector goes in the same direction
///_ = 0; perpendicular direction
///_ > 0: same direction
///_ < 0: opposite direction
#[macro_export]
macro_rules! dot {
    ($ax:expr, $ay:expr, $bx:expr, $by:expr) => {
        $ax * $bx + $ay * $by
    };
}

#[macro_export]
macro_rules! abs {
    ($n:expr) => {
        if $n < 0.0 {
            -$n
        } else {
            $n
        }
    };
}
