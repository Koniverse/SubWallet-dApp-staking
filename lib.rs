/*
Project overview:
SubWallet is the leading comprehensive non-custodial wallet solution for Polkadot, Substrate & Ethereum ecosystems. With the mission of bringing users closer to Web3, we envision a Web3 multiverse gateway through which users can enjoy multichain services with utmost ease and absolute security. Connecting and using blockchain-based applications is smoother than ever with SubWallet Browser Extension, SubWallet Mobile App and SubWallet Web Dashboard.

Browser Extension: Available on Google Chrome, Brave, Firefox, and Microsoft Edge.
Mobile App: Available on iOS and Android. Use it anywhere on the go.
Web Dashboard: Explore a comprehensive dashboard for asset management and diverse blockchain activities without installations.

Tractions & Key Features
- 500,000 users across all platforms
- 40,000 community members on Twitter, Telegram, and Discord
- A graduate of the Substrate Builders’ Program
- Working with 150+ teams in the ecosystem with the ultimate goal of bringing the best UI/UX to users
- Manage multiple seed-phrase accounts with only 1 master password
- Built-in XCM transfer with a super easy experience
- Manage multi-chain assets (tokens & NFTs) with a token-centric experience
- View multi-chain transaction history
- Stake in-app: direct nomination, nomination pool, collator delegation and liquid staking
- Support all types of hardware wallet: Ledger, Keystone, Polkadot Vault
- Compatible with MetaMask and Polkadot.{js}
- Support EVM chains and Polkadot SDK-based chains.
- Support Substrate Connect - Light Client
- Manage crowdloan portfolio
- Support fiat on-ramp: Transak, Banxa, Coinbase Pay
- Support WalletConnect V2

We are part of the builders program, check out our proposal here: https://forum.astar.network/t/subwallet-dapp-staking-application/6254

You can find us here:
Telegram: https://t.me/subwallet
Discord: https://discord.com/invite/PvjV5mu6Gq
Twitter: https://twitter.com/subwalletapp
*/

#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod subwallet_dapp_staking {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct DappStaking {}

    impl DappStaking {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn say_hello(&self) -> String {
            "Hello, this is SubWallet dApp Staking!".into()
        }
    }
}
