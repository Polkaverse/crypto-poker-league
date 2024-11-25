# cargo clean
anchor build
rm ./target/deploy/codetest-keypair.json
cp ./privatekeys/codetest-keypair.json ./target/deploy/
anchor deploy
node client_invoker.mjs
kill -9 $(lsof -t -i tcp:8899)
rm -rf test-ledger