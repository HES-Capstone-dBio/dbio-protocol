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
        let (capsule, ciphertext) = encrypt(&alice_pk, PLAINTEXT).unwrap();
        let decrypted = decrypt_original(&alice, &capsule, &ciphertext).unwrap();
        assert_eq!(PLAINTEXT as &[u8], &(*decrypted));
    }

    #[test]
    fn proxy_single_roundtrip() {
        // CLIENT: Alice is patient with custody of key
        let alice = SecretKey::random();
        let alice_pk = alice.public_key();
        let signer = Signer::new(SecretKey::random());
        // PROTOCOL: Verifier key stored as part of Alice's access agreement with Bob
        let verifier = signer.verifying_key();
        // CLIENT: Perform initial encryption
        let (capsule, ciphertext) = encrypt(&alice_pk, PLAINTEXT).unwrap();
        // FHIR: Bob is 3rd party, secret key stored on server
        let bob = SecretKey::random();
        // PROTOCOL: Bob's public key stored in backend
        let bob_pk = bob.public_key();
        // CLIENT: Generate a single fragment transform key between Alice and Bob
        let frags = generate_kfrags(&alice, &bob_pk, &signer, 1, 1, true, true);
        // PROTOCOL: Store transform key in backend
        let keyfrag = KeyFrag::from_array(&frags[0].to_array())
            .unwrap()
            .verify(&verifier, Some(&alice_pk), Some(&bob_pk))
            .unwrap();
        // PROTOCOL: Generate re-encrypted capsule fragment from encrypted data at rest
        let capfrag = CapsuleFrag::from_array(&reencrypt(&capsule, keyfrag).to_array())
            .unwrap()
            .verify(&capsule, &verifier, &alice_pk, &bob_pk)
            .unwrap();
        // FHIR: Decrypt re-encrypted capsule fragment into plaintext
        let decrypted = decrypt_reencrypted(&bob, &alice_pk, &capsule, [capfrag], &ciphertext)
            .unwrap();
        assert_eq!(PLAINTEXT as &[u8], &(*decrypted))
    }

}
