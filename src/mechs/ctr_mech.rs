use super::gen_mech::DRBG_Mechanism_Functions;
use generic_array::ArrayLength;
use std::any::TypeId;
use super::utility::*;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

/*  The life of each generated seed of this DRBG. */
const SEED_LIFE: usize = 1000;

/*  The length of the counter used by the block cipher in bytes. */
const CTR_LEN: usize = 4;

/*  Implementation of the CTR-DRBG mechanisms without the use of a DF as specified in section 10.2.1 of NIST SP 800-90A.
    According to NIST SP 800-57 AES 128/192/256 support security strengths of respectively 128/192/256 bits. Thus, since this
    implementation supports every one of these block ciphers, it also can support any security strength in the range [128, 256].
    
    - k: key of the underlying block cipher
    - v: vector used for block encryptions
    - count: reseed counter
    - zeroized: indicates whether the instance has been zeroized (a new instance is needed)
    - seedlen: length of the parameters used by this mechanism (=> blocklen + keylen)
    - blocklen: length of the input/output blocks of the block cipher
    - keylen: length of the key of the blockcipher */
pub struct CtrDrbgMech<D: 'static>
where
    D: BlockCipher + BlockEncrypt + BlockDecrypt + KeyInit,
    D::BlockSize: ArrayLength<u8>,
    D::KeySize: ArrayLength<u8>,
{
    k: GenericArray<u8, D::KeySize>,
    v: GenericArray<u8, D::BlockSize>,
    count: usize,
    zeroized: bool,
    seedlen: usize,
    blocklen: usize,
    keylen: usize,
}

/*  Implementing functions that are specific of the CTR-DRBG mechanism. */
impl<D> CtrDrbgMech<D>
where
    D: BlockCipher + BlockEncrypt + BlockDecrypt + KeyInit,
    D::BlockSize: ArrayLength<u8>,
    D::KeySize: ArrayLength<u8>,
{
    /*  This function is used to update the internal state of the CTR-DRBG.
        (see NIST SP 800-90A, section 10.2.1.2)
        
        Parameters:
            - provided_data: the data to be used for the update (exaclty seedlen bytes) */
    fn update(&mut self, provided_data: &Vec<u8>) {
        // Provided data must not be empty and must be seedlen long
        if provided_data.is_empty() || provided_data.len() != self.seedlen {
            return;
        }

        // Init local variables (step 1)
        let mut temp = Vec::<u8>::new();
        let cipher = self.block_cipher();

        // Fill temporary vector block by block until seedlen is reached (step 2)
        let mut i: usize = 0;
        while i < self.seedlen {
            // Appropriately increment the counter based on his size (step 2.1)
            if CTR_LEN < self.blocklen {
                let mid_point = self.blocklen - CTR_LEN;
                
                // Increment the rigth-most CTR_LEN bytes of V (step 2.1.1)
                let mut right_v = self.v[mid_point..].to_vec();
                modular_add(&mut right_v, 0x01);

                // Creating a clone of V with the incremented right-most CTR_LEN bytes
                let mut v_clone = GenericArray::<u8, D::BlockSize>::default();
                let (left, right) = v_clone.split_at_mut(mid_point);
                left.clone_from_slice(&self.v[..mid_point]);
                right.clone_from_slice(&right_v.as_slice());

                // Update V (step 2.1.2)
                self.v.clone_from(&v_clone);
            }
            else {
                // Increment V (step 2.1 alternative)
                let mut v_clone = self.v.to_vec();
                modular_add(&mut v_clone, 0x01);

                // Update V
                self.v.clone_from_slice(&v_clone);
            }

            // Encrypt V (step 2.2)
            let mut block = self.v.clone();
            cipher.encrypt_block(&mut block);

            // Append encrypted block to temporary vector (step 2.3)
            temp.append(&mut block.to_vec());

            // Increment counter
            i += self.blocklen;
        }

        // Taking only seedlen bytes (step 3)
        temp.resize(self.seedlen, 0x00);

        // Performing temp XOR provided_data (step 4)
        xor_vecs(&mut temp, provided_data);

        // Update K (step 5)
        self.k.clone_from_slice(&temp[..self.keylen]);

        // Update V (step 6)
        self.v.clone_from_slice(&temp[self.keylen..]);
    }

    /*  Retrieves and instance of the hmac primitive that uses self.k as a key.
    
        Return values:
            - a pointer to an hmac primitive */
    fn block_cipher(&self) -> D {
        D::new(&self.k)
    }

    /*  Takes a vector in input and adjusts it to be exactly seedlen bytes long. If a shorter (or empty) vector is received
        0's padding is added. */
    fn to_be_len(vec: &[u8], len: usize) -> Vec<u8>{
        let mut res_vec = vec.clone().to_vec();

        res_vec.resize(len, 0x00);

        res_vec
    }
}

