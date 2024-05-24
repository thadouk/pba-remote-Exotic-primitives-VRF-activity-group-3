
use rand::{Rng, rngs::OsRng};
use schnorrkel::{Keypair,Signature};
// schnorkrkel::vrf

fn main() {
    println!("Game started!");

    //https://docs.rs/schnorrkel/latest/schnorrkel/index.html#
    let p1: Keypair = Keypair::generate_with(OsRng);
    let p2: Keypair = Keypair::generate_with(OsRng);

    // Pick input for VRF... 
    

}



