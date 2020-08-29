extern crate rsa;
extern crate rand;

extern crate num_bigint_dig as num_bigint;
extern crate num_traits;

use rsa::{BigUint, RSAPrivateKey};
use rand::rngs::OsRng;
// use num_bigint_dig::traits::ModInverse;


pub struct DistributedRSAPrivateKey {
    pub d: BigUint
}

pub struct DistributedRSAPrivateKeySet {
    pub private_keys: Vec<DistributedRSAPrivateKey>
}

impl DistributedRSAPrivateKeySet {
    pub fn from_rsa_private_key (private_key: &RSAPrivateKey) {
        private_key.d();
        let primes = private_key.primes();

        let one = BigUint::from(1 as u32);
        let lambda = (&primes[0].clone() - &one) * (&primes[1] - &one);

        println!("{}", lambda);
    }
}


#[test]
fn test_generate_private_key_set() {
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    
    DistributedRSAPrivateKeySet::from_rsa_private_key(&priv_key)
}