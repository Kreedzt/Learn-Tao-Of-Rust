#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use csv_challenge::{
        Opt,
        {load_csv, write_csv},
        replace_column,
    };

    #[test]
    fn test_csv_challenge() {
        let filename = PathBuf::from("./input/challenge.csv");
        let csv_data = load_csv(filename).unwrap();

        let modified_data = replace_column(
            csv_data, "City", "Beijing"
        ).unwrap();

        let output_file = write_csv(
            &modified_data,
            "output/test.csv"
        );
        assert!(output_file.is_ok());
    }
}
