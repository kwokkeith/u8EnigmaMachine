#![allow(warnings, unused)]

use std::collections::HashMap;
use std::collections::HashSet;

// Attributes of the Rotor Struct
struct Rotor {
    orig: Vec<u8>,
    map: HashMap<u8,u8>,
    imap: HashMap<u8,u8>,
    n: u16,
    pos: u8,
    notch: u8,
}


// Implementation of the Rotor Struct methods
impl Rotor {
    fn new(orig: &[u8], mapped: &[u8], notch: u8) -> Rotor {
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
    fn step(&mut self) {
        self.pos = ((self.pos as u16 + 1) % self.n) as u8;
    }


    // Rotate the rotor/wheel by `size` steps
    fn step_n(&mut self, size: u8) {
        self.pos = ((self.pos as u16 + size as u16) % self.n) as u8;
    }


    // Direct mapping of rotor internal mapping (Orig Input to Rotor Input)
    fn encipher(&self, input: u8) -> char {
        self.map[&input] as char
    }


    // Scrambler does not use letter mapping
    // it will make use of the concept of positioning instead
    // This function, enciphers using the position
    // NOTE: The letter mapping will only be done by obtaining the final position
    //  of the input signal to the stator
    fn encipherPos(&self, mut input: u8) -> char {

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
    fn decipher(&self, input: u8) -> char {
        self.imap[&input] as char
    }


    // Scrambler does not use letter mapping
    // it will make use of the concept of positioning instead
    // This function, deciphers using the position
    // NOTE: The letter mapping will only be done by obtaining the final position
    //  of the input signal to the stator
    fn decipherPos(&self, mut input: u8) -> char {
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


struct Plugboard {
    n: u8, // Number of characters used
    map: HashMap<u8,u8>, 
    imap: HashMap<u8,u8>,
}

// Implementation of the Rotor Struct methods
impl Plugboard {
    fn new(mapping: &[(u8, u8)]) -> Plugboard {
        let mut map: HashMap<u8,u8> = HashMap::new();
        let mut imap: HashMap<u8,u8> = HashMap::new();
        for t in mapping {
            let (a, b) = t;
            map.insert(*a, *b); 
            imap.insert(*b, *a);
        }

        return Plugboard {
            n: mapping.len() as u8,
            map,
            imap
        };
    }


    // Puts signal through plugboard and get new value
    fn encipher(&self, input: u8) -> u8 {
        // If no connection between two keys (wire), MAP back to same value
        if !self.map.contains_key(&input) {
            return input;
        }
        return self.map[&input];
    }


    fn decipher(&self, input: u8) -> u8 {
        if !self.imap.contains_key(&input) {
            return input;
        }
        return self.imap[&input];
    }
}


// Actual Machine
struct Enigma {
    slowRotor: Rotor,
    midRotor: Rotor,
    fastRotor: Rotor,
    plugboard: Plugboard,
    reflector: HashMap<u8,u8>,
}

impl Enigma {
    fn new(reflectorRotor: Rotor, slowRotor: Rotor, midRotor: Rotor, fastRotor: Rotor, plugboard: Plugboard) -> Enigma {
        Enigma {
            slowRotor,
            midRotor,
            fastRotor,
            plugboard,
            reflector: reflectorRotor.map,
        }
    }


    fn step(&mut self) {
        // Double Stepping Mid rotor (ANOMALY FOUND IN THE ORIGINAL ENIGMA)
        // In real machine, as long as mid rotor is at notch and fast rotor starts
        // to move, the mid rotor will step immediately.
        if self.midRotor.pos == self.midRotor.notch {
            self.slowRotor.step();
            self.midRotor.step();
        }
        // Single Stepping Mid Rotor
        if self.fastRotor.pos == self.fastRotor.notch {
            // Single Stepping Slow rotor
            if self.midRotor.pos == self.midRotor.notch {
                self.slowRotor.step();
            }
            self.midRotor.step();
        }
        // Step Fast rotor
        self.fastRotor.step();
    }


    // Encryption method
    fn encrypt(&mut self, mut input: u8) -> char {
        // Step the rotor/wheel
        self.step();

        // Plugboard
        input = self.plugboard.encipher(input);

        // Input is mutated for the next rotor/wheel
        input = self.fastRotor.encipherPos(input) as u8;
        input = self.midRotor.encipherPos(input) as u8;
        input = self.slowRotor.encipherPos(input) as u8;
        input = self.reflector[&input];
        input = self.slowRotor.decipherPos(input) as u8;
        input = self.midRotor.decipherPos(input) as u8;
        input = self.fastRotor.decipherPos(input) as u8;

        // Plugboard
        input = self.plugboard.decipher(input);
        return input as char;
    }


    // Starting position of the rotor (Part of Key)
    fn rotorSettings(&mut self, pos1: u8, pos2: u8, pos3: u8) {
        self.slowRotor.pos = pos1;
        self.midRotor.pos = pos2;
        self.fastRotor.pos = pos3;
    }
}




fn main() {
    let plugboard = Plugboard::new(&[(1,2),(3,4)]);
    
    let mut ufw_B = Rotor::new(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255],
                                &[103,148,248,233,54,166,89,175,161,128,71,162,176,55,49,69,83,30,208,251,136,35,156,137,51,201,172,255,135,133,17,158,119,178,94,21,236,223,238,65,68,196,124,102,75,230,153,92,159,14,187,24,241,62,4,13,118,74,209,98,130,231,53,239,190,39,155,143,40,15,210,10,142,85,57,44,101,200,140,150,188,229,204,16,221,73,243,181,112,6,147,246,47,191,34,250,232,226,59,157,120,76,43,0,163,127,198,126,170,218,154,224,88,129,179,207,213,141,56,32,100,160,165,174,42,254,107,105,9,113,60,177,244,29,212,28,20,23,252,152,78,117,72,67,171,219,211,90,1,220,79,253,139,46,110,66,22,99,31,48,121,8,11,104,173,122,5,169,237,167,108,144,26,164,123,7,12,131,33,114,202,87,194,206,242,235,249,50,80,247,64,93,214,227,182,205,41,240,106,225,77,25,180,217,82,195,183,115,18,58,70,146,134,116,192,234,222,203,109,145,149,84,216,37,111,199,97,193,245,81,45,61,96,3,215,185,36,168,38,63,197,52,184,86,132,228,91,189,2,186,95,19,138,151,125,27],
                                0);
    let mut r1 = Rotor::new(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255],
                            &[166, 123, 104, 40, 96, 192, 58, 224, 210, 59, 23, 69, 134, 250, 237, 50, 142, 221, 173, 107, 92, 37, 45, 126, 42, 83, 98, 12, 181, 118, 161, 253, 135, 0, 68, 191, 168, 17, 183, 214, 229, 203, 103, 243, 180, 25, 244, 249, 62, 132, 200, 86, 232, 35, 179, 241, 238, 97, 53, 242, 88, 217, 153, 16, 30, 6, 170, 230, 185, 109, 171, 149, 137, 106, 148, 78, 111, 85, 239, 127, 130, 225, 163, 125, 72, 74, 29, 56, 160, 193, 61, 44, 31, 10, 164, 188, 129, 55, 174, 122, 155, 87, 93, 156, 115, 19, 143, 187, 172, 1, 157, 234, 195, 5, 235, 139, 198, 196, 228, 131, 119, 247, 38, 201, 91, 205, 219, 14, 146, 60, 211, 117, 66, 24, 71, 162, 77, 21, 112, 100, 141, 36, 233, 199, 184, 54, 121, 75, 65, 82, 165, 11, 145, 34, 79, 220, 209, 120, 95, 190, 177, 226, 15, 48, 26, 216, 182, 67, 124, 13, 206, 169, 101, 46, 207, 133, 236, 218, 90, 154, 27, 178, 4, 147, 248, 63, 8, 108, 33, 70, 105, 89, 246, 144, 64, 114, 39, 227, 251, 116, 223, 110, 76, 94, 43, 208, 2, 240, 222, 151, 213, 41, 159, 113, 245, 231, 175, 150, 215, 81, 136, 186, 22, 158, 152, 9, 73, 202, 176, 52, 49, 3, 167, 204, 212, 252, 7, 84, 140, 20, 18, 32, 99, 128, 51, 47, 189, 197, 57, 28, 102, 80, 254, 255, 138, 194],
                            86);
    let mut r2 = Rotor::new(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255],
                            &[77, 79, 227, 193, 49, 19, 14, 138, 93, 226, 88, 194, 28, 113, 68, 6, 245, 59, 238, 34, 154, 12, 142, 124, 70, 101, 45, 225, 118, 112, 152, 207, 206, 137, 135, 75, 81, 161, 190, 133, 107, 182, 151, 188, 244, 2, 110, 237, 145, 121, 83, 11, 212, 180, 169, 179, 209, 159, 221, 246, 1, 164, 13, 3, 203, 129, 177, 106, 86, 254, 198, 8, 144, 149, 80, 56, 185, 158, 131, 248, 192, 216, 247, 219, 141, 132, 215, 239, 147, 187, 211, 25, 184, 27, 208, 30, 139, 63, 153, 20, 43, 33, 89, 85, 66, 168, 92, 103, 224, 90, 157, 116, 126, 38, 123, 223, 50, 32, 0, 199, 117, 54, 146, 150, 95, 181, 214, 195, 251, 136, 175, 210, 233, 202, 120, 240, 166, 58, 24, 148, 52, 140, 170, 172, 48, 5, 196, 243, 98, 205, 252, 53, 10, 26, 249, 100, 253, 204, 102, 74, 156, 99, 155, 41, 183, 29, 241, 84, 235, 231, 71, 234, 189, 134, 165, 17, 78, 109, 197, 213, 72, 67, 73, 62, 9, 7, 222, 114, 236, 64, 111, 163, 23, 22, 125, 218, 91, 122, 37, 18, 55, 217, 69, 143, 230, 44, 35, 105, 36, 201, 60, 4, 115, 160, 97, 130, 228, 171, 39, 51, 220, 16, 174, 173, 127, 42, 96, 46, 104, 21, 255, 229, 82, 167, 76, 186, 119, 65, 232, 242, 57, 176, 191, 128, 94, 108, 31, 162, 200, 47, 40, 15, 87, 250, 178, 61],
                            202);
    let mut r3 = Rotor::new(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255],
                            &[91, 55, 231, 159, 93, 52, 17, 26, 47, 50, 172, 16, 225, 162, 175, 218, 235, 97, 164, 61, 194, 66, 180, 82, 83, 12, 84, 76, 39, 28, 219, 6, 136, 154, 146, 51, 15, 100, 212, 14, 38, 103, 133, 110, 88, 46, 41, 230, 182, 206, 107, 9, 178, 43, 11, 72, 191, 45, 141, 29, 240, 59, 99, 32, 123, 132, 95, 161, 144, 208, 207, 108, 138, 233, 53, 30, 200, 186, 244, 205, 223, 7, 48, 243, 238, 21, 23, 252, 152, 179, 192, 75, 255, 22, 122, 204, 90, 92, 156, 114, 242, 27, 49, 74, 33, 3, 19, 201, 214, 227, 87, 2, 190, 197, 1, 98, 128, 0, 71, 101, 44, 142, 160, 216, 119, 56, 135, 228, 251, 236, 134, 35, 226, 67, 127, 118, 89, 34, 168, 139, 213, 121, 237, 217, 158, 174, 247, 109, 232, 113, 31, 185, 143, 147, 246, 60, 166, 63, 64, 145, 239, 126, 234, 129, 58, 4, 181, 137, 65, 211, 203, 57, 149, 96, 130, 153, 199, 150, 42, 249, 221, 198, 85, 245, 40, 117, 125, 151, 8, 254, 170, 78, 253, 18, 111, 229, 62, 112, 5, 187, 165, 173, 169, 157, 202, 209, 79, 106, 140, 115, 248, 25, 13, 102, 188, 131, 241, 167, 86, 36, 163, 77, 54, 183, 176, 10, 171, 94, 220, 148, 20, 73, 184, 116, 189, 69, 68, 215, 250, 37, 24, 70, 210, 81, 177, 104, 193, 195, 224, 124, 196, 80, 222, 105, 155, 120],
                            30);
    //let mut ufw_B = Rotor::new(&[b'A',b'B',b'C',b'D',b'E',b'F',b'G',b'H',b'I',b'J',b'K',b'L',b'M',b'N',b'O',b'P',b'Q',b'R',b'S',b'T',b'U',b'V',b'W',b'X',b'Y',b'Z'],
    //                           &[b'Y',b'R',b'U',b'H',b'Q',b'S',b'L',b'D',b'P',b'X',b'N',b'G',b'O',b'K',b'M',b'I',b'E',b'B',b'F',b'Z',b'C',b'W',b'V',b'J',b'A',b'T'],
    //                           0);
    //let mut r1 = Rotor::new(&[b'A',b'B',b'C',b'D',b'E',b'F',b'G',b'H',b'I',b'J',b'K',b'L',b'M',b'N',b'O',b'P',b'Q',b'R',b'S',b'T',b'U',b'V',b'W',b'X',b'Y',b'Z'],
    //                        &[b'E',b'K',b'M',b'F',b'L',b'G',b'D',b'Q',b'V',b'Z',b'N',b'T',b'O',b'W',b'Y',b'H',b'X',b'U',b'S',b'P',b'A',b'I',b'B',b'R',b'C',b'J'],
    //                        16);
    //let mut r2 = Rotor::new(&[b'A',b'B',b'C',b'D',b'E',b'F',b'G',b'H',b'I',b'J',b'K',b'L',b'M',b'N',b'O',b'P',b'Q',b'R',b'S',b'T',b'U',b'V',b'W',b'X',b'Y',b'Z'],
    //                        &[b'A',b'J',b'D',b'K',b'S',b'I',b'R',b'U',b'X',b'B',b'L',b'H',b'W',b'T',b'M',b'C',b'Q',b'G',b'Z',b'N',b'P',b'Y',b'F',b'V',b'O',b'E'],
    //                        4);
    //let mut r3 = Rotor::new(&[b'A',b'B',b'C',b'D',b'E',b'F',b'G',b'H',b'I',b'J',b'K',b'L',b'M',b'N',b'O',b'P',b'Q',b'R',b'S',b'T',b'U',b'V',b'W',b'X',b'Y',b'Z'],
    //                        &[b'B',b'D',b'F',b'H',b'J',b'L',b'C',b'P',b'R',b'T',b'X',b'V',b'Z',b'N',b'Y',b'E',b'I',b'W',b'G',b'A',b'K',b'M',b'U',b'S',b'Q',b'O'],
    //                        21);

