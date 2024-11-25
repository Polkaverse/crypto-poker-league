import * as web3 from "@solana/web3.js"
import * as anchor from "@project-serum/anchor";
// import { Codetest } from "../target/types/codetest";
import dotenv from 'dotenv';
dotenv.config();
const CodetestProgram = anchor.workspace.Codetest;


async function main() {
    // First Time GAME PDA Check
    let [gamePda_P1] = await web3.PublicKey.findProgramAddress(
        [
            Buffer.from("GAME"),
            // new Uint8Array(last_game_index) // // numeric type check - It's not working here
            Buffer.from("1") // // string is working
        ],
        new web3.PublicKey("AxmxjMWgQcMQRCbT6oF4PMhJfNGY9YnWNZcm4TowE3H9")
    );
    console.log(`🚀 ~ file: client_invoker.mjs ~ line 191 ~ main ~ gamePda_P1: ${gamePda_P1.toBase58()}`);

    // First Time GAME PDA Check
    let [gamePda_P2] = await web3.PublicKey.findProgramAddress(
        [
            Buffer.from("GAME"),
            // new Uint8Array(last_game_index) // // numeric type check - It's not working here
            Buffer.from("2") // // string is working
        ],
        new web3.PublicKey("AxmxjMWgQcMQRCbT6oF4PMhJfNGY9YnWNZcm4TowE3H9")
    );
    console.log(`🚀 ~ file: client_invoker.mjs ~ line 191 ~ main ~ gamePda_P2: ${gamePda_P2.toBase58()}`);


    // First Time GAME PDA Check
    let [gamePda_P3] = await web3.PublicKey.findProgramAddress(
        [
            Buffer.from("GAME"),
            // new Uint8Array(last_game_index) // // numeric type check - It's not working here
            Buffer.from("3") // // string is working
        ],
        new web3.PublicKey("AxmxjMWgQcMQRCbT6oF4PMhJfNGY9YnWNZcm4TowE3H9")
    );
    console.log(`🚀 ~ file: client_invoker.mjs ~ line 191 ~ main ~ gamePda_P3: ${gamePda_P3.toBase58()}`);


    // 6RM3NZ7BA1R1zw9ZvxyCyJvh3jgSmkLXJBfxN1XLJEfN
    // 5muLpeCAYg1SHnNRktE2ZZMtHp2P23GUG1fNrnfE6g9X
    // BmPACWcLnHDJwKkxX8wjai9ZYknZrnU8FLqX1FKn1ELS

    let gameType = "FP83dhzdPdgvK2LHYeYRDWy2r1XP1Gha53uymtZSg5sW";
    gameType = new web3.PublicKey(gameType);
    console.log("🚀 ~ file: generatePDA.mjs ~ line 45 ~ main ~ gameType", gameType);

    // After Players are added, checking gameTypeAccount.activeGamesInOneType
    let gameTypeResult_for_check = await CodetestProgram.account.gameType.fetch(gameType);
    console.log("🚀 ~ file: Codetest.ts ~ line 265 ~ it ~ gameTypeResult_for_check", gameTypeResult_for_check);
    console.log("🚀 ~ file: Codetest.ts ~ line 266 ~ it ~ gameTypeResult_for_check", gameTypeResult_for_check.activeGamesInOneType);
}

main()