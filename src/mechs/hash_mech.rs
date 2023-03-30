
use super::gen_mech::DRBG_Mechanism_Functions;
use std::{any::TypeId, ops::Add};
use digest::{BlockInput, FixedOutput, Reset, Update, Digest};
use generic_array::ArrayLength;
use super::utility::*;



/*  The life of each generated seed of this DRBG. */
const SEED_LIFE: usize = 1000;

/*  Implementation of the Hash-DRBG mechanism. This mechanism can be instantiated only using Sha256 or Sha512
    (see FIPS 140-3 IG section D.R). Since both hashing algorithms support a security strength of 256 bits
    (see NIST SP 800-57pt1r5), this mechanism offers a security strength of max 256 bits.

    - v,c: internal state secret value that are used for he generation of pseudorandombits
    - count: the reseed counter
    - reseed_interval: the maximum number of generate requests that can be served between reseedings
    - zeroized: boolean flag indicating whether the particular instance has been zeroized
    - seed_len: lengths of the internal state values that depends on the hash function that is used
    - hash_fun: handle to the hash function that is used.
*/
pub struct HashDrbgMech<D: 'static>
where
    D: Update + BlockInput + FixedOutput + Default,
    D::BlockSize: ArrayLength<u8>,
    D::OutputSize: ArrayLength<u8>,
{
    v: Vec<u8>,
    c: Vec<u8>,
    count: usize,
    zeroized: bool,
    seedlen: usize,
    hash_fun: D,
    sec_str: usize,
}

/*  Implementing functions that are specific of the HMAC-DRBG mechanism. */
impl<D> HashDrbgMech<D>
where
    D: Update + FixedOutput + BlockInput + Reset + Clone + Default,
    D::BlockSize: ArrayLength<u8>,
    D::OutputSize: ArrayLength<u8>,
{
    /*  This is a derivation function used by the mechanism to generate bits from the underlying Hash function.
        (NIST SP 800-90A, section 10.3.1)
    
        Parameters:
            - result: the output vector for the generated bits
            - input: string to be hashed by the df
            - num_bytes: the number of bytes to be produced by the df.
    */
    fn hash_df(&mut self, result: &mut Vec<u8>, input: Vec<u8>, num_bytes: usize){
        // An empty output vector is required for the generated bytes.
        if !result.is_empty() {
            result.clear();
        }

        // Initial setup (step 1-2-3)
        let mut counter: u8 = 0x01;
        let string_bytes = num_bytes.to_string();

        // Generating hash_len byted at a time (step 4)
        let mut i: usize = 0;
        while i < num_bytes {
            // Hashing the input data and appending the hash to the output vector (step 4.1)
            self.hash_fun.update(counter.to_string());
            self.hash_fun.update(&string_bytes);
            self.hash_fun.update(&input);
            let hash = self.hash_fun.finalize_reset().to_vec();
            let hash_len = hash.len();
            for j in 0..hash_len {
                // The requested number of bytes has beem reached (step 5)
                if j+i >= num_bytes {
                    return;
                }

                result.push(hash[j]);
            }

            i += hash_len;

            // Updating the counter (step 4.2)
            counter = counter.add(0x01);
        }
    }

    /*  This function is used by the generation algorithm to generate pseudo-random bytes from the underlying hash function 
        (NIST SP 800-90A, section 10.1.1.4).
        
        Parameters:
            - result: the output vector for the generated bytes
            - num_bytes: the number of bytes to be generated 
    */
    fn hashgen(&mut self, result: &mut Vec<u8>, num_bytes: usize){
        // An empty output vector is required for the generated bytes
        if !result.is_empty() {
            result.clear();
        }
        
        // Initial data (step 1-2-3)
        let mut data = self.v.clone();

        // Generate the requested bytes hash_len bytes at a time (step 4)
        let mut i: usize = 0;
        while i < num_bytes {
            // Hashing the data (step 4.1)
            self.hash_fun.update(&data);
            let w = self.hash_fun.finalize_reset().to_vec();
            let hash_len = w.len();

            // Appending the hash to the output vector (step 4.2)
            for j in 0..hash_len {
                // Required number of bytes has been reached (step 5)
                if j+i >= num_bytes {
                    return;
                }

                result.push(w[j]);
            }

            // Incrementing the data for the hash (step 4.3)
            modular_add(&mut data, 1);

            // Updating the number of generated bytes
            i += hash_len;
        }
    }
}

