use bdk::blockchain::{AnyBlockchain, AnyBlockchainConfig, ElectrumBlockchain, ConfigurableBlockchain};
use bdk::database::MemoryDatabase;
use bdk::keys::{ExtendedKey, GeneratableKey, GeneratedKey};
use bdk::wallet::Wallet;
use bdk::keys::bip39::{Mnemonic, Language};
use bdk::bitcoin::Network;

fn main() -> Result<(), bdk::Error> {
    // Define the networks you want to support
    let networks = vec![
        Network::Bitcoin,
        Network::Testnet,
        Network::Signet,
        Network::Regtest,
    ];

    // Example mnemonic for wallet creation (in a real application, keep this secure)
    let mnemonic = Mnemonic::generate_in(Language::English, 12).unwrap();

    for network in networks {
        println!("Network: {:?}", network);

        // Create an extended key from the mnemonic
        let xkey: ExtendedKey = mnemonic.clone().into_extended_key().unwrap();
        let descriptor = format!("wpkh({}/84'/{}'/0'/0/*)", xkey.xprv(network).unwrap().to_string(), network as u32);

        // Create a new BDK wallet
        let wallet = Wallet::new(&descriptor, None, network, MemoryDatabase::default())?;
        println!("{:?} Wallet Descriptor: {}", network, wallet.descriptor().unwrap());

        // Configure the blockchain for the network
        let electrum_url = match network {
            Network::Bitcoin => "ssl://electrum.blockstream.info:60002",
            Network::Testnet => "ssl://electrum.blockstream.info:60004",
            Network::Signet => "ssl://signet-electrum.blockstream.info:60002",
            Network::Regtest => "tcp://localhost:60401",
        };

        let blockchain = AnyBlockchain::from_config(&AnyBlockchainConfig::Electrum(ElectrumBlockchain::from_config(&electrum_url).unwrap()))?;

        // Sync the wallet with the blockchain
        wallet.sync(&blockchain, bdk::wallet::SyncOptions::default())?;
        println!("{:?} Wallet balance: {:?}", network, wallet.get_balance()?);
    }

    Ok(())
}
