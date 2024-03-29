use crate::drbg::gen_drbg::{DRBG, DRBG_Functions};
use crate::mechs::gen_mech::DRBG_Mechanism_Functions;
use crate::self_tests::formats::*;
use crate::self_tests::constants::*;

/*  Aggregator that runs all the tests in this file. */
pub fn run_tests<T: DRBG_Mechanism_Functions + 'static>(strength: usize) -> usize {
    return norm_op::<T>(strength) +
            non_empty_out_vec::<T>(strength) +
            int_state_not_valid::<T>(strength) +
            req_too_many_bytes::<T>(strength) +
            ss_not_supported::<T>(strength) +
            add_in_too_long::<T>(strength);
}

/*  Verifying that the reseed of an invalid internal state is not allowed. */
fn norm_op<T: DRBG_Mechanism_Functions + 'static>(strength: usize) -> usize{
    let res = DRBG::<T>::new(strength, None);
    let mut drbg;
    let mut bytes = Vec::<u8>::new();

    match res{
        Err(_) => {
            write_to_log(format_message(true, "DRBG_TESTS".to_string(),
                                    "generate_test".to_string(), 
                                    "failed to instantiate DRBG.".to_string()
                                )
            );
            return 1;
        }
        Ok(inst) => {
            drbg = inst;
        }
    }

    let res = drbg.generate(&mut bytes, MAX_BYTES, strength, true, Some(&ADD_IN_256[..strength]));

    return check_res(res, 0, 
        "norm_op".to_string(), 
        "DRBG_TESTS::generate_test".to_string(), 
        "generate normal operation failed.".to_string(), 
        "success on generate normal operation.".to_string());
}

/*  Verifying that an intially non-empty output vector is cleared. */
fn non_empty_out_vec<T: DRBG_Mechanism_Functions + 'static>(strength: usize) -> usize {
    let res = DRBG::<T>::new(strength, None);
    let mut drbg;
    let mut bytes = Vec::<u8>::new();
    bytes.push(0x00);

    match res{
        Err(_) => {
            write_to_log(format_message(true, "DRBG_TESTS".to_string(),
                                    "generate_test".to_string(), 
                                    "failed to instantiate DRBG.".to_string()
                                )
            );
            return 1;
        }
        Ok(inst) => {
            drbg = inst;
        }
    }

    // Making the generate fail for security strength not supported.
    drbg.generate(&mut bytes, MAX_BYTES, strength+64, false, None);

    return check_res(bytes.is_empty(), true, 
        "non_empty_out_vec".to_string(), 
        "DRBG_TESTS::generate_test".to_string(), 
        "initially non-empty out vector was not cleared before use.".to_string(), 
        "initially non-empty out vector cleared before use as expected.".to_string());
}

/*  Verifying that a generate on an invalid internal state is refused. */
fn int_state_not_valid<T: DRBG_Mechanism_Functions + 'static>(strength: usize) -> usize {
    let res = DRBG::<T>::new(strength, None);
    let mut drbg;
    let mut bytes = Vec::<u8>::new();

    match res{
        Err(_) => {
            write_to_log(format_message(true, "DRBG_TESTS".to_string(),
                                    "generate_test".to_string(), 
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
    let res = drbg.generate(&mut bytes, MAX_BYTES, strength, false, None);

    return check_res(res, 1, 
        "int_state_not_valid".to_string(), 
        "DRBG_TESTS::generate_test".to_string(), 
        "generate on invalid empty state succeeded.".to_string(), 
        "generate on invalid empty state failed as expected.".to_string());
}

/*  Verifying that a request of too many pseudo-random bytes is actually refused. */
fn req_too_many_bytes<T: DRBG_Mechanism_Functions + 'static>(strength: usize) -> usize {
    let res = DRBG::<T>::new(strength, None);
    let mut drbg;
    let mut bytes = Vec::<u8>::new();

    match res{
        Err(_) => {
            write_to_log(format_message(true, "DRBG_TESTS".to_string(),
                                    "generate_test".to_string(), 
                                    "failed to instantiate DRBG.".to_string()
                                )
            );
            return 1;
        }
        Ok(inst) => {
            drbg = inst;
        }
    }

    let res = drbg.generate(&mut bytes, NS_BYTES, strength, false, None);

    return check_res(res, 2, 
        "req_too_many_bytes".to_string(), 
        "DRBG_TESTS::generate_test".to_string(), 
        "generated too many bytes.".to_string(), 
        "refused to generate too many bytes as expected.".to_string());
}

/*  Verifying that a security strength that is not supported is actually refused. */
fn ss_not_supported<T: DRBG_Mechanism_Functions + 'static>(strength: usize) -> usize {
    let res = DRBG::<T>::new(strength, None);
    let mut drbg;
    let mut bytes = Vec::<u8>::new();

    match res{
        Err(_) => {
            write_to_log(format_message(true, "DRBG_TESTS".to_string(),
                                    "generate_test".to_string(), 
                                    "failed to instantiate DRBG.".to_string()
                                )
            );
            return 1;
        }
        Ok(inst) => {
            drbg = inst;
        }
    }

    let res = drbg.generate(&mut bytes, MAX_BYTES, strength+8, false, None);

    return check_res(res, 3, 
        "ss_not_supported".to_string(), 
        "DRBG_TESTS::generate_test".to_string(), 
        "generated bytes with unsufficient security strength.".to_string(), 
        "refused to generate bytes on not supported security strength as expected.".to_string());
}

/*  Verifying that a too long additional input is actually refused. */
fn add_in_too_long<T: DRBG_Mechanism_Functions + 'static>(strength: usize) -> usize {
    let res = DRBG::<T>::new(strength, None);
    let mut drbg;
    let mut bytes = Vec::<u8>::new();

    match res{
        Err(_) => {
            write_to_log(format_message(true, "DRBG_TESTS".to_string(),
                                    "generate_test".to_string(), 
                                    "failed to instantiate DRBG.".to_string()
                                )
            );
            return 1;
        }
        Ok(inst) => {
            drbg = inst;
        }
    }

    let res = drbg.generate(&mut bytes, MAX_BYTES, strength, false, Some(&ADD_IN_TOO_LONG));

    return check_res(res, 4, 
        "add_in_too_long".to_string(), 
        "DRBG_TESTS::generate_test".to_string(), 
        "generated bytes on additional input too long.".to_string(), 
        "refused to generate bytes on on additional input too long as expected.".to_string());
}