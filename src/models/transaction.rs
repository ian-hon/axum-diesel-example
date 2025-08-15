use bigdecimal::BigDecimal;
use uuid::Uuid;

#[derive(Debug)]
pub struct Transaction {
    pub id: Uuid,
    pub amount: BigDecimal,
    pub recipient: Uuid,
    pub sender: Uuid,
    pub timestamp: jiff::Timestamp,
}
