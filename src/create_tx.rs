use std::error::Error;
use std::str::FromStr;
use bdk::{bitcoin, FeeRate, SignOptions, SyncOptions, Wallet};
use bdk::bitcoin::{Address, Transaction};
use bdk::bitcoin::util::psbt::serialize::Serialize;
use bdk::blockchain::ElectrumBlockchain;
use bdk::database::MemoryDatabase;
use bdk::electrum_client::Client;
use bitcoin_bech32::WitnessProgram;
use hex::FromHex;

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

    // testing that the wallet gets created and can sync
    // let address = wallet.get_address(AddressIndex::New).unwrap().to_string();
    wallet.sync(&blockchain, SyncOptions::default())?;
    // let balance = wallet.get_balance().unwrap().to_string();

    let output_script_raw: Vec<u8> = Vec::from_hex(&output_script_hex).expect("Transforming the hex into raw bytes didn't work");

    let address = WitnessProgram::from_scriptpubkey(
        &output_script_raw[..],
        bitcoin_bech32::constants::Network::Testnet
    )
        .expect("Lightning funding tx should always be to a SegWit output")
        .to_address();

    // thread 'main' panicked at 'Deserialization didn't work: ParseFailed("data not consumed entirely when explicitly deserializing")'
    // let output_script: Script = deserialize(&output_script_raw).expect("Deserialization didn't work");

    let fee_rate = FeeRate::from_sat_per_vb(2.0);

    let (mut psbt, details) = {
        let mut builder = wallet.build_tx();
        builder
            // .add_recipient(address.script_pubkey(), channel_value_satoshis)
            .add_recipient(Address::from_str(address.as_str()).unwrap().script_pubkey(), channel_value_satoshis)
            .fee_rate(fee_rate)
            .enable_rbf();
        builder
            .finish()?
    };

    wallet.sign(&mut psbt, SignOptions::default());

    let funding_tx: Transaction = psbt.extract_tx();
    let funding_tx_encoded = funding_tx.serialize();

    let tx = hex::encode(&funding_tx_encoded);

    return Ok(tx);
}
