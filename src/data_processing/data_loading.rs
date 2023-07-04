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
    while let Some(c) = input_chars.next() {
        match c {
            c if is_valid_in_name(c) => {
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
            }
            c if c.is_whitespace() => {} // skip whitespace
            '|' => {}                    // skip delimiters
            _ => return Err(format!("Unexpected char '{c}' in variable name.")),
        }
    }

    if var_names.is_empty() {
        return Err("No variable names provided".to_string());
    }
    Ok(var_names)
}

/// Parse type of the data. Must be one of the valid options.
fn parse_observation_type(type_string: String) -> Result<ObservationType, String> {
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
    let observation_type = parse_observation_type(raw_type_str)?;

    let num_vars = var_names.len();
    for observation in &observations {
        if observation.values.len() != num_vars {
            return Err(format!("Observation {observation} has invalid length."));
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
    // TODO: properly test functions parse_var_names, parse_observations, parse_observation_type
    // TODO: properly test function generate_observation_list
}
