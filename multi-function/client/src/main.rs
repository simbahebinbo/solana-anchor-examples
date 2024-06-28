use std::str::FromStr;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_cli_config::Config;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::AccountMeta, message::Message, pubkey::Pubkey, signer::Signer, system_program,
    transaction::Transaction,
};

const FUNCTION_A_DISCRIMINANT: [u8; 8] = [
    160,
    199,
    19,
    162,
    8,
    81,
    54,
    155
];

const FUNCTION_B_DISCRIMINANT: [u8; 8] = [
    175,
    8,
    124,
    141,
    46,
    0,
    38,
    6
];


const PROGRAM_ID: &str = "343nNiDeE2ekrgTzNEwxoDsDmZYkfm7xZc1nBL5yR9ps";



fn main() {
    let field1=42_u64;
    let field2= "hello".to_string();

    let config_file = solana_cli_config::CONFIG_FILE
        .as_ref()
        .expect("unable to get config file path");
    let cli_config: Config = Config::load(config_file).expect("Unable to load config file");

    let connection = RpcClient::new(cli_config.json_rpc_url);

    let signer = solana_clap_utils::keypair::keypair_from_path(
        &Default::default(),
        &cli_config.keypair_path,
        "keypair",
        false,
    )
    .map_err(|err| println!("Unable to get signer from path: {}", err))
    .unwrap();
    let signer_pubkey = signer.pubkey();

    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();

    let ix1 = solana_sdk::instruction::Instruction::new_with_borsh(
        program_id,
        &(FUNCTION_A_DISCRIMINANT, field1),
        vec![
            AccountMeta::new_readonly(signer_pubkey, true),
        ],
    );

    let ix2 = solana_sdk::instruction::Instruction::new_with_borsh(
        program_id,
        &(FUNCTION_B_DISCRIMINANT, field2),
        vec![
            AccountMeta::new_readonly(signer_pubkey, true),
        ],
    );

    let message = Message::new(&[ix1,ix2], Some(&signer_pubkey));

    let mut tx = Transaction::new_unsigned(message);

    tx.sign(&[&signer], connection.get_latest_blockhash().unwrap());

    let tx_id = connection
        .send_and_confirm_transaction_with_spinner(&tx)
        .map_err(|err| {
            println!("{:?}", err);
        })
        .unwrap();
    println!("Program uploaded successfully. Transaction ID: {}", tx_id);
}
