use rand::Rng;
use rand::seq::SliceRandom;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use std::str;
// potential types for storing bit strings
//use bitstring::BitLengthString;
//use bitstring::FixedBitString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Atom{
    Var(String, u32),
    Pow(bool, Box<(Atom, Atom)>),
    Fn(bool, String, Vec<Atom>),
    Term(bool, Vec<Atom>),
    Num(bool, u32)
}

#[derive(Debug, Clone)]
pub struct Atom_seq {
    seq: Vec<u8>
}

impl Iterator for Atom_seq {
    type Item = Atom_seq;

    fn next(&mut self) -> Option::<Self::Item> {
        if self.seq.len() > 0{
            if self.seq[0] == 0 {
                if self.seq.len() >= 6 {
                    let current = Atom_seq{
                        seq: self.clone().seq,
                    };
                    self.seq = self.seq[6..].to_vec();
                    return Some(current);
                } else {
                    return None;
                }
            } else if self.seq[0] == 1 {
                if self.seq.len() >= 6 {
                    let current = Atom_seq{
                        seq: self.clone().seq,
                    };
                    self.seq = self.seq[6..].to_vec();
                    return Some(current);
                } else {
                    return None;
                }
            } else if self.seq[0] == 3 {
                if self.seq.len() >= 7 {
                    let current = Atom_seq{
                        seq: self.clone().seq,
                    };
                    self.seq = self.seq[7..].to_vec();
                    return Some(current);
                } else {
                    return None;
                }
            } else if self.seq[0] == 5 {
                if self.seq.len() >= 6 {
                    let current = Atom_seq{
                        seq: self.clone().seq,
                    };
                    self.seq = self.seq[6..].to_vec();
                    return Some(current);
                } else {
                    return None;
                }
            } else if self.seq[0] == 7 {
                if self.seq.len() >= 6 {
                    let current = Atom_seq{
                        seq: self.clone().seq,
                    };
                    self.seq = self.seq[6..].to_vec();
                    return Some(current);
                } else {
                    return None;
               }
            }
        }
        None
    }
}

// generate vector of Atoms without existing Atoms
fn gen_init_vec(num_new_vars: usize, var_counter: &mut usize) -> Vec<Atom> {
    let mut init_vec: Vec<Atom> = Vec::new();
    for _i in 0..num_new_vars {
        // randomly choose what kind of Atom to add
        let mut rng = rand::thread_rng();
        let n1: u32 = rng.gen_range(1..3);
        if n1 == 1 {
            let mut rng = rand::thread_rng();
            let n2: u32 = rng.gen_range(1..10);
            init_vec.push(Atom::Num(true, n2));
        }
        else if n1 == 2 {
            let mut rng = rand::thread_rng();
            let n2: u32 = rng.gen_range(1..10);
            init_vec.push(Atom::Var(format!("var_{}",var_counter), n2));
            *var_counter += 1;
        }
    }
    init_vec
}

// generate vector of Atoms based on existing Atoms
fn gen_vec(num_new_vars: usize, old_vec: Vec<Atom>, var_counter: &mut usize) -> Vec<Atom> {
    let mut vec: Vec<Atom> = Vec::new();
    for _i in 0..num_new_vars {
        // randomly choose what kind of Atom to add
        let mut rng = rand::thread_rng();
        let n1: u32 = rng.gen_range(1..6);
        if n1 == 1 {
            let mut rng = rand::thread_rng();
            let n2: u32 = rng.gen_range(1..10);
            vec.push(Atom::Num(true, n2))

        }
        else if n1 == 2 {
            let mut rng = rand::thread_rng();
            let n2: u32 = rng.gen_range(1..10);
            vec.push(Atom::Var(format!("var_{}",var_counter), n2));
            *var_counter += 1;
        }
        else if n1 == 3 {
            let mut rng = rand::thread_rng();
            let size: usize = old_vec.len().try_into().unwrap();
            let n2: usize = rng.gen_range(0..size);
            let n3: usize = rng.gen_range(0..size);
            vec.push(Atom::Pow(true,Box::new((old_vec[n2].clone(),old_vec[n3].clone()))));
        }
        else if n1 == 4 {
            let mut rng = rand::thread_rng();
            let size: usize = old_vec.len().try_into().unwrap();
            let n2: usize = rng.gen_range(1..size+1);
            vec.push(Atom::Fn(true,format!("var_{}",var_counter),old_vec.choose_multiple(&mut rng, n2).cloned().collect()));
            *var_counter += 1;
        }
        else if n1 == 5 {
            let mut rng = rand::thread_rng();
            let size: usize = old_vec.len().try_into().unwrap();
            let n2: usize = rng.gen_range(1..size+1);
            vec.push(Atom::Fn(true,format!("var_{}",var_counter),old_vec.choose_multiple(&mut rng, n2).cloned().collect()));
            *var_counter += 1;
        }
        else if n1 == 6 {
            let mut rng = rand::thread_rng();
            let size: usize = old_vec.len().try_into().unwrap();
            let n2: usize = rng.gen_range(1..size+1);
            vec.push(Atom::Term(true,old_vec.choose_multiple(&mut rng, n2).cloned().collect()));
        }
    }
    vec
}

