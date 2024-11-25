use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, program::invoke_signed, system_instruction},
};
use std::str::FromStr;

declare_id!("AxmxjMWgQcMQRCbT6oF4PMhJfNGY9YnWNZcm4TowE3H9");

#[program]
pub mod codetest {
    use super::*;

    // creation of gamelist account
    pub fn init_gamelist_account(ctx: Context<InitGamelistAccount>) -> Result<()> {
        msg!("Line 15: Invoked init_gamelist_account endpoint that has ctx of InitGamelistAccount");

        let gamelist = &mut ctx.accounts.game_list;

        // // // Transaction simulation failed: Transaction leaves an "account" with a lower balance than rent-exempt minimum
        // // // $ solana rent 0 -um
        // // // Rent per byte-year: 0.00000348 SOL
        // // // Rent per epoch: 0.000002439 SOL
        // // // Rent-exempt minimum: 0.00089088 SOL

        // invoke(
        //     &system_instruction::transfer(
        //         &ctx.accounts.server.key(), //authority === server
        //         &gamelist.key(),
        //         890880,
        //     ),
        //     &[
        //         ctx.accounts.server.to_account_info(),
        //         gamelist.to_account_info(),
        //         ctx.accounts.system_program.to_account_info(),
        //     ],
        // )?;

        gamelist.game_type_index = 1; // setting up game_type_index in gamelist account
                                      // this same reference will also be stored in gametype account under game_type_index_of_gamelist

        gamelist.list_of_game_type_data = Vec::new();
        gamelist.authority = ctx.accounts.server.key();

        Ok(())
    }

    // creation of associated gametype account
    pub fn create_game_type(
        ctx: Context<CreateGameType>,
        entry_price: u64,
        max_games: u64,
        max_players: u64,
    ) -> Result<()> {
        msg!("Line 34: Invoked create_game_type endpoint that has ctx of CreateGameType");

        let gamelist = &mut ctx.accounts.game_list;
        let gametype = &mut ctx.accounts.game_type;

        let authority = ctx.accounts.authority.key();
        // let authority = gamelist.authority; // not required

        // // // Transaction simulation failed: Transaction leaves an "account" with a lower balance than rent-exempt minimum
        // // // $ solana rent 0 -um
        // // // Rent per byte-year: 0.00000348 SOL
        // // // Rent per epoch: 0.000002439 SOL
        // // // Rent-exempt minimum: 0.00089088 SOL

        // invoke(
        //     &system_instruction::transfer(&ctx.accounts.authority.key(), &gametype.key(), 890880),
        //     &[
        //         ctx.accounts.authority.to_account_info(),
        //         gametype.to_account_info(),
        //         ctx.accounts.system_program.to_account_info(),
        //     ],
        // )?;

        // let (global_treasury_pda, global_treasury_pda_bump_seed) =
        //     Pubkey::find_program_address(&[b"GlobalTreasury"], ctx.program_id);

        // // // Transaction simulation failed: Transaction leaves an "account" with a lower balance than rent-exempt minimum
        // // // $ solana rent 0 -um
        // // // Rent per byte-year: 0.00000348 SOL
        // // // Rent per epoch: 0.000002439 SOL
        // // // Rent-exempt minimum: 0.00089088 SOL

        // invoke(
        //     &system_instruction::transfer(
        //         &ctx.accounts.authority.key(),
        //         &global_treasury_pda,
        //         890880,
        //     ),
        //     &[
        //         ctx.accounts.authority.to_account_info(),
        //         global_treasury_pda.to_account_info(),
        //         ctx.accounts.system_program.to_account_info(),
        //     ],
        // )?;

        // setting up all data in gametype account
        gametype.last_game_index = 1; // a fresh account starts the game index with 1
        gametype.last_game_index_to_string = gametype.last_game_index.to_string(); // converted to string to pass in seed for derivation of gamepda
        gametype.authority = authority;
        gametype.entry_price = entry_price;
        gametype.max_players = max_players;
        gametype.game_type_index_of_gamelist = gamelist.game_type_index; // setting up the game_type_index associative to gamelist
        gametype.max_games = max_games;

        // storing gametype account's data in gamelist account's list_of_game_type_data
        let gametypedata = GameTypeData {
            game_type_key: gametype.key(),
            game_type_index_of_gamelist: gamelist.game_type_index,
            entry_price: entry_price,
            max_games: max_games,
            max_players: max_players,
            authority: authority,
        };
        gamelist.list_of_game_type_data.push(gametypedata);

        gamelist.game_type_index += 1; // increasing to +1 for case when next gametype account created, there can be mapping with gamelist account

        Ok(())
    }

