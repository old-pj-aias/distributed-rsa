use rsa::{BigUint};


pub fn generate_random_ubigint(size: usize) -> BigUint {
    let size = size / 32; 
    let random_bytes: Vec<u32> = (0..size).map(|_| { rand::random::<u32>() }).collect();
    return BigUint::new(random_bytes);
}

pub fn submod(a: BigUint, b: BigUint, n: BigUint) -> BigUint {
    let a  = a % n.clone();
    let b  = b % n.clone();

    if a > b {
        a - b
    }
    else {
        n - (b - a)
    }

}

#[test]
fn test_generate_random_ubigint() {
    for i in 1..20 {
        let size = i * 64;
        let random = generate_random_ubigint(size);
        println!("{:x}\n\n\n", random);        
    }
}

#[test]
fn test_submod1() {
    let n = BigUint::from(10 as u16);

    let a = BigUint::from(5 as u16);
    let b = BigUint::from(8 as u16);

    let expected = BigUint::from(7 as u16);
    let result = submod(a, b, n);

    println!("{},{}", result, expected);

    assert_eq!(result, expected);
}

#[test]
fn test_submod2() {
    let n = BigUint::from(10 as u16);

    let a = BigUint::from(5 as u16);
    let b = BigUint::from(8 as u16);

    let expected = BigUint::from(3 as u16);
    let result = submod(b, a, n);

    println!("{},{}", result, expected);

    assert_eq!(result, expected);
}