#[allow(unused)]
#[doc(hidden)]
macro_rules! lerp {
    ($start:expr, $end:expr, $alpha:expr) => {
        $start + (($end - $start) * $alpha)
    };
}
pub(crate) use lerp;
