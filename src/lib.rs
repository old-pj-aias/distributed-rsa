mod utils;

use crate::utils::{generate_random_ubigint, submod};

extern crate rsa;
extern crate rand;

extern crate num_bigint_dig as num_bigint;
extern crate num_traits;

use std::vec::Vec;

use rsa::{BigUint, RSAPublicKey, RSAPrivateKey, PublicKeyParts};
use rand::rngs::OsRng;

use serde::{Serialize, Deserialize};


#[derive(Clone, Serialize, Deserialize)]
pub struct PlainShare {
    pub s: BigUint,
    pub n: BigUint
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PlainShareSet {
    pub plain_shares: Vec<PlainShare>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DistributedRSAPrivateKey {
    pub d: BigUint,
    pub n: BigUint
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DistributedRSAPrivateKeySet {
    pub private_keys: Vec<DistributedRSAPrivateKey>
}

impl PlainShareSet {
    pub fn decrypt(&self) -> BigUint {
        let mut m = BigUint::from(1 as u32);

        for share in &self.plain_shares {
            m *= &share.s;
            m %= &share.n;
        }
    
        return m;
    }
}

impl DistributedRSAPrivateKey {
    pub fn generate_share(&self, c: BigUint) -> PlainShare {
        let s = c.modpow(&self.d, &self.n);
        return PlainShare { s: s, n: self.n.clone() }
    }
}

impl DistributedRSAPrivateKeySet {
    pub fn from_rsa_private_key (
            private_key: &RSAPrivateKey,
            public_key: &RSAPublicKey,
            partitions: u32,
            random_size: usize) -> Result<Self, String> {
        let mut private_keys = Vec::new();

        let n = public_key.n();

        let d = private_key.d();        
        let primes = private_key.primes();

        let one = BigUint::from(1 as u16);
        let lambda = (&primes[0].clone() - &one) * (&primes[1] - &one);

        let mut remain = d.clone();

        for _ in 0..partitions -1 {
            let random = generate_random_ubigint(random_size) % lambda.clone();
            let key = DistributedRSAPrivateKey{ 
                d: random.clone() % lambda.clone(),
                n: n.clone()
            };
            
            private_keys.push(key);

            remain = match submod(remain, random, n.clone()) {
                Ok(result) => result,
                Err(_) => { return Err("partitions and random_size too large".to_string()) }
            };
        }

        let key = DistributedRSAPrivateKey{ 
                d: remain,
                n: n.clone()
        };

        private_keys.push(key);

        Ok(DistributedRSAPrivateKeySet {
            private_keys: private_keys
        })
    }
}


#[test]
fn test_generate_private_key_set() {
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RSAPublicKey::from(&priv_key);

    let d = priv_key.d();
    let primes = priv_key.primes();

    let one = BigUint::from(1 as u16);
    let lambda = (&primes[0].clone() - &one) * (&primes[1] - &one);

    let keys = DistributedRSAPrivateKeySet::from_rsa_private_key(&priv_key, &pub_key, 10, 1024).unwrap();

    let mut sum = BigUint::from(0 as u16);
    for key in keys.private_keys {
        sum += key.d % lambda.clone();
    }

    let d = d % lambda;
    assert_eq!(sum, d);
}

#[test]
fn test_decrypt() {
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RSAPublicKey::from(&priv_key);

    let m = BigUint::from_bytes_le(b"!!");
    let n = pub_key.n();
    let e = pub_key.e();

    let c = m.modpow(e, n);

    let keys = DistributedRSAPrivateKeySet::from_rsa_private_key(&priv_key, &pub_key, 30, 1024).unwrap();

    let mut shares = Vec::new();
    for key in keys.private_keys {
        let share = key.generate_share(c.clone());
        shares.push(share);
    }

    let share_set = PlainShareSet { plain_shares: shares };

    let plain = share_set.decrypt();
    println!("{}", plain);

    assert_eq!(plain, m);
}