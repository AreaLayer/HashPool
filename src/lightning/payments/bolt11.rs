use crate::util::error::Error;
use std::str::FromStr;
use std::fmt;

use lightning_invoice::Invoice;
use lightning_invoice::InvoiceError;
use lightning_invoice::InvoiceFeatures;
use lightning_invoice::BOLT11;
use lightning_invoice::InvoiceSignature;
use lightning_invoice::InvoiceSignatureHashType;

pub struct LightningInvoice {
    pub invoice: Invoice,
    pub signature: InvoiceSignature,
}

impl LightningInvoice {
    pub fn new(invoice: Invoice, signature: InvoiceSignature) -> Self {
        LightningInvoice {
            invoice,
            signature,
        }
    }

pub fn get_payment_hash(&self) -> Result<[u8; 32], Error> {
        let payment_hash = self.invoice.payment_hash()?;
        Ok(payment_hash)
    }

    pub fn get_payment_secret(&self) -> Result<[u8; 32], Error> {
        let payment_secret = self.invoice.payment_secret()?;
        Ok(payment_secret)
    }

    pub fn get_description(&self) -> Result<String, Error> {
        let description = self.invoice.description();
        Ok(description)
    }

    pub fn get_description_hash(&self) -> Result<[u8; 32], Error> {
        let description_hash = self.invoice.description_hash()?;
        Ok(description_hash)
    }
