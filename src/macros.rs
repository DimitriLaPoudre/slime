#[macro_export]
macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        ($r << 16) | ($g << 8) | $b
    };
}
