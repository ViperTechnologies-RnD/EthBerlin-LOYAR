        use fuels::prelude::*;
        use fuels::signers::fuel_crypto::SecretKey;
        use std::str::FromStr;

        // Create a provider pointing to the testnet.
        let provider = Provider::connect("node-beta-1.fuel.network").await.unwrap();

        let phrase = "two task reflect transfer club shy home dragon guilt upon left zone bridge guitar fuel shaft what noodle over scatter buffalo dress unaware force";
                
                
        // Create first account from mnemonic phrase.
        let wallet = WalletUnlocked::new_from_mnemonic_phrase_with_path(
            phrase,
            Some(provider.clone()),
            "m/44'/1179993420'/0'/0/0",
        )?;

        // Create the wallet
        // let wallet = WalletUnlocked::new_from_private_key(secret, Some(provider));

        // Get the wallet address. Used later with the faucet
        dbg!(wallet.address().to_string());
