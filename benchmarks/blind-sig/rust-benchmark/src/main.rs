use blind_rsa_signatures::{Hash, Options, SecretKey};
use sightglass_api as bench;

fn main() {
    let secret_key = SecretKey::from_der(&std::fs::read("./secret.der").unwrap()).unwrap();
    let public_key = secret_key.public_key().unwrap();

    let randomize = false;
    let message = b"hello!";
    let options = Options::new(Hash::Sha384, !randomize, 0);

    bench::start();

    let blinding_result = public_key.blind(message, randomize, &options).unwrap();
    let blinded = secret_key
        .blind_sign(&blinding_result.blind_msg, &options)
        .unwrap();
    let signature = public_key
        .finalize(
            &blinded,
            &blinding_result.secret,
            blinding_result.msg_randomizer,
            message,
            &options,
        )
        .unwrap();

    bench::end();

    for line in signature.chunks(32) {
        for &byte in line.iter() {
            print!("{:02X}", byte);
        }
        println!();
    }
}
