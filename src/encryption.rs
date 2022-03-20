use pad::PadStr;
use recrypt::prelude::*;
use recrypt::api::{Plaintext, PrivateKey, PublicKey, RecryptErr};

/* As required by Plaintext */
static WIDTH: usize = 384;

/**
 * Given an Ethereum address and signed message, derive a keypair for the user.
 *
 * TODO: Currently does not validate the signature on the message.
 * NB: This code is intended as a proof of concept and not for use in production.
 */
#[allow(dead_code)]
fn derive_keypair(addr: &str, sig: &str) -> Result<(PrivateKey, PublicKey), RecryptErr> {
    let rc = Recrypt::new();
    let plaintext = &format!("{addr}{sig}").pad_to_width(WIDTH);
    Plaintext::new_from_slice(plaintext.as_bytes())
        .map(|pt| rc.derive_private_key(&pt))
        .and_then(|pk| rc.compute_public_key(&pk).map(|pb| (pk, pb)))
}

#[cfg(test)]
mod test {
    use super::*;

    /* ETH wallet address and signed message used when generating keypair */
    static ADDR: &str =
        "0x2a3052ef570a031400BffD61438b2D19e0E8abef";
    static SIG: &str =
        "0x4e1ce8ea60bc6dfd4068a35462612495850cb645a1c9f475eb969bff21d0b0fb414112aaf13f01dd18a3527cb648cdd51b618ae49d4999112c33f86b7b26e9731b";

    #[test]
    fn derive_is_deterministic() {
        let (priv_key, pub_key) = derive_keypair(ADDR, SIG).unwrap();
        let (priv_key_, pub_key_) = derive_keypair(ADDR, SIG).unwrap();
        assert_eq!(priv_key, priv_key_);
        assert_eq!(pub_key, pub_key_);
    }

    #[test]
    fn derive_roundtrip_works() {
        let rc = Recrypt::new();
        let (priv_key, pub_key) = derive_keypair(ADDR, SIG).unwrap();
        let text = "hey jerry, you wanna check out that soup place?"
            .pad_to_width(WIDTH);
        let plaintext = Plaintext::new_from_slice(text.as_bytes()).unwrap();
        let signing = rc.generate_ed25519_key_pair();
        let ciphertext = rc.encrypt(&plaintext, &pub_key, &signing).unwrap();
        let plaintext_ = rc.decrypt(ciphertext, &priv_key).unwrap();
        assert_eq!(plaintext, plaintext_);
    }

    #[test]
    fn derive_transform_roundtrip() {
        let rc = Recrypt::new();
        let (priv_key, pub_key) = derive_keypair(ADDR, SIG).unwrap();
        let text = "hey jerry, you wanna check out that soup place?".pad_to_width(WIDTH);
        let plaintext = Plaintext::new_from_slice(text.as_bytes()).unwrap();
        let (to_priv_key, to_pub_key) = rc.generate_key_pair().unwrap();
        let signing = rc.generate_ed25519_key_pair();
        let transform_key = rc.generate_transform_key(&priv_key, &to_pub_key, &signing).unwrap();
        let ciphertext = rc.encrypt(&plaintext, &pub_key, &signing).unwrap();
        let transformed = rc.transform(ciphertext, transform_key, &signing).unwrap();
        let plaintext_ = rc.decrypt(transformed, &to_priv_key).unwrap();
        assert_eq!(plaintext, plaintext_)
    }

}
