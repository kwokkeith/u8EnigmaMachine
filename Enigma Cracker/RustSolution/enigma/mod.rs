pub mod rotor;
use enigma::rotor::Rotor;
pub mod plugboard;
use enigma::plugboard::Plugboard;

use std::collections::HashMap;

// Actual Machine
pub struct Enigma {
    pub slow_rotor: Rotor,
    pub mid_rotor: Rotor,
    pub fast_rotor: Rotor,
    pub plugboard: Plugboard,
    pub reflector: HashMap<u8,u8>,
}

impl Enigma {
    pub fn new(reflector_rotor: Rotor, slow_rotor: Rotor, mid_rotor: Rotor, fast_rotor: Rotor, plugboard: Plugboard) -> Enigma {
        Enigma {
            slow_rotor,
            mid_rotor,
            fast_rotor,
            plugboard,
            reflector: reflector_rotor.map,
        }
    }


    pub fn step(&mut self) {
        // Double Stepping Mid rotor (ANOMALY FOUND IN THE ORIGINAL ENIGMA)
        // In real machine, as long as mid rotor is at notch and fast rotor starts
        // to move, the mid rotor will step immediately.
        if self.mid_rotor.pos == self.mid_rotor.notch {
            self.slow_rotor.step();
            self.mid_rotor.step();
        }
        // Single Stepping Mid Rotor
        if self.fast_rotor.pos == self.fast_rotor.notch {
            // Single Stepping Slow rotor
            if self.mid_rotor.pos == self.mid_rotor.notch {
                self.slow_rotor.step();
            }
            self.mid_rotor.step();
        }
        // Step Fast rotor
        self.fast_rotor.step();
    }


    // Encryption method
    pub fn encrypt(&mut self, mut input: u8) -> char {
        // Step the rotor/wheel
        self.step();

        // Plugboard
        input = self.plugboard.encipher(input);

        // Input is mutated for the next rotor/wheel
        input = self.fast_rotor.encipher_pos(input) as u8;
        input = self.mid_rotor.encipher_pos(input) as u8;
        input = self.slow_rotor.encipher_pos(input) as u8;
        input = self.reflector[&input];
        input = self.slow_rotor.decipher_pos(input) as u8;
        input = self.mid_rotor.decipher_pos(input) as u8;
        input = self.fast_rotor.decipher_pos(input) as u8;

        // Plugboard
        input = self.plugboard.encipher(input);
        return input as char;
    }


    // Starting position of the rotor (Part of Key)
    pub fn rotor_settings(&mut self, pos1: u8, pos2: u8, pos3: u8) {
        self.slow_rotor.pos = pos1;
        self.mid_rotor.pos = pos2;
        self.fast_rotor.pos = pos3;
    }

    // Change the settings of the plugboard
    pub fn set_plugboard(&mut self, new_plug_board: Plugboard) {
        self.plugboard = new_plug_board;
    }
}


