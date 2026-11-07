pub enum TransferMode {
    Internal,
    External,
}

pub enum TransferStatus {
    Pending,
    Reserved,
    InCustody,
    Approved,
    Completed,
    Rejected,
    Reversed,
    Cancelled,
}

pub struct Transfer {
    id: u32,
    sender_account_id: u32,
    receiver_account_id: Option<u32>,
    asset_id: u32,
    amount: u32,
    mode: TransferMode,
    status: TransferStatus,
    sender_approved: bool,
    receiver_approved: bool,
    external_reference: Option<String>,
}
