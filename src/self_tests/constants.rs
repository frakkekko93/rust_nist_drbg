/*  Number of bits/bytes to be generated per request during testing. */
pub const MIN_BITS: usize = 8;
pub const MAX_BITS: usize = 1024;
pub const NS_BITS: usize = 2048;
pub const MIN_BYTES: usize = MIN_BITS/8;
pub const MAX_BYTES: usize = MAX_BITS/8;
pub const NS_BYTES: usize = NS_BITS/8;

/*  Security stregths used to test DRBG and Mechs. */
pub const SEC_STR: usize = 256;
pub const NS_SEC_SRT: usize = 512;

/*  Entropy constants */
pub const ENTROPY: [u8; 32] = 
    [156, 186, 175, 146, 33, 53, 148, 237,
     178, 239, 255, 10, 79, 212, 99, 33, 
     26, 251, 9, 222, 11, 1, 191, 101, 
     255, 249, 146, 254, 26, 210, 183, 235];
pub const ENTROPY_CTR: [u8; 48] = 
    [156, 186, 175, 146, 33, 53, 148, 237,
     178, 239, 255, 10, 79, 212, 99, 33, 
     26, 251, 9, 222, 11, 1, 191, 101, 
     255, 249, 146, 254, 26, 210, 183, 235,
     239, 43, 103, 243, 3, 22, 168, 150,
     198, 204, 150, 174, 202, 171, 114, 14];
pub const ENTROPY_TOO_SHORT: [u8; 16] =  
    [156, 186, 175, 146, 33, 53, 148, 237,
     178, 239, 255, 10, 79, 212, 99, 33];

/*  Nonce constants */
pub const NONCE: [u8; 16] = 
    [16, 155, 36, 155, 57, 142, 88, 2,
     19, 20, 33, 231, 8, 252, 103, 171];
pub const NONCE_TOO_SHORT: [u8; 8] = 
    [16, 155, 36, 155, 57, 142, 88, 2];

/*  Personalization string constants */
pub const PERS: [u8; 32] = 
    [82, 141, 239, 218, 116, 11, 127, 185,
     92, 37, 138, 5, 154, 36, 172, 19,
     101, 18, 206, 96, 7, 76, 3, 241,
     254, 172, 253, 166, 182, 26, 167, 169];
pub const PERS_TOO_LONG: [u8; 33] = 
    [82, 141, 239, 218, 116, 11, 127, 185,
     92, 37, 138, 5, 154, 36, 172, 19,
     101, 18, 206, 96, 7, 76, 3, 241,
     254, 172, 253, 166, 182, 26, 167, 169,
     2];

/* Additional input constants */
pub const ADD_IN: [u8; 32] = 
    [7, 105, 103, 193, 196, 157, 39, 168,
     95, 112, 93, 23, 64, 111, 15, 106,
     93, 45, 44, 55, 59, 216, 6, 99,
     65, 216, 220, 211, 198, 7, 221, 132];
pub const ADD_IN_TOO_LONG: [u8; 33] = 
    [7, 105, 103, 193, 196, 157, 39, 168,
     95, 112, 93, 23, 64, 111, 15, 106,
     93, 45, 44, 55, 59, 216, 6, 99,
     65, 216, 220, 211, 198, 7, 221, 132,
     247];