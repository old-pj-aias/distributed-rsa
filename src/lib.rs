mod utils;

use crate::utils::generate_random_ubigint;

extern crate rsa;
extern crate rand;

extern crate num_bigint_dig as num_bigint;
extern crate num_traits;

use std::vec::Vec;

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
    pub fn from_rsa_private_key (private_key: &RSAPrivateKey, partitions: u32) -> Self {
        let mut private_keys = Vec::new();

        let d = private_key.d();
        let primes = private_key.primes();

        let one = BigUint::from(1 as u16);
        let lambda = (&primes[0].clone() - &one) * (&primes[1] - &one);

        let mut remain = d.clone();

        for _ in 0..partitions -1 {
            let random = generate_random_ubigint(512) % lambda.clone();
            let key = DistributedRSAPrivateKey{ d: random.clone() % lambda.clone() };
            private_keys.push(key);

            remain = remain - random;
        }

        let key = DistributedRSAPrivateKey{ d: remain };
        private_keys.push(key);

        return DistributedRSAPrivateKeySet {
            private_keys: private_keys
        }
    }
}


#[test]
fn test_generate_private_key_set() {
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    
    let d = priv_key.d();
    let primes = priv_key.primes();

    let one = BigUint::from(1 as u16);
    let lambda = (&primes[0].clone() - &one) * (&primes[1] - &one);

    let keys = DistributedRSAPrivateKeySet::from_rsa_private_key(&priv_key, 10);

    let mut sum = BigUint::from(0 as u16);
    for key in keys.private_keys {
        sum += key.d % lambda.clone();
    }

    let d = d % lambda;
    assert_eq!(sum, d);
}