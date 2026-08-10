// use std::collections;

use num_bigint::{BigUint, RandBigInt};
use rand::{self, Rng};
use hex;

pub struct ZKP{
    pub p:BigUint,
    pub q:BigUint,
    pub alpha:BigUint,
    pub beta:BigUint,
}




impl ZKP{
    // Defining all the function
    /// alpha^x mod p
    pub fn exponentiate(n:&BigUint, exponent:&BigUint, modulus:&BigUint) -> BigUint{
        n.modpow(exponent, modulus)
    }
    pub fn new(alpha: BigUint, beta: BigUint, p: BigUint, q: BigUint) -> Self {
        ZKP { p, q, alpha, beta }
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

    pub fn generate_random_stirng_below(size :usize)-> String{
        rand::thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(size)
            .map(char::from)
            .collect()
    }
    pub fn get_constants() -> (BigUint, BigUint, BigUint, BigUint){
        let p = hex::decode(
        "B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C6\
            9A6A9DCA52D23B616073E28675A23D189838EF1E2EE652C0\
            13ECB4AEA906112324975C3CD49B83BFACCBDD7D90C4BD70\
            98488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0\
            A151AF5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708\
            DF1FB2BC2E4A4371"
        ).unwrap();
        let p = BigUint::from_bytes_be(&p);
        let q = hex::decode("F518AA8781A8DF278ABA4E7D64B7CB9D49462353").unwrap();
        let q = BigUint::from_bytes_be(&q);
       

        // let alpha = BigUint::from(4u32);
        let alpha = hex::decode("A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5").unwrap();
        let alpha = BigUint::from_bytes_be(&alpha);
        // let beta = BigUint::from(9u32);
        let exp = hex::decode("3B9A92EE1909D0D2263F80").unwrap();
        let exp = BigUint::from_bytes_be(&exp);
        let beta = alpha.modpow(&exp, &p);

        (alpha, beta, p, q)
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

    // 1024 -bit unit test 
    #[test]
    fn test_1024_bits_constants(){

        let p = hex::decode(
        "B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C6\
            9A6A9DCA52D23B616073E28675A23D189838EF1E2EE652C0\
            13ECB4AEA906112324975C3CD49B83BFACCBDD7D90C4BD70\
            98488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0\
            A151AF5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708\
            DF1FB2BC2E4A4371"
        ).unwrap();
        let p = BigUint::from_bytes_be(&p);
        let q = hex::decode("F518AA8781A8DF278ABA4E7D64B7CB9D49462353").unwrap();
        let q = BigUint::from_bytes_be(&q);
       

        // let alpha = BigUint::from(4u32);
        let alpha = hex::decode("A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5").unwrap();
        let alpha = BigUint::from_bytes_be(&alpha);
        // let beta = BigUint::from(9u32);
        let beta = alpha.modpow(&BigUint::from(9284924u32), &p);
        // let q = BigUint::from(11u32);
        // let zkp = ZKP {p,q,alpha, beta};

        let x = BigUint::from(6u32);
        let k = ZKP::generate_random_less_than(&q);

        let c = ZKP::generate_random_less_than(&q);

        let y1 = ZKP::exponentiate(&alpha, &x, &p);
        let y2 = ZKP::exponentiate(&beta, &x, &p);

        // Testing by toy expample
        // assert_eq!(y1, BigUint::from(2u32));
        // assert_eq!(y2, BigUint::from(3u32));

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