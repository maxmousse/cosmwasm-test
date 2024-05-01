# Cosmwasm Test

This repository contains a simple Cosmwasm smart contract that implements a donation platform.

## Features

- Contract is instantiated with a list of projects and a fee collector wallet address
- Contract allows users to donate CW20 tokens to a project.
- A project has a name and the wallet address of its creator
- A project keeps track of the donations received from each user
- Donations history can be queried by user wallet address
- Donations are sent to the project creator's wallet address (minus fees)
- Contract deducts 10% fee if the donation is less than 10,000 CW20 tokens, 5% otherwise. Fees are sent to the fee collector wallet
