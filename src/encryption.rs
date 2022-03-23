use umbral_pre::*;
use std::boxed::Box;

#[allow(dead_code)]
fn encrypt_plaintext(pt: &str, pk: &PublicKey) -> Result<(Capsule, Box<[u8]>), EncryptionError> {
   encrypt(pk, pt.as_bytes())
}

#[cfg(test)]
mod test {
    use super::*;

    static PLAINTEXT: &[u8; 562] = b"
        Jerry and Newman are at the Super Bowl.

        Newman:  Great streak of luck I'm having.  First, Kramer almost beat me at Risk
        but I narrowly escaped, and then Tim Whatley gives me his Super Bowl ticket.

        Jerry:  Can you move over at all?!

        Newman:  And then, just as I'm about to go, these boxes show up at the post
        office with no labels.  No labels, Jerry.  You know what that means?  Freebies!!
        I got this great mini-TV and a VCR, oh it's unbelievable.

        Jerry:  An inch!  Can you move over an inch?!?";

    #[test]
    fn simple_roundtrip() {
        let alice = SecretKey::random();
        let alice_pk = alice.public_key();
        let signer = Signer::new(alice);
        let signer_pk = signer.verifying_key();
        let (capsule, ciphertext) = encrypt(&signer_pk, PLAINTEXT).unwrap();
        let decrypted = decrypt_original(&alice, &capsule, &ciphertext).unwrap();
    }
}
