pub struct Plugboard {
    n: u8, // Number of characters used
    map: HashMap<u8,u8>, 
}

// Implementation of the Rotor Struct methods
impl Plugboard {
    pub fn new(mapping: &[(u8, u8)]) -> Plugboard {
        let mut map: HashMap<u8,u8> = HashMap::new();
        for t in mapping {
            let (a, b) = t;
            map.insert(*a, *b);
            map.insert(*b, *a); 
        }

        return Plugboard {
            n: mapping.len() as u8,
            map
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