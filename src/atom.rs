use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use rand::seq::SliceRandom;
use rand::Rng;

use crate::parser::{VAR_START_SIG, POW_START_SIG, FN_START_SIG, TERM_START_SIG, NUM_START_SIG};
 
 pub const VAR_BIT_LEN: usize = 6;
 pub const POW_BIT_LEN: usize = 6;
 pub const FN_BIT_LEN: usize = 7;
 pub const TERM_BIT_LEN: usize = 6;
 pub const NUM_BIT_LEN: usize = 6;
 pub const NUM_STAT_ATOM_TYPES: usize = 2;
 pub const NUM_DYN_ATOM_TYPES: usize = 3;
 pub const NUM_ATOM_TYPES: usize = NUM_STAT_ATOM_TYPES + NUM_DYN_ATOM_TYPES;
 pub const MAX_VAL_ATOMS: u32 = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Atom {
    Var(String, u32),
    Pow(bool, Box<(Atom, Atom)>),
    Fn(bool, String, Vec<Atom>),
    Term(bool, Vec<Atom>),
    Num(bool, u32),
}

#[derive(Debug, Clone)]
pub struct Atom_seq {
    pub seq: Vec<u8>,
}

//pub struct Atom_seq_view<'a> {
//    seq: &'a [u8],
//}

//trait LendingIterator {
//    type Item<'a>
//    where
//        Self: 'a;
//
//    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
//}

//impl<'a> Atom_seq_view<'a> {
//
//    pub fn next(&'a mut self) -> Option<&'a [u8]> {
//        if self.seq.is_empty() {
//            return None;
//        }
//        match self.seq[0] {
//            0 => {
//               let (var,rest) =  self.seq.split_at(6);
//               self.seq = rest;
//               Some(var)
//            }
//            _ => None,
//        }
//    }
//}

impl Iterator for Atom_seq {
    type Item = Atom_seq;

