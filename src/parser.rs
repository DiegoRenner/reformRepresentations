
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

use crate::atom::*;

const NAME_PREFIX_LEN: usize = 4;
pub const VAR_START_SIG: u8 = 0;
pub const POW_START_SIG: u8 = 1;
pub const POW_END_SIG: u8 = 2;
pub const FN_START_SIG: u8 = 3;
pub const FN_END_SIG: u8 = 4;
pub const TERM_START_SIG: u8 = 5;
pub const TERM_END_SIG: u8 = 6;
pub const NUM_START_SIG: u8 = 7;


// convert an atom from an enum to a string of bits
pub fn enum_to_strofbits(atom: Atom) -> String {
    let mut output: String = "".to_string();
    // depending on the type of atom add corresponding signature
    // and the content of the atom
    // recursively call the function for atoms that contain atoms
    if let Atom::Var(name, exponential) = atom.clone() {
        //println!("This atom is a variable with name {} and exponential {}", name, exponential);
        output.push_str("00000000");
        let name_int: usize = (name[NAME_PREFIX_LEN..]).parse::<usize>().unwrap();
        let name_str: String = format!("{name_int:08b}");
        output.push_str(&name_str);
        output.push_str(&format!("{exponential:032b}"))
    } else if let Atom::Pow(flag, box_of_atoms) = atom.clone() {
        //println!("This atom is a power with flag {} and box of atoms {:#?}", flag, box_of_atoms);
        output.push_str("00000001");
        let flag_int: usize = flag as usize;
        let flag_str: String = format!("{flag_int:08b}");
        output.push_str(&flag_str);
        output.push_str(&enum_to_strofbits((*box_of_atoms).0));
        output.push_str(&enum_to_strofbits((*box_of_atoms).1));
        output.push_str("00000010");
    } else if let Atom::Fn(flag, name, vect_of_atoms) = atom.clone() {
        //println!("This atom is a function with flag {}, name {}, and vector of atoms {:#?}", flag, name, vect_of_atoms);
        output.push_str("00000011");
        let flag_int: usize = flag as usize;
        let flag_str: String = format!("{flag_int:08b}");
        let name_int: usize = (name[NAME_PREFIX_LEN..]).parse::<usize>().unwrap();
        let name_str: String = format!("{name_int:08b}");
        output.push_str(&flag_str);
        output.push_str(&name_str);
        for a in vect_of_atoms {
            output.push_str(&enum_to_strofbits(a));
        }
        output.push_str("00000100");
    } else if let Atom::Term(flag, vect_of_atoms) = atom.clone() {
        //println!("This atom is a term with flag {} and vector of atoms {:#?}", flag, vect_of_atoms);
        output.push_str("00000101");
        let flag_int: usize = flag as usize;
        let flag_str: String = format!("{flag_int:08b}");
        output.push_str(&flag_str);
        for a in vect_of_atoms {
            output.push_str(&enum_to_strofbits(a));
        }
        output.push_str("00000110");
    } else if let Atom::Num(flag, value) = atom.clone() {
        //println!("This atom is a number with flag {} and value {}", flag.to_string(), value.to_string());
        output.push_str("00000111");
        let flag_int: usize = flag as usize;
        let flag_str: String = format!("{flag_int:08b}");
        output.push_str(&flag_str);
        output.push_str(&format!("{value:032b}"))
    }
    return output;
}

// convert an atom from an enum to a vector of bits
pub fn enum_to_vecofbits(atom: Atom) -> Vec<u8> {
    let mut output: Vec<u8> = vec![];
    // depending on the type of atom add corresponding signature
    // and the content of the atom
    // recursively call the function for atoms that contain atoms
    if let Atom::Var(name, exponential) = atom.clone() {
        //println!("This atom is a variable with name {} and exponential {}", name, exponential);
        output.write_u8(VAR_START_SIG);
        let name_int: u8 = (name[NAME_PREFIX_LEN..]).parse::<u8>().unwrap();
        output.write_u8(name_int);
        output.write_u32::<LittleEndian>(exponential);
    } else if let Atom::Pow(flag, box_of_atoms) = atom.clone() {
        //println!("This atom is a power with flag {} and box of atoms {:#?}", flag, box_of_atoms);
        output.write_u8(POW_START_SIG);
        let flag_int: u8 = flag as u8;
        output.write_u8(flag_int);
        output.append(&mut enum_to_vecofbits((*box_of_atoms).0));
        output.append(&mut enum_to_vecofbits((*box_of_atoms).1));
        output.write_u8(POW_END_SIG);
    } else if let Atom::Fn(flag, name, vect_of_atoms) = atom.clone() {
        //println!("This atom is a function with flag {}, name {}, and vector of atoms {:#?}", flag, name, vect_of_atoms);
        output.write_u8(FN_START_SIG);
        let flag_int: u8 = flag as u8;
        output.write_u8(flag_int);
        let name_int: u8 = (name[NAME_PREFIX_LEN..]).parse::<u8>().unwrap();
        output.write_u8(name_int);
        for a in vect_of_atoms {
            output.append(&mut enum_to_vecofbits(a));
        }
        output.write_u8(FN_END_SIG);
    } else if let Atom::Term(flag, vect_of_atoms) = atom.clone() {
        //println!("This atom is a term with flag {} and vector of atoms {:#?}", flag, vect_of_atoms);
        output.write_u8(TERM_START_SIG);
        let flag_int: u8 = flag as u8;
        output.write_u8(flag_int);
        for a in vect_of_atoms {
            output.append(&mut enum_to_vecofbits(a));
        }
        output.write_u8(TERM_END_SIG);
    } else if let Atom::Num(flag, value) = atom.clone() {
        //println!("This atom is a number with flag {} and value {}", flag.to_string(), value.to_string());
        output.write_u8(NUM_START_SIG);
        let flag_int: u8 = flag as u8;
        output.write_u8(flag_int);
        output.write_u32::<LittleEndian>(value);
    }
    return output;
}

