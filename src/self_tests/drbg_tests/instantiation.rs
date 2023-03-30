use crate::drbgs::gen_drbg::{DRBG, DRBG_Functions};
use crate::mechs::gen_mech::DRBG_Mechanism_Functions;
use crate::self_tests::formats::*;

/*  Aggregator that runs all the tests in this file. */
pub fn run_tests<T: DRBG_Mechanism_Functions>() -> usize {
    return norm_op::<T>() +
            test_ss_not_supported::<T>() +
            ps_is_too_long::<T>();
}

/*  Testing that any security strength that is <=MAX_STR is actually accepted by the DRBG. */
fn norm_op<T: DRBG_Mechanism_Functions>() -> usize{
    let res = DRBG::<T>::new(256, Some("Trial pers".as_bytes()));
    let mut drbg = None;

    match res{
        Err(_) => {}
        Ok(inst) => {
            drbg = Some(inst);
        }
    }

    if check_res(drbg.is_none(), false, 
    "norm_op".to_string(), 
    "DRBG_TESTS::instantiation_test".to_string(), 
    "failed to instantiate DRBG using a supported security strength.".to_string(), 
    "succeeded to instantiate DRBG using a supported security strength as expected.".to_string()) != 0{
        return 1;
    }
    0
}

/*  Testing that not supported security strengths are actually rejected by the DRBG. */
fn test_ss_not_supported<T: DRBG_Mechanism_Functions>() -> usize{
    let res = DRBG::<T>::new(512, None);
    let mut err= 0;
    let mut drbg = None;

    match res{
        Err(error) => {
            err = error;
        }
        Ok(inst) => {
            drbg = Some(inst);
        }
    }

    if check_res((err, true), (1, drbg.is_none()), 
    "test_ss_not_supported".to_string(), 
    "DRBG_TESTS::instantiation_test".to_string(), 
    "succeeded to instantiate DRBG using not supported security strength.".to_string(), 
    "failed to instantiate DRBG using not supported security strength as expected.".to_string()) != 0{
        return 1;
    }
    0
}

/*  Testing that the limit on the length of the personalization string is actually enforced. */
fn ps_is_too_long<T: DRBG_Mechanism_Functions>() -> usize{
    let ps: [u8; 33] = [0; 33];
    let res = DRBG::<T>::new(256, Some(&ps));
    let mut err= 0;
    let mut drbg = None;

    match res{
        Err(error) => {
            err = error;
        }
        Ok(inst) => {
            drbg = Some(inst);
        }
    }

    if check_res((err, true), (2, drbg.is_none()), 
    "ps_is_too_long".to_string(), 
    "DRBG_TESTS::instantiation_test".to_string(), 
    "succeeded to instantiate DRBG using not supported personalization string.".to_string(), 
    "failed to instantiate DRBG using not supported personalization string as expected.".to_string()) != 0{
        return 1;
    }
    0
}

// fn fail_test() -> usize {
//     return check_res(1, 0, 
//     "fail_test".to_string(), 
//     "DRBG_TESTS::instantiation_test".to_string(), 
//     "this test always fails.".to_string(), 
//     "cannot happen.".to_string());
// }