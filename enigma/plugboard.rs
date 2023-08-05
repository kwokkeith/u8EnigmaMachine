use std::collections::HashMap;

pub struct Plugboard {
    pub n: u8, // Number of characters used
    pub map: HashMap<u8,u8>, 
    pub one_way_map: HashMap<char,char>, 
}

// Implementation of the Rotor Struct methods
impl Plugboard {
    pub fn new(mapping: &[(u8,u8)]) -> Plugboard {
        let mut map: HashMap<u8,u8> = HashMap::new();
        let mut one_way_map: HashMap<char,char> = HashMap::new();
        for t in mapping {
            let (a, b) = t;
            one_way_map.insert(*a as char, *b as char);
            map.insert(*a, *b);
            map.insert(*b, *a); 
        }

        return Plugboard {
            n: mapping.len() as u8,
            map,
            one_way_map
        };
    }


    // Puts signal through plugboard and get new value
    pub fn encipher(&self, input: u8) -> u8 {
        // If no connection between two keys (wire), MAP back to same value
        if !self.map.contains_key(&input) {
            return input;
        }
        return self.map[&input];
    }
}

// Allows plugboard struct to be printed
impl std::fmt::Display for Plugboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::new();

        for (key, value) in &self.one_way_map {
            output.push_str("( '");
            output.push(*key);
            output.push_str("' <-> '");
            output.push(*value);
            output.push_str("' ), ");
        }

        output.pop();
        output.pop();

        return write!(f, "{}", output)
    }
}