// convert an atom from an enum to a vector of bits
pub fn enum_to_vecofbits_size(atom: Atom) -> (u32, Vec<u8>) {
    let mut output: Vec<u8> = vec![];
    let mut size: u32 = 0;
    // depending on the type of atom add corresponding signature
    // and the content of the atom
    // recursively call the function for atoms that contain atoms
    if let Atom::Var(name, exponential) = atom.clone() {
        //println!("This atom is a variable with name {} and exponential {}", name, exponential);
        output.write_u8(VAR_START_SIG);
        let name_int: u8 = (name[NAME_PREFIX_LEN..]).parse::<u8>().unwrap();
        output.write_u8(name_int);
        output.write_u32::<LittleEndian>(exponential);
        size = 1;
    } else if let Atom::Pow(flag, box_of_atoms) = atom.clone() {
        //println!("This atom is a power with flag {} and box of atoms {:#?}", flag, box_of_atoms);
        output.write_u8(POW_START_SIG);
        let flag_int: u8 = flag as u8;
        output.write_u8(flag_int);
        let (temp_size1, mut temp_output1) = enum_to_vecofbits_size((*box_of_atoms).0);
        let (temp_size2, mut temp_output2) = enum_to_vecofbits_size((*box_of_atoms).1);
        size = temp_size1 + temp_size2 + 1;
        output.write_u32::<LittleEndian>(size);
        output.append(&mut temp_output1);
        output.append(&mut temp_output2);
    } else if let Atom::Fn(flag, name, vect_of_atoms) = atom.clone() {
        //println!("This atom is a function with flag {}, name {}, and vector of atoms {:#?}", flag, name, vect_of_atoms);
        output.write_u8(FN_START_SIG);
        let flag_int: u8 = flag as u8;
        output.write_u8(flag_int);
        let name_int: u8 = (name[NAME_PREFIX_LEN..]).parse::<u8>().unwrap();
        output.write_u8(name_int);
        let mut temp_output_vec: Vec<u8> = vec![];
        for a in vect_of_atoms {
            let (temp_size, mut temp_output) = enum_to_vecofbits_size(a);
            temp_output_vec.append(&mut temp_output);
            size += temp_size;
        }
        size += 1;
        output.write_u32::<LittleEndian>(size);
        output.append(&mut temp_output_vec);
    } else if let Atom::Term(flag, vect_of_atoms) = atom.clone() {
        //println!("This atom is a term with flag {} and vector of atoms {:#?}", flag, vect_of_atoms);
        output.write_u8(TERM_START_SIG);
        let flag_int: u8 = flag as u8;
        output.write_u8(flag_int);
        let mut temp_output_vec: Vec<u8> = vec![];
        for a in vect_of_atoms {
            let (temp_size, mut temp_output) = enum_to_vecofbits_size(a);
            temp_output_vec.append(&mut temp_output);
            size += temp_size;
        }
        size += 1;
        output.write_u32::<LittleEndian>(size);
        output.append(&mut temp_output_vec);
    } else if let Atom::Num(flag, value) = atom.clone() {
        //println!("This atom is a number with flag {} and value {}", flag.to_string(), value.to_string());
        output.write_u8(NUM_START_SIG);
        let flag_int: u8 = flag as u8;
        output.write_u8(flag_int);
        output.write_u32::<LittleEndian>(value);
        size = 1;
    }
    return (size, output);
}

// convert a vector of atoms from an enum to a vector of bits
pub fn vecofenums_to_vecofbits_size(atoms: Vec<Atom>) -> (u32, Vec<u8>) {
    let mut output_all: Vec<u8> = vec![];
    let mut size_all: u32 = 0;
    for atom in atoms {
        let (size, mut output) = enum_to_vecofbits_size(atom);
        output_all.append(&mut output);
        size_all += size
    }
    (size_all, output_all)

}

