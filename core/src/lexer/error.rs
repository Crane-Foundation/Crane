
#[macro_export]
macro_rules! throw {
    ($error:expr, $line:expr) => {
        println!("{}: {}", Red.paint(format!("Error on line {}", $line)), $error);
        std::process::exit(0);
    };
    ($error:expr) => {
        println!("{}: {}", Red.paint("Error"), $error);
        std::process::exit(0);
    }
}

