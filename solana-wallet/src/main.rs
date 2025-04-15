// https://accu.cc/content/solana/prikey_keypair/
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signer;

fn main() {
    let pair = Keypair::new();

    println!("Pubkey:\n{:}", &pair.pubkey());
    println!("Base58 prikey+pubkey:\n{:}", pair.to_base58_string());
    println!("JSON prikey+pubkey:\n{:?}", pair.to_bytes());
    let mut prikey = [0u8; 32];
    getrandom::fill(&mut prikey).unwrap();
    let encoded = const_hex::encode_prefixed(prikey);
    println!("encoded prikey:\n{:}", encoded);

    // id.json to private key and public key
    // id.json = 32 byte private key + 32 byte public key
    let expected_pubkey = "3xQDNR5TT79AvFFdEdrqzmmXcHfhSNEbhcmUHe37WScN";
    println!("Expected pubkey:\n{:}", expected_pubkey);
    let id_json = [23,111,36,147,9,41,156,20,208,244,51,214,37,205,59,111,114,244,64,40,159,24,8,77,118,25,180,5,9,105,207,108,43,232,144,207,215,244,218,73,4,214,240,50,74,181,147,16,163,167,221,160,210,139,70,203,128,38,26,216,197,220,238,239];
    let keypair_from_bytes = Keypair::from_bytes(&id_json).unwrap();
    println!("Pubkey from id_json:\n{:}", keypair_from_bytes.pubkey());
    println!("base58 string:\n{:}", keypair_from_bytes.to_base58_string());
    let expected_prikey_pubkey_base58 = "UB7cBhKaNe4vXdR6mBPNEg59wYbr85iMeYhMirCrTBP5n7Gif59PwnnbTxvy4jyesMMbjwrcKgB6Wguv7uRhyDQ";
    let keypair = Keypair::from_base58_string(expected_prikey_pubkey_base58);
    println!("Pubkey from base58 string:\n{:}", keypair.pubkey());
}
