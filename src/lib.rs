use num_bigint::{BigUint, RandBigInt};
use rand;

pub struct ZKP {
    p: BigUint,
    q: BigUint,
    alpha: BigUint,
    beta: BigUint,
}

impl ZKP {
    /// output = n^exp mod p
    pub fn exponentiate(n: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
        n.modpow(exponent, modulus)
    }

    /// output = s = k- c*x
    pub fn solve(&self, k: &BigUint, c: &BigUint, x: &BigUint) -> BigUint {
        if *k >= c * x {
            return (k - c*x).modpow(&BigUint::from(1u32), &self.q);
        }
        return &self.q - (c*x - k).modpow(&BigUint::from(1u32), &self.q);
    }

    /// cond1: r1 = alpha^s * y1^c
    /// cond2: r2 = beta^s * y2^c
    pub fn verify(&self, r1: &BigUint, r2: &BigUint, y1: &BigUint, y2: &BigUint, c: &BigUint, s: &BigUint) -> bool {
        let cond1 = *r1 == (&self.alpha.modpow(s, &self.p) * y1.modpow(c, &self.p)).modpow(&BigUint::from(1u32), &self.p);
        let cond2 = *r2 == (&self.beta.modpow(s, &self.p) * y2.modpow(c, &self.p)).modpow(&BigUint::from(1u32), &self.p);
        cond1 && cond2
    }

    pub fn generate_rand_below(bound: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();

        rng.gen_biguint_below(bound)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_toy_example() {
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);

        let zkp = ZKP{p, q, alpha, beta};

        let x = BigUint::from(6u32);
        let k = BigUint::from(7u32);

        let c = BigUint::from(4u32);

        let y1 = ZKP::exponentiate(&zkp.alpha, &x, &zkp.p);
        let y2 = ZKP::exponentiate(&zkp.beta, &x, &zkp.p);
        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        let r1 = ZKP::exponentiate(&zkp.alpha, &k, &zkp.p);
        let r2 = ZKP::exponentiate(&zkp.beta, &k, &zkp.p);
        assert_eq!(r1, BigUint::from(8u32));
        assert_eq!(r2, BigUint::from(4u32));

        let s = zkp.solve(&k, &c, &x);
        assert_eq!(s, BigUint::from(5u32));

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result);

        let x_fake = BigUint::from(7u32);
        let s_fake = zkp.solve(&k, &c, &x_fake);

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s_fake);
        assert!(!result);
    }

    #[test]
    fn test_toy_example_with_random_numbers() {
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);

        let zkp = ZKP{p, q, alpha, beta};

        let x = BigUint::from(6u32);
        let k = ZKP::generate_rand_below(&zkp.q);

        let c = ZKP::generate_rand_below(&zkp.q);

        let y1 = ZKP::exponentiate(&zkp.alpha, &x, &zkp.p);
        let y2 = ZKP::exponentiate(&zkp.beta, &x, &zkp.p);
        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        let r1 = ZKP::exponentiate(&zkp.alpha, &k, &zkp.p);
        let r2 = ZKP::exponentiate(&zkp.beta, &k, &zkp.p);

        let s = zkp.solve(&k, &c, &x);

        let result = zkp.verify(&r1, &r2, &y1, &y2,&c, &s);
        assert!(result);
    }

    #[test]
    fn test_1024_bit_constants() {
        // https://datatracker.ietf.org/doc/html/rfc5114#section-2.1

        let p_hex = hex::decode("B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B616073E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCBDD7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371").expect("coudl not convert to hex");
        let p = BigUint::from_bytes_be(&p_hex);

        let q_hex = hex::decode("F518AA8781A8DF278ABA4E7D64B7CB9D49462353").expect("could not convert to hex");
        let q = BigUint::from_bytes_be(&q_hex);

        let alpha_hex = hex::decode("A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5").expect("could not convert to hex");
        let alpha = BigUint::from_bytes_be(&alpha_hex);

        // alpha^i is also a generator
        let beta = alpha.modpow(&ZKP::generate_rand_below(&q), &p);
       
        let zkp = ZKP{p, q, alpha, beta};
        

        let x = ZKP::generate_rand_below(&zkp.q);
        let k = ZKP::generate_rand_below(&zkp.q);
        let c = ZKP::generate_rand_below(&zkp.q);

        let y1 = ZKP::exponentiate(&zkp.alpha, &x, &zkp.p);
        let y2 = ZKP::exponentiate(&zkp.beta, &x, &zkp.p);

        let r1 = ZKP::exponentiate(&zkp.alpha, &k, &zkp.p);
        let r2 = ZKP::exponentiate(&zkp.beta, &k, &zkp.p);

        let s = zkp.solve(&k, &c, &x);

        let result = zkp.verify(&r1, &r2, &y1, &y2,&c, &s);
        assert!(result);
    }

}