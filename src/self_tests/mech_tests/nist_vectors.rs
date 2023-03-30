use crate::mechs::gen_mech::DRBG_Mechanism_Functions;
use crate::self_tests::formats::*;
use serde::Deserialize;

/*  The name of the test module to be printed in the log. */
const AL_NAME: &str = "MECH-TESTS::nist_vectors";

/*  This test is designed to perform KATs over some predefined vectors taken directly from NIST. */
pub fn test_vectors<T: DRBG_Mechanism_Functions>() -> usize{
    #[derive(Deserialize, Debug)]
    struct Fixture {
        name: String,
        entropy: String,
        nonce: String,
        pers: Option<String>,
        add: [Option<String>; 2],
        expected: String,
    }

    let tests: Vec<Fixture>;

    if T::drbg_name() == "Hash-DRBG" {
        // tests = serde_json::from_str(include_str!("fixtures/hash_nist_vectors.json")).unwrap();
        return 0;
    }
    else if T::drbg_name() == "HMAC-DRBG"{
        tests = serde_json::from_str(include_str!("fixtures/hmac_nist_vectors.json")).unwrap();
    }
    else {
        // tests = serde_json::from_str(include_str!("fixtures/ctr_nist_vectors.json")).unwrap();
        return 0;
    }

    for test in tests {
        let res = T::new(
            &hex::decode(&test.entropy).unwrap(),
            &hex::decode(&test.nonce).unwrap(),
            &hex::decode(&test.pers.unwrap_or("".to_string())).unwrap(),
            &mut 256
        );
        
        let mut drbg;
        match res{
            None => {
                write_to_log(format_message(true, AL_NAME.to_string(),
                                    "test_vectors".to_string(), 
                                    "failed to instantiate DRBG.".to_string()
                                )
                );
                return 1;
            }
            Some(inst) => {
                drbg = inst;
            }
        }

        let expected = hex::decode(&test.expected).unwrap();
        let mut result = Vec::new();
        let full_len = expected.len();
        let add0 = test.add[0].as_ref().map(|v| hex::decode(&v).unwrap());
        let add1 = test.add[1].as_ref().map(|v| hex::decode(&v).unwrap());

        drbg.generate(&mut result, full_len,
                               match add0 {
                                   Some(ref add0) => Some(add0.as_ref()),
                                   None => None,
                               });

        //result.clear();
        drbg.generate(&mut result, full_len,
                               match add1 {
                                   Some(ref add1) => Some(add1.as_ref()),
                                   None => None,
                               });
        
        if check_res(result, expected, test.name, AL_NAME.to_string(), 
            "failed nist vector generation.".to_string(),
            "completed nist vector generation.".to_string()) != 0 {
            return 1;
        }
    }

    write_to_log(format_message(false, AL_NAME.to_string(),
                                                            "test_vectors".to_string(), 
                                                            "all nist vectors have passed.".to_string())
    );

    return 0;
}