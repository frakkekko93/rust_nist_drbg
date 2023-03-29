pub mod drbg_demo;
pub mod utility;

use crate::mechs::ctr_mech::CtrDrbgMech;
use crate::mechs::hmac_mech::HmacDrbgMech;
use crate::mechs::hash_mech::HashDrbgMech;
use crate::demos::{utility::*, drbg_demo::*};
use aes::{Aes128, Aes192, Aes256};
use sha2::*;

pub fn run_demo() {
    let mut scelta_drbg;
    let mut user_choice: usize = 1;

    print!("\n/****************************************************************************");
    println!("****************************************************************************\\");
    println!("Welcome to a demo of this DRBG implementation. This DRBG uses all three of the mechanisms that are prescribed in NIST SP 800-90a (HMAC-DRBG, Hash-DRBG");
    println!("and CTR-DRBG). The goal of this demo is to show the capabilities of these implementations. The DRBGs that are used in this crate are supposed to have");
    println!("access to a direct entropy source that provides FULL-ENTROPY bits. This means that each DRBG can always be reseeded using fresh entropy and");
    println!("you can request prediction resistance at any time during bit generation. The DRBGs are also designed to have a reseed counter that allows for a");
    println!("limited number of consecutive generations without accessing the entropy source for fresh entropy. Once this limit has been reached, the DRBG will");
    println!("handle the reseeding by itself and you will be able to continue using the active instance.");
    println!("Each DRBG implemented here supports a security strength in the interval [112, 256]. It is suggested to request strengths that are multiples of");
    println!("8 as everything is handled in the form of bytes and eventually truncated (e.g.: sec_str=135 is equivalent to sec_str=128). Same goes for bit generation,");
    println!("no padding bits are used for requested lenghts that are not multiples of 8.");
    print!("\\****************************************************************************");
    println!("****************************************************************************/");
    println!("\nThe first step to test this design is to choose which mechanism you would like to use.");
    
    while user_choice != 0 {
        println!("-------------------------------------------------------------------------------------");
        println!("Choose between the following:");
        println!("\t1- Instantiate HMAC-DRBG");
        println!("\t2- Instantiate Hash-DRBG");
        println!("\t3- Instantiate CTR-DRBG");
        println!("\tAnything else - Interrupt the demo");
        print!("\nYour choice: ");

        scelta_drbg = get_input();

        println!("-------------------------------------------------------------------------------------");

        if scelta_drbg == 0 {
            println!("\n\nThanks for testing my drbg!");
            return;
        }

        print!("> Which security strength do you need? (must be 112 <= sec_str <= 256): ");

        let strength = get_input();

        println!("> Would you like to use a personalization string for the instantiation?\n\t1- Yes\n\t2- No");
        print!("\nYour choice: ");

        let need_ps = get_input();

        match scelta_drbg {
            1 => {
                println!("-------------------------------------------------------------------------------------");
                println!("Which mechanism would you like to use?:");
                println!("\t1- HMAC-DRBG with Sha 256 (supports a security strength of 256)");
                println!("\t2- HMAC-DRBG with Sha 256 (supports a security strength of 256)");
                println!("\tAnything else - Interrupt the demo");
                print!("\nYour choice: ");

                let mech = get_input();

                match mech {
                    1 => {
                        let res = inst_drbg::<HmacDrbgMech<Sha256>>(strength, need_ps);

                        let mut drbg;
                        match res {
                            Err(err) => {
                                match err {
                                    1 => {println!("\nInstantiation failed with error {}: inappropriate security strength (112 <= sec_str <= 256).", err);}
                                    2 => {println!("\nInstantiation failed with error {}: personalization string is too long (max sec_str bits).", err);}
                                    _ => {println!("\nInstantiation failed with error {}: instantiation of the HMAC mechanism failed.", err);}
                                }

                                continue;
                            }
                            Ok(inst) => {
                                drbg = inst;
                            }
                        }
                        user_choice = drbg_demo(&mut drbg);
                    }
                    2 => {
                        let res = inst_drbg::<HmacDrbgMech<Sha512>>(strength, need_ps);

                        let mut drbg;
                        match res {
                            Err(err) => {
                                match err {
                                    1 => {println!("\nInstantiation failed with error {}: inappropriate security strength (112 <= sec_str <= 256).", err);}
                                    2 => {println!("\nInstantiation failed with error {}: personalization string is too long (max sec_str bits).", err);}
                                    _ => {println!("\nInstantiation failed with error {}: instantiation of the HMAC mechanism failed.", err);}
                                }

                                continue;
                            }
                            Ok(inst) => {
                                drbg = inst;
                            }
                        }
                        user_choice = drbg_demo(&mut drbg);
                    }
                    _ => {
                        println!("\n\nThanks for testing my drbg!");
                        return;
                    }
                }
                
            }
            2 => {
                println!("-------------------------------------------------------------------------------------");
                println!("Which mechanism would you like to use?:");
                println!("\t1- Hash-DRBG with Sha 256 (supports a security strength of 256)");
                println!("\t2- Hash-DRBG with Sha 256 (supports a security strength of 256)");
                println!("\tAnything else - Interrupt the demo");
                print!("\nYour choice: ");

                let mech = get_input();

                match mech {
                    1 => {
                        let res = inst_drbg::<HashDrbgMech<Sha256>>(strength, need_ps);

                        let mut drbg;
                        match res {
                            Err(err) => {
                                match err {
                                    1 => {println!("\nInstantiation failed with error {}: inappropriate security strength (112 <= sec_str <= 256).", err);}
                                    2 => {println!("\nInstantiation failed with error {}: personalization string is too long (max sec_str bits).", err);}
                                    _ => {println!("\nInstantiation failed with error {}: instantiation of the HMAC mechanism failed.", err);}
                                }

                                continue;
                            }
                            Ok(inst) => {
                                drbg = inst;
                            }
                        }
                        user_choice = drbg_demo(&mut drbg);
                    }
                    2 => {
                        let res = inst_drbg::<HashDrbgMech<Sha512>>(strength, need_ps);

                        let mut drbg;
                        match res {
                            Err(err) => {
                                match err {
                                    1 => {println!("\nInstantiation failed with error {}: inappropriate security strength (112 <= sec_str <= 256).", err);}
                                    2 => {println!("\nInstantiation failed with error {}: personalization string is too long (max sec_str bits).", err);}
                                    _ => {println!("\nInstantiation failed with error {}: instantiation of the HMAC mechanism failed.", err);}
                                }

                                continue;
                            }
                            Ok(inst) => {
                                drbg = inst;
                            }
                        }
                        user_choice = drbg_demo(&mut drbg);
                    }
                    _ => {
                        println!("\n\nThanks for testing my drbg!");
                        return;
                    }
                }
            }
            3 => {
                println!("-------------------------------------------------------------------------------------");
                println!("Which mechanism would you like to use?:");
                println!("\t1- CTR-DRBG (no df) with AES 128 (supports a security strength of 128)");
                println!("\t2- CTR-DRBG (no df) with AES 192 (supports a security strength of 192)");
                println!("\t3- CTR-DRBG (no df) with AES 256 (supports a security strength of 256)");
                print!("\nYour choice: ");

                let mech = get_input();

                match mech {
                    1 => {
                        let res = inst_drbg::<CtrDrbgMech<Aes128>>(strength, need_ps);

                        let mut drbg;
                        match res {
                            Err(err) => {
                                match err {
                                    1 => {println!("\nInstantiation failed with error {}: inappropriate security strength (112 <= sec_str <= 256).", err);}
                                    2 => {println!("\nInstantiation failed with error {}: personalization string is too long (max sec_str bits).", err);}
                                    _ => {println!("\nInstantiation failed with error {}: instantiation of the HMAC mechanism failed.", err);}
                                }

                                continue;
                            }
                            Ok(inst) => {
                                drbg = inst;
                            }
                        }
                        user_choice = drbg_demo(&mut drbg);
                    }
                    2 => {
                        let res = inst_drbg::<CtrDrbgMech<Aes192>>(strength, need_ps);

                        let mut drbg;
                        match res {
                            Err(err) => {
                                match err {
                                    1 => {println!("\nInstantiation failed with error {}: inappropriate security strength (112 <= sec_str <= 256).", err);}
                                    2 => {println!("\nInstantiation failed with error {}: personalization string is too long (max sec_str bits).", err);}
                                    _ => {println!("\nInstantiation failed with error {}: instantiation of the HMAC mechanism failed.", err);}
                                }

                                continue;
                            }
                            Ok(inst) => {
                                drbg = inst;
                            }
                        }
                        user_choice = drbg_demo(&mut drbg);
                    }
                    3 => {
                        let res = inst_drbg::<CtrDrbgMech<Aes256>>(strength, need_ps);

                        let mut drbg;
                        match res {
                            Err(err) => {
                                match err {
                                    1 => {println!("\nInstantiation failed with error {}: inappropriate security strength (112 <= sec_str <= 256).", err);}
                                    2 => {println!("\nInstantiation failed with error {}: personalization string is too long (max sec_str bits).", err);}
                                    _ => {println!("\nInstantiation failed with error {}: instantiation of the HMAC mechanism failed.", err);}
                                }

                                continue;
                            }
                            Ok(inst) => {
                                drbg = inst;
                            }
                        }
                        user_choice = drbg_demo(&mut drbg);
                    }
                    _ => {
                        println!("\n\nThanks for testing my drbg!");
                        return;
                    }
                }
            }
            0 => {
                println!("\n\nThanks for testing my drbg!");
                return;
            }
            _ => {
                println!("\nInvalid choice: {}", scelta_drbg);
                continue;
            }
        }
    }
}