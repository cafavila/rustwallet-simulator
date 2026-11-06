pub struct Wallet {
    id: u32,
    owner_id: u32,
}

impl Wallet {
    pub fn new(id: u32, owner_id: u32) -> Self {
        Self { id, owner_id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn owner_id(&self) -> u32 {
        self.owner_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_wallet() {
        let wallet = Wallet::new(1, 1);
        assert_eq!(wallet.id(), 1);
        assert_eq!(wallet.owner_id(), 1);
    }
}