    fn next(&mut self) -> Option<Self::Item> {
        if self.seq.len() > 0 {
            if self.seq[0] == VAR_START_SIG {
                if self.seq.len() >= VAR_BIT_LEN {
                    let current = Atom_seq {
                        seq: self.clone().seq,
                    };
                    self.seq = self.seq[VAR_BIT_LEN..].to_vec();
                    return Some(current);
                } else {
                    return None;
                }
            } else if self.seq[0] == POW_START_SIG {
                if self.seq.len() >= POW_BIT_LEN {
                    let mut reader = Cursor::new(self.seq.clone());
                    let sig = reader.read_u8().unwrap();
                    let flag = reader.read_u8().unwrap() != 0;
                    let size = reader.read_u32::<LittleEndian>().unwrap();
                    let current = Atom_seq {
                        seq: self.clone().seq,
                    };
                    let mut counter = 0;
                    for i in 0..size {
                        if self.seq[counter] == FN_START_SIG {
                            counter += FN_BIT_LEN
                        } else {
                            counter += POW_BIT_LEN 
                        }
                    }
                    self.seq = self.seq[counter..].to_vec();
                    //self.seq = self.seq[6..].to_vec();
                    //match self.skip(size.try_into().unwrap()).next(){
                    //    Some(mut a) => self.seq = a.seq,
                    //    None => return None
                    //}
                    return Some(current);
                } else {
                    return None;
                }
            } else if self.seq[0] == FN_START_SIG {
                if self.seq.len() >= FN_BIT_LEN {
                    let mut reader = Cursor::new(self.seq.clone());
                    let sig = reader.read_u8().unwrap();
                    let flag = reader.read_u8().unwrap() != 0;
                    let name = format!("var_{}", reader.read_u8().unwrap());
                    let size = reader.read_u32::<LittleEndian>().unwrap();
                    //println!("{}", size);
                    let current = Atom_seq {
                        seq: self.clone().seq,
                    };
                    let mut counter = 0;
                    for i in 0..size {
                        if self.seq[counter] == FN_START_SIG {
                            counter += FN_BIT_LEN
                        } else {
                            counter += POW_BIT_LEN
                        }
                    }
                    self.seq = self.seq[counter..].to_vec();
                    //match self.skip(size.try_into().unwrap()).next(){
                    //    Some(mut a) => self.seq = a.seq,
                    //    None => println!("test")
                    //}
                    return Some(current);
                } else {
                    return None;
                }
            } else if self.seq[0] == TERM_START_SIG {
                if self.seq.len() >= TERM_BIT_LEN {
                    let mut reader = Cursor::new(self.seq.clone());
                    let sig = reader.read_u8().unwrap();
                    let flag = reader.read_u8().unwrap() != 0;
                    let size = reader.read_u32::<LittleEndian>().unwrap();
                    let current = Atom_seq {
                        seq: self.clone().seq,
                    };
                    let mut counter = 0;
                    for i in 0..size {
                        if self.seq[counter] == FN_START_SIG {
                            counter += FN_BIT_LEN
                        } else {
                            counter += POW_BIT_LEN
                        }
                    }
                    self.seq = self.seq[counter..].to_vec();
                    match self.skip(size.try_into().unwrap()).next() {
                        Some(mut a) => self.seq = a.seq,
                        None => return None,
                    }
                    return Some(current);
                } else {
                    return None;
                }
            } else if self.seq[0] == NUM_START_SIG {
                if self.seq.len() >= NUM_BIT_LEN {
                    let current = Atom_seq {
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
pub fn gen_init_vec(num_new_vars: usize, var_counter: &mut usize) -> Vec<Atom> {
    let mut init_vec: Vec<Atom> = Vec::new();
    for _i in 0..num_new_vars {
        // randomly choose what kind of Atom to add
        let mut rng = rand::thread_rng();
        let n1: usize = rng.gen_range(0..NUM_STAT_ATOM_TYPES);
        let stat_atom_sigs = vec![VAR_START_SIG,NUM_START_SIG];
        let atom_type = stat_atom_sigs[n1];
        if atom_type == NUM_START_SIG {
            let mut rng = rand::thread_rng();
            let n2: u32 = rng.gen_range(1..MAX_VAL_ATOMS+1);
            init_vec.push(Atom::Num(true, n2));
        } else if atom_type == VAR_START_SIG {
            let mut rng = rand::thread_rng();
            let n2: u32 = rng.gen_range(1..MAX_VAL_ATOMS+1);
            init_vec.push(Atom::Var(format!("var_{}", var_counter), n2));
            *var_counter += 1;
        }
    }
    init_vec
}

// generate vector of Atoms based on existing Atoms
pub fn gen_vec(num_new_vars: usize, old_vec: Vec<Atom>, var_counter: &mut usize) -> Vec<Atom> {
    let mut vec: Vec<Atom> = Vec::new();
    for _i in 0..num_new_vars {
        // randomly choose what kind of Atom to add
        let mut rng = rand::thread_rng();
        let n1: usize = rng.gen_range(0..NUM_ATOM_TYPES);
        let atom_sigs = vec![VAR_START_SIG, POW_START_SIG, FN_START_SIG, TERM_START_SIG, NUM_START_SIG];
        let atom_type = atom_sigs[n1];
        if atom_type == NUM_START_SIG {
            let mut rng = rand::thread_rng();
            let n2: u32 = rng.gen_range(1..MAX_VAL_ATOMS+1);
            vec.push(Atom::Num(true, n2))
        } else if  atom_type == VAR_START_SIG {
            let mut rng = rand::thread_rng();
            let n2: u32 = rng.gen_range(1..MAX_VAL_ATOMS+1);
            vec.push(Atom::Var(format!("var_{}", var_counter), n2));
            *var_counter += 1;
        } else if atom_type == POW_START_SIG {
            let mut rng = rand::thread_rng();
            let size: usize = old_vec.len().try_into().unwrap();
            let n2: usize = rng.gen_range(0..size);
            let n3: usize = rng.gen_range(0..size);
            vec.push(Atom::Pow(
                true,
                Box::new((old_vec[n2].clone(), old_vec[n3].clone())),
            ));
        } else if atom_type == FN_START_SIG {
            let mut rng = rand::thread_rng();
            let size: usize = old_vec.len().try_into().unwrap();
            let n2: usize = rng.gen_range(1..size + 1);
            vec.push(Atom::Fn(
                true,
                format!("var_{}", var_counter),
                old_vec.choose_multiple(&mut rng, n2).cloned().collect(),
            ));
            *var_counter += 1;
        } else if atom_type == TERM_START_SIG {
            let mut rng = rand::thread_rng();
            let size: usize = old_vec.len().try_into().unwrap();
            let n2: usize = rng.gen_range(1..size + 1);
            vec.push(Atom::Term(
                true,
                old_vec.choose_multiple(&mut rng, n2).cloned().collect(),
            ));
        }
    }
    vec
}