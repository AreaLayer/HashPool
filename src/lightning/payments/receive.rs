use crate::lightning::{
    invoice::Invoice,
    keysend::{Keysend, KeysendResponse},
    payment::{Payment, PaymentStatus},
    utils::{get_invoice_amount, get_invoice_expiry},
};

fn main () {
    let invoice = Invoice {
        amount: 1000,
        description: "test".to_string(),
        description_hash: None,
        expiry: 1000,
        payment_hash: None,
        payment_hash_algo: None,
        payment_secret: None,
        timestamp: None,
        min_final_cltv_expiry: None,
        fallback_address: None,
        routing_info: None,
        routes: None,
        features: None,
        signature: None,
    };

}

impl Invoice {
    pub fn new(amount: u64, description: String) -> Self {
        Self {
            amount,
            description,
            description_hash: None,
            expiry: 1000,
            payment_hash: None,
            payment_hash_algo: None,
            payment_secret: None,
            timestamp: None,
            min_final_cltv_expiry: None,
            fallback_address: None,
            routing_info: None,
            routes: None,
            features: None,
            signature: None,
        }
    }
    pub fn get_invoice_amount(&self) -> u64 {
        self.amount
    }
    pub fn get_invoice_expiry(&self) -> u64 {
        self.expiry
    }
}