    pub fn add_player(ctx: Context<AddPlayer>) -> Result<()> {
        msg!("Line 67: Invoked add_player endpoint that has ctx of AddPlayer");

        let gamelist = &mut ctx.accounts.game_list; // data account
        let gametype = &mut ctx.accounts.game_type; // data account
        let game = &mut ctx.accounts.game_pda; // PDA account
                                               // game.authority = ctx.accounts.authority.key();
        game.authority = gamelist.authority;
        let globaltreasury = &mut ctx.accounts.global_treasury_pda; // PDA account

        let (global_treasury_pda, global_treasury_pda_bump_seed) =
            Pubkey::find_program_address(&[b"GlobalTreasury"], ctx.program_id); // this is used only for transferring commission to solemInc

        // // because tx simulation error: tx will not pass validators check; hence below airdrop is of no use

        // // // Transaction simulation failed: Transaction leaves an "account" with a lower balance than rent-exempt minimum
        // // // $ solana rent 0 -um
        // // // Rent per byte-year: 0.00000348 SOL
        // // // Rent per epoch: 0.000002439 SOL
        // // // Rent-exempt minimum: 0.00089088 SOL

        // // airdropping globaltreasury
        // invoke(
        //     &system_instruction::transfer(
        //         &ctx.accounts.authority.key(),
        //         &global_treasury_pda,
        //         890880,
        //     ),
        //     &[
        //         ctx.accounts.authority.to_account_info(),
        //         globaltreasury.to_account_info(),
        //         ctx.accounts.system_program.to_account_info(),
        //     ],
        // )?;

        // // because tx simulation error: tx will not pass validators check; hence below airdrop is of no use
        // airdropping gamepda
        // invoke(
        //     &system_instruction::transfer(&ctx.accounts.authority.key(), &game.key(), 890880),
        //     &[
        //         ctx.accounts.authority.to_account_info(),
        //         game.to_account_info(),
        //         ctx.accounts.system_program.to_account_info(),
        //     ],
        // )?;

        let solem_inc_pk =
            Pubkey::from_str("C8G8fK6G6tzPeFDXArqXPJusd1vDfQAftLwBNu3qmaRb").unwrap();

        msg!(
            "Line 64 ~ pub fn add_player ~ gamelist account: {} ",
            gamelist.key()
        );
        msg!(
            "Line 85: gametype key {} gametype.last_game_index: {}",
            gametype.key(),
            gametype.last_game_index
        );
        msg!(
            "Line 64 ~ pub fn add_player ~ game PDA account: {} ",
            game.key()
        );

        // ctx.accounts.data.select_id_string=ctx.accounts.data.select_id_string.to_string();

        // let game_type=&mut ctx.accounts.game_type;

        let entryprice = gametype.entry_price; // local var

        // why < ?   and not ===>     <=
        // because the third player is entering using this instruction
        // that means the current game.Players.len() should be < gametype.max_player
        // gamelist.last_game_index <= gamelist.max_games // ensures the max_players
        if game.players.len() < gametype.max_players as usize
            && gametype.last_game_index <= gametype.max_games
        {
            if ctx.accounts.player.lamports() >= entryprice {
                msg!("Line 108: lamports greater then required");

                if game.players.len() == 0 {
                    msg!("Line 111: setting reward multiplicator as 0 when there are no players in the vector");
                    game.rm = 0; // setting reward multiplicator as 0
                }

                // else we are printing rm of each game PDA
                msg!("Line 116: game reward multiplicator {}", game.rm);

                let mut i = 0; // indexer used for duplicate entry player check == Phase 1
                let mut can_add = true; // true by default == Phase 2
                                        // checking full state / room state of the game == Phase 3
                let mut full = false;
                msg!("Line 96 ~ if ctx.accounts.player.lamports ~ full {}", full);

                // duplicate entry player check
                // will not work for first player
                // total 3 players:  => i=0;1;2; (total 3 times)
                loop {
                    msg!("Line 131: duplicate entry player check loop");
                    if i < game.players.len() {
                        msg!("Line 115 ~ if i<game.Players.len ~ i {}", i);
                        msg!("Line 137 ~ if i<game.Players.len ~ game.Players.len() current players in room: {}", game.players.len());

                        if game.players[i].to_string() == ctx.accounts.player.key.to_string() {
                            msg!("Line 199: In this case the all players in room will be checked with current player passed, and match found here");
                            msg!("Line 120: Cannot add this player, terminating the instruction!");

                            can_add = false; // terminate the instruction in case of false
                            break;
                        }
                    } else {
                        msg!("Line 129: proceed for next phase i.e can_add");
                        break; // simple breaking when no match found; with can_add == true; proceed for next phase
                    }
                    i = i + 1;
                }

                // now only player that are not duplicated can enter this next phase
                if can_add {
                    msg!("Line 155: we are inside can_add phase, which is second phase");
                    msg!(
                        "Line 159 ~ if ctx.accounts.player.lamports ~  game.players.len() {}",
                        game.players.len()
                    );

                    let mut pre_add_state: usize = 0; // by default we are saying that there are no players in gamePda
                    let mut post_add_state: usize = 0;

                    pre_add_state = game.players.len(); //pre add
                    msg!(
                        "Line 146 ~ ifctx.accounts.player.lamports ~ pre_add_state {}",
                        pre_add_state
                    );

                    // added player public key in game.players
                    // transfered player entry fee to game account

                    msg!(
                        "Line 153 ~ if ctx.accounts.player.lamports ~ gametype.max_players {}",
                        gametype.max_players
                    );

                    if pre_add_state < gametype.max_players as usize {
                        // i<=gametype.max_player  => i<gametype.max_player
                        // because when pre_add_state 3==3, we'll be inside a new gamePda
                        msg!("Line 172: same gamePda (for 1,2,3 player), pre_add_state < gametype.max_player");

                        game.players.push(ctx.accounts.player.key()); // 1st,2nd,3rd player
                        post_add_state = game.players.len(); // In case of 3 players added, this value will be == 3
                        msg!("Line 159 ~ ifctx.accounts.player.lamports ~ post_add_state == game.Players.len() Your player is been successfully added: {}", post_add_state);
                        msg!("Line 160 ~ ifctx.accounts.player.lamports ~ pre_add_state Your player is been successfully added: {}", pre_add_state);

                        // transfered player entry fee to global_treasury_pda account
                        invoke(
                            &system_instruction::transfer(
                                &ctx.accounts.player.key(),
                                // &game.key(), // local var .key()
                                &global_treasury_pda.key(), // local var .key() // transfer will direct to this double checked PDA only
                                entryprice,
                            ),
                            &[
                                ctx.accounts.player.to_account_info(),
                                // game.to_account_info(),
                                globaltreasury.to_account_info(), // this is only possible to sign here, if we get this account in ctx.accounts
                                ctx.accounts.system_program.to_account_info(),
                            ],
                        )?;
                        msg!("Line 177: entry fee is successfully transferred to global_treasury_pda Account")
                    }
                    // entry fee is successfully transferred to global_treasury_pda Account

                    // will still be false; not yet updated after initialized
                    msg!("Line 182 ~ if ctx.accounts.player.lamports ~ full {}", full); // full status
                    msg!(
                        "Line 186 ~ if ctx.accounts.player.lamports ~ gametype.max_players {}",
                        gametype.max_players
                    ); // max_players in game

                    post_add_state = game.players.len(); // In case of 3 players added, this value will be == 3 // repeated for convenience
                    msg!(
                        "Line 185 ~ ifctx.accounts.player.lamports ~ post_add_state {}",
                        post_add_state
                    );
                    if post_add_state as usize == (gametype.max_players) as usize {
                        // >=   ===>  ==

                        full = true; // here the value of full gets updated
                        msg!("Line 188 ~ if ctx.accounts.player.lamports ~ UPDATED full - PHASE 2 {}", full);
                        msg!(
                            "Line 213: Player {} has entered in game, and entryfee is also deducted. And also the game if full",
                            ctx.accounts.player.key.to_string()
                        );
                        msg!(
                            "Line 200: gametype key {} gametype last_game_index {}",
                            gametype.key(),
                            gametype.last_game_index
                        );
                        msg!(
                            "Line 205: game key {} game struct game.Players {:#?}",
                            game.key(),
                            game.players
                        );
                    } else {
                        msg!(
                            "Line 228: Player {} has entered in game, and entryfee is also deducted. But the game has still some space",
                            ctx.accounts.player.key.to_string()
                        );
                        msg!(
                            "Line 211: gametype key {} gametype last_game_index {}",
                            gametype.key(),
                            gametype.last_game_index
                        );
                    }

                    // // phase 3: once all players have fulfilled the room space, we should mark current gamePda account as full, and increase the counter of gameType account
                    // // so that from next time from client side they can direct to next gamePda
                    // // 2 main tasks occurs here: Updates last_game_index+=1 in game_type account
                    // // transfers commission only when game room is full
                    if full {
                        msg!("Line 243: You are inside Full");
                        msg!(
                            "Line 224: Current gametype.key {} Current gametype.last_game_index {}",
                            gametype.key(),
                            gametype.last_game_index
                        );

                        // added logic to store all active full games waiting for winners to declare
                        gametype.active_games_in_one_type.push(game.key()); // added here to fetch later all instances of active_games_in_one_type
                        game.game_full_status = true; // setting this flag as true so that further no player can remove themselves

                        // After entering third player: 3/3 == full
                        // we have to update last_game_index
                        // this will create a new gamePda from client side next time based on

                        gametype.last_game_index += 1; // here is the update that we are looking at.

                        gametype.last_game_index_to_string = gametype.last_game_index.to_string(); // string type

                        msg!("Line 248: Below logic is only for setting up game reward multiplicator");
                        // let treasury_funds = ctx.accounts.game_pda.to_account_info().lamports.borrow(); // donator_program_account.to_account_info().try_borrow_mut_lamports()?
                        let treasury_funds = globaltreasury.lamports(); // In case of game_treasury_pda // // let treasury_funds = ctx.accounts.game_treasury_pda.lamports();

                        let now_ts = Clock::get().unwrap().unix_timestamp;
                        let random = now_ts % 1000 + 1;
                        let players_funds = 3 * entryprice * 9 / 10;

                        // Logic Implementation in Rust Issue: binary operation `>=` cannot be applied to type `Ref<'_, &mut u64>`rustcE0369
                        if random > 690 + 210 + 70 + 29 && treasury_funds >= players_funds * 50 {
                            game.rm = 50;
                        } else if random > 690 + 210 + 70 && treasury_funds >= players_funds * 10 {
                            game.rm = 10;
                        } else if random > 690 + 210 && treasury_funds >= players_funds * 5 {
                            game.rm = 5;
                        } else if random > 690 && treasury_funds >= players_funds * 3 {
                            game.rm = 3;
                        } else {
                            game.rm = 2;
                        }

                        // // Hard coded rm value for the time being when using gamePda itself as a global_treasury_pda
                        // game.rm = 2;

                        msg!("Line 285: game.rm is set under if full");
                        // let final_reward = entryprice * (game.rm as u64); // no more required to send to each game_treasury_account;
                        // already taken in global_treasury_pda account

                        let gametype_previous_last_game_index = gametype.last_game_index - 1;
                        let gametype_previous_last_game_index_string =
                            gametype_previous_last_game_index.to_string();

                        let (game_pda, game_seed) = Pubkey::find_program_address(
                            &[
                                b"GAME".as_ref(),
                                // gametype.last_game_index_to_string.as_ref(),
                                gametype_previous_last_game_index_string.as_ref(),
                            ],
                            ctx.program_id,
                        );
                        msg!(
                            "Line 286 ~ if ctx.accounts.player.lamports ~ game_seed {}",
                            game_seed
                        );
                        msg!(
                            "Line 287 ~ if ctx.accounts.player.lamports ~ game_pda {}",
                            game_pda
                        );
                        msg!("Line 288 ~ if ctx.accounts.player.lamports ~ gametype_previous_last_game_index_string {}", gametype_previous_last_game_index_string);

                        let comission = entryprice * 3 / 10;

                        // transfer final_reward from global_treasury_pda to game_pda // no more required, can be done using global_treasury_itself
                        // invoke_signed(
                        //     &system_instruction::transfer(&global_treasury_pda, &game.key(), final_reward),
                        //     &[
                        //         ctx.accounts.global_treasury_pda.to_account_info(),
                        //         ctx.accounts.game_pda.to_account_info(),
                        //         ctx.accounts.system_program.to_account_info()
                        //     ],
                        //     &[&["Treasury".as_ref(),
                        //         &[bump_seed],
                        //     ]],
                        // )?;
                        msg!("Line 293: About to transfer commission to SolemInc");

                        // // 'Program 11111111111111111111111111111111 invoke [2]', 'Transfer: `from` must not carry data', 'Program 11111111111111111111111111111111 failed: invalid program argument'
                        invoke_signed(
                            &system_instruction::transfer(
                                &global_treasury_pda,
                                &solem_inc_pk,
                                comission,
                            ),
                            &[
                                ctx.accounts.global_treasury_pda.to_account_info(), // &globaltreasury.key()
                                ctx.accounts.solem_inc.to_account_info(),
                                ctx.accounts.system_program.to_account_info(),
                            ],
                            &[&[
                                // In case of GamePDA
                                // "GAME".as_ref(),
                                // // gametype.last_game_index_to_string.as_ref(), // this is already updated; and hence we need to take the existing value
                                // gametype_previous_last_game_index_string.as_ref(), // this is already updated; and hence we need to take the existing value
                                // &[game_seed],

                                // In case of GlobalTreasuryPDA
                                "GlobalTreasury".as_ref(), // TREASURY_PDA_SEED.as_ref(),
                                &[global_treasury_pda_bump_seed],
                            ]],
                        )?;
                        msg!("Line 311, Commission transferred to solem inc only when game room is full");
                    }
                }
            }
        }
        emit!(AddPlayerEvent {
            label: "AddPlayer Method Invoked".to_string(),
        });

        Ok(())
    }

