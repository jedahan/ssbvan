use ed25519_dalek::Keypair;

use ed25519_dalek::PUBLIC_KEY_LENGTH as LEN;

use rand::prelude::*;
use sha2::Sha512;
use base64::encode;

fn main() {
    let mut csprng: ThreadRng = thread_rng();
    let mut iters: u32 = 0;

    let keypair = loop {
        iters += 1;
        let keypair: Keypair = Keypair::generate::<Sha512, _>(&mut csprng);

        let pubkey = encode(&keypair.public.to_bytes());

        let begin = &pubkey[0..4];
        let _end = &pubkey[LEN-4..LEN];

        // should this be to_base64?
        if begin.to_lowercase() == "dead" {
            break keypair
        }

        if iters % 1_000 == 0 {
            println!("{} thousand iterations", iters/1_000);
        }
    };

    println!("keypair: {:?}", keypair);
}