// convert an atom from an enum to a string of bits
fn enum_to_strofbits(atom: Atom) -> String {
    let mut output: String = "".to_string();
    // depending on the type of atom add corresponding signature 
    // and the content of the atom
    // recursively call the function for atoms that contain atoms
    if let Atom::Var(name, exponential) = atom.clone() {
        //println!("This atom is a variable with name {} and exponential {}", name, exponential);
        output.push_str("00000000");
        let name_int: usize = (name[4..]).parse::<usize>().unwrap();
        let name_str: String = format!("{name_int:08b}");
        output.push_str(&name_str);
        output.push_str(&format!("{exponential:032b}"))
    }
    else if let Atom::Pow(flag, box_of_atoms) = atom.clone() {
        //println!("This atom is a power with flag {} and box of atoms {:#?}", flag, box_of_atoms);
        output.push_str("00000001");
        let flag_int: usize = flag as usize;
        let flag_str: String = format!("{flag_int:08b}");
        output.push_str(&flag_str);
        output.push_str(&enum_to_strofbits((*box_of_atoms).0));
        output.push_str(&enum_to_strofbits((*box_of_atoms).1));
        output.push_str("00000010");
    }
    else if let Atom::Fn(flag, name, vect_of_atoms) = atom.clone() {
        //println!("This atom is a function with flag {}, name {}, and vector of atoms {:#?}", flag, name, vect_of_atoms);
        output.push_str("00000011");
        let flag_int: usize = flag as usize;
        let flag_str: String = format!("{flag_int:08b}");
        let name_int: usize = (name[4..]).parse::<usize>().unwrap();
        let name_str: String = format!("{name_int:08b}");
        output.push_str(&flag_str);
        output.push_str(&name_str);
        for a in vect_of_atoms {
            output.push_str(&enum_to_strofbits(a));
        }
        output.push_str("00000100");
    }
    else if let Atom::Term(flag, vect_of_atoms) = atom.clone() {
        //println!("This atom is a term with flag {} and vector of atoms {:#?}", flag, vect_of_atoms);
        output.push_str("00000101");
        let flag_int: usize = flag as usize;
        let flag_str: String = format!("{flag_int:08b}");
        output.push_str(&flag_str);
        for a in vect_of_atoms {
            output.push_str(&enum_to_strofbits(a));
        }
        output.push_str("00000110");
    }
    else if let Atom::Num(flag, value) = atom.clone() {
        //println!("This atom is a number with flag {} and value {}", flag.to_string(), value.to_string());
        output.push_str("00000111");
        let flag_int: usize = flag as usize;
        let flag_str: String = format!("{flag_int:08b}");
        output.push_str(&flag_str);
        output.push_str(&format!("{value:032b}"))
    }
    return output
}

