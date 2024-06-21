use std::collections::HashMap;
use ldk::lightning::Lightning_invoice;
use ldk::lightning::Lightning_invoice::*;
use ldk::bitcoin::secp256k1_zkp::PublicKey;
use ldk::bitcoin::secp256k1_zkp::Signature;
use ldk::bitcoin::secp256k1_zkp::SignatureHash;
use ldk::bitcoin::secp256k1_zkp::Message;
use ldk::lightning::BOLT12;
use ldk_node::BOLT12_PREFIX;
use ldk_node::BOLT12_SEPARATOR;

pub fn invoice_to_bolt12(invoice: &Lightning_invoice) -> String {
    let mut bolt12 = BOLT12_PREFIX.to_string();
    bolt12.push_str(&invoice.payment_hash.to_hex());
    bolt12.push_str(BOLT12_SEPARATOR);
    bolt12.push_str(&invoice.description);
    bolt12.push_str(BOLT12_SEPARATOR);
    bolt12.push_str(&invoice.expiry_time.to_string());
    bolt12.push_str(BOLT12_SEPARATOR);
    bolt12.push_str(&invoice.cltv_expiry.to_string());
    bolt12.push_str(BOLT12_SEPARATOR);
    bolt12.push_str(&invoice.payment_secret.to_hex());
    bolt12.push_str(BOLT12_SEPARATOR);
    bolt12.push_str(&invoice.features.to_hex());

impl bolt12_to_invoice(bolt12: &str) -> Result<Lightning_invoice, String> {
    let mut split = bolt12.split(BOLT12_SEPARATOR);
    let mut invoice = Lightning_invoice::new();
    invoice.payment_hash = split.next().ok_or("Invalid bolt12 string
    (missing payment_hash)")?.from_hex().map_err(|_| "Invalid bolt12 string
    (invalid payment_hash)")?;
    invoice.description = split.next().ok_or("Invalid bolt12 string
    (missing description)")?.to_string();
    invoice.expiry_time = split.next().ok_or("Invalid bolt12 string
    (missing expiry_time)")?.parse::<u32>().map_err(|_| "Invalid bolt12 string
    (invalid expiry_time)")?;
    invoice.cltv_expiry = split.next().ok_or("Invalid bolt12 string
    (missing cltv_expiry)")?.parse::<u32>().map_err(|_| "Invalid bolt12 string
    (invalid cltv_expiry)")?;
    invoice.payment_secret = split.next().ok_or("Invalid bolt12 string
    (missing payment_secret)")?.from_hex().map_err(|_| "Invalid bolt12 string
    (invalid payment_secret)")?;
    invoice.features = split.next().ok_or("Invalid bolt12 string
    (missing features)")?.from_hex().map_err(|_| "Invalid bolt12 string
    (invalid features)")?;
    Ok(invoice)
    }
}
