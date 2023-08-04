pub mod rotor;
use enigma::rotor::Rotor;
pub mod plugboard;
use enigma::plugboard::Plugboard;

use std::collections::HashMap;

// Actual Machine
pub struct Enigma {
    pub slowRotor: Rotor,
    pub midRotor: Rotor,
    pub fastRotor: Rotor,
    pub plugboard: Plugboard,
    pub reflector: HashMap<u8,u8>,
}

impl Enigma {
    pub fn new(reflectorRotor: Rotor, slowRotor: Rotor, midRotor: Rotor, fastRotor: Rotor, plugboard: Plugboard) -> Enigma {
        Enigma {
            slowRotor,
            midRotor,
            fastRotor,
            plugboard,
            reflector: reflectorRotor.map,
        }
    }


    pub fn step(&mut self) {
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
    pub fn encrypt(&mut self, mut input: u8) -> char {
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
    pub fn rotorSettings(&mut self, pos1: u8, pos2: u8, pos3: u8) {
        self.slowRotor.pos = pos1;
        self.midRotor.pos = pos2;
        self.fastRotor.pos = pos3;
    }

    // Change the settings of the plugboard
    pub fn setPlugboard(&mut self, newPlugBoard: Plugboard) {
        self.plugboard = newPlugBoard;
    }
}

