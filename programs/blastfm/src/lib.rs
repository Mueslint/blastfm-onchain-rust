use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("XNFCVpn6ujGoEEsDJxQWJ36k7HnUQVPzrWc3YBosq9u");

#[program]
pub mod blastfm {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_musics = 0;
        Ok(())
    }

    // The function now accepts a music_id param from the user. We also reference the user from the Context
    pub fn add_music(ctx: Context<AddMusic>, music_id: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // Build the struct.
        let item = ItemStruct {
            music_id: music_id.to_string(),
            user_address: *user.to_account_info().key,
        };

        // Add it to the music_list vector.
        base_account.music_list.push(item);
        base_account.total_musics += 1;
        Ok(())
    }

    pub fn delete_music(ctx: Context<AddMusic>, deleted_music_id: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;

        // filter the previous list .
        let filtered_music_list: Vec<ItemStruct> = base_account
            .music_list
            .iter()
            .filter(|music_item| music_item.music_id == deleted_music_id)
            .cloned()
            .collect();

        // replace the precious value with the filtered one
        base_account.music_list = filtered_music_list;
        base_account.total_musics -= 1;

        Ok(())
    }

    pub fn update_music(
        ctx: Context<AddMusic>,
        old_music_id: String,
        new_music_id: String,
    ) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let new_music_item = ItemStruct {
            music_id: new_music_id.to_string(),
            user_address: *user.to_account_info().key,
        };
        // filter the previous list .
        let mut filtered_music_list: Vec<ItemStruct> = base_account
            .music_list
            .iter()
            .filter(|music_item| music_item.music_id == old_music_id)
            .cloned()
            .collect();
        filtered_music_list.push(new_music_item);

        // replace the precious value with the filtered one
        base_account.music_list = filtered_music_list;

        Ok(())
    }

    pub fn reset_all_music(ctx: Context<ResetMusic>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.music_list = Vec::new();
        base_account.total_musics = 0;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Add the signer who calls the AddMusic method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddMusic<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResetMusic<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub music_id: String,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount {
    pub total_musics: u64,
    // Attach a Vector of type ItemStruct to the account.
    pub music_list: Vec<ItemStruct>,
}