    pub fn remove_player(ctx: Context<RemovePlayer>) -> Result<()> {
        // let gamelist = &mut ctx.accounts.game_list; // data account
        let gametype = &mut ctx.accounts.game_type; // data account
        let game = &mut ctx.accounts.game_pda; // PDA account

        let globaltreasury = &mut ctx.accounts.global_treasury_pda; // PDA account
        let (global_treasury_pda, global_treasury_pda_bump_seed) =
            Pubkey::find_program_address(&[b"GlobalTreasury"], ctx.program_id);

        let refund_amount = gametype.entry_price;

        let player = &ctx.accounts.player; // the player as a signer who wants to remove themselves out

        // If game.game_full_status == false; then only we can proceed to remove a player
        if !game.game_full_status {
            // mutable type
            let list_of_players_available_in_game = &mut game.players;
            let mut flag_to_check_for_player_to_remove = false; // initially we say that the player to remove isn't available in list

            if list_of_players_available_in_game
                .iter()
                .any(|&i| i == player.key())
            {
                flag_to_check_for_player_to_remove = true; // he we check and set true
            };

            let index_at_which_player_found = list_of_players_available_in_game
                .iter()
                .position(|x| *x == player.key())
                .unwrap();

            list_of_players_available_in_game.remove(index_at_which_player_found); //updated list

            if flag_to_check_for_player_to_remove {
                invoke_signed(
                    &system_instruction::transfer(
                        &global_treasury_pda.key(), // local var .key()
                        &ctx.accounts.player.key(),
                        refund_amount,
                    ),
                    &[
                        globaltreasury.to_account_info(),
                        ctx.accounts.player.to_account_info(),
                        ctx.accounts.system_program.to_account_info(),
                    ],
                    &[&[
                        // In case of GamePDA
                        // "GAME".as_ref(),
                        // // gametype.last_game_index_to_string.as_ref(), // this is already updated; and hence we need to take the existing value
                        // gametype_previous_last_game_index_string.as_ref(), // this is already updated; and hence we need to take the existing value
                        // &[game_seed],

                        // In case of GlobalTreasuryPDA
                        "GlobalTreasury".as_ref(), // TREASURY_PDA_SEED.as_ref(),
                        &[global_treasury_pda_bump_seed],
                    ]],
                )?;
            }
        }

        msg!("Line 412: Here we complete RemovePlayer endpoint");

        Ok(())
    }

