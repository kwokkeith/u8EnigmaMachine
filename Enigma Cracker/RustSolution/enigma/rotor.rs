//use std::collections::HashMap;

// Attributes of the Rotor Struct
pub struct Rotor {
    pub orig: Vec<u8>,
    //pub map: HashMap<u8,u8>,
    //pub imap: HashMap<u8,u8>,
    //pub orig_idx_map: HashMap<u8,u8>,
    pub map: [u8;256],
    pub imap: [u8;256],
    pub orig_idx_map: [u8;256],
    pub n: u16,
    pub pos: u8,
    pub notch: u8,
}


// Implementation of the Rotor Struct methods
impl Rotor {
    pub fn new(orig: &[u8], mapped: &[u8], notch: u8) -> Rotor {
        assert!(orig.len() <= 256, "Array too large");
        let mut map = [0u8;256];
        let mut imap= [0u8;256];
        let mut orig_idx_map= [0u8;256];
        let mut idx = 0u8;
        for it in orig.iter().zip(mapped.iter()) {
            let (a,b) = it;
            map[*a as usize] = *b;
            imap[*b as usize] = *a;
            orig_idx_map[*a as usize] = idx;
            idx += 1;
        }
        
        return Rotor {
            orig: orig.to_vec(),
            map,
            imap,
            orig_idx_map,
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
    pub fn _step_n(&mut self, size: u8) {
        self.pos = ((self.pos as u16 + size as u16) % self.n) as u8;
    }


    // Direct mapping of rotor internal mapping (Orig Input to Rotor Input)
    pub fn encipher(&self, input: u8) -> u8 {
        self.map[input as usize]
    }


    // Scrambler does not use letter mapping
    // it will make use of the concept of positioning instead
    // This function, enciphers using the position
    // NOTE: The letter mapping will only be done by obtaining the final position
    //  of the input signal to the stator
    pub fn encipher_pos(&self, mut input: u8) -> char {

        // Get the index position in the original input vector for the `input`
        let mut idx = self.orig_idx_map[input as usize] as i16;

        // Add offset of the rotor/wheel to the index and the get the actual 
        idx += self.pos as i16;

        if(idx >= self.n as i16) {
            idx -= self.n as i16;
        }
        //idx %= self.n as i16;

        // Map back to get the input element using the index calculated
        input = self.orig[idx as usize]; 

        // Map to the input element through the rotor/wheel
        let output = self.encipher(input);
        
        //print!("{} -> {}\n",input,output);

        idx = self.orig_idx_map[output as usize] as i16;

        // Map back to original stator position (to allow recursive calling of same method)
        idx -= self.pos as i16;

        if idx < 0 {
            idx += self.n as i16;
        }

        return self.orig[idx as usize] as char;
    }


    // Direct mapping of rotor internal mapping (Rotor output to Stator Input)
    // Gets the final mapped value
    pub fn decipher(&self, input: u8) -> u8 {
        self.imap[input as usize]
    }


    // Scrambler does not use letter mapping
    // it will make use of the concept of positioning instead
    // This function, deciphers using the position
    // NOTE: The letter mapping will only be done by obtaining the final position
    //  of the input signal to the stator
    pub fn decipher_pos(&self, mut input: u8) -> char {
        let mut idx = self.orig_idx_map[input as usize] as i16;
        idx += self.pos as i16;
        if(idx >= self.n as i16) {
            idx -= self.n as i16;
        }
        //idx %= self.n as i16;
        input = self.orig[idx as usize];
        let output = self.decipher(input);
        //print!("REVERSE: {} -> {}\n",input,output);
        idx = self.orig_idx_map[output as usize] as i16;
        idx -= self.pos as i16;
        if idx < 0 {
            idx += self.n as i16;
        }

        return self.orig[idx as usize] as char;
    }
}

