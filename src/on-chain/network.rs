use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::blockdata::constants::genesis_block;
use bitcoin::util::key::PrivateKey;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::bip32::ExtendedPrivKey;

fn main() {
    // Define the networks you want to support
    let networks = vec![
        Network::Bitcoin,
        Network::Testnet,
        Network::Signet,
        Network::Regtest,
        Network::Testnet4
    ];

    // Example: Print the genesis block for each network
    for network in networks {
        println!("{:?} Genesis Block: {:?}", network, genesis_block(network));
    }

    // Example: Generate a new Bitcoin address for each network
    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();
    let private_key = PrivateKey::new(&secp, Network::Bitcoin, &mut rng);

    for network in networks {
        let address = Address::p2pkh(&private_key.public_key(&secp), network);
        println!("{:?} Address: {:?}", network, address);
    }

    // Example: Generate an extended private key for each network
    let seed = [0u8; 32]; // Replace with your seed
    for network in networks {
        let xpriv = ExtendedPrivKey::new_master(network, &seed).unwrap();
        println!("{:?} Extended Private Key: {:?}", network, xpriv);
    }
}

