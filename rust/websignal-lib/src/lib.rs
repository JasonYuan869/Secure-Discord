use libsignal_protocol::kem::{KeyPair, KeyType};

fn aaa() {
    KeyPair::generate(KeyType::Kyber1024);
}
