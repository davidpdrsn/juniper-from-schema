#[allow(dead_code)]
macro_rules! not_supported {
    ($info:expr) => {
        panic!(
            "{} are not yet supported. Feel free to submit an issue if you really need this.",
            $info
        )
    };
}
