#![allow(warnings, unused)]
use std::collections::HashMap;
use std::collections::HashSet;
//use std::thread;
use std::{thread, time};
use std::sync::mpsc;
use std::{thread, time};

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
}

// Implementation of the Rotor Struct methods
impl Plugboard {
    fn new(mapping: &[(u8, u8)]) -> Plugboard {
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
    fn encipher(&self, input: u8) -> u8 {
        // If no connection between two keys (wire), MAP back to same value
        if !self.map.contains_key(&input) {
            return input;
        }
        return self.map[&input];
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
        input = self.plugboard.encipher(input);
        return input as char;
    }


    // Starting position of the rotor (Part of Key)
    fn rotorSettings(&mut self, pos1: u8, pos2: u8, pos3: u8) {
        self.slowRotor.pos = pos1;
        self.midRotor.pos = pos2;
        self.fastRotor.pos = pos3;
    }

    // Change the settings of the plugboard
    fn setPlugboard(&mut self, newPlugBoard: Plugboard) {
        self.plugboard = newPlugBoard;
    }
}


fn main() {
    let plugboard = Plugboard::new(&[(b'a', b'u'),(b'9', b'T'),(b'Y', b'='),(b'3', b'y'),(b';', b'"'),(b't', b'#'),(b'f', b'r'),(b'C', b'_'),(b'i', b'%'),(b'/', b'$')]);
    
    let mut ufw_B = Rotor::new(&[b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'!', b'"', b'#', b'$', b'%', b'&', b' ', b'(', b')', b'*', b'+', b',', b'-', b'.', b'/', b':', b';', b'<', b'=', b'>', b'?', b'@', b'[', b'\\', b']', b'^', b'_', b'`', b'{', b'|', b'}', b'~'],
                            &[b'w', b'z', b'u', b'L', b'?', b's', b'"', b'8', b'7', b'M', b'J', b'F', b'd', b'c', b'/', b':', b'#', b'$', b'>', b'q', b'o', b'|', b'@', b',', b'k', b'B', b'j', b' ', b'5', b'R', b'2', b'G', b'0', b'}', b'T', b'1', b'{', b'p', b'N', b'Q', b'K', b'b', b'v', b'O', b')', b'a', b'E', b'3', b'9', b'C', b'H', b'+', b'D', b't', b'^', b'y', b'Z', b'_', b';', b'`', b'!', b'U', b'Y', b'6', b'g', b'h', b'&', b'%', b'r', b'=', b'I', b'.', b'P', b'n', b'<', b'*', b'e', b'f', b'W', b'-', b'(', b'i', b'4', b'm', b']', b'~', b'[', b'S', b'V', b'X', b'A', b'l', b'x', b'\\'],
                            0);
    let mut r1 = Rotor::new(&[b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'!', b'"', b'#', b'$', b'%', b'&', b' ', b'(', b')', b'*', b'+', b',', b'-', b'.', b'/', b':', b';', b'<', b'=', b'>', b'?', b'@', b'[', b'\\', b']', b'^', b'_', b'`', b'{', b'|', b'}', b'~'],
                            &[b':', b'-', b'l', b'e', b'r', b'_', b'$', b't', b'J', b'B', b';', b'w', b'F', b'P', b'h', b'.', b'U', b' ', b'G', b'9', b')', b'2', b'4', b'f', b'=', b'j', b'v', b'>', b'+', b'0', b'x', b'Q', b'@', b'W', b'o', b'(', b'L', b'8', b'`', b'D', b'Y', b'k', b'g', b'I', b'E', b'V', b'Z', b'<', b'%', b'7', b'"', b'm', b'/', b'y', b'^', b',', b'#', b'C', b'O', b'b', b'}', b'3', b'?', b'!', b'\\', b'd', b']', b'*', b'H', b'|', b'i', b'z', b'&', b'n', b'5', b'A', b'T', b'6', b'c', b'[', b'p', b'~', b'K', b'X', b'{', b'a', b'u', b'N', b'1', b'R', b'q', b'M', b'S', b's'],
                            12);
    let mut r2 = Rotor::new(&[b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'!', b'"', b'#', b'$', b'%', b'&', b' ', b'(', b')', b'*', b'+', b',', b'-', b'.', b'/', b':', b';', b'<', b'=', b'>', b'?', b'@', b'[', b'\\', b']', b'^', b'_', b'`', b'{', b'|', b'}', b'~'],
                            &[b'{', b'/', b'1', b'^', b'V', b':', b'_', b'a', b'9', b'e', b'd', b'b', b'2', b'0', b'y', b'&', b'[', b';', b'w', b'k', b'j', b'P', b'*', b'M', b'D', b'Q', b'u', b'~', b'B', b'%', b'R', b'q', b'<', b'C', b'v', b'|', b'5', b'W', b't', b'p', b')', b'Y', b'-', b'4', b'i', b'.', b'H', b',', b'K', b'A', b'S', b'"', b'x', b'6', b'=', b'}', b'3', b'E', b'7', b'N', b'!', b'F', b'm', b'T', b'J', b'?', b'#', b'+', b'c', b'r', b'z', b'U', b'L', b'Z', b']', b'$', b'h', b'n', b'\\', b'g', b'o', b'>', b'@', b'l', b's', b' ', b'f', b'I', b'8', b'(', b'O', b'G', b'X', b'`'],
                            14);
            let mut r3 = Rotor::new(&[b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'!', b'"', b'#', b'$', b'%', b'&', b' ', b'(', b')', b'*', b'+', b',', b'-', b'.', b'/', b':', b';', b'<', b'=', b'>', b'?', b'@', b'[', b'\\', b']', b'^', b'_', b'`', b'{', b'|', b'}', b'~'],
            &[b'T', b'y', b'l', b':', b'p', b'2', b'>', b'E', b'o', b'Q', b'+', b'f', b' ', b'!', b'(', b'M', b'L', b's', b'J', b'&', b'P', b'r', b'U', b'$', b'.', b'Y', b'|', b']', b'C', b';', b'^', b'W', b'9', b'?', b'{', b'D', b'n', b'%', b'@', b'V', b'I', b'7', b'B', b'N', b'u', b'6', b'~', b'"', b'5', b'q', b'c', b'K', b'0', b'4', b'`', b'\\', b'}', b'-', b'e', b'w', b'i', b'h', b'g', b'm', b',', b'b', b'k', b'3', b'X', b'=', b'[', b'j', b'R', b'O', b'_', b'v', b'H', b'd', b'<', b'x', b'#', b'A', b'*', b'z', b'/', b')', b't', b'S', b'a', b'G', b'F', b'1', b'8', b'Z'],
            47);
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
    static plaintext:&str = "Wetterbericht: // Datum: 15. Oktober 1944 // Einsatzort: Sonnenberg // Meldung! Meldung! Hier spricht der Wetterdienst fur den 15. Oktober 1944 im Einsatzgebiet Sonnenberg. // Die Wetterlage fur morgen wird voraussichtlich bedeckt sein, mit starkem Wind aus Osten. Die Temperaturen erreichen ein Maximum von rund 12C, was kuhler als gestern ist. // Es besteht eine hohe Wahrscheinlichkeit fur Niederschlage, mit einer Moglichkeit von Regen wahrend des Nachmittags. Alle Einheiten werden darauf hingewiesen, dass entsprechende Kleidung und Ausrustung fur die geplanten Operationen mitgefuhrt werden mussen. // Sicherheitshinweis: Bei Anderungen der Wetterlage sind die Kommandanten verantwortlich, die notwendigen Massnahmen zum Schutz der Truppen und Ausrustung zu ergreifen. // Das war der Wetterbericht. Bleiben Sie wachsam und passen Sie sich den Wetterbedingungen an. // Weitere Befehle oder Informationen konnen angefordert werden. Das war der Wetterdienst. // Heil Hitler!";
    static mut ciphertext:String = String::new();
  
    enigma.rotorSettings(11,93,93);

    for c in plaintext.chars() {
        let i = enigma.encrypt(c as u8) as u8;
        unsafe{ ciphertext.push(i as char);}
    }
    unsafe{println!("{:?}", ciphertext);}

    let mut decrypted:String = String::new();
    enigma.rotorSettings(11,93,93);

    unsafe {
        for c in ciphertext.chars() {
            let i = enigma.encrypt(c as u8) as u8;
            decrypted.push(i as char);
        }
    }
    println!("{:?}",decrypted);

    // Make 8 threads
    //let mut threads : Vec<thread::JoinHandle<_>> = Vec::new();
    let (sender, receiver) = mpsc::channel();

    static mut done: bool = false;
    println!("\nStarting 8 threads...");

    for idx in 0..8 {
        // static mut index: u8 = 0;
        // unsafe{index += i;}
        let sender = sender.clone();
        let t = thread::spawn(move || {
            let plugboard = Plugboard::new(&[]);
            let mut ufw_B = Rotor::new(&[b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'!', b'"', b'#', b'$', b'%', b'&', b' ', b'(', b')', b'*', b'+', b',', b'-', b'.', b'/', b':', b';', b'<', b'=', b'>', b'?', b'@', b'[', b'\\', b']', b'^', b'_', b'`', b'{', b'|', b'}', b'~'],
            &[b'w', b'z', b'u', b'L', b'?', b's', b'"', b'8', b'7', b'M', b'J', b'F', b'd', b'c', b'/', b':', b'#', b'$', b'>', b'q', b'o', b'|', b'@', b',', b'k', b'B', b'j', b' ', b'5', b'R', b'2', b'G', b'0', b'}', b'T', b'1', b'{', b'p', b'N', b'Q', b'K', b'b', b'v', b'O', b')', b'a', b'E', b'3', b'9', b'C', b'H', b'+', b'D', b't', b'^', b'y', b'Z', b'_', b';', b'`', b'!', b'U', b'Y', b'6', b'g', b'h', b'&', b'%', b'r', b'=', b'I', b'.', b'P', b'n', b'<', b'*', b'e', b'f', b'W', b'-', b'(', b'i', b'4', b'm', b']', b'~', b'[', b'S', b'V', b'X', b'A', b'l', b'x', b'\\'],
            0);
            let mut r1 = Rotor::new(&[b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'!', b'"', b'#', b'$', b'%', b'&', b' ', b'(', b')', b'*', b'+', b',', b'-', b'.', b'/', b':', b';', b'<', b'=', b'>', b'?', b'@', b'[', b'\\', b']', b'^', b'_', b'`', b'{', b'|', b'}', b'~'],
            &[b':', b'-', b'l', b'e', b'r', b'_', b'$', b't', b'J', b'B', b';', b'w', b'F', b'P', b'h', b'.', b'U', b' ', b'G', b'9', b')', b'2', b'4', b'f', b'=', b'j', b'v', b'>', b'+', b'0', b'x', b'Q', b'@', b'W', b'o', b'(', b'L', b'8', b'`', b'D', b'Y', b'k', b'g', b'I', b'E', b'V', b'Z', b'<', b'%', b'7', b'"', b'm', b'/', b'y', b'^', b',', b'#', b'C', b'O', b'b', b'}', b'3', b'?', b'!', b'\\', b'd', b']', b'*', b'H', b'|', b'i', b'z', b'&', b'n', b'5', b'A', b'T', b'6', b'c', b'[', b'p', b'~', b'K', b'X', b'{', b'a', b'u', b'N', b'1', b'R', b'q', b'M', b'S', b's'],
            12);
            let mut r2 = Rotor::new(&[b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'!', b'"', b'#', b'$', b'%', b'&', b' ', b'(', b')', b'*', b'+', b',', b'-', b'.', b'/', b':', b';', b'<', b'=', b'>', b'?', b'@', b'[', b'\\', b']', b'^', b'_', b'`', b'{', b'|', b'}', b'~'],
            &[b'{', b'/', b'1', b'^', b'V', b':', b'_', b'a', b'9', b'e', b'd', b'b', b'2', b'0', b'y', b'&', b'[', b';', b'w', b'k', b'j', b'P', b'*', b'M', b'D', b'Q', b'u', b'~', b'B', b'%', b'R', b'q', b'<', b'C', b'v', b'|', b'5', b'W', b't', b'p', b')', b'Y', b'-', b'4', b'i', b'.', b'H', b',', b'K', b'A', b'S', b'"', b'x', b'6', b'=', b'}', b'3', b'E', b'7', b'N', b'!', b'F', b'm', b'T', b'J', b'?', b'#', b'+', b'c', b'r', b'z', b'U', b'L', b'Z', b']', b'$', b'h', b'n', b'\\', b'g', b'o', b'>', b'@', b'l', b's', b' ', b'f', b'I', b'8', b'(', b'O', b'G', b'X', b'`'],
            14);
            let mut r3 = Rotor::new(&[b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'!', b'"', b'#', b'$', b'%', b'&', b' ', b'(', b')', b'*', b'+', b',', b'-', b'.', b'/', b':', b';', b'<', b'=', b'>', b'?', b'@', b'[', b'\\', b']', b'^', b'_', b'`', b'{', b'|', b'}', b'~'],
            &[b'T', b'y', b'l', b':', b'p', b'2', b'>', b'E', b'o', b'Q', b'+', b'f', b' ', b'!', b'(', b'M', b'L', b's', b'J', b'&', b'P', b'r', b'U', b'$', b'.', b'Y', b'|', b']', b'C', b';', b'^', b'W', b'9', b'?', b'{', b'D', b'n', b'%', b'@', b'V', b'I', b'7', b'B', b'N', b'u', b'6', b'~', b'"', b'5', b'q', b'c', b'K', b'0', b'4', b'`', b'\\', b'}', b'-', b'e', b'w', b'i', b'h', b'g', b'm', b',', b'b', b'k', b'3', b'X', b'=', b'[', b'j', b'R', b'O', b'_', b'v', b'H', b'd', b'<', b'x', b'#', b'A', b'*', b'z', b'/', b')', b't', b'S', b'a', b'G', b'F', b'1', b'8', b'Z'],
            47);

            let cptext = unsafe{ &*ciphertext };
            let ptext = &*plaintext;
            let mut decrypted = "".to_string();
            let mut enigma = Enigma::new(ufw_B,r2,r1,r3, plugboard);
            let start : u8;

            start = idx * 12;
            //println!("I am thread {}", idx.to_string());
            'outer: for mut r in start..start+12 {
                if (r > 93) { r = 93; }
                //println!("{:?}",r);
                for j in 0..94 {
                    for k in 0..94 {
                        unsafe { if (done) {break 'outer;}}
                        enigma.rotorSettings(r,j,k);
                        let mut decrypted = "".to_string();
                        for c in cptext.chars() {
                            decrypted.push(enigma.encrypt(c as u8));
                        }
                        let f = fitness(&decrypted, &plaintext.to_string()) as u8;
                        if f > 100 {
                            sender.send((idx,r,j,k,f)).unwrap();
                            println!("{:?}",decrypted);
                            unsafe { done = true; }
                        }
                    }
                }
            }
            println!("Thread {idx} completed");
        });       
    }

    // Calculate the maximum
    let mut chosen_rotorConfig: (u8, u8, u8, u8) = (0,0,0,0);

    for received in &receiver {
        let (i,r,j, k, f) = received;
        println!("Thread {} found: {} {} {} Fitness: {}",i,r,j,k,f);
        unsafe { if(done) {break;} }
    }


    // use the chosen plugboard wire configuration and create new enigma machine
    let plugboardChosen = Plugboard::new(&chosen_wire_positions);
    enigma.setPlugboard(plugboardChosen);
    let (cr, cj, ck, cf) = chosen_rotorConfig;
    
    // Set rotor settings based on found best fit
    enigma.rotorSettings(cr, cj, ck);

    // Get final decrypted plaintext
    let mut decrypted = "".to_string();
    unsafe{
        for c in ciphertext.chars() {
            decrypted.push(enigma.encrypt(c as u8));
        }
    }
    
    // Print result
    println!("Decrypted plaintext from final configured: ");
    println!("{}", decrypted);
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
