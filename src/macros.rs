macro_rules! todo {
    () => {
        panic!("TODO at {}:{}", file!(), line!())
    };
}

macro_rules! pp {
    ($expr:expr) => {
        println!("--- pp @ {}:{} --------", file!(), line!());
        println!("{}", $expr);
    };
}

macro_rules! db {
    ($expr:expr) => {
        println!("--- db @ {}:{} --------", file!(), line!());
        println!("{:?}", $expr);
    };
}

macro_rules! doit {
    ($expr:expr) => {
        match $expr {
            Ok(x) => x,
            Err(e) => panic!("{}", e),
        }
    };
}