// convert an atom from an enum to a vector of bits
fn enum_to_vecofbits(atom: Atom) -> Vec<u8> {
    let mut output: Vec<u8> = vec![];
    // depending on the type of atom add corresponding signature 
    // and the content of the atom
    // recursively call the function for atoms that contain atoms
    if let Atom::Var(name, exponential) = atom.clone() {
        //println!("This atom is a variable with name {} and exponential {}", name, exponential);
        output.write_u8(0);
        let name_int: u8 = (name[4..]).parse::<u8>().unwrap();
        output.write_u8(name_int);
        output.write_u32::<LittleEndian>(exponential);
    }
    else if let Atom::Pow(flag, box_of_atoms) = atom.clone() {
        //println!("This atom is a power with flag {} and box of atoms {:#?}", flag, box_of_atoms);
        output.write_u8(1);
        let flag_int: u8 = flag as u8;
        output.write_u8(flag_int);
        output.append(&mut enum_to_vecofbits((*box_of_atoms).0));
        output.append(&mut enum_to_vecofbits((*box_of_atoms).1));
        output.write_u8(2);
    }
    else if let Atom::Fn(flag, name, vect_of_atoms) = atom.clone() {
        //println!("This atom is a function with flag {}, name {}, and vector of atoms {:#?}", flag, name, vect_of_atoms);
        output.write_u8(3);
        let flag_int: u8 = flag as u8;
        output.write_u8(flag_int);
        let name_int: u8 = (name[4..]).parse::<u8>().unwrap();
        output.write_u8(name_int);
        for a in vect_of_atoms {
            output.append(&mut enum_to_vecofbits(a));
        }
        output.write_u8(4);
    }
    else if let Atom::Term(flag, vect_of_atoms) = atom.clone() {
        //println!("This atom is a term with flag {} and vector of atoms {:#?}", flag, vect_of_atoms);
        output.write_u8(5);
        let flag_int: u8 = flag as u8;
        output.write_u8(flag_int);
        for a in vect_of_atoms {
            output.append(&mut enum_to_vecofbits(a));
        }
        output.write_u8(6);
    }
    else if let Atom::Num(flag, value) = atom.clone() {
        //println!("This atom is a number with flag {} and value {}", flag.to_string(), value.to_string());
        output.write_u8(7);
        let flag_int: u8 = flag as u8;
        output.write_u8(flag_int);
        output.write_u32::<LittleEndian>(value);
    }
    return output
}

// convert an atom from an enum to a vector of bits
fn enum_to_vecofbits_size(atom: Atom) -> (u32, Vec<u8>) {
    let mut output: Vec<u8> = vec![];
    let mut size: u32 = 0;
    // depending on the type of atom add corresponding signature 
    // and the content of the atom
    // recursively call the function for atoms that contain atoms
    if let Atom::Var(name, exponential) = atom.clone() {
        //println!("This atom is a variable with name {} and exponential {}", name, exponential);
        output.write_u8(0);
        let name_int: u8 = (name[4..]).parse::<u8>().unwrap();
        output.write_u8(name_int);
        output.write_u32::<LittleEndian>(exponential);
        size = 1;
    }
    else if let Atom::Pow(flag, box_of_atoms) = atom.clone() {
        //println!("This atom is a power with flag {} and box of atoms {:#?}", flag, box_of_atoms);
        output.write_u8(1);
        let flag_int: u8 = flag as u8;
        output.write_u8(flag_int);
        let (temp_size1, mut temp_output1) = enum_to_vecofbits_size((*box_of_atoms).0);
        let (temp_size2, mut temp_output2) = enum_to_vecofbits_size((*box_of_atoms).1);
        size = temp_size1 + temp_size2 + 1;
        output.write_u32::<LittleEndian>(size);
        output.append(&mut temp_output1);
        output.append(&mut temp_output2);
    }
    else if let Atom::Fn(flag, name, vect_of_atoms) = atom.clone() {
        //println!("This atom is a function with flag {}, name {}, and vector of atoms {:#?}", flag, name, vect_of_atoms);
        output.write_u8(3);
        let flag_int: u8 = flag as u8;
        output.write_u8(flag_int);
        let name_int: u8 = (name[4..]).parse::<u8>().unwrap();
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
    }
    else if let Atom::Term(flag, vect_of_atoms) = atom.clone() {
        //println!("This atom is a term with flag {} and vector of atoms {:#?}", flag, vect_of_atoms);
        output.write_u8(5);
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
    }
    else if let Atom::Num(flag, value) = atom.clone() {
        //println!("This atom is a number with flag {} and value {}", flag.to_string(), value.to_string());
        output.write_u8(7);
        let flag_int: u8 = flag as u8;
        output.write_u8(flag_int);
        output.write_u32::<LittleEndian>(value);
        size = 1;
    }
    return (size,output)
}

