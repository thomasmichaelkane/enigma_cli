use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use yaml_rust::Yaml;

// Custom error type for validation errors
#[derive(Debug)]
pub enum ValidationError {
    MissingField(String),
    InvalidPermutation(String, String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::MissingField(field) => write!(f, "Missing required field: {}", field),
            ValidationError::InvalidPermutation(field, value) => {
                write!(f, "Invalid permutation in {}: '{}' - must contain exactly all letters A-Z once", field, value)
            }
        }
    }
}

impl Error for ValidationError {}

// Function to validate a single permutation string
fn validate_permutation(perm: &str, field_name: &str) -> Result<(), ValidationError> {
    // Check if the string contains exactly 26 characters
    if perm.len() != 26 {
        return Err(ValidationError::InvalidPermutation(
            field_name.to_string(),
            perm.to_string(),
        ));
    }

    // Ensure all letters A-Z are present
    let unique_chars: HashSet<char> = perm.chars().collect();
    let alphabet: HashSet<char> = ('A'..='Z').collect();
    if unique_chars != alphabet {
        return Err(ValidationError::InvalidPermutation(
            field_name.to_string(),
            perm.to_string(),
        ));
    }

    Ok(())
}

// Main validation function
pub fn validate_yaml(yaml: &Yaml) -> Result<(), Box<dyn Error>> {
    // Validate rotor field exists and is a sequence
    let rotors = yaml["rotor"].as_vec()
        .ok_or_else(|| ValidationError::MissingField("rotor".to_string()))?;
    
    // Validate each rotor permutation
    for (i, rotor) in rotors.iter().enumerate() {
        let rotor_str = rotor.as_str()
            .ok_or_else(|| ValidationError::InvalidPermutation(
                format!("rotor[{}]", i),
                format!("{:?}", rotor),
            ))?;
            
        // Extract only the characters before any comment
        let rotor_str = if let Some(comment_pos) = rotor_str.find('#') {
            rotor_str[..comment_pos].trim()
        } else {
            rotor_str.trim()
        };
        
        validate_permutation(rotor_str, &format!("rotor[{}]", i))?;
    }
    
    // Validate reflector field if it exists
    if let Some(reflector) = yaml["reflector"].as_str() {
      validate_permutation(reflector, "reflector")?;
    }
    
    Ok(())
    
}

