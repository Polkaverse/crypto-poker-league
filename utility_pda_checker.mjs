import * as anchor from '@project-serum/anchor'
import { Connection, PublicKey, Keypair, SystemProgram, LAMPORTS_PER_SOL } from '@solana/web3.js'

import { BN } from 'bn.js'

import { Program } from "@project-serum/anchor";

import fs from "fs"

// // required for way 01 and 02: setting up anchor provider using a .env file
import dotenv from 'dotenv';
dotenv.config();


async function main() {


  // // required for way 02: setting up anchor provider using a .env file
  // let connection = new Connection("http://127.0.0.1:8899");
  // let wallet = Keypair.fromSecretKey(new Uint8Array([174,43,21,124,235,242,183,95,89,171,100,127,116,183,210,163,62,17,158,201,65,188,108,8,115,61,176,91,62,135,167,250,118,94,241,20,85,34,25,213,153,93,160,113,100,20,36,164,213,160,146,24,173,202,35,132,251,152,207,201,46,251,197,14]))

  const provider = anchor.AnchorProvider.local();                     // way 01 : local solana config get
  // const provider = anchor.AnchorProvider.env();                    // way 02 : using a .env file
  // const provider = new anchor.AnchorProvider(connection, wallet )  // way 03`: Hardcoding the details for provider here itself

  console.log("🚀 ~ file: client_invoker.mjs ~ line 12 ~ main ~ provider connection", provider.connection._rpcEndpoint);
  console.log("🚀 ~ file: client_invoker.mjs ~ line 12 ~ main ~ provider wallet", provider.wallet.publicKey.toBase58());

  anchor.setProvider(provider)

  const CodetestProgram = anchor.workspace.Codetest;
  console.log("🚀 ~ file: client_invoker.mjs ~ line 17 ~ main ~ CodetestProgram programId", CodetestProgram.programId.toBase58());
  console.log("🚀 ~ file: client_invoker.mjs ~ line 17 ~ main ~ CodetestProgram rpc", CodetestProgram.rpc);

  // create a data_account // this should be created only with the init method, and later should be saved
  // let data_account = anchor.web3.Keypair.generate();
  let data_account = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync("./privatekeys/data_account.json").toString())))
  console.log("🚀 ~ file: client_invoker.mjs ~ line 35 ~ main ~ data_account", data_account.publicKey.toBase58());

  // derive a global gameListPda account
  let [gameListPda] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from("GAME_LIST")],
    CodetestProgram.programId
  );

  console.log("🚀 ~ file: client_invoker.mjs ~ line 41 ~ main ~ gameListPda", gameListPda.toBase58());

  console.log("🚀 ~ file: client_invoker.mjs ~ line 41 ~ main ~ provider.wallet.payer", provider.wallet.payer.publicKey.toBase58());
  console.log("🚀 ~ file: client_invoker.mjs ~ line 41 ~ main ~ typeof(provider.wallet.payer)", typeof (provider.wallet.payer));


  // Invoking init Endpoint // required to invoke only one time
  let transaction_id = await CodetestProgram.methods
    .init()
    .accounts({
      server: provider.wallet.publicKey,
      gameListPda: gameListPda,
      data: data_account.publicKey,
      systemProgram: SystemProgram.programId
    })
    .signers([data_account])
    .rpc();
  console.log("🚀 ~ file: client_invoker.mjs ~ line 42 ~ main ~ init transaction_id: ", transaction_id);

  // Fetching Data from gameListPda account => pub struct GameList == CodetestProgram.account.gameList
  let gameListPdaResult = await CodetestProgram.account.gameList.fetch(gameListPda);
  console.log("🚀 ~ file: client_invoker.mjs ~ line 61 ~ main ~ gameListPdaResult", gameListPdaResult);

  // Assigning last_game_type_index to a variable
  let last_game_type_index = gameListPdaResult.gameTypeIndex;
  console.log("🚀 ~ file: client_invoker.mjs ~ line 68 ~ main ~ last_game_type_index", last_game_type_index);
  console.log("🚀 ~ file: client_invoker.mjs ~ line 68 ~ main ~ typeof(last_game_type_index)", typeof (last_game_type_index));

  let string_last_game_type_index = last_game_type_index.toString()
  console.log("🚀 ~ file: utility_pda_checker.mjs ~ line 74 ~ main ~ string_last_game_type_index", string_last_game_type_index);
  console.log("🚀 ~ file: utility_pda_checker.mjs ~ line 74 ~ main ~ typeof(string_last_game_type_index)", typeof(string_last_game_type_index));

  // Fetching data inside the data account => pub struct Data == CodetestProgram.account.data
  let data_accountResult = await CodetestProgram.account.data.fetch(data_account.publicKey);
  console.log("🚀 ~ file: client_invoker.mjs ~ line 73 ~ main ~ data_accountResult", data_accountResult);
  console.log("🚀 ~ file: client_invoker.mjs ~ line 73 ~ main ~ data_accountResult", data_accountResult.selectId);

  // derive a last_game_type_index specific gameTypePda account
  let [gameTypePda] = await anchor.web3.PublicKey.findProgramAddress(
    [
      Buffer.from("GAME_TYPE"),
      // Buffer.from("1")
      // new Uint8Array(last_game_type_index) // // hardcoded is working
      Buffer.from(last_game_type_index.toString()) // // hardcoded is working
    ],
    CodetestProgram.programId
  );
  console.log("🚀 ~ file: client_invoker.mjs ~ line 88 ~ main ~ gameListPda", gameListPda.toBase58());
  console.log("🚀 ~ file: client_invoker.mjs ~ line 88 ~ main ~ gameTypePda with Buffer.from(last_game_type_index.toString()): ", gameTypePda.toBase58());

  // derive a last_game_type_index specific gameTypePda account
  [gameTypePda] = await anchor.web3.PublicKey.findProgramAddress(
    [
      Buffer.from("GAME_TYPE"),
      // Buffer.from("1")
      new Uint8Array(last_game_type_index) 
      // Buffer.from(last_game_type_index) // // hardcoded is working
    ],
    CodetestProgram.programId
  );
  console.log("🚀 ~ file: client_invoker.mjs ~ line 88 ~ main ~ gameTypePda with new Uint8Array(last_game_type_index): ", gameTypePda.toBase58());

   // derive a last_game_type_index specific gameTypePda account
   [gameTypePda] = await anchor.web3.PublicKey.findProgramAddress(
    [
      Buffer.from("GAME_TYPE"),
      // Buffer.from("1")
      // new Uint8Array(last_game_type_index) // // hardcoded is working
      Buffer.from(string_last_game_type_index) // // hardcoded is working
    ],
    CodetestProgram.programId
  );
  console.log("🚀 ~ file: client_invoker.mjs ~ line 88 ~ main ~ gameTypePda with Buffer.from(string_last_game_type_index): ", gameTypePda.toBase58());



  // // Invoking createGameType Endpoint // required to invoke every time we create a new game type (different based on entry fee)
  // transaction_id = await CodetestProgram.methods
  //   .createGameType(new BN(1 * LAMPORTS_PER_SOL), 3, 6)
  //   .accounts({
  //     gameListPda: gameListPda,
  //     authority: provider.wallet.publicKey,
  //     gameTypePda: gameTypePda,
  //     systemProgram: SystemProgram.programId
  //   })
  //   // .signers([])
  //   .rpc();
  // console.log("🚀 ~ file: client_invoker.mjs ~ line 102 ~ main ~ createGameType transaction_id", transaction_id);

  // // Fetching Data from gameTypePda account => pub struct GameList == CodetestProgram.account.gameList
  // let gameTypePdaResult = await CodetestProgram.account.gameType.fetch(gameTypePda);
  // console.log("🚀 ~ file: client_invoker.mjs ~ line 106 ~ main ~ gameTypePdaResult", gameTypePdaResult);

// gameListPda EJrpvgQEh7cVQ58H9WvXu3fmRw2TY3WG7Nj7XJtjPtsD
// gameTypePda DZmAu2u9LtoXfUrhro3s4aTYpFsLPzjAJMEeDFMSaHMZ
// with "1" as hard coded string gameTypePda: 2K5s7K1Go45ThtQkRjXPeWnKTNVXyP5r5GSdrc6PRLAn
// authority GruSFAjP7gtmJ9k3SBAiCrMXyUByGJKR885MhKWM9KJD

  


}

main()


// Privilege Extension
// CPIs extend the privileges of the caller to the callee. 
// The puppet account was passed as a mutable account to the puppet-master 
// but it was still mutable in the puppet program as well

// Privilege extension is convenient but also dangerous. 
// If a CPI is unintentionally made to a malicious program, 
// this program has the same privileges as the caller. 
// Anchor protects you from CPIs to malicious programs with two measures. 
// First, the Program<'info, T> type checks that the given account is the expected program T. 
// Should you ever forget to use the Program type, the automatically generated cpi function 
// (in the previous example this was puppet::cpi::set_data) also checks that the cpi_program argument equals the expected program.