    let mut enigma = Enigma::new(ufw_B,r2,r1,r3, plugboard);
    let plaintext = "But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system, and expound the actual teachings of the great explorer of the truth, the master-builder of human happiness. No one rejects, dislikes, or avoids pleasure itself, because it is pleasure, but because those who do not know how to pursue pleasure rationally encounter consequences that are extremely painful. Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain, but because occasionally circumstances occur in which toil and pain can procure him some great pleasure. To take a trivial example, which of us ever undertakes laborious physical exercise, except to obtain some advantage from it? But who has any right to find fault with a man who chooses to enjoy a pleasure that has no annoying consequences, or one who avoids a pain that produces no resultant pleasure?";
    let mut ciphertext = "".to_string();
    let mut decrypted = "".to_string();
    enigma.rotorSettings(15,120,51);
    for c in plaintext.chars() {
        let i = enigma.encrypt(c as u8) as u8;
        ciphertext.push(i as char);
    }
    println!("{:?}", ciphertext);
    enigma.rotorSettings(15,120,51);
    for c in ciphertext.chars() {
        let i = enigma.encrypt(c as u8) as u8;
        decrypted.push(i as char);
    }
    println!("{:?}", decrypted);
    for j in 0..=255 {
        for k in 0..=255 {
            enigma.rotorSettings(0,j,k);
            let mut decrypted = "".to_string();
            for c in ciphertext.chars() {
                decrypted.push(enigma.encrypt(c as u8));
            }
            let f = fitness(&decrypted, &plaintext.to_string());
            if f > 10 {
                println!("{} {} Fitness: {}",j,k,f);
            }
        }
    }
}

fn fitness(s: &String, known_plaintext: &String) -> u64 {
    let mut counter = 0u64;
    for it in s.chars().zip(known_plaintext.chars()) {
        let (a,b) = it;
        if a == b {
            counter += 1; 
        }
    }
    counter
}

