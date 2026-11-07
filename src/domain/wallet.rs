use crate::domain::account::*;

pub enum WalletError {
    AccountAlreadyExists,
    AccountNotFound,
    AccountNotBelongsToWallet,
}

pub struct Wallet {
    id: u32,
    owner_id: u32,
    accounts: Vec<Account>,
}

impl Wallet {
    pub fn new(id: u32, owner_id: u32) -> Self {
        Self {
            id,
            owner_id,
            accounts: Vec::new(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn owner_id(&self) -> u32 {
        self.owner_id
    }

    pub fn accounts(&self) -> &[Account] {
        &self.accounts
    }

    pub fn add_account(&mut self, account: Account) -> Result<(), WalletError> {
        if account.wallet_id() != self.id() {
            return Err(WalletError::AccountNotBelongsToWallet);
        }
        if self.accounts.iter().any(|acc| acc.id() == account.id()) {
            return Err(WalletError::AccountAlreadyExists);
        }
        self.accounts.push(account);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::asset::Asset;

    #[test]
    fn new_wallet() {
        let wallet = Wallet::new(1, 1);
        assert_eq!(wallet.id(), 1);
        assert_eq!(wallet.owner_id(), 1);
    }

    #[test]
    fn add_account_successfully() {
        let mut wallet = Wallet::new(1, 1);
        Asset::new(1, String::from("KRNC"), String::from("Kraken Coin"), 8);
        let account = Account::new(10, 1, 1, AccountKind::Available);
        match wallet.add_account(account) {
            Ok(()) => assert_eq!(wallet.accounts().len(), 1),
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn add_account_wallet_invalid() {
        let mut wallet = Wallet::new(1, 1);
        Asset::new(1, String::from("KRNC"), String::from("Kraken Coin"), 8);
        let account = Account::new(10, 2, 2, AccountKind::Available);
        assert!(matches!(
            wallet.add_account(account),
            Err(WalletError::AccountNotBelongsToWallet)
        ));
    }

    #[test]
    fn add_account_already_exists() {
        let mut wallet = Wallet::new(1, 1);
        Asset::new(1, String::from("KRNC"), String::from("Kraken Coin"), 8);
        let accounta = Account::new(10, 1, 1, AccountKind::Available);
        let account = Account::new(10, 1, 1, AccountKind::Available);
        let _ = wallet.add_account(accounta);
        assert!(matches!(
            wallet.add_account(account),
            Err(WalletError::AccountAlreadyExists)
        ));
    }
}
