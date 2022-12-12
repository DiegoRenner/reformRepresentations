use crate::atom::*;
use crate::parser::*;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use rand::Rng;
use std::io::Cursor;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_size() {
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
        init_vec = gen_init_vec(n1, &mut var_counter);
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
            final_at = Atom::Fn(true, format!("var_{}", var_counter), prev_vec.clone());
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
        println!();
        //for x in vec_of_bits_size.clone() {
        //    print!("{}",format!("{x:08b}"))
        //}
        let test_seq = Atom_seq {
            seq: vec_of_bits_size.clone(),
        };
        for s in test_seq {
            for x in s.clone().seq {
                print!("{}", format!("{x:08b}"))
            }
            //println!();
            //println!("{:#?}", vecofbits_to_vecofenums_size(s.clone().count().try_into().unwrap(),&mut  Cursor::new(s.clone().seq)));
            println!();
        }

        println!();
        // deparse randomly generated atom and and compare to initial one
        let (s, deparsed_enum) =
            vecofbits_to_vecofenums_size(size, &mut Cursor::new(vec_of_bits_size.clone()));
        println!("{:#?}", deparsed_enum);

        if deparsed_enum == prev_vec {
            println!("parsing and deparsing succesful",)
        }
        assert_eq!(deparsed_enum, prev_vec);
    }

    #[test]
    fn test_argsize() {

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
        init_vec = gen_init_vec(n1, &mut var_counter);

        // build random amount of layers using atoms of the previous ones
        let mut prev_vec: Vec<Atom> = init_vec;
        let n2: usize = rng.gen_range(1..max_depth);
        for i in 0..n2 {
            // build each layer with random amount of atoms
            let n3: usize = rng.gen_range(1..max_elements);
            prev_vec = gen_vec(n3, prev_vec, &mut var_counter);
        }

        // convert to randomly generated atom to vector of bits with size signature
        let vec_of_bits_argsize = vecofenums_to_vecofbits_argsize(prev_vec.clone());

        // deparse randomly generated atom and and compare to initial one
        let deparsed_enum =
            vecofbits_to_vecofenums_argsize(prev_vec.len().try_into().unwrap(), &mut Cursor::new(vec_of_bits_argsize.clone()));
        println!("{:#?}", deparsed_enum);

        if deparsed_enum == prev_vec {
            println!("parsing and deparsing succesful",)
        }
        assert_eq!(deparsed_enum, prev_vec);
    }
}