// convert an atom from a vector of bits to an enum
pub fn vecofbits_to_enum_size(reader: &mut Cursor<Vec<u8>>) -> (u32, Atom) {
    let mut output: Atom = Atom::Num(false, 0);
    let mut size: u32 = 0;
    // depending on the type of atom add corresponding signature
    // and the content of the atom
    // recursively call the function for atoms that contain atoms
    //println!("{}",reader.read_u8().unwrap()==3);
    let sig = reader.read_u8().unwrap();
    //println!("{}", sig);
    //println!("{:#?}",reader);
    if sig == VAR_START_SIG {
        size = 1;
        output = Atom::Var( format!("var_{}", reader.read_u8().unwrap()), reader.read_u32::<LittleEndian>().unwrap());
    } else if sig == POW_START_SIG {
        let flag = reader.read_u8().unwrap() != 0;
        size = reader.read_u32::<LittleEndian>().unwrap();
        let (s1, atom1) = vecofbits_to_enum_size(reader);
        let (s2, atom2) = vecofbits_to_enum_size(reader);
        output = Atom::Pow(flag, Box::new((atom1, atom2)));
    } else if sig == FN_START_SIG {
        let flag = reader.read_u8().unwrap() != 0;
        let name = format!("var_{}", reader.read_u8().unwrap());
        size = reader.read_u32::<LittleEndian>().unwrap();
        let mut vect_of_atoms: Vec<Atom> = vec![];
        let mut i: u32 = 0;
        while i < size - 1 {
            let (s, at) = vecofbits_to_enum_size(reader);
            vect_of_atoms.push(at);
            i += s
        }
        output = Atom::Fn(flag, name, vect_of_atoms);
    } else if sig == TERM_START_SIG {
        let flag = reader.read_u8().unwrap() != 0;
        size = reader.read_u32::<LittleEndian>().unwrap();
        let mut vect_of_atoms: Vec<Atom> = vec![];
        let mut i: u32 = 0;
        while i < size - 1 {
            let (s, at) = vecofbits_to_enum_size(reader);
            vect_of_atoms.push(at);
            i += s
        }
        output = Atom::Term(flag, vect_of_atoms);
    } else if sig == NUM_START_SIG {
        size = 1;
        output = Atom::Num( reader.read_u8().unwrap() != 0, reader.read_u32::<LittleEndian>().unwrap());
    }
    (size,output)
    
}

pub fn vecofbits_to_vecofenums_size(size: u32, reader: &mut Cursor<Vec<u8>>) -> (u32, Vec<Atom>) {
    let mut output: Vec<Atom> = vec![];
    // depending on the type of atom add corresponding signature
    // and the content of the atom
    // recursively call the function for atoms that contain atoms
    //println!("{}",reader.read_u8().unwrap()==3);
    let mut i: u32 = 0;
    while i < size {
        let sig = reader.read_u8().unwrap();
        //println!("{}", sig);
        //println!("{:#?}",reader);
        if sig == VAR_START_SIG {
            output.push(Atom::Var(
                format!("var_{}", reader.read_u8().unwrap()),
                reader.read_u32::<LittleEndian>().unwrap(),
            ));
            i += 1;
        } else if sig == POW_START_SIG {
            let flag = reader.read_u8().unwrap() != 0;
            let size1 = reader.read_u32::<LittleEndian>().unwrap();
            let (s1, atom1) = vecofbits_to_enum_size(reader);
            let (s2, atom2) = vecofbits_to_enum_size(reader);
            output.push(Atom::Pow(flag, Box::new((atom1, atom2))));
            i += size1;
        } else if sig == FN_START_SIG {
            let flag = reader.read_u8().unwrap() != 0;
            let name = format!("var_{}", reader.read_u8().unwrap());
            let size1 = reader.read_u32::<LittleEndian>().unwrap();
            let mut vect_of_atoms: Vec<Atom> = vec![];
            let mut j: u32 = 0;
            while j < size1 - 1 {
                let (s, at) = vecofbits_to_enum_size(reader);
                vect_of_atoms.push(at);
                j += s
            }
            output.push(Atom::Fn(flag, name, vect_of_atoms));
            i += size1;
        } else if sig == TERM_START_SIG {
            let flag = reader.read_u8().unwrap() != 0;
            let size1 = reader.read_u32::<LittleEndian>().unwrap();
            let mut vect_of_atoms: Vec<Atom> = vec![];
            let mut j: u32 = 0;
            while j < size1 - 1 {
                let (s, at) = vecofbits_to_enum_size(reader);
                vect_of_atoms.push(at);
                j += s
            }
            output.push(Atom::Term(flag, vect_of_atoms));
            i += size1;
        } else if sig == NUM_START_SIG{
            output.push(Atom::Num(
                reader.read_u8().unwrap() != 0,
                reader.read_u32::<LittleEndian>().unwrap(),
            ));
            i += 1;
        }
    }
    (size, output)
}
