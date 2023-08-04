use std::collections::HashMap;

// Attributes of the Rotor Struct
pub struct Rotor {
    pub orig: Vec<u8>,
    map: HashMap<u8,u8>,
    imap: HashMap<u8,u8>,
    n: u16,
    pub pos: u8,
    notch: u8,
}


// Implementation of the Rotor Struct methods
impl Rotor {
    pub fn new(orig: &[u8], mapped: &[u8], notch: u8) -> Rotor {
        assert!(orig.len() <= 256, "Array too large");
        let mut map: HashMap<u8,u8> = HashMap::new();
        let mut imap: HashMap<u8,u8> = HashMap::new();
        for it in orig.iter().zip(mapped.iter()) {
            let (a,b) = it;
            map.insert(*a,*b);
            imap.insert(*b,*a);
        }
        
        return Rotor {
            orig: orig.to_vec(),
            map,
            imap,
            n: orig.len() as u16,
            pos: 0,
            notch,
        };
    }


    // Rotate the rotor/wheel by one step
    pub fn step(&mut self) {
        self.pos = ((self.pos as u16 + 1) % self.n) as u8;
    }


    // Rotate the rotor/wheel by `size` steps
    pub fn step_n(&mut self, size: u8) {
        self.pos = ((self.pos as u16 + size as u16) % self.n) as u8;
    }


    // Direct mapping of rotor internal mapping (Orig Input to Rotor Input)
    pub fn encipher(&self, input: u8) -> char {
        self.map[&input] as char
    }


    // Scrambler does not use letter mapping
    // it will make use of the concept of positioning instead
    // This function, enciphers using the position
    // NOTE: The letter mapping will only be done by obtaining the final position
    //  of the input signal to the stator
    pub fn encipherPos(&self, mut input: u8) -> char {

        // Get the index position in the original input vector for the `input`
        let mut idx: i16 = self.orig
            .iter()
            .position(|&x| x == input)
            .unwrap() as i16;

        // Add offset of the rotor/wheel to the index and the get the actual 
        idx += self.pos as i16;
        idx %= self.n as i16;

        // Map back to get the input element using the index calculated
        input = self.orig[idx as usize]; 

        // Map to the input element through the rotor/wheel
        let mut output = self.map[&input];
        
        //print!("{} -> {}\n",input,output);
        idx = self.orig
            .iter()
            .position(|&x| x == output)
            .unwrap() as i16;

        // Map back to original stator position (to allow recursive calling of same method)
        idx -= self.pos as i16;

        if idx < 0 {
            idx += self.n as i16;
        }

        // idx %= self.n as i16; // May be redundant
        return self.orig[idx as usize] as char;
    }


    // Direct mapping of rotor internal mapping (Rotor output to Stator Input)
    // Gets the final mapped value
    pub fn decipher(&self, input: u8) -> char {
        self.imap[&input] as char
    }


    // Scrambler does not use letter mapping
    // it will make use of the concept of positioning instead
    // This function, deciphers using the position
    // NOTE: The letter mapping will only be done by obtaining the final position
    //  of the input signal to the stator
    pub fn decipherPos(&self, mut input: u8) -> char {
        let mut idx: i16 = self.orig
            .iter()
            .position(|&x| x == input)
            .unwrap() as i16;
        idx += self.pos as i16;
        idx %= self.n as i16;
        input = self.orig[idx as usize];
        let mut output = self.imap[&input];
        //print!("REVERSE: {} -> {}\n",input,output);
        idx = self.orig
            .iter()
            .position(|&x| x == output)
            .unwrap() as i16;
        idx -= self.pos as i16;
        if idx < 0 {
            idx += self.n as i16;
        }
        idx %= self.n as i16;
        self.orig[idx as usize] as char
    }
}

