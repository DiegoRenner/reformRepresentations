use rand::Rng;
use rand::seq::SliceRandom;

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
    for i in 0..num_new_vars {
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
    for i in 0..num_new_vars {
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

fn main() {
    let mut rng = rand::thread_rng();
    // set parameters for generating random Atom
    // maximum amount of elements per layer
    max_elements = 5;
    // maximum amount of layers
    max_depth = 5;

    // determine number of Atoms in first layer
    let n1: usize = rng.gen_range(1..max_elements);
    
    // initialize first layer and global variable counter
    let init_vec: Vec<Atom>;
    let mut var_counter: usize = 0;
    init_vec = gen_init_vec(n1,&mut var_counter);
    println!("Layer 1");
    println!("{:#?}",init_vec);
    println!("");

    // build random amount of layers using elements of the previous ones
    let mut prev_vec: Vec<Atom> = init_vec;
    let n2: usize = rng.gen_range(1..max_depth);
    for i in 0..n2 {
        // build each layer with random amount of elements
        let n3: usize = rng.gen_range(1..max_elements);
        prev_vec = gen_vec(n3, prev_vec, &mut var_counter);
        println!("Layer {}", i);
        println!("{:#?}", prev_vec);
        println!("");
    }

    // wrap everyting in an element to get final element
    println!("Final Atom");
    let n4: usize = rng.gen_range(0..2);
    if n4 == 0 {
        println!("{:#?}", Atom::Fn(true,format!("var_{}",var_counter), prev_vec));
    } else {
        println!("{:#?}", Atom::Term(true, prev_vec));
    }

}
