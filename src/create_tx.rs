use std::error::Error;
use bdk::{bitcoin, FeeRate, SignOptions, SyncOptions, Wallet};
use bdk::bitcoin::{Address, Script, Transaction};
use bdk::bitcoin::hashes::hex::FromHex;
use bdk::bitcoin::hashes::hex::ToHex;
// use bdk::bitcoin::consensus::deserialize;
use bdk::bitcoin::util::address::Payload;
use bdk::bitcoin::util::psbt::serialize::Serialize;
use bdk::blockchain::ElectrumBlockchain;
use bdk::database::MemoryDatabase;
use bdk::electrum_client::Client;
use bitcoin::util::address::WitnessVersion;

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

    let output_script_raw: Vec<u8> = Vec::from_hex(&output_script_hex).expect("Transforming the hex into raw bytes didn't work");

    // technique 1 (you need to strip the first two bytes of the output_script given by LDK for this to work)
    // let payload: Payload = Payload::WitnessProgram { version: WitnessVersion::V0, program: output_script_raw[2..].to_vec() };
    // let address: Address = Address {
    //     payload,
    //     network
    // };

    // technique 2 (better)
    // let script: Script = Script::from_hex(&output_script_hex).unwrap();
    let script: Script = Script::from(output_script_raw);


    let fee_rate = FeeRate::from_sat_per_vb(2.0);

    let (mut psbt, _details) = {
        let mut builder = wallet.build_tx();
        builder
            // technique 1
            //.add_recipient(address.script_pubkey(), channel_value_satoshis)
            // technique 2 (better)
            .add_recipient(script, channel_value_satoshis)
            .fee_rate(fee_rate)
            .enable_rbf();
        builder
            .finish()?
    };

    wallet.sign(&mut psbt, SignOptions::default())?;

    let funding_tx: Transaction = psbt.extract_tx();
    let funding_tx_encoded = funding_tx.serialize();

    let tx: String = funding_tx_encoded.to_hex();

    return Ok(tx);
}
