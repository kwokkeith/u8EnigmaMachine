//#![allow(warnings, unused)]
mod enigma;
use enigma::Enigma;
use enigma::rotor::Rotor;
use enigma::plugboard::Plugboard;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};

fn main() {
    // let min_fitness = 20u64;
    let plugboard = Plugboard::new(&[(b'B', b'U'),(b'`', b'N'),(b']', b'4'),(b'I', b'%'),(b'"', b'f'),(b'}', b'Z'),(b'D', b'+'),(b'A', b'9'),(b'3', b'8'),(b'*', b'2'),]);
    let num_wires = (plugboard.map.len() as u8) / 2;
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
    let mut rotorSettings = (1, 25, 24);
    let mut enigma = Enigma::new(ufw_B, r2, r1, r3, plugboard);

    static plaintext:&str = "Wetterbericht: // Datum: 15. Oktober 1940 // Einsatzort: Sonnenberg // Meldung! Meldung! Hier spricht der Wetterdienst fur den 15. Oktober 1944 im Einsatzgebiet Sonnenberg. // Die Wetterlage fur morgen wird voraussichtlich bedeckt sein, mit starkem Wind aus Osten. Die Temperaturen erreichen ein Maximum von rund 12C, was kuhler als gestern ist. // Es besteht eine hohe Wahrscheinlichkeit fur Niederschlage, mit einer Moglichkeit von Regen wahrend des Nachmittags. Alle Einheiten werden darauf hingewiesen, dass entsprechende Kleidung und Ausrustung fur die geplanten Operationen mitgefuhrt werden mussen. // Sicherheitshinweis: Bei Anderungen der Wetterlage sind die Kommandanten verantwortlich, die notwendigen Massnahmen zum Schutz der Truppen und Ausrustung zu ergreifen. // Das war der Wetterbericht. Bleiben Sie wachsam und passen Sie sich den Wetterbedingungen an. // Weitere Befehle oder Informationen konnen angefordert werden. Das war der Wetterdienst. // Heil Hitler!";

    let ciphertext = setupActualEngima(&enigma, &rotorSettings, plaintext);
    
    // Reset enigma machine
    // Setup engima machine for attack
    enigma.rotorSettings(0, 0, 0);
    let plugboard = Plugboard::new(&[]);
    enigma.setPlugboard(plugboard);


    // ***************************
    // ***** ATTACK 
    // ***************************
    // Make 8 threads AND ATTACK
    //let mut threads : Vec<thread::JoinHandle<_>> = Vec::new();
    // let (sender, receiver) = mpsc::channel();

    // static mut done: [bool;8] = [false;8];
    println!("\nStarting 8 threads...");

    // static mut maximum_fitness: u64 = 0;
    // static mut chosen_rotorConfig: (u8, u8, u8, u64);

    let mut maximum_fitness = Arc::new(Mutex::new(0));
    let mut chosen_rotor_Config = Arc::new(Mutex::new((0 as u8,0 as u8,0 as u8,0 as u64)));
    let mut handles = vec![];
    
    // Create threads
    for idx in 0..8 {
        let ciphertext = ciphertext.clone();
        let maximum_fitness = Arc::clone(&maximum_fitness);
        let chosen_rotor_Config = Arc::clone(&chosen_rotor_Config);

        let handle = thread::spawn(move || {
            println!("Thread {idx} running");

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
                        enigma.rotorSettings(r,j,k);
                        let mut decrypted = "".to_string();
                        for c in cptext.chars() {
                            decrypted.push(enigma.encrypt(c as u8));
                        }
                        let f = fitness(&decrypted, &plaintext.to_string());
                        let mut max = maximum_fitness.lock().unwrap();
                            if f > *max {
                                *max = f;
                                let mut chosenRotor = chosen_rotor_Config.lock().unwrap();
                                *chosenRotor = (r, j, k, f);
                                // sender.send((idx,r,j,k,f,decrypted)).unwrap();
                                // println!("{:?}",decrypted);
                            }
                        }
                    }
                }
                println!("Thread {idx} completed");
            });
            handles.push(handle);
        
    }

    for handle in handles {
        handle.join().unwrap();
    }
    
    let chosen_rotorConfig = *chosen_rotor_Config.lock().unwrap();
    
    let mut decrypted: String = String::new();
    let mut current_fitness: u64 = 0; // the current fitness of the text before plugboard with the correct rotor settings
    
    let (r, j, k, f) = chosen_rotorConfig;
    current_fitness = f;
    enigma.rotorSettings(r,j,k);
    let mut decrypted = "".to_string();
    let cptext = unsafe{ &*ciphertext }; 
    for c in cptext.chars() {
        decrypted.push(enigma.encrypt(c as u8));
    }

    // for received in &receiver {
    //     let (i,r,j, k, f, d) = received;
    //     decrypted = d;
    //     println!("Thread {} found: {} {} {} Fitness: {}",i,r,j,k,f);
        
    //     chosen_rotorConfig = (r, j, k, f);
    //     current_fitness = f;

    //     unsafe { if(done) {break;} }
    // }

    println!("Plugboard: {:?}", enigma.plugboard.map);
    println!("decrypted before plugboard: {}", decrypted);
    println!("Chosen rotor configuration {:?}", chosen_rotorConfig);
    

    // ******************************
    // Find the plugboard settings
    // ******************************

    let mut chosen_wire_positions: Vec<(u8, u8)> = Vec::new(); 
    let mut wire_idx = 0u8;

    let (cr, cj, ck, cf) = chosen_rotorConfig;
    enigma.rotorSettings(cr, cj, ck);

    let mut printables: Vec<u8> = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'!', b'"', b'#', b'$', b'%', b'&', b' ', b'(', b')', b'*', b'+', b',', b'-', b'.', b'/', b':', b';', b'<', b'=', b'>', b'?', 
    b'@', b'[', b'\\', b']', b'^', b'_', b'`', b'{', b'|', b'}', b'~'].to_vec();

    // Compare the cipher and plaintext based on the OPTIMUM rotor configuration
    for it in plaintext.chars().zip(decrypted.chars()) {
        if wire_idx >= num_wires {
            break;
        }
        let (a,b) = it;
        if a != b {
            // Try to connect cipher to known plaintext
            if printables.contains(&(a as u8)) && printables.contains(&(b as u8)) {
                // Push the wire position to be tested for fitness
                chosen_wire_positions.push((a as u8, b as u8));

                // Create a new enigma machine instance to test fitness of this plugboard setting
                let plugboardguessed = Plugboard::new(&(chosen_wire_positions.as_slice()));
                enigma.setPlugboard(plugboardguessed);  

                // Get the new decrypted with this setting
                let mut decrypted = "".to_string();
                unsafe{
                    for c in ciphertext.chars() {
                        decrypted.push(enigma.encrypt(c as u8));
                    }
                }
                enigma.rotorSettings(cr, cj, ck);

                let f = fitness(&decrypted, &plaintext.to_string());

                // Check the fitness of the text
                if f > current_fitness + 10 {
                    println!("Current Fitness {} to {}", current_fitness, f);
                    // Possibly a correct wire connection
                    current_fitness = f;

                    // Drop the printables 
                    printables.retain(|x| *x != a as u8);
                    printables.retain(|x| *x != b as u8);
                    println!("{} {}", a, b);
                    
                    // Increment number of wires
                    wire_idx += 1;
                }
                else {
                    // probably not a correct wire connection
                    chosen_wire_positions.pop();
                }
            }
        }
        else{
            // same mapping, found a slot that is not mapped
            printables.retain(|x| *x != a as u8)
        }
    }


    // DO 1 wire at the time and find the best fitness
    // using position instead of mapping from cipher to plaintext
    while wire_idx < num_wires {
        let mut maximum = 0 as u8;
        let mut chosen_position = (0, 0);

        for s in 0..(printables.len()-1) { // plug first wire
            for t in s+1..printables.len() { // plug second wire
                // Create a new enigma machine instance to test fitness of this plugboard setting
                chosen_wire_positions.push((printables[s] as u8, printables[t] as u8));

                let plugboardguessed = Plugboard::new(chosen_wire_positions.as_slice());
                enigma.setPlugboard(plugboardguessed);            

                let mut decrypted = "".to_string();
                unsafe{
                    for c in ciphertext.chars() {
                        decrypted.push(enigma.encrypt(c as u8));
                    }
                }
                let f = fitness(&decrypted, &plaintext.to_string()) as u8;
                
                // if fitness f is better than use this configuration
                if maximum < f {
                    maximum = f;
                    chosen_position = (printables[s], printables[t]);
                    // println!("#{} wire connect between {} & {} with fitness {}", wire_idx, printables[s] as char, printables[t] as char, maximum)
                }

                // Remove the current wiring setting that we are trying
                chosen_wire_positions.pop();
                
                // Reset rotor settings
                enigma.rotorSettings(cr, cj, ck);
            }
        }

        // remove the available wire positions
        // Drop the printables 
        let (s, t) = chosen_position;
        
        printables.retain(|x| *x != s as u8);
        printables.retain(|x| *x != t as u8);
        println!("{} {}", s as char, t as char);

        chosen_wire_positions.push((s, t));

        // Iterate counter
        wire_idx += 1;
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
    assert_eq!(plaintext,decrypted);
}

fn setupActualEngima(enigma: &Enigma, 
    rotorSettings: &(u8, u8, u8), plaintext: &str) -> String{
        println!("Creating Actual Enigma Machine...");

        let mut ciphertext:String = String::new();

        let (pos1, pos2, pos3) = rotorSettings;
        enigma.rotorSettings(*pos1, *pos2, *pos3);

        for c in plaintext.chars() {
            let i = enigma.encrypt(c as u8) as u8;
            ciphertext.push(i as char)
        }

        println!("Ciphertext generated: {:?}", ciphertext);

        let mut decrypted:String = String::new();
        enigma.rotorSettings(*pos1, *pos2, *pos3);
        for c in ciphertext.chars() {
            let i = enigma.encrypt(c as u8) as u8;
            decrypted.push(i as char);
        }

        println!("Decrypted Ciphertext: {:?}", decrypted);

        return ciphertext
    }


fn fitness(s: &String, known_plaintext: &String) -> u64 {
    let mut counter = 0u64;
    for it in s.chars().zip(known_plaintext.chars()) {
        let (a,b) = it;
        if a == b {
            counter += 1; 
        }
    }
    return counter;
}
