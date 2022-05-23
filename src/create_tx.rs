use std::error::Error;
use bdk::{bitcoin, FeeRate, SignOptions, SyncOptions, Wallet};
use bdk::bitcoin::Transaction;
// use bdk::bitcoin::Network;
// use bdk::bitcoin::util::address::Payload::WitnessProgram;
use bdk::blockchain::ElectrumBlockchain;
use bdk::database::MemoryDatabase;
use bdk::electrum_client::Client;
use hex::FromHex;
use hex::ToHex;

pub fn create_tx(
    descriptor: String, 
    network_string: String,
    output_script_hex: String,
    channel_value_satoshis: u64,
) -> Result<String, Box<dyn Error>> {
    
    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let blockchain = ElectrumBlockchain::from(client);
    let network: bitcoin::Network = match network_string.as_str() {
        "testnet" => bitcoin::Network::Testnet,
        "regtest" => bitcoin::Network::Regtest,
        _         => panic!("Network not supported")
    };
    let wallet = Wallet::new(
        descriptor.as_str(),
        None,
        network,
        MemoryDatabase::default(),
    )?;

    wallet.sync(&blockchain, SyncOptions::default())?;

    let output_script: Vec<u8> = Vec::from_hex(output_script_hex).unwrap();

    // let _addr = WitnessProgram::from_scriptpubkey(
    //     &output_script[..],
    //     match network {
    //         Network::Bitcoin => panic!("Mainnet unsupported"),
    //         Network::Testnet => bitcoin_bech32::constants::Network::Testnet,
    //         Network::Regtest => bitcoin_bech32::constants::Network::Regtest,
    //         Network::Signet  => panic!("Signet unsupported"),
    //     },
    // )
    // .expect("Lightning funding tx should always be to a SegWit output")
    // .to_address();

    let fee_rate = FeeRate::from_sat_per_vb(2.0);

    // let mut tx_builder = wallet.build_tx();
    // tx_builder
    //     .add_recipient(output_script.clone(), channel_value_satoshis)
    //     .fee_rate(fee_rate)
    //     .enable_rbf();
    // let mut psbt = tx_builder.finish().unwrap();

    let (mut psbt, tx_details) = wallet.build_tx()
        .add_recipient(output_script.clone(), channel_value_satoshis)
        .fee_rate(fee_rate)
        .enable_rbf()
        .finish()
        .unwrap();

    wallet.sign(&mut psbt, SignOptions::default());

    let mut funding_tx: Transaction = psbt.extract_tx();

    let mut funding_tx_hex = String::new();

    // serialize not found in bdk::bitcoin::Transaction
    // funding_tx_hex.write_hex(&mut funding_tx.serialize());

    funding_tx_hex.write_hex(&mut funding_tx);

    return Ok(funding_tx_hex);
}
