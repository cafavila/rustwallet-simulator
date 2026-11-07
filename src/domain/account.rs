#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountKind {
    Available,
    Blocked,
    Staking,
    InTransit,
}

pub struct Account {
    id: u32,
    wallet_id: u32,
    asset_id: u32,
    account_kind: AccountKind,
}

impl Account {
    pub fn new(id: u32, wallet_id: u32, asset_id: u32, account_kind: AccountKind) -> Self {
        Self {
            id,
            wallet_id,
            asset_id,
            account_kind,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn wallet_id(&self) -> u32 {
        self.wallet_id
    }
    pub fn asset_id(&self) -> u32 {
        self.asset_id
    }
    pub fn account_kind(&self) -> AccountKind {
        self.account_kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_new() {
        let account = Account::new(1, 1, 1, AccountKind::Available);
        assert_eq!(account.id(), 1);
        assert_eq!(account.wallet_id(), 1);
        assert_eq!(account.asset_id(), 1);
        assert_eq!(account.account_kind(), AccountKind::Available);
    }
}
