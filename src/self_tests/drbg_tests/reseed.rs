use crate::drbgs::gen_drbg::{DRBG, DRBG_Functions};
use crate::mechs::gen_mech::DRBG_Mechanism_Functions;
use crate::self_tests::formats::*;

/*  Aggregator that runs all the tests in this file. */
pub fn run_tests<T: DRBG_Mechanism_Functions>() -> usize {
    return internal_state_not_valid::<T>() +
            add_in_too_long::<T>() +
            norm_op::<T>();
}

/*  Verifying normal reseed operation. */
fn norm_op<T: DRBG_Mechanism_Functions>() -> usize {
    let res = DRBG::<T>::new(256, None);
    let mut drbg;
    let add_in: [u8; 32] = [0; 32];

    match res{
        Err(_) => {
            write_to_log(format_message(true, "HMAC-DRBG".to_string(),
                                    "reseed_test".to_string(), 
                                    "failed to instantiate DRBG.".to_string()
                                )
            );
            return 1;
        }
        Ok(inst) => {
            drbg = inst;
        }
    }

    let res = drbg.reseed(Some(add_in.as_slice()));

    return check_res(res, 0, 
        "add_in_too_long".to_string(), 
        "reseed_test".to_string(), 
        "reseed normal operation failed.".to_string(), 
        "success on reseed normal operation.".to_string());
}

/*  Verifying that the reseed of an invalid internal state is not allowed. */
fn internal_state_not_valid<T: DRBG_Mechanism_Functions>() -> usize{
    let res = DRBG::<T>::new(256, None);
    let mut drbg;

    match res{
        Err(_) => {
            write_to_log(format_message(true, "HMAC-DRBG".to_string(),
                                    "reseed_test".to_string(), 
                                    "failed to instantiate DRBG.".to_string()
                                )
            );
            return 1;
        }
        Ok(inst) => {
            drbg = inst;
        }
    }
    
    drbg.uninstantiate();

    let res = drbg.reseed(None);

    return check_res(res, 1, 
        "internal_state_not_valid".to_string(), 
        "reseed_test".to_string(), 
        "error expected on reseed of empty internal state.".to_string(), 
        "reseed of empty internal state failed es expected.".to_string());
}

/*  Verifying that additional inputs that are too long are rejected. */
fn add_in_too_long<T: DRBG_Mechanism_Functions>() -> usize {
    let res = DRBG::<T>::new(256, None);
    let mut drbg;
    let add_in: [u8; 33] = [0; 33];

    match res{
        Err(_) => {
            write_to_log(format_message(true, "HMAC-DRBG".to_string(),
                                    "reseed_test".to_string(), 
                                    "failed to instantiate DRBG.".to_string()
                                )
            );
            return 1;
        }
        Ok(inst) => {
            drbg = inst;
        }
    }

    let res = drbg.reseed(Some(add_in.as_slice()));

    return check_res(res, 2, 
        "add_in_too_long".to_string(), 
        "reseed_test".to_string(), 
        "error expected on additional input too long.".to_string(), 
        "reseed on additional input too long failed es expected.".to_string());
}