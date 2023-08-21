import java.util.HashMap;

// ************************************************
// ****** JAVA Skeleton Code for u8 Enigma Machine
// ************************************************

// Team RaisinGang
// Kwok Keith
// Jon Koo Jia Jun
// John-David Tan Ming Sheng
// Joel Sng Kiat Loong
// Jiang Hong Bei

// Created for SUTD: 50.042 CTF Challenge 2023
// Have fun :)


// Attributes of the Rotor class
class Rotor {
    private byte[] orig;
    private HashMap<Byte, Byte> map;
    private HashMap<Byte, Byte> imap;
    private int n;
    private int pos;
    private int notch;

    // Constructor for Rotor class
    public Rotor(byte[] orig, byte[] mapped, int notch) {
        // TODO: initialize map and imap
        // TODO: populate the hashmaps with the array ccontents
        // TODO: initialize position of the rotor to 0
        // TODO: set the notch position
        // TODO: set the size of the charset n to the size of the rotor
    }

    public HashMap<Byte, Byte> getMap() {
        return map;
    }
    public HashMap<Byte, Byte> getImap() {
        return imap;
    }
    public int getN() {
        return n;
    }
    public int getNotch() {
        return notch;
    }
    public byte[] getOrig() {
        return orig;
    }
    public int getPos() {
        return pos;
    }
    public void setImap(HashMap<Byte, Byte> imap) {
        this.imap = imap;
    }
    public void setMap(HashMap<Byte, Byte> map) {
        this.map = map;
    }
    public void setN(int n) {
        this.n = n;
    }
    public void setNotch(int notch) {
        this.notch = notch;
    }
    public void setOrig(byte[] orig) {
        this.orig = orig;
    }
    public void setPos(int pos) {
        this.pos = pos;
    }

    // Rotate the rotor/wheel by one step
    public void step() {
        // TODO: Implement this method
    }

    // Rotate the rotor/wheel by `size` steps
    public void stepN(int size) {
        // TODO: Implement this method (even though it is not being used, it helps with debugging)
    }

    // Direct mapping of rotor internal mapping (Orig Input to Rotor Input)
    public char encipher(byte input) {
        // TODO: Implement this method
    }

    // Scrambler does not use letter mapping, it will make use of the concept of positioning instead.
    // This function enciphers using the position.
    // NOTE: The letter mapping will only be done by obtaining the final position
    //  of the input signal to the stator.
    public char encipherPos(byte input) {
        // TODO: Get the index position of input within the original array
        // TODO: Add offset of the rotor/wheel position to this index 
        // TODO: Use the hashmaps to obtain the mapped char 
        // TODO: Get the index position of the mapped char
        // TODO: Subtract offset of the rotor/wheel position from the index 
        // TODO: Return the final output char based on the index
    }

    // Direct mapping of rotor internal mapping (Rotor output to Stator Input)
    // Gets the final mapped value.
    public char decipher(byte input) {
        // TODO: Implement this method
    }

    // Scrambler does not use letter mapping, it will make use of the concept of positioning instead.
    // This function deciphers using the position.
    // NOTE: The letter mapping will only be done by obtaining the final position
    //  of the input signal to the stator.
    public char decipherPos(byte input) {
        // TODO: Implement this method, logic is similar to encipherPos
    }
}

// Plugboard class
class Plugboard {
    private HashMap<Byte, Byte> map;

    // Constructor for Plugboard class
    public Plugboard(byte[][] mapping) {
        // TODO: initialize the map
        // TODO: populate the map based on the mapping input 
    }

    // Puts signal through the plugboard and gets the new value
    public byte encipher(byte input) {
        // TODO: Implement this method
    }
}

// Enigma class
class Enigma {
    private Rotor slowRotor;
    private Rotor midRotor;
    private Rotor fastRotor;
    private Plugboard plugboard;
    private HashMap<Byte, Byte> reflector;

    // Constructor for Enigma class
    public Enigma(Rotor reflectorRotor, Rotor slowRotor, Rotor midRotor, Rotor fastRotor, Plugboard plugboard) {
        // TODO: Initialize new Enigma instance with given settings
    }

    // Step the rotor/wheel
    private void step() {

        // TODO: Implement this method to step through all the Rotors based on the notch positions
        // Double Stepping "glitch" happens on Mid rotor, this happens on the real Enigma as well.
        // When mid rotor is at notch and fast rotor steps, the mid rotor will also step.
        // NOTE: There are some edge cases here, (think of what happens mid rotor starts at notch position)
        // if you are feeling bored you may implement them, but they do not affect this CTF Challenge.
        //
        // TODO: Implement Double stepping
        // TODO: Implement Single stepping for mid and slow rotors
        // TODO: Lastly, step the fast rotor
    }

    // Encryption method
    public char encrypt(byte input) {
        // TODO: Step the enigma before doing anything
        // TODO: Go through the plugboard
        // TODO: Go through all three Rotors in order
        // TODO: Go through the reflector
        // TODO: Go through all three Rotors in reverse order
        // TODO: Go through the plugboard again and return the result
    }

    // Starting position of the rotor (Part of Key)
    public void rotorSettings(int pos1, int pos2, int pos3) {
        // TODO: Implement this method to set/reset rotor settings
    }

    // Change the settings of the plugboard
    public void setPlugBoard(Plugboard newPlugBoard){
        // TODO: Implement this method to set/reset plugboard
    }
}
