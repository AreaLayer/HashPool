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
    
    impl bolt12_to_invoice(bolt12, &str) -> Result<LightningInvoice, String> {
        let mut split = bolt12.split(BOLT12_SEPARATOR);
        
        let mut invoice = LightningInvoice::new();
        
        // Extract payment_hash
        invoice.payment_hash = split
            .next()
            .ok_or_else(|| "Invalid bolt12 string: missing payment_hash".to_string())?
            .from_hex()
            .map_err(|_| "Invalid bolt12 string: invalid payment_hash".to_string())?;
        
        // Extract description
        invoice.description = split
            .next()
            .ok_or_else(|| "Invalid bolt12 string: missing description".to_string())?
            .to_string();
        
        // Extract expiry_time
        invoice.expiry_time = split
            .next()
            .ok_or_else(|| "Invalid bolt12 string: missing expiry_time".to_string())?
            .parse::<u32>()
            .map_err(|_| "Invalid bolt12 string: invalid expiry_time".to_string())?;
        
        // Extract cltv_expiry
        invoice.cltv_expiry = split
            .next()
            .ok_or_else(|| "Invalid bolt12 string: missing cltv_expiry".to_string())?
            .parse::<u32>()
            .map_err(|_| "Invalid bolt12 string: invalid cltv_expiry".to_string())?;
        
        // Extract payment_secret
        invoice.payment_secret = split
            .next()
            .ok_or_else(|| "Invalid bolt12 string: missing payment_secret".to_string())?
            .from_hex()
            .map_err(|_| "Invalid bolt12 string: invalid payment_secret".to_string())?;
        
        // Extract features
        invoice.features = split
            .next()
            .ok_or_else(|| "Invalid bolt12 string: missing features".to_string())?
            .from_hex()
            .map_err(|_| "Invalid bolt12 string: invalid features".to_string())?;
        
        Ok(invoice)
    }
    