// convert a vector of atoms from an enum to a vector of bits
fn vecofenums_to_vecofbits_size(atoms: Vec<Atom>) -> (u32, Vec<u8>) {
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
fn vecofbits_to_enum_size(reader: &mut Cursor<Vec<u8>>) -> (u32,Atom) {
    let mut output: Atom;
    // depending on the type of atom add corresponding signature 
    // and the content of the atom
    // recursively call the function for atoms that contain atoms
    //println!("{}",reader.read_u8().unwrap()==3);
    let sig = reader.read_u8().unwrap();
    //println!("{}", sig);
    //println!("{:#?}",reader);
    if sig == 0 {
        (1,Atom::Var(format!("var_{}",reader.read_u8().unwrap()),reader.read_u32::<LittleEndian>().unwrap()))
    }
    else if sig == 1 {
        let flag = reader.read_u8().unwrap() != 0;
        let size = reader.read_u32::<LittleEndian>().unwrap();
        let (s1,atom1) = vecofbits_to_enum_size(reader);
        let (s2,atom2) = vecofbits_to_enum_size(reader);
        (size,Atom::Pow(flag,Box::new((atom1,atom2))))
    }
    else if sig == 3 {
        let flag = reader.read_u8().unwrap() != 0;
        let name = format!("var_{}",reader.read_u8().unwrap());
        let size = reader.read_u32::<LittleEndian>().unwrap();
        let mut vect_of_atoms: Vec<Atom> = vec![];
        let mut i: u32 = 0;
        while i < size-1 {
            let (s,at) = vecofbits_to_enum_size(reader);
            vect_of_atoms.push(at);
            i += s
        }
        (size,Atom::Fn(flag,name,vect_of_atoms))
    }
    else if sig == 5 {
        let flag = reader.read_u8().unwrap()  != 0;
        let size = reader.read_u32::<LittleEndian>().unwrap();
        let mut vect_of_atoms: Vec<Atom> = vec![];
        let mut i: u32 = 0;
        while i < size-1 {
            let (s,at) = vecofbits_to_enum_size(reader);
            vect_of_atoms.push(at);
            i += s
        }
        (size,Atom::Term(flag,vect_of_atoms))
    }
    else {
        (1,Atom::Num(reader.read_u8().unwrap() != 0,reader.read_u32::<LittleEndian>().unwrap()))
    }
}

fn vecofbits_to_vecofenums_size(size: u32, reader: &mut Cursor<Vec<u8>>) -> (u32,Vec<Atom>) {
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
        if sig == 0 {
            output.push(Atom::Var(format!("var_{}",reader.read_u8().unwrap()),reader.read_u32::<LittleEndian>().unwrap()));
            i += 1;
        }
        else if sig == 1 {
            let flag = reader.read_u8().unwrap() != 0;
            let size1 = reader.read_u32::<LittleEndian>().unwrap();
            let (s1,atom1) = vecofbits_to_enum_size(reader);
            let (s2,atom2) = vecofbits_to_enum_size(reader);
            output.push(Atom::Pow(flag,Box::new((atom1,atom2))));
            i += size1;
        }
        else if sig == 3 {
            let flag = reader.read_u8().unwrap() != 0;
            let name = format!("var_{}",reader.read_u8().unwrap());
            let size1 = reader.read_u32::<LittleEndian>().unwrap();
            let mut vect_of_atoms: Vec<Atom> = vec![];
            let mut j: u32 = 0;
            while j < size1-1 {
                let (s,at) = vecofbits_to_enum_size(reader);
                vect_of_atoms.push(at);
                j += s
            }
            output.push(Atom::Fn(flag,name,vect_of_atoms));
            i += size1;
        }
        else if sig == 5 {
            let flag = reader.read_u8().unwrap()  != 0;
            let size1 = reader.read_u32::<LittleEndian>().unwrap();
            let mut vect_of_atoms: Vec<Atom> = vec![];
            let mut j: u32 = 0;
            while j < size1-1 {
                let (s,at) = vecofbits_to_enum_size(reader);
                vect_of_atoms.push(at);
                j += s
            }
            output.push(Atom::Term(flag,vect_of_atoms));
            i += size1;
        }
        else {
            output.push(Atom::Num(reader.read_u8().unwrap() != 0,reader.read_u32::<LittleEndian>().unwrap()));
            i += 1;
        }
    }
    (size,output)
}


fn main() {

    // experiments with writing vector of bits
    //let mut buf: Vec<u8> = vec![];
    //buf.write_u32::<LittleEndian>(8);
    //buf.write_u32::<LittleEndian>(24);
    //for s in &buf {
    //    println!("{}",format!("{s:b}"))
    //}
    //let s = match str::from_utf8(&buf) {
    //    Ok(v) => v,
    //    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    //};
    //println!("{:#?}", buf);
    //println!("{:#?}", s);

    let mut rng = rand::thread_rng();
    // set parameters for generating random Atom
    // maximum amount of elements per layer
    let max_elements = 5;
    // maximum amount of layers
    let max_depth = 5;

    // determine number of atoms in first layer
    let n1: usize = rng.gen_range(1..max_elements);
    
    // initialize first layer and global variable counter
    let init_vec: Vec<Atom>;
    let mut var_counter: usize = 0;
    init_vec = gen_init_vec(n1,&mut var_counter);
    // debugging prompts for randomly generating an atom
    //println!("Layer 1");
    //println!("{:#?}",init_vec);
    //println!("");

    // build random amount of layers using atoms of the previous ones
    let mut prev_vec: Vec<Atom> = init_vec;
    let n2: usize = rng.gen_range(1..max_depth);
    for i in 0..n2 {
        // build each layer with random amount of atoms
        let n3: usize = rng.gen_range(1..max_elements);
        prev_vec = gen_vec(n3, prev_vec, &mut var_counter);
        // debugging prompts for randomly generating an atom
        //println!("Layer {}", i);
        //println!("{:#?}", prev_vec);
        //println!("");
    }

    // wrap everyting in an atom to get final atom
    println!("Final atom");
    let n4: usize = rng.gen_range(0..2);
    let final_at: Atom; 
    if n4 == 0 {
        final_at = Atom::Fn(true,format!("var_{}",var_counter), prev_vec.clone());
    } else {
        final_at = Atom::Term(true, prev_vec.clone());
    }
    println!("{:#?}", prev_vec);
    
    // debugging prompts for enum_to_strofbits
    //println!("{}",enum_to_strofbits(Atom::Var("var_2".to_string(),84)));
    //println!("{}",enum_to_strofbits(Atom::Pow(true,Box::new((Atom::Num(true,84),Atom::Num(true,84))))));
    //println!("{}",enum_to_strofbits(Atom::Fn(true,"var_3".to_string(),vec![Atom::Num(true,84),Atom::Num(true,84)])));
    //println!("{}",enum_to_strofbits(Atom::Term(true,vec![Atom::Num(true,84),Atom::Num(true,84)])));
    //println!("{}",enum_to_strofbits(Atom::Num(true,84)));

    // convert randomly generated atom to string of bits  with start/end signature
    //println!("{}",enum_to_strofbits(final_at.clone()));

    // convert to randomly generated atom to vector of bits with start/end signature
    //let vec_of_bits: Vec<u8> = enum_to_vecofbits(final_at.clone());
    //for s in &vec_of_bits {
    //    print!("{}",format!("{s:08b}"))
    //}

    // convert to randomly generated atom to vector of bits with size signature
    //println!();
    let (size, vec_of_bits_size) = vecofenums_to_vecofbits_size(prev_vec.clone());
    //for s in &vec_of_bits_size {
    //    print!("{}",format!("{s:08b}"))
    //}

    // iterate over a serialized vector of atoms
    //println!();
    //let test_seq = Atom_seq{
    //    seq: vec_of_bits_size.clone(),
    //};
    //for s in test_seq {
    //    for x in s.clone().seq {
    //        print!("{}",format!("{x:08b}"))
    //    }
    //    println!();
    //}

    // deparse randomly generated atom and and compare to initial one
    let (s, deparsed_enum) = vecofbits_to_vecofenums_size(size,&mut Cursor::new(vec_of_bits_size.clone()));
    println!("{:#?}", deparsed_enum);

    if deparsed_enum  == prev_vec {
        println!("parsing and deparsing succesful",)
    }
    
    
}
