use anchor_lang::{
    InstructionData, solana_program::{self}, ToAccountMetas,
};
use solana_program::instruction::Instruction;
use solana_program_test::{ProgramTest, tokio};
use solana_sdk::{account::Account, signature::Keypair, signer::Signer, transaction::Transaction};

#[tokio::test]
async fn test_all() {
    let SetUpTest {
        validator,
        user,
    } = SetUpTest::new();

    let mut context = validator.start_with_context().await;

    let init_ix1 = Instruction {
        program_id: multi_function::ID,
        accounts: multi_function::accounts::FunctionA {
            signer: user.pubkey(),
        }
            .to_account_metas(None),
        data: multi_function::instruction::FunctionA { field1: 42 }.data(),
    };

    let init_ix2 = Instruction {
        program_id: multi_function::ID,
        accounts: multi_function::accounts::FunctionB {
            signer: user.pubkey(),
        }
            .to_account_metas(None),
        data: multi_function::instruction::FunctionB { field2: "hello".to_string() }.data(),
    };

    let init_tx = Transaction::new_signed_with_payer(
        &[init_ix1, init_ix2],
        Some(&user.pubkey()),
        &[&user],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(init_tx)
        .await
        .unwrap();
}


/// Struct set up to hold the validator, an optional user account, and the counter PDA.
/// Use SetUpTest::new() to create a new instance.
pub struct SetUpTest {
    pub validator: ProgramTest,
    pub user: Keypair,
}

/// Returns the validator, an optional funded user account, and the counter PDA
impl SetUpTest {
    pub fn new() -> Self {
        //Both of these work

        // let mut validator = ProgramTest::default();
        // validator.add_program("multi_function", multi_function::ID, None);
        let mut validator = ProgramTest::new("multi_function", multi_function::ID, None);

        //create a new user and fund with 1 SOL
        //add the user to the validator / ledger
        let user = Keypair::new();
        validator.add_account(
            user.pubkey(),
            Account {
                lamports: 1_000_000_000,
                ..Account::default()
            },
        );

        Self {
            validator,
            user,
        }
    }
}