    pub fn end_game(ctx: Context<EndGame>) -> Result<()> {
        // let gamelist = &mut ctx.accounts.game_list; // data account
        let gametype = &mut ctx.accounts.game_type; // data account
        let game = &mut ctx.accounts.game_pda; // PDA account
                                               // game.winner = ctx.accounts.winner.key();
        let list_of_players_available_in_game = &mut game.players;
        let mut flag_to_declare_winner = false; // initially we say that the player to remove isn't available in list

        if list_of_players_available_in_game
            .iter()
            .any(|&i| i == ctx.accounts.winner.key())
        {
            flag_to_declare_winner = true; // he we check and set true
        };

        if flag_to_declare_winner {
            game.winner = ctx.accounts.winner.key();

            let reward_amount = gametype.entry_price * (game.rm as u64);

            // // below account is not required; as the funds are available in global treasury account
            // let (game_pda, game_seed) = Pubkey::find_program_address(&[&game.key().to_bytes()], ctx.program_id );

            // Instead here we'll derive global_treasury_account
            let globaltreasury = &mut ctx.accounts.global_treasury_pda; // PDA account
            let (global_treasury_pda, global_treasury_pda_bump_seed) =
                Pubkey::find_program_address(&[b"GlobalTreasury"], ctx.program_id);

            invoke_signed(
                &system_instruction::transfer(
                    &global_treasury_pda,
                    &ctx.accounts.winner.key(),
                    reward_amount,
                ),
                &[
                    globaltreasury.to_account_info(),
                    ctx.accounts.winner.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                &[&[
                    // game.key().to_bytes().as_ref(),
                    // &[game_seed],

                    // In case of GamePDA
                    // "GAME".as_ref(),
                    // // gametype.last_game_index_to_string.as_ref(), // this is already updated; and hence we need to take the existing value
                    // gametype_previous_last_game_index_string.as_ref(), // this is already updated; and hence we need to take the existing value
                    // &[game_seed],

                    // In case of GlobalTreasuryPDA
                    "GlobalTreasury".as_ref(), // TREASURY_PDA_SEED.as_ref(),
                    &[global_treasury_pda_bump_seed],
                ]],
            )?;

            // // game remove mechanism from running active games list which is stored in gametype.active_games_in_one_type
            // mutable type
            let list_of_active_games_available_in_gametype = &mut gametype.active_games_in_one_type;

            let index_at_which_game_found = list_of_active_games_available_in_gametype
                .iter()
                .position(|x| *x == game.key())
                .unwrap();

            list_of_active_games_available_in_gametype.remove(index_at_which_game_found); //updated list of active games on endgame endpoint

            // gametype.active_games_in_one_type.pop(game.key());

            msg!(
            "Line 477: The game {} is over. Winner is {} and has been credited of {} lamports, Also game is removed from active games available",
            game.key(),
            &ctx.accounts.winner.key(),
            reward_amount
        );
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGamelistAccount<'info> {
    #[account(mut)]
    pub server: Signer<'info>,

    //#[account(init,payer = server,space = 10000,seeds = [b"GAME_LIST".as_ref()],bump)]
    #[account(init, payer = server, space = 10000)]
    pub game_list: Account<'info, GameList>,

    // #[account(init, payer = server, space = 9000)]
    // pub data : Account<'info,Data>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CreateGameType<'info> {
    //#[account(mut,seeds = [b"GAME_LIST".as_ref()],bump)]
    #[account(mut,constraint=game_list.authority==authority.key())]
    pub game_list: Account<'info, GameList>,

    // #[account(init, payer = authority, space = 9000,seeds = [b"GAME_TYPE".as_ref(),&[game_list_pda.game_type_index]],bump)]
    // #[account(init, payer = authority, space = 9000,seeds = [b"GAME_TYPE".as_ref(),b"1".as_ref()],bump)] // hardcoded string is working
    //#[account(init, payer = authority, space = 9000,seeds = [b"GAME_TYPE".as_ref(),game_list_pda.game_type_index_to_string.as_ref()],bump)] // string is working
    #[account(init, payer = authority, space = 9000)]
    pub game_type: Account<'info, GameType>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddPlayer<'info> {
    //#[account(mut,seeds = [b"GAME_LIST".as_ref()],bump)]
    #[account(mut,constraint=game_list.authority==game_type.authority)]
    pub game_list: Account<'info, GameList>,

    // #[account(mut,seeds = [b"GAME_TYPE".as_ref(),&[data.select_id]],bump)]
    // #[account(mut,seeds = [b"GAME_TYPE".as_ref(),data.select_id_string.as_ref()],bump)]
    #[account(mut,constraint=game_list.authority==game_type.authority)]
    pub game_type: Account<'info, GameType>,

    // #[account(init,payer = authority, space = 10000,seeds = [b"GAME".as_ref(),game_type.last_game_index_to_string.as_ref()],bump)] // will break the code //  'Allocate: account Address { address: 6RM3NZ7BA1R1zw9ZvxyCyJvh3jgSmkLXJBfxN1XLJEfN, base: None } already in use'
    // #[account(init_if_needed,payer = authority, space = 10000,seeds = [b"GAME".as_ref(),game_type.key().as_ref(),game_type.last_game_index_to_string.as_ref()],bump)]
    #[account(init_if_needed,payer = player, space = 10000,seeds = [b"GAME".as_ref(),game_type.key().as_ref(),game_type.last_game_index_to_string.as_ref()],bump)]  // fee payer player instead
    pub game_pda: Account<'info, Game>, // this isnt AccountInfo, in which we can direcly use .lamports()

    /// CHECK:
    #[account(mut)]
    pub global_treasury_pda: AccountInfo<'info>, // since we want to use .lamports() method

    #[account(mut)]
    pub player: Signer<'info>,

    /// CHECK:
    #[account(mut)]
    pub solem_inc: AccountInfo<'info>,

    // no more required
    // #[account(mut)]
    // pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemovePlayer<'info> {
    // //#[account(mut,seeds = [b"GAME_LIST".as_ref()],bump)]
    // #[account(mut)]
    // pub game_list: Account<'info, GameList>,

    // #[account(mut,seeds = [b"GAME_TYPE".as_ref(),&[data.select_id]],bump)]
    //#[account(mut,seeds = [b"GAME_TYPE".as_ref(),data.select_id_string.as_ref()],bump)]
    #[account(mut,constraint=game_pda.authority==game_type.authority)]
    pub game_type: Account<'info, GameType>,

    // #[account(mut,seeds = [b"GAME".as_ref(),game_type.last_game_index_to_string.as_ref()],bump)]
    #[account(mut,constraint=game_pda.authority==game_type.authority)]
    // the seed check constraints are removed here because the game_pda derivation leads to next gamePda because of game_type.last_game_index
    pub game_pda: Account<'info, Game>, // this isnt AccountInfo, in which we can direcly use .lamports()

    /// CHECK:
    #[account(mut)]
    pub global_treasury_pda: AccountInfo<'info>, // since we want to use .lamports() method

    #[account(mut)]
    pub player: Signer<'info>,

    // #[account(mut)]
    // pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EndGame<'info> {
    // //#[account(mut,seeds = [b"GAME_LIST".as_ref()],bump)]
    // #[account(mut)]
    // pub game_list: Account<'info, GameList>,

    // #[account(mut,seeds = [b"GAME_TYPE".as_ref(),&[data.select_id]],bump)]
    //#[account(mut,seeds = [b"GAME_TYPE".as_ref(),data.select_id_string.as_ref()],bump)]
    #[account(mut,has_one=authority)]
    pub game_type: Account<'info, GameType>,

    // #[account(mut,has_one=authority, seeds = [b"GAME".as_ref(),game_type.last_game_index_to_string.as_ref()],bump)]
    #[account(mut,has_one=authority)]
    // the seed check constraints are removed here because the winner would not always be available in lastgameindex + 1
    pub game_pda: Account<'info, Game>, // this isnt AccountInfo, in which we can direcly use .lamports()

    /// CHECK:
    #[account(mut)]
    pub global_treasury_pda: AccountInfo<'info>, // since we want to use .lamports() method

    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub winner: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct GameList {
    pub list_of_game_type_data: Vec<GameTypeData>, // this stores all gametype accounts data
    pub game_type_index: u64,
    pub authority: Pubkey, // added later
                           //pub game_type_index_to_string: String,
}
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GameTypeData {
    // additional struct to store GameType's data in GameList
    pub game_type_key: Pubkey,
    pub game_type_index_of_gamelist: u64, // rm
    pub authority: Pubkey,                // rm
    pub entry_price: u64,
    pub max_players: u64, // rm
    pub max_games: u64,   // rm
}

#[account]
#[derive(Default)]
pub struct GameType {
    pub game_type_index_of_gamelist: u64, // game_type_index_of_gamelist: the index of gametype in gamelist //u32
    pub authority: Pubkey,
    pub entry_price: u64, // u16
    pub max_players: u64, // u8
    pub max_games: u64,   // u64

    pub last_game_index: u64,              // GamePDA specific
    pub last_game_index_to_string: String, // used to pass in seed for derivation of GamePDA
    pub active_games_in_one_type: Vec<Pubkey>,
}

#[account]
#[derive(Default)]
pub struct Game {
    pub authority: Pubkey,
    pub game_type: Pubkey,
    pub players: Vec<Pubkey>,
    pub winner: Pubkey,
    pub rm: u64,                // u8
    pub game_full_status: bool, // flag for checking if the game room is full or not
}

#[event]
pub struct AddPlayerEvent {
    #[index]
    pub label: String,
}