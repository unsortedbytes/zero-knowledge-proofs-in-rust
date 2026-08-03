use num_bigint::{BigUint, RandBigInt};
use rand;

pub struct ZKP{
    p:BigUint,
    q:BigUint,
    alpha:BigUint,
    beta:BigUint,
}

impl ZKP{
    // Defining all the function
    /// alpha^x mod p
    pub fn exponentiate(n:&BigUint, exponent:&BigUint, modulus:&BigUint) -> BigUint{
        n.modpow(exponent, modulus)
    }

    // Solve
    /// S = K-C*X mod p;
    pub fn solve(&self, k: &BigUint, c: &BigUint, x: &BigUint) -> BigUint {
        let cx = (c * x) % &self.q;
        let k_mod = k % &self.q;

        if k_mod >= cx {
            (k_mod - cx) % &self.q
        } else {
            (&self.q + k_mod - cx) % &self.q
        }
    }

    // Verify
    /// r1 = alpha^s *y1^c
    /// r2 = beta^s *y2^c
    pub fn verify(&self, r1: &BigUint, r2:&BigUint, y1: &BigUint, y2:&BigUint, s:&BigUint, c:&BigUint) -> bool{
        let cond1 =r1 % &self.p== (&self.alpha.modpow(s, &self.p) * y1.modpow(c,&self.p)) % &self.p;
        let cond2 =r2 % &self.p== (&self.beta.modpow(s, &self.p) * y2.modpow(c,&self.p)) % &self.p;

        cond1 && cond2

    }

    pub fn generate_random_less_than(bound: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng(); 
        rng.gen_biguint_below(bound)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_toy_expample(){
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);

        // let zkp = ZKP {p,q,alpha, beta};

        let x = BigUint::from(6u32);
        let k = BigUint::from(7u32);

        let c = BigUint::from(4u32);

        let y1 = ZKP::exponentiate(&alpha, &x, &p);
        let y2 = ZKP::exponentiate(&beta, &x,  &p);

        // Testing by toy expample
        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        let r1 = ZKP::exponentiate(&alpha, &k, &p);
        let r2 = ZKP::exponentiate(&beta, &k, &p);

        // Testing by toy expample
        assert_eq!(r1, BigUint::from(8u32));
        assert_eq!(r2, BigUint::from(4u32));

        let zkp = ZKP {p,q,alpha, beta};
        let s = zkp.solve(&k, &c, &x,);
        assert_eq!(s, BigUint::from(5u32));

        let result = zkp.verify(&r1, &r2, &y1,  &y2, &s, &c);
        assert!(result);
    }

    #[test]
    fn test_toy_expample_with_random_numbers(){
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);
        // let zkp = ZKP {p,q,alpha, beta};

        let x = BigUint::from(6u32);
        let k = ZKP::generate_random_less_than(&q);

        let c = ZKP::generate_random_less_than(&q);

        let y1 = ZKP::exponentiate(&alpha, &x, &p);
        let y2 = ZKP::exponentiate(&beta, &x, &p);

        // Testing by toy expample
        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        let r1 = ZKP::exponentiate(&alpha, &k, &p);
        let r2 = ZKP::exponentiate(&beta, &k, &p);

        // Testing by toy expample
        // assert_eq!(r1, BigUint::from(8u32));
        // assert_eq!(r2, BigUint::from(4u32));
        let zkp = ZKP {p,q,alpha, beta};
        let s = zkp.solve(&k, &c, &x,);
        // assert_eq!(s, BigUint::from(5u32));

        let result = zkp.verify(&r1, &r2, &y1, &y2, &s, &c);
        assert!(result);
    }
}


/*     // Defining all the function
    /// alpha^x mod p
    pub fn exponentiate(n:&BigUint, exponent:&BigUint, modulus:&BigUint) -> BigUint{
        n.modpow(exponent, modulus)
    }

    // Solve
    /// S = K-C*X mod p;
    pub fn solve(k: &BigUint, c: &BigUint, x: &BigUint, q: &BigUint) -> BigUint {
        let cx = (c * x) % q;
        let k_mod = k % q;

        if k_mod >= cx {
            (k_mod - cx) % q
        } else {
            (q + k_mod - cx) % q
        }
    }

    // Verify
    /// r1 = alpha^s *y1^c
    /// r2 = beta^s *y2^c
    pub fn verify(r1: &BigUint, r2:&BigUint, y1: &BigUint, y2:&BigUint, alpha:&BigUint, beta:&BigUint, s:&BigUint, c:&BigUint, q:&BigUint) -> bool{
        let cond1 =r1%q== (alpha.modpow(s, q) * y1.modpow(c,q)) %q;
        let cond2 =r2%q== (beta.modpow(s, q) * y2.modpow(c,q)) %q;

        cond1 && cond2

    }

    pub fn generate_random_less_than(bound: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng(); 
        rng.gen_biguint_below(bound)
    } */