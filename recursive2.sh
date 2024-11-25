kill -9 $(lsof -t -i tcp:8899)
rm -rf test-ledger
solana config set -u localhost
solana-test-validator > /dev/null &
sleep 4