use rand::Rng;
use rand::seq::SliceRandom;
// potential types for storing bit strings
//use bitstring::BitLengthString;
//use bitstring::FixedBitString;

#[derive(Debug, Clone)]
pub enum Atom{
    Var(String, usize),
    Pow(bool, Box<(Atom, Atom)>),
    Fn(bool, String, Vec<Atom>),
    Term(bool, Vec<Atom>),
    Num(bool, usize),
}

// generate vector of Atoms without existing Atoms
fn gen_init_vec(num_new_vars: usize, var_counter: &mut usize) -> Vec<Atom> {
    let mut init_vec: Vec<Atom> = Vec::new();
    for _i in 0..num_new_vars {
        // randomly choose what kind of Atom to add
        let mut rng = rand::thread_rng();
        let n1: usize = rng.gen_range(1..3);
        if n1 == 1 {
            let mut rng = rand::thread_rng();
            let n2: usize = rng.gen_range(1..10);
            init_vec.push(Atom::Num(true, n2));
        }
        else if n1 == 2 {
            let mut rng = rand::thread_rng();
            let n2: usize = rng.gen_range(1..10);
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
        let n1: usize = rng.gen_range(1..6);
        if n1 == 1 {
            let mut rng = rand::thread_rng();
            let n2: usize = rng.gen_range(1..10);
            vec.push(Atom::Num(true, n2))

        }
        else if n1 == 2 {
            let mut rng = rand::thread_rng();
            let n2: usize = rng.gen_range(1..10);
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
fn enum_to_bits(atom: Atom) -> String {
    let mut output: String = "".to_string();
    // depending on the type of atom add corresponding signature 
    // and the content of the atom
    // recursively call the function for atoms that contain atoms
    if let Atom::Var(name, exponential) = atom.clone() {
        //println!("This atom is a variable with name {} and exponential {}", name, exponential);
        output.push_str("000");
        let name_int: usize = (name[4..]).parse::<usize>().unwrap();
        let name_str: String = format!("{name_int:010b}");
        output.push_str(&name_str);
        output.push_str(&format!("{exponential:034b}"))
    }
    else if let Atom::Pow(flag, box_of_atoms) = atom.clone() {
        //println!("This atom is a power with flag {} and box of atoms {:#?}", flag, box_of_atoms);
        output.push_str("001");
        let flag_int: usize = flag as usize;
        let flag_str: String = format!("{flag_int:b}");
        output.push_str(&flag_str);
        output.push_str(&enum_to_bits((*box_of_atoms).0));
        output.push_str(&enum_to_bits((*box_of_atoms).1));
        output.push_str("010");
    }
    else if let Atom::Fn(flag, name, vect_of_atoms) = atom.clone() {
        //println!("This atom is a function with flag {}, name {}, and vector of atoms {:#?}", flag, name, vect_of_atoms);
        output.push_str("011");
        let flag_int: usize = flag as usize;
        let flag_str: String = format!("{flag_int:b}");
        let name_int: usize = (name[4..]).parse::<usize>().unwrap();
        let name_str: String = format!("{name_int:010b}");
        output.push_str(&name_str);
        output.push_str(&flag_str);
        for a in vect_of_atoms {
            output.push_str(&enum_to_bits(a));
        }
        output.push_str("100");
    }
    else if let Atom::Term(flag, vect_of_atoms) = atom.clone() {
        //println!("This atom is a term with flag {} and vector of atoms {:#?}", flag, vect_of_atoms);
        output.push_str("101");
        let flag_int: usize = flag as usize;
        let flag_str: String = format!("{flag_int:b}");
        output.push_str(&flag_str);
        for a in vect_of_atoms {
            output.push_str(&enum_to_bits(a));
        }
        output.push_str("110");
    }
    else if let Atom::Num(flag, value) = atom.clone() {
        //println!("This atom is a number with flag {} and value {}", flag.to_string(), value.to_string());
        output.push_str("111");
        let flag_int: usize = flag as usize;
        let flag_str: String = format!("{flag_int:b}");
        output.push_str(&flag_str);
        output.push_str(&format!("{value:034b}"))
    }
    return output
}

fn main() {
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
        final_at = Atom::Fn(true,format!("var_{}",var_counter), prev_vec);
    } else {
        final_at = Atom::Term(true, prev_vec);
    }
    println!("{:#?}", final_at);
    
    // debugging prompts for enum_to_bits
    //println!("{}",enum_to_bits(Atom::Var("var_2".to_string(),84)));
    //println!("{}",enum_to_bits(Atom::Pow(true,Box::new((Atom::Num(true,84),Atom::Num(true,84))))));
    //println!("{}",enum_to_bits(Atom::Fn(true,"var_3".to_string(),vec![Atom::Num(true,84),Atom::Num(true,84)])));
    //println!("{}",enum_to_bits(Atom::Term(true,vec![Atom::Num(true,84),Atom::Num(true,84)])));
    //println!("{}",enum_to_bits(Atom::Num(true,84)));

    // convert randomly generated atom
    println!("{}",enum_to_bits(final_at));

    
}