/*  Implementing common DRBG mechanism functions taken from the DRBG_Mechanism_Functions trait (see 'gen_mech'). */
impl<D> DRBG_Mechanism_Functions for CtrDrbgMech<D>
where
    D: BlockCipher + BlockEncrypt + BlockDecrypt + KeyInit,
    D::BlockSize: ArrayLength<u8>,
    D::KeySize: ArrayLength<u8>,
{   
    /*  This function is implemented following the algorithm described at 10.2.1.3.2 for a CTR-DRBG that doesn't use a df. */
    fn new(entropy: &[u8], _nonce: &[u8], pers: &[u8], req_str: &mut usize) -> Option<Self> {
        let seed_len: usize;
        let key_len: usize;
        let block_len: usize = 16;

        // Runtime check on the use of any unallowed hash function and according parameter setup.
        let this_id = TypeId::of::<D>();
        let aes128_id = TypeId::of::<aes::Aes128>();
        let aes192_id = TypeId::of::<aes::Aes192>();
        let aes256_id = TypeId::of::<aes::Aes256>();

        if this_id == aes128_id {
            if *req_str > 16 {return None}
            key_len = 16;
            *req_str = 16;
        }
        else if this_id == aes192_id {
            if *req_str > 24 {return None}
            key_len = 24;
            *req_str = 24;
        }
        else if this_id == aes256_id {
            if *req_str > 32 {return None}
            key_len = 32;
            *req_str = 32;
        }
        else {return None;}
        seed_len = block_len + key_len;
        
        // Entropy parameter must be present and of seedlen bytes.
        let mut new_entropy = Vec::<u8>::new();
        if entropy.len() >= seed_len {
            new_entropy.append(&mut entropy[..seed_len].to_vec());
        }
        else {
            return None;
        }

        // Taking exactly seedlen bytes from the PS that has been passed (step 1,2).
        // If an empty pers is received we will use 0^seedlen as pers.
        let new_pers = CtrDrbgMech::<D>::to_be_len(pers, seed_len);

        // Setting initial values for the internal state (step 4,5,7).
        let mut k = GenericArray::<u8, D::KeySize>::default();
        let mut v = GenericArray::<u8, D::BlockSize>::default();

        for i in 0..k.as_slice().len() {
            k[i] = 0x0;
        }

        for i in 0..v.as_slice().len() {
            v[i] = 0x0;
        }

        let mut this = Self{
            k,
            v,
            count: 1,
            zeroized: false,
            seedlen: seed_len,
            blocklen: block_len,
            keylen: key_len,
        };

        // Updating the internal state using the entropy and given personalization string (step 3,6)
        let mut seed_material = new_entropy.clone();
        xor_vecs(&mut seed_material, &new_pers);
        this.update(&seed_material);

        // Returning a reference to this instance (step 8)
        Some(this)
    }

    /*  This function is implemented following the algorithm described at 10.2.1.5.1 for a CTR-DRBG that doesn't use a df. */
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

        // Restricting add-in to be of seedlen bytes and eventually using 0^seedlen if add is None (step 2)
        let mut new_add_in = Vec::<u8>::new();
        match add {
            None => {
                for _i in 0..self.seedlen {
                    new_add_in.push(0x00);
                }
            }
            Some(add_in) => {
                new_add_in = CtrDrbgMech::<D>::to_be_len(add_in, self.seedlen);

                self.update(&new_add_in);
            }
        }

        // Generating blocklen bytes at a time using the underlying block cipher (step 3,4).
        let cipher = self.block_cipher();
        let mut i: usize = 0;
        while i < req_bytes {
            // Appropriately increment the counter based on his size (step 4.1)
            if CTR_LEN < self.blocklen {
                let mid_point = self.blocklen - CTR_LEN;

                // Increment the rigth-most CTR_LEN bytes of V (step 4.1.1)
                let mut right_v = self.v[mid_point..].to_vec();
                modular_add(&mut right_v, 0x01);

                // Creating a clone of V with the incremented right-most CTR_LEN bytes
                let mut v_clone = GenericArray::<u8, D::BlockSize>::default();
                let (left, right) = v_clone.split_at_mut(mid_point);
                left.clone_from_slice(&self.v[..mid_point]);
                right.clone_from_slice(&right_v.as_slice());

                // Update V (step 4.1.2)
                self.v.clone_from(&v_clone);
            }
            else {
                // Increment V (step 4.1 alternative)
                let mut v_clone = self.v.to_vec();
                modular_add(&mut v_clone, 0x01);

                // Update V
                self.v.clone_from_slice(&v_clone);
            }

            // Encrypt V (step 4.2)
            let mut block = self.v.clone();
            cipher.encrypt_block(&mut block);

            // Append encrypted block to temporary vector (step 4.3)
            result.append(&mut block.to_vec());

            // Increment counter
            i += self.blocklen;
        }

        // Taking only req_bytes (step 5)
        result.resize(req_bytes, 0x00);

        // Updating internal state (step 6)
        self.update(&new_add_in);

        // Incrementing reseed counter (step 7)
        self.count += 1;

        0
    }

    /*  This function is implemented following the algorithm described at 10.2.1.4.1 for a CTR-DRBG that doesn't use a df. */
    fn reseed(&mut self, entropy: &[u8], add: Option<&[u8]>) -> usize {
        // Nothing to be done if zeroized (ERROR_FLAG returned to the application).
        if self.zeroized {
            return 1;
        }

        // Taking exactly seedlen bytes from the AI that has been passed (step 1,2).
        // If an empty add is received we will use 0^seedlen as additional input.
        let mut new_add_in = Vec::<u8>::new();
        match add {
            None => {
                for _i in 0..self.seedlen {
                    new_add_in.push(0x00);
                }
            }
            Some(add_in) => {
                new_add_in = CtrDrbgMech::<D>::to_be_len(add_in, self.seedlen);
            }
        }

        // Entropy parameter must be present and of seedlen bytes.
        let mut new_entropy = Vec::<u8>::new();
        if entropy.len() >= self.seedlen {
            new_entropy.append(&mut entropy[..self.seedlen].to_vec());
        }
        else {
            return 2;
        }

        // Updating the internal state using the entropy and given additional input (step 3,4)
        let mut seed_material = new_entropy.to_vec();
        xor_vecs(&mut seed_material, &new_add_in);
        self.update(&seed_material);

        // Resetting the reseed counter (step 5)
        self.count = 1;

        0
    }

    fn zeroize(&mut self) -> usize{
        // Instance is already zeroized (ERROR_FLAG=1)
        if self.zeroized {
            return 1;
        }

        // Zeroizing internal state values
        for i in 0..self.k.as_slice().len() {
            self.k[i] = 0x0;
        }

        for i in 0..self.v.as_slice().len() {
            self.v[i] = 0x0;
        }

        self.count = 0;
        self.seedlen = 0;
        self.keylen = 0;
        self.blocklen = 0;
        self.zeroized = true;
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
        return "CTR-DRBG".to_string();
    }

    fn seed_life() -> usize {
        return SEED_LIFE;
    }
}