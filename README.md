# ZKmine
Welcome to the zk-mine;

In this example, our program retrieves the user's Ethereum address and checks the difficulty modulus. If the difficulty is set to 2, this means 1 out of every 2 addresses can receive a reward, airdrop, or any other incentive you choose to provide.

If you'd like the airdrop to target 1 out of every 10 users, simply adjust the difficulty to 10.

## Contact

[![Gitter](https://img.shields.io/gitter/room/nwjs/nw.js.svg)](https://x.com/yasinaktimur/)
[![GitHub Issues](https://img.shields.io/badge/open%20issues-0-yellow.svg)](https://github.com/omgbbqhaxx/zkmine/issues)

- Chat in [twitter](https://x.com/yasinaktimur).
- Submit your zk-app [Google forms](https://forms.gle/YKo3P8XSDx4KYD768).



## Getting Started

Documents about zkRust hosts the **[alignedlayer docs](https://docs.alignedlayer.com/)**  which
has a Quick Start section.

ZKVM's | Status
---------------- | ----------
SP1 | [![TravisCI](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://travis-ci.org/cloudbank/cloudbank-github)
RISC0         | [![AppVeyor](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://ci.appveyor.com/project/cloudbank/cloudbank-github)

 


## Quick Ardrops

You can distribute quick airdrops to your community. Participants can check their eligibility on your website or program, and if they’re lucky, they can claim their airdrops.

## Random Number Generator

This secret ZK code generates a random number from the user's address. The formula is unknown to others, allowing you to create luck-based games or other chance-driven activities in a secure and private manner.

## Synthetic Sports Games

By using this Random Number Generator model, you can create synthetic sports games where synthetic winners are derived from participants' wallets.


## Nonce problem

In the current setup, one wallet can generate only one random number. However, we will find a way to add a nonce, allowing each wallet to generate multiple signatures.

## Origin of the "ZKMine" Name

If you increase the difficulty, people can develop mining programs to test various wallets in an attempt to win rewards. This concept allows users to engage in "wallet mining" to mint or earn rewards from the contract.

## Usagement
First of all please add zkmine program in your zkRust files example folder.
sudo apt-get install nginx
You must be in zkRust folder

with this commant you can test zkMine program instantly.

```shell
sudo cargo run --release -- prove-sp1 examples/zkmine

sudo cargo run --release -- prove-sp1 examples/zkmine --submit-to-aligned --keystore-path ~/.foundry/keystores/key.json
```

When you create an ELF file.

Clone the aligned repo and run 

sudo make install_aligned_compiling

If the binary doesn’t work

```shell
aligned get-vk-commitment --verification_key_file <path_to_input_file> --proving_system SP1
```

## License

[![License](https://img.shields.io/github/license/ethereum/cpp-ethereum.svg)](LICENSE)

All contributions are made under the [GNU General Public License v3](https://www.gnu.org/licenses/gpl-3.0.en.html). See [LICENSE](LICENSE).