# Steps to Deploy

## Change the Anchor.toml file 
- under provider **change** `cluster = "Localnet"` to `cluster = "devnet"` and `wallet.. ` to `wallet = "./id.json"`. 
- change `[programs.localnet]` to `[programs.devnet]` 

## Create a new solana wallet to deploy
- cd to your crowd funding project and run : 

```bash
solana-keygen new -o id.json 
# pub key : Hp7kGgTFm5uPSfjRr6qxpocDfJBW3QU9WQmfKsdo91BP
``` 
- airdrop sol to have some funds to deploy

```bash
solana airdrop 1 Hp7kGgTFm5uPSfjRr6qxpocDfJBW3QU9WQmfKsdo91BP --url devnet
```

## get the id 

```bash
solana address -k ./target/deploy/crowd_funding-keypair.json
# GqnNEhcmYVAUjKLXSWZLQR7nXSeTgAG56Fw1ADs4DToT
```

## update the program id 

- go to Anchor.toml file 
- replace 

-----

## Incase of Error 

```bash
# generate a new program keypair
solana-keygen new -o ./target/deploy/crowd_funding-keypair.json --force

# get the new program id
solana address -k ./target/deploy/crowd_funding-keypair.json

# in lib.rs
# declare_id!("NEW_PROGRAM_ID"); // Replace with ID from step 2

# in Anchor.toml
# [programs.devnet]
# crowd_funding = "NEW_PROGRAM_ID" # Replace with ID from step 2
```

```bash
# Clean previous build
anchor clean

# Build program
anchor build

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

----

## Deployed
- CRL1NdEMLm5ccWQGGHRMQMWPA2iN6Au51S37QeqmEVhB