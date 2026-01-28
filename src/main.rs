use sips::instructions::pump::accounts::CreateAccountPayload;

fn main() {
    let accounts = CreateAccountPayload::new(
        mint, 
        mint_authority, 
        bonding_curve, 
        associated_bonding_curve, 
        global, 
        metaplex_token_metadata, 
        metadata, 
        user, 
        system_program, 
        token_program, 
        associated_token_program, 
        rent, 
        event_authority, 
        program
    );
}