/*  Implementing common DRBG mechanism functions taken from the DRBG_Mechanism_Functions trait (see 'gen_mech'). */
impl<D> DRBG_Mechanism_Functions for HashDrbgMech<D>
where
    D: Update + FixedOutput + BlockInput + Reset + Clone + Default,
    D::BlockSize: ArrayLength<u8>,
    D::OutputSize: ArrayLength<u8>,
{
    fn new(entropy: &[u8], nonce: &[u8], pers: &[u8], req_str: &mut usize) -> Option<Self> {
        // Runtime check on the use of any unallowed hash function.
        let seedlen;
        let this_id = TypeId::of::<D>();
        let sha256_id = TypeId::of::<sha2::Sha256>();
        let sha512_id = TypeId::of::<sha2::Sha512>();
        if this_id != sha256_id && this_id != sha512_id{
            return None;
        }
        else if this_id == sha256_id {
            seedlen = 440;
        }
        else {
            seedlen = 888;
        }

        // Security strength not supported
        if *req_str > 256 {return None}
        *req_str = 256;

        // Entropy and nonce parameters must be present.
        if entropy.len() < *req_str/8 || nonce.len() < *req_str/16 {
            return None
        }

        // Init internal state.
        let mut this = Self{ 
            v: Vec::<u8>::new(), 
            c: Vec::<u8>::new(), 
            count: 1,
            zeroized: false,
            seedlen, 
            hash_fun: D::new(),
            sec_str: *req_str,
        };

        // Derive V (step 1-2-3).
        let mut res = Vec::<u8>::new();
        let mut seed_material = entropy.clone().to_vec();
        seed_material.append(&mut nonce.to_vec());
        seed_material.append(&mut pers.to_vec());
        this.hash_df(&mut res, seed_material, seedlen/8);
        this.v.append(&mut res);

        // Derive C (step 4).
        let mut seed_material = this.v.clone();
        seed_material.insert(0, 0x00);
        this.hash_df(&mut res, seed_material, seedlen/8);
        this.c.append(&mut res);

        // Return instance (step 5-6)
        Some(this)
    }

    fn generate(&mut self, result: &mut Vec<u8>, req_bytes: usize, add: Option<&[u8]>) -> usize {
        // Eventually deleting data in result
        if !result.is_empty() {
            result.clear();
        }
        
        // No generate on a zeroized status (ERROR_FLAG=1)
        if self.zeroized {
            return 1;
        }
        
        // Reached reseed interval (ERROR_FLAG=2, step 1)
        if self.count >= SEED_LIFE{
            return 2;
        }

        // Updating internal state using additional input (step 2)
        if let Some(add) = add {
            let mut seed_material = self.v.clone();
            seed_material.insert(0, 0x02);
            seed_material.append(&mut add.to_vec());
            self.hash_fun.update(seed_material);
            let w = self.hash_fun.finalize_reset().to_vec();

            // V = (V+w) mod 2^seedlen
            let mut v_clone = self.v.clone();
            modular_add_vec(&mut v_clone, w);
            self.v.clear();
            self.v.append(&mut v_clone);
        }

        // Generating the requested bits (step 3)
        self.hashgen(result, req_bytes);

        // Updating V (step 4-5)
        let mut seed_material = self.v.clone();
        seed_material.insert(0, 0x03);
        self.hash_fun.update(seed_material);
        let w = self.hash_fun.finalize_reset().to_vec();

        // V = (V+w+C) mod 2^seedlen
        let mut v_clone = self.v.clone();
        modular_add_vec(&mut v_clone, w);
        self.v.clear();
        self.v.append(&mut v_clone);

        let mut v_clone = self.v.clone();
        modular_add_vec(&mut v_clone, self.c.clone());
        self.v.clear();
        self.v.append(&mut v_clone);

        // Updating the reseed counter (step 6)
        self.count += 1;

        0
    }

    fn reseed(&mut self, entropy: &[u8], add: Option<&[u8]>) -> usize {
        // Nothing to be done if zeroized (ERROR_FLAG returned to the application).
        if self.zeroized {
            return 1;
        }
        
        // Entropy and nonce parameters must be present.
        if entropy.len() < self.sec_str/8 {
            return 2;
        }

        // Derive V (step 1-2-3).
        let mut res = Vec::<u8>::new();
        let mut seed_material = self.v.clone();
        seed_material.insert(0, 0x01);
        seed_material.append(&mut entropy.to_vec());
        match add {
            None => {}
            Some(add_in) => {
                seed_material.append(&mut add_in.to_vec());
            }
        }
        self.hash_df(&mut res, seed_material, self.seedlen/8);
        self.v.clear();
        self.v.append(&mut res);

        // Derive C (step 4).
        res.clear();
        let mut seed_material = Vec::<u8>::new();
        seed_material.push(0x00);
        seed_material.append(&mut self.v.clone());
        self.hash_df(&mut res, seed_material, self.seedlen/8);
        self.c.clear();
        self.c.append(&mut res);

        // Re-init reseed counter (step 5).
        self.count = 1;

        0
    }

    fn zeroize(&mut self) -> usize{
        // Instance is already zeroized (ERROR_FLAG=1)
        if self.zeroized {
            return 1;
        }
        
        // Zeroizing internal state values
        for i in 0..self.v.as_slice().len() {
            self.v[i] = 0x0;
        }

        for i in 0..self.c.as_slice().len() {
            self.c[i] = 0x0;
        }

        self.count = 0;
        self.zeroized = true;
        self.seedlen = 0;
        self.hash_fun.reset();

        0
    }

    fn count(&self) -> usize {
        self.count
    }

    fn reseed_needed(&self) -> bool{
        self.count >= SEED_LIFE
    }

    fn _is_zeroized(&self) -> bool{
        self.zeroized
    }

    fn drbg_name() -> String {
        return "Hash-DRBG".to_string();
    }

    fn seed_life() -> usize {
        return SEED_LIFE;
    }
}