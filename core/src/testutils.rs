#[cfg(test)]
pub fn load_data(path: &str, field: &str) -> Vec<f64> {
    use std::fs;

    let data = fs::read_to_string(path).expect("Unable to read test data file");
    let res: Vec<Vec<f64>> = serde_json::from_str(&data).expect("Unable to parse test data");
    let field_index = match field {
        "o" => 1,
        "h" => 2,
        "l" => 3,
        "c" => 4,
        "v" => 5,
        "vv" => 6,
        _ => panic!("Invalid field: {}", field),
    };

    res.iter().map(|x| x[field_index]).collect()
}

#[cfg(test)]
pub fn load_data_nullable(path: &str, field: &str) -> Vec<Option<f64>> {
    load_data(path, field).into_iter().map(Some).collect()
}

#[cfg(test)]
pub fn load_expected<T: serde::de::DeserializeOwned>(path: &str) -> Vec<T> {
    use std::fs;

    let data = fs::read_to_string(path).expect(&format!("Unable to read test data file: {}", path));
    let res: Vec<T> = serde_json::from_str(&data).expect("Unable to parse test data");
    res
}
