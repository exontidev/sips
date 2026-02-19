use crate::address::Address;
use crate::helper::ata;
use crate::instructions::account::{AccountMeta, IntoAccountMetaArray};
use crate::instructions::pump::instructions::PumpInstruction;
use ix_macros::Accounts;

#[derive(Accounts, Debug)]
pub struct CreateAccounts {
    #[signer]
    #[writable]
    pub mint: Address,
    pub mint_authority: Address,

    #[writable]
    // #[seeds = b"bonding_curve" + mint]
    pub bonding_curve: Address,

    #[writable]
    pub associated_bonding_curve: Address,

    pub global: Address,
    pub metaplex_token_metadata_program: Address,

    #[writable]
    pub metadata: Address,

    #[signer]
    #[writable]
    pub user: Address,

    pub system_program: Address,
    pub token_program: Address,
    pub associated_token_program: Address,
    pub rent: Address,
    pub event_authority: Address,
    pub program: Address,
}

impl CreateAccounts {
    pub fn new(mint: Address, user: Address) -> Self {
        let program = PumpInstruction::PROGRAM;

        let token_program = Address::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

        let (bonding_curve, _bump) = Address::pda(&program, &[b"bonding-curve", mint.as_ref()]);
        let (associated_bonding_curve, _bump) = ata(&bonding_curve, &token_program, &mint);

        let mint_authority = Address::from_str_const("TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM");
        let global = Address::from_str_const("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf");
        let system_program = Address::from_str_const("11111111111111111111111111111111");

        let event_authority =
            Address::from_str_const("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1");

        let rent = Address::from_str_const("SysvarRent111111111111111111111111111111111");
        let associated_token_program =
            Address::from_str_const("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

        let metaplex_program =
            Address::from_str_const("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

        let (metadata, _bump) = Address::pda(
            &metaplex_program,
            &[
                b"metadata".as_ref(),
                metaplex_program.as_ref(),
                mint.as_ref(),
            ],
        );

        Self {
            mint,
            mint_authority,
            bonding_curve,
            associated_bonding_curve,
            global,
            metaplex_token_metadata_program: metaplex_program,
            metadata,
            user,
            system_program,
            token_program,
            associated_token_program,
            rent,
            event_authority,
            program,
        }
    }
}

#[derive(Accounts, Debug)]
pub struct CreateV2Accounts {
    #[signer]
    #[writable]
    pub mint: Address,
    pub mint_authority: Address,

    #[writable]
    pub bonding_curve: Address,

    #[writable]
    pub associated_bonding_curve: Address,

    pub global: Address,

    #[signer]
    #[writable]
    pub user: Address,

    pub system_program: Address,
    pub token_program: Address,
    pub associated_token_program: Address,

    #[writable]
    pub mayhem_program: Address,

    pub global_params: Address,

    #[writable]
    pub sol_vault: Address,

    #[writable]
    pub mayhem_state: Address,

    #[writable]
    pub mayhem_token_vault: Address,

    pub event_authority: Address,
    pub program: Address,
}

impl CreateV2Accounts {
    pub fn new(mint: Address, user: Address) -> Self {
        let program = PumpInstruction::PROGRAM;
        let mayhem_program = Address::from_str_const("MAyhSmzXzV1pTf7LsNkrNwkWKTo4ougAJ1PPg47MD4e");
        let token_program = Address::from_str_const("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");
        let associated_token_program =
            Address::from_str_const("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
        let mint_authority = Address::from_str_const("TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM");
        let (bonding_curve, _bump) = Address::pda(&program, &[b"bonding-curve", mint.as_ref()]);
        let (associated_bonding_curve, _bump) = ata(&bonding_curve, &token_program, &mint);

        let global = Address::from_str_const("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf");
        let system_program = Address::from_str_const("11111111111111111111111111111111");
        let global_params = Address::from_str_const("13ec7XdrjF3h3YcqBTFDSReRcUFwbCnJaAQspM4j6DDJ");
        let sol_vault = Address::from_str_const("BwWK17cbHxwWBKZkUYvzxLcNQ1YVyaFezduWbtm2de6s");
        let (mayhem_state, _bump) =
            Address::pda(&mayhem_program, &[b"mayhem-state", mint.as_ref()]);
        let (mayhem_token_vault, _bump) = ata(&sol_vault, &token_program, &mint);

        let event_authority =
            Address::from_str_const("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1");

        Self {
            mint,
            bonding_curve,
            user,
            mayhem_program,
            token_program,
            associated_token_program,
            mint_authority,
            associated_bonding_curve,
            global,
            system_program,
            global_params,
            sol_vault,
            mayhem_state,
            mayhem_token_vault,
            event_authority,
            program,
        }
    }
}

#[derive(Accounts, Debug)]
pub struct BuyAccounts {
    pub global: Address,
    #[writable]
    pub fee_address: Address,
    pub mint: Address,
    #[writable]
    pub bonding_curve: Address,
    #[writable]
    pub associated_bonding_curve: Address,
    #[writable]
    pub associated_user: Address,

    #[signer]
    #[writable]
    pub user: Address,

    pub system_program: Address,
    pub token_program: Address,

    #[writable]
    pub creator_vault: Address,
    pub event_authority: Address,
    pub program: Address,

    #[writable]
    pub global_volume_accumulator: Address,

    #[writable]
    pub user_volume_accumulator: Address,
    pub fee_config: Address,
    pub fee_program: Address,
}

impl BuyAccounts {
    pub fn new(mint: Address, user: Address, creator: Address, token_program: Address) -> Self {
        let program = PumpInstruction::PROGRAM;

        let global = Address::from_str_const("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf");
        let fee = Address::from_str_const("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV");
        let (bonding_curve, _bump) = Address::pda(&program, &[b"bonding-curve", mint.as_ref()]);
        let (associated_bonding_curve, _bump) = ata(&bonding_curve, &token_program, &mint);
        let (associated_user, _bump) = ata(&user, &token_program, &mint);
        let system_program = Address::from_str_const("11111111111111111111111111111111");

        let (creator_vault, _bump) = Address::pda(&program, &[b"creator-vault", creator.as_ref()]);
        let event_authority =
            Address::from_str_const("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1");

        let global_volume_accumulator =
            Address::from_str_const("Hq2wp8uJ9jCPsYgNHex8RtqdvMPfVGoYwjvF1ATiwn2Y");

        let (user_volume_accumulator, _bump) =
            Address::pda(&program, &[b"user_volume_accumulator", user.as_ref()]);

        let fee_config = Address::from_str_const("8Wf5TiAheLUqBrKXeYg2JtAFFMWtKdG2BSFgqUcPVwTt");
        let fee_program = Address::from_str_const("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ");
        Self {
            global,
            fee_address: fee,
            mint,
            bonding_curve,
            associated_bonding_curve,
            associated_user,
            user,
            system_program,
            token_program,
            creator_vault,
            event_authority,
            program,
            global_volume_accumulator,
            user_volume_accumulator,
            fee_config,
            fee_program,
        }
    }
}

#[derive(Accounts, Debug)]
pub struct SellAccounts {
    pub global: Address,
    #[writable]
    pub fee_address: Address,
    pub mint: Address,
    #[writable]
    pub bonding_curve: Address,
    #[writable]
    pub associated_bonding_curve: Address,
    #[writable]
    pub associated_user: Address,

    #[signer]
    #[writable]
    pub user: Address,

    pub system_program: Address,

    #[writable]
    pub creator_vault: Address,

    pub token_program: Address,

    pub event_authority: Address,
    pub program: Address,

    pub fee_config: Address,
    pub fee_program: Address,
}

impl SellAccounts {
    pub fn new(mint: Address, user: Address, creator: Address, token_program: Address) -> Self {
        let program = PumpInstruction::PROGRAM;

        let global = Address::from_str_const("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf");
        let fee = Address::from_str_const("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV");
        let (bonding_curve, _bump) = Address::pda(&program, &[b"bonding-curve", mint.as_ref()]);
        let (associated_bonding_curve, _bump) = ata(&bonding_curve, &token_program, &mint);
        let (associated_user, _bump) = ata(&user, &token_program, &mint);
        let system_program = Address::from_str_const("11111111111111111111111111111111");

        let (creator_vault, _bump) = Address::pda(&program, &[b"creator-vault", creator.as_ref()]);
        let event_authority =
            Address::from_str_const("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1");

        let fee_config = Address::from_str_const("8Wf5TiAheLUqBrKXeYg2JtAFFMWtKdG2BSFgqUcPVwTt");
        let fee_program = Address::from_str_const("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ");
        Self {
            global,
            fee_address: fee,
            mint,
            bonding_curve,
            associated_bonding_curve,
            associated_user,
            user,
            system_program,
            token_program,
            creator_vault,
            event_authority,
            program,
            fee_config,
            fee_program,
        }
    }
}

#[derive(Accounts, Debug)]
pub struct CloseUserVolumeAccumulatorAccounts {
    #[signer]
    #[writable]
    user: Address,
    #[writable]
    user_volume_accumulator: Address,
    event_authority: Address,
    program: Address,
}
