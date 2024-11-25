rm -rf test-ledger
solana-test-validator > /dev/null &
sleep 4
anchor build
anchor deploy
node client_invoker.mjs
# node utility_pda_checker.mjs
# close solana-test-validator
kill -9 $(lsof -t -i tcp:8899)
rm -rf test-ledger