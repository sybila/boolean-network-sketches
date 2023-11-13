//! Contains functionality to load (or directly encode) binarized data from files.

use crate::data_processing::observations::{Observation, ObservationList, ObservationType};
use std::fs::read_to_string;

/// Read the data observations from the specified file. Ignore lines starting with `#` (comments).
/// This function does not validate the inputs.
/// Returns vector of trimmed observation strings, string with var names and string with data type.
fn load_raw_observations(data_path: &str) -> Result<(Vec<String>, String, String), String> {
    let Ok(data_file_string) = read_to_string(data_path) else {
        return Err("Cannot read content of the specified file.".to_string());
    };

    let mut observation_strings: Vec<String> = Vec::new();
    let mut var_name_string = String::new();
    let mut data_type_string = String::new();
    for (i, line) in data_file_string.lines().enumerate() {
        let trimmed_line = line.trim();
        if i == 0 {
            var_name_string.push_str(trimmed_line);
            continue;
        }
        if i == 1 {
            data_type_string.push_str(trimmed_line);
            continue;
        }
        if !trimmed_line.is_empty() && !trimmed_line.starts_with('#') {
            observation_strings.push(trimmed_line.to_string());
        }
    }
    Ok((observation_strings, var_name_string, data_type_string))
}

/// Parse observations from their string representation.
/// The observations must only characters '1', '0' or '-' (for missing).
/// All observations must be strings of the same length (where length is the number of variables).
fn parse_observations(observation_strings: Vec<String>) -> Result<Vec<Observation>, String> {
    let mut observations: Vec<Observation> = Vec::new();
    for observation_string in observation_strings {
        match Observation::try_from_str(observation_string) {
            Ok(observation) => observations.push(observation),
            Err(e) => return Err(e),
        }
    }
    if observations.is_empty() {
        return Err("No observations provided".to_string());
    }

    Ok(observations)
}

/// Check if given char can appear in a name.
fn is_valid_in_name(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Tokenize and parse variable names from the single string representation.
/// The string must contain only valid variable names delimited by '|'.
fn parse_var_names(var_string: String) -> Result<Vec<String>, String> {
    let mut var_names: Vec<String> = Vec::new();
    let mut input_chars = var_string.chars().peekable();
    let mut parsed_something = false;
    while let Some(c) = input_chars.next() {
        match c {
            c if is_valid_in_name(c) => {
                // if we already parsed a name after previous delimiter, there must have been space
                if parsed_something {
                    return Err("Variable name can't contain spaces.".to_string());
                }

                let mut name = String::new();
                name.push(c);
                while let Some(c) = input_chars.peek() {
                    if !is_valid_in_name(*c) {
                        break;
                    } else {
                        name.push(*c);
                        input_chars.next(); // advance iterator
                    }
                }
                var_names.push(name);
                parsed_something = true;
            }
            c if c.is_whitespace() => {} // skip whitespace
            '|' => {
                // delimiters
                // in case that 2 delimiters follow without any var in between
                if !parsed_something {
                    return Err("Variable name can't be empty.".to_string());
                }
                parsed_something = false;
            }
            _ => return Err(format!("Unexpected char '{c}' in variable name.")),
        }
    }

    if var_names.is_empty() {
        return Err("No variable names provided.".to_string());
    }
    Ok(var_names)
}

/// Parse type of the data. Must be one of the valid options.
fn parse_observ_type(type_string: String) -> Result<ObservationType, String> {
    match type_string.as_str() {
        "Attractor" => Ok(ObservationType::Attractor),
        "FixedPoint" => Ok(ObservationType::FixedPoint),
        "TimeSeries" => Ok(ObservationType::TimeSeries),
        "Unspecified" => Ok(ObservationType::Unspecified),
        _ => Err(format!("Invalid data type \"{type_string}\"")),
    }
}

/// Process and combine strings for individual components of the `ObservationList` struct.
pub fn generate_observation_list(
    raw_observation_strs: Vec<String>,
    raw_vars_str: String,
    raw_type_str: String,
) -> Result<ObservationList, String> {
    let observations = parse_observations(raw_observation_strs)?;
    let var_names = parse_var_names(raw_vars_str)?;
    let observation_type = parse_observ_type(raw_type_str)?;

    let num_vars = var_names.len();
    for observation in &observations {
        if observation.values.len() != num_vars {
            return Err(format!("Observation '{observation}' has invalid length."));
        }
    }
    Ok(ObservationList::new(
        observations,
        var_names,
        observation_type,
    ))
}

/// Read the data observations from the specified file. Ignore lines starting with `#` (comments).
/// The first line must contain variable names in order, delimited by '|'.
/// The second line must contain valid string for observation type (above) or "Unspecified".
/// The observations can contain only characters '1', '0' or '-' (for missing).
/// All observations must be strings of the same length (where length is the number of variables).
pub fn load_observations(data_path: &str) -> Result<ObservationList, String> {
    let loaded_data = load_raw_observations(data_path);
    if loaded_data.is_err() {
        return Err("Unable to load observations from given file.".to_string());
    }
    let (raw_observation_strs, raw_vars_str, raw_type_str) = loaded_data.unwrap();
    generate_observation_list(raw_observation_strs, raw_vars_str, raw_type_str)
}

#[cfg(test)]
mod tests {
    use crate::data_processing::data_loading::{
        generate_observation_list, parse_observ_type, parse_observations, parse_var_names,
    };
    use crate::data_processing::observations::{
        Observation, ObservationList, ObservationType, VarValue,
    };

    #[test]
    /// Test parsing of variable names.
    fn test_observation_var_names_parsing() {
        let var_name_str = "a  | bb|ccc|f_5".to_string();
        let expected = vec![
            "a".to_string(),
            "bb".to_string(),
            "ccc".to_string(),
            "f_5".to_string(),
        ];
        assert_eq!(parse_var_names(var_name_str).unwrap(), expected);

        let var_name_str = "a  | |ccc|f_5".to_string();
        let res = parse_var_names(var_name_str);
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            "Variable name can't be empty.".to_string()
        );

        let var_name_str = "  ".to_string();
        let res = parse_var_names(var_name_str);
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            "No variable names provided.".to_string()
        );

        let var_name_str = "a | j7& |ccc|f_5".to_string();
        let res = parse_var_names(var_name_str);
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            "Unexpected char '&' in variable name.".to_string()
        );

        let var_name_str = "a o | j7& |ccc|f_5".to_string();
        let res = parse_var_names(var_name_str);
        assert!(res.is_err());
        assert_eq!(
            res.err().unwrap(),
            "Variable name can't contain spaces.".to_string()
        );
    }

    #[test]
    /// Test parsing of observation type.
    fn test_observation_type_parsing() {
        let observ_type_str = "Attractor".to_string();
        assert_eq!(
            parse_observ_type(observ_type_str).unwrap(),
            ObservationType::Attractor
        );
        let observ_type_str = "FixedPoint".to_string();
        assert_eq!(
            parse_observ_type(observ_type_str).unwrap(),
            ObservationType::FixedPoint
        );
        let observ_type_str = "TimeSeries".to_string();
        assert_eq!(
            parse_observ_type(observ_type_str).unwrap(),
            ObservationType::TimeSeries
        );
        let observ_type_str = "Unspecified".to_string();
        assert_eq!(
            parse_observ_type(observ_type_str).unwrap(),
            ObservationType::Unspecified
        );

        let observ_type_str = "Idk".to_string();
        assert!(parse_observ_type(observ_type_str).is_err());
    }

    #[test]
    /// Test parsing of observations.
    fn test_observations_parsing() {
        let observation_strings = vec!["000".to_string(), "0--".to_string(), "1-1".to_string()];
        let expected = vec![
            Observation::new(vec![VarValue::False, VarValue::False, VarValue::False]),
            Observation::new(vec![VarValue::False, VarValue::Any, VarValue::Any]),
            Observation::new(vec![VarValue::True, VarValue::Any, VarValue::True]),
        ];
        assert_eq!(parse_observations(observation_strings).unwrap(), expected);

        let observation_strings = vec![];
        assert!(parse_observations(observation_strings).is_err());

        let observation_strings = vec!["0i0".to_string()];
        assert!(parse_observations(observation_strings).is_err());

        let observation_strings = vec!["000".to_string(), "".to_string()];
        assert!(parse_observations(observation_strings).is_err());
    }

    #[test]
    /// Test whole combined parsing step that generates ObservationList object.
    fn test_combined_parsing() {
        // TODO: properly test function generate_observation_list
        let observation_strings = vec!["01".to_string(), "0-".to_string()];
        let var_name_str = "a  | B_2".to_string();
        let observ_type_str = "Attractor".to_string();

        let expected = ObservationList::new(
            vec![
                Observation::new(vec![VarValue::False, VarValue::True]),
                Observation::new(vec![VarValue::False, VarValue::Any]),
            ],
            vec!["a".to_string(), "B_2".to_string()],
            ObservationType::Attractor,
        );
        assert_eq!(
            generate_observation_list(observation_strings, var_name_str, observ_type_str).unwrap(),
            expected,
        );

        let invalid = generate_observation_list(
            vec!["01".to_string()],
            "a".to_string(),
            "Attractor".to_string(),
        );
        assert!(invalid.is_err());
        assert_eq!(
            invalid.err().unwrap(),
            "Observation '01' has invalid length.".to_string()
        );
    }
}
