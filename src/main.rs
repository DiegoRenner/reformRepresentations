//use number::Number;
//extern crate rand;
use rand::Rng;
use rand::seq::SliceRandom;
/// Internal ID numbers for variables.
pub type VarName = u32;

#[derive(Debug, Clone)]
pub enum Element/*<ID: Id = VarName>*/ {
    //VariableArgument(ID),                                        // ?a
    //Wildcard(ID, Vec<Element<ID>>),                              // x?{...}
    //FnWildcard(ID, Box<(Vec<Element<ID>>, Vec<Element<ID>>)>),   // f?{...}(...)
    //Dollar(ID, Vec<Element<ID>>),                                // $x[y]
    Var(String, usize),                                             // x^n
    Pow(bool, Box<(Element, Element)>),                  // (1+x)^3; dirty, base, exponent
    //NumberRange(Number, Ordering),                               // >0, <=-5/2
    //Comparison(bool, Box<(Element<ID>, Element<ID>)>, Ordering), // x < y, x >= y, $a == 2
    Fn(bool, String, Vec<Element>),                              // f(...)
    Term(bool, Vec<Element>),
    //SubExpr(bool, Vec<Element>),
    Num(bool, usize),
    //RationalPolynomialCoefficient(bool, Box<(Polynomial, Polynomial)>),
    //Expression(ID), // a reference to an expression
}

fn gen_init_vec(num_new_vars: usize, var_counter: &mut usize) -> Vec<Element> {
    let mut init_vec: Vec<Element> = Vec::new();
    for i in 0..num_new_vars {
        let mut rng = rand::thread_rng();
        let n1: usize = rng.gen_range(1..3);
        if n1 == 1 {
            let mut rng = rand::thread_rng();
            let n2: usize = rng.gen_range(1..10);
            init_vec.push(Element::Num(true, n2));
        }
        else if n1 == 2 {
            let mut rng = rand::thread_rng();
            let n2: usize = rng.gen_range(1..10);
            init_vec.push(Element::Var(format!("var_{}",var_counter), n2));
            *var_counter += 1;
        }
    }
    init_vec
}

fn gen_vec(num_new_vars: usize, old_vec: Vec<Element>, var_counter: &mut usize) -> Vec<Element> {
    let mut vec: Vec<Element> = Vec::new();
    for i in 0..num_new_vars {
        let mut rng = rand::thread_rng();
        let n1: usize = rng.gen_range(1..6);
        if n1 == 1 {
            let mut rng = rand::thread_rng();
            let n2: usize = rng.gen_range(1..10);
            vec.push(Element::Num(true, n2))

        }
        else if n1 == 2 {
            let mut rng = rand::thread_rng();
            let n2: usize = rng.gen_range(1..10);
            vec.push(Element::Var(format!("var_{}",var_counter), n2));
            *var_counter += 1;
        }
        else if n1 == 3 {
            let mut rng = rand::thread_rng();
            let size: usize = old_vec.len().try_into().unwrap();
            let n2: usize = rng.gen_range(0..size);
            let n3: usize = rng.gen_range(0..size);
            vec.push(Element::Pow(true,Box::new((old_vec[n2].clone(),old_vec[n3].clone()))));
        }
        else if n1 == 4 {
            let mut rng = rand::thread_rng();
            let size: usize = old_vec.len().try_into().unwrap();
            let n2: usize = rng.gen_range(1..size+1);
            vec.push(Element::Fn(true,format!("var_{}",var_counter),old_vec.choose_multiple(&mut rng, n2).cloned().collect()));
            *var_counter += 1;
        }
        else if n1 == 5 {
            let mut rng = rand::thread_rng();
            let size: usize = old_vec.len().try_into().unwrap();
            let n2: usize = rng.gen_range(1..size+1);
            vec.push(Element::Fn(true,format!("var_{}",var_counter),old_vec.choose_multiple(&mut rng, n2).cloned().collect()));
            *var_counter += 1;
        }
        else if n1 == 6 {
            let mut rng = rand::thread_rng();
            let size: usize = old_vec.len().try_into().unwrap();
            let n2: usize = rng.gen_range(1..size+1);
            vec.push(Element::Term(true,old_vec.choose_multiple(&mut rng, n2).cloned().collect()));
        }
    }
    vec
}

fn main() {
    let mut rng = rand::thread_rng();
    // determine number of Elements in first layer
    let n1: usize = rng.gen_range(1..10);
    
    // initialize first layer and global variable counter
    let init_vec: Vec<Element>;
    let mut var_counter: usize = 0;
    init_vec = gen_init_vec(n1,&mut var_counter);
    println!("Layer 1");
    println!("{:#?}",init_vec);
    println!("");

    // build random amount of layers using elements of the previous ones
    let mut prev_vec: Vec<Element> = init_vec;
    let n2: usize = rng.gen_range(1..10);
    for i in 0..n2 {
        // build each layer with random amount of elements
        let n3: usize = rng.gen_range(1..10);
        prev_vec = gen_vec(n3, prev_vec, &mut var_counter);
        println!("Layer {}", i);
        println!("{:#?}", prev_vec);
        println!("");
    }
    // wrap everyting in an element
    println!("Final Element");
    let n4: usize = rng.gen_range(0..2);
    if n4 == 0 {
        println!("{:#?}", Element::Fn(true,format!("var_{}",var_counter), prev_vec));
    } else {
        println!("{:#?}", Element::Term(true, prev_vec));
    }

}
