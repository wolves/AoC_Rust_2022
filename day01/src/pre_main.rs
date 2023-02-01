// #[derive(Debug)]
// struct PathedIoError {
//     path: String,
//     inner: std::io::Error,
// }

// impl std::fmt::Debug for PathedIoError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "for file {:?}: {}", self.path, self.inner)
//     }
// }

// Equivalent to #[derive(Debug)]
// impl std::fmt::Debug for PathedIoError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("PathedIoError")
//             .field("path", &self.path)
//             .field("inner", &self.inner)
//             .finish()
//     }
// }
//
use color_eyre::eyre::Context;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = read_input()?;
    println!("{input}");

    Ok(())
}

fn read_input() -> color_eyre::Result<String> {
    let input = std::fs::read_to_string("src/input.txt").wrap_err("reading src/input.txt")?;
    Ok(input)
}

// fn read_input() -> Result<String, PathedIoError> {
//     let path = "src/input.txt";
//
//     match std::fs::read_to_string(path) {
//         Ok(s) => Ok(s),
//         Err(e) => Err(PathedIoError {
//             path: path.into(),
//             inner: e,
//         }),
//     }
// }
