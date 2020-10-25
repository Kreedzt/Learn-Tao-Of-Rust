// use std::path::PathBuf;
// use std::fs::File;

// 移动到上级
use super::{Error, PathBuf, File, Read, Write};
use std::io::prelude::*;

pub fn load_csv(csv_file: PathBuf) -> Result<String, Error> {
    let file = read(csv_file)?;
    Ok(file)
}

/// # Usage
/// ```ignore
/// let filename = PathBuf::from("./files/chjallenge.csv");
/// let csv_data = load_csv(filename).unwrap();
/// let modified_data = replace_column(
///    csv_data, "City", "Beijing").unwrap();
/// let ouput_file = write_csv(&modified_data, "output/test.csv");
/// assert!(output_file.is_ok());
/// ```
pub fn write_csv(csv_data: &str, filename: &str) -> Result<(), Error> {
    write(csv_data, filename)?;
    Ok(())
}

fn read(path: PathBuf) -> Result<String, Error> {
    let mut buffer = String::new();
    let mut file = open(path)?;
    file.read_to_string(&mut buffer)?;

    if buffer.is_empty() {
        return Err("Input file Missing")?
    }
    Ok(buffer)
}

fn open(path: PathBuf) -> Result<File, Error> {
    let file = File::open(path)?;
    Ok(file)
}

fn write(data: &str, filename: &str) -> Result<(), Error> {
    let mut buffer = File::create(filename)?;
    buffer.write_all(data.as_bytes())?;
    Ok(())
}


#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use super::load_csv;

    #[test]
    fn test_valid_load_csv() {
        let filename = PathBuf::from("./input/challenge.csv");
        let csv_data = load_csv(filename);
        assert!(csv_data.is_ok());
    }
}
