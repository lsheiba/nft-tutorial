use crate::*;
use near_sdk::{ext_contract, Gas, PromiseResult, PromiseOrValue};

#[near_bindgen]
impl Contract {


    #[payable]
    pub fn nft_update_license(&mut self, authorized_id: Option<String>, token_id: TokenId, license: TokenLicense, receiver_id: AccountId){
       //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        let master_id = env::predecessor_account_id();
    
        self.internal_replace_license(&master_id,&token_id,&license);
    
        // Construct the mint log as per the events standard.
        let nft_license_update_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_LICENSE_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_LICENSE_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftUpdateLicense(vec![NftUpdateLicenseLog {
                authorized_id,
                // Owner of the token.
                owner_id: receiver_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_license_update_log.to_string());

        //calculate the required storage which was the used - initial
        let storage_usage = env::storage_usage();
        if storage_usage > initial_storage_usage {
            //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
            refund_deposit(storage_usage - initial_storage_usage);
        }
  }

    #[payable]
    pub fn nft_approve_license(&mut self, authorized_id: Option<String>, token_id: TokenId, receiver_id: AccountId){
       //measure the initial storage being used on the contract
        // assert_one_yocto();

       let initial_storage_usage = env::storage_usage();

       let token = self.tokens_by_id.get(&token_id).expect("No token");
       if receiver_id != token.owner_id {
           panic!("Only the owner can approve a license");
       }

       let master_id = env::predecessor_account_id();

       self.internal_update_license(&master_id, &token_id); 
    
        // Construct the mint log as per the events standard.
        let nft_license_update_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_LICENSE_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_LICENSE_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftApproveLicense(vec![NftApproveLicenseLog {
                authorized_id,
                // Owner of the token.
                owner_id: receiver_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_license_update_log.to_string());

        //calculate the required storage which was the used - initial
        let storage_usage = env::storage_usage();
        if storage_usage > initial_storage_usage {
            //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
            refund_deposit(storage_usage - initial_storage_usage);
        }
    }

    #[payable]
    pub fn nft_propose_license(&mut self, authorized_id: Option<String>,token_id: TokenId, proposed_license: TokenLicense, receiver_id: AccountId){
       //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();
        
        let master_id = env::predecessor_account_id();
    
        self.internal_propose_license(&master_id, &token_id, &proposed_license);

        // Construct the mint log as per the events standard.
        let nft_propose_license_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_LICENSE_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_LICENSE_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftProposeLicense(vec![NftProposeLicenseLog {
                authorized_id,
                // Owner of the token.
                owner_id: receiver_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_propose_license_log.to_string());

        //calculate the required storage which was the used - initial
        let storage_usage = env::storage_usage();
        if storage_usage > initial_storage_usage {
            //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
            refund_deposit(storage_usage - initial_storage_usage);
        }
    }

    //get the information for a specific token ID
    pub fn nft_license(&self, token_id: TokenId) -> Option<JsonTokenLicense> {
        //if there is some token ID in the tokens_by_id collection
        if let Some(token) = self.tokens_by_id.get(&token_id) {
            //we'll get the metadata for that token
            let license = self.token_license_by_id.get(&token_id).unwrap();
            //we return the JsonTokenLicense (wrapped by Some since we return an option)
            Some(JsonTokenLicense {
                token_id,
                owner_id: token.owner_id,
                license,
            })
        } else { //if there wasn't a token ID in the tokens_by_id collection, we return None
            None
        }
    }

    //get the information for a specific token ID
    pub fn nft_proposed_license(&self, token_id: TokenId) -> Option<JsonTokenLicense> {
        //if there is some token ID in the tokens_by_id collection
        if let Some(token) = self.tokens_by_id.get(&token_id) {
            //we'll get the metadata for that token
            let license = self.token_license_by_id.get(&token_id).unwrap();
            let license = self.token_proposed_license_by_id.get(&token_id).unwrap();
            //we return the JsonTokenLicense (wrapped by Some since we return an option)
            Some(JsonTokenLicense {
                token_id,
                owner_id: token.owner_id,
                license,
            })
        } else { //if there wasn't a token ID in the tokens_by_id collection, we return None
            None
        }
    }
    pub fn internal_propose_license(&mut self, account_id: &AccountId, token_id: &TokenId, proposed_license: &TokenLicense) {
        println!("==>internal_propose_license");
        self.token_proposed_license_by_id.remove(&token_id);
        self.token_proposed_license_by_id.insert(&token_id, &proposed_license);
    }

    pub fn internal_update_license(&mut self, account_id: &AccountId, token_id: &TokenId) {
        println!("==>internal_update_license");
        let proposed_license = self.token_proposed_license_by_id.get(&token_id).unwrap();
        self.token_license_by_id.remove(&token_id);
        self.token_license_by_id.insert(&token_id, &proposed_license);
    }

    pub fn internal_replace_license(&mut self, account_id: &AccountId, token_id: &TokenId, license: &TokenLicense) {
        println!("==>internal_replace_license");
        self.token_license_by_id.remove(&token_id);
        self.token_license_by_id.insert(&token_id, &license);
    }

    /*
    #[payable]
    fn request_approval(&mut self, 
        account_id: AccountId, 
        token_id: TokenId, 
        receiver_id: AccountId, 
        proposed_license: TokenLicense,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> {

        println!("==>request_approval");
        assert_one_yocto();
    */
}