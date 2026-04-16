use hmac::{Hmac, KeyInit, Mac};
use num_bigint::{BigInt, Sign};
use sha2::Sha256;

use crate::{crypto::{s256_point::{G, N, S256Point}, secret_field::SecretField, signature::Signature, to_32_bytes::to_32bytes_vec_big_endian}, elliptic_curve::ecc_point::Point, finite_fields::modulo_helper::{modulo, pow_modulo}};

#[derive(Debug, Clone)]
pub struct PrivateKey {
    pub secret: BigInt,
    pub point: S256Point
}

impl PrivateKey {
    pub fn new<T: SecretField>(input: T) -> Self {
        let g = G;
        let secret = input.into_bigint();
        let point = (*g).clone() * secret.clone();
        Self { secret, point }
    }
    pub fn hex(self) -> String {
        let width = 64;
        format!("{:0width$x}", self.secret, width = width)
    }

    // z is the message (BigInt) to sign
    pub fn sign(&self, z: BigInt) -> Signature {
        // let k = BigInt::from(58566871483907786640881324537349845131589498124957989066426190658802793240507); // not random but deterministic
        let k = self.clone().deterministic(z.clone());
        let g = G;
        let n = N;
        let r = if let Point::Finite { x, y:_, a:_, b:_ } = ((*g).clone() * k.clone()).inner {
            x.inner.num
        } else { panic!("No r point found to sign") };
        let k_inv = pow_modulo(k.clone(), (*n).clone() - 2, (*n).clone());
        let mut s = modulo((z.clone() + (&self.secret * r.clone())) * k_inv, (*n).clone());
        if s > (*n).clone() / 2 {
            s = (*n).clone() - s;
        }
        Signature::new(r, s)
    }

    pub fn deterministic(self, z: BigInt) -> BigInt {
        let n = N;
        let mut z = z;
        let mut k = vec![0x00u8;32];
        let mut v = vec![0x01u8;32];
        if z > (*n).clone() {
            z -= (*n).clone();
        }

        let z_bytes = to_32bytes_vec_big_endian(&z);
        let secret_bytes = to_32bytes_vec_big_endian(&self.secret);
        
        // for k
        let mut k_hmac = Hmac::<Sha256>::new_from_slice(&k).unwrap();
        k_hmac.update(&v);
        k_hmac.update(&[0x00]);
        k_hmac.update(&secret_bytes);
        k_hmac.update(&z_bytes);
        k = k_hmac.finalize().into_bytes().to_vec();

        // for v
        let mut v_hmac = Hmac::<Sha256>::new_from_slice(&k).unwrap();
        v_hmac.update(&v);  
        v = v_hmac.finalize().into_bytes().to_vec();

        // again for k
        let mut k_hmac = Hmac::<Sha256>::new_from_slice(&k).unwrap();
        k_hmac.update(&v);
        k_hmac.update(&[0x01]);
        k_hmac.update(&secret_bytes);
        k_hmac.update(&z_bytes);
        k = k_hmac.finalize().into_bytes().to_vec();

        // again for v
        let mut v_hmac = Hmac::<Sha256>::new_from_slice(&k).unwrap();
        v_hmac.update(&v);  
        v = v_hmac.finalize().into_bytes().to_vec();

        loop {
            let mut v_hmac = Hmac::<Sha256>::new_from_slice(&k).unwrap();
            v_hmac.update(&v);  
            v = v_hmac.finalize().into_bytes().to_vec();

            let candidate = BigInt::from_bytes_be(Sign::Plus, &v);
            if candidate.clone() >= BigInt::from(1) && candidate.clone() < (*n).clone() {
                break;
            }

            let mut k_hmac = Hmac::<Sha256>::new_from_slice(&k).unwrap();
            k_hmac.update(&v);
            k_hmac.update(&[0x00]);
            k = k_hmac.finalize().into_bytes().to_vec();

            let mut v_hmac = Hmac::<Sha256>::new_from_slice(&k).unwrap();
            v_hmac.update(&v);  
            v = v_hmac.finalize().into_bytes().to_vec();
        }
        BigInt::from_bytes_be(Sign::Plus, &v)
    }
}
