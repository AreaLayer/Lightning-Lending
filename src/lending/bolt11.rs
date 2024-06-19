

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use lightning_invoice::Invoice;
use lightning_invoice::InvoiceError;
use lightning_invoice::Encode;
use lightning_invoice::Decode;
use lightning_invoice::Payee;
use lightning_invoice::Amount;
use lightning::BOLT11;

pub fn read_invoice(path: &Path) -> Result<Invoice, InvoiceError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut invoice = None;
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("lnbc") {
            invoice = Some(line);
        }
    }
    if invoice.is_none() {
        return Err(InvoiceError::InvalidInvoice);
    }
    let invoice = invoice.unwrap();
    let invoice = BOLT11::decode(&invoice)?;
    Ok(invoice)
}

pub fn read_invoices(path: &Path) -> Result<HashMap<String, Invoice>, InvoiceError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut invoices = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("lnbc") {
            let invoice = BOLT11::decode(&line)?;
            invoices.insert(invoice.payment_hash.to_string(), invoice);
        }
    }
    Ok(invoices)
}

pub fn write_invoice(path:
    &Path, invoice: &Invoice) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    let invoice = invoice.encode()?;
    file.write_all(invoice.as_bytes())?;
    Ok(())
}

pub fn write_invoices(path:
    &Path, invoices: &HashMap<String, Invoice>) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    for invoice in invoices.values() {
        let invoice = invoice.encode()?;
        file.write_all(invoice.as_bytes())?;
    }
    Ok(())
}

pub fn get_payee(invoice: &Invoice) -> Payee {
    Payee {
        payee: invoice.payee.clone(),
        amount: Amount {
            amount: invoice.amount.amount,
            currency: invoice.amount.currency,
        },
    }
}

pub fn get_amount(invoice: &Invoice) -> Amount {
    Amount {
        amount: invoice.amount.amount,
        currency: invoice.amount.currency,
    }
}

pub fn get_payment_hash(invoice: &Invoice) -> String {
    invoice.payment_hash.to_string()
}

pub fn get_description(invoice: &Invoice) -> String {
    invoice.description.clone()
}

pub fn get_expiry(invoice: &Invoice) -> u32 {
    invoice.expiry
}

pub fn get_timestamp(invoice: &Invoice) -> u32 {
    invoice.timestamp
}

pub fn get_min_final_cltv_expiry(invoice: &Invoice) -> u32 {
    invoice.min_final_cltv_expiry
}

pub fn get_payment_secret(invoice: &Invoice) -> String {
    invoice.payment_secret.clone()
}

pub fn get_features(invoice: &Invoice) -> Vec<u8> {
    invoice.features.clone()
}

pub fn get_routes(invoice: &Invoice) -> Vec<String> {
    invoice.routes.clone()
}

pub fn get_payment_hash_string(invoice: &Invoice) -> String {
    invoice.payment_hash.to_string()
}

pub fn get_payment_secret_string(invoice: &Invoice) -> String {
    invoice.payment_secret.clone()
}

pub fn get_description_string(invoice: &Invoice) -> String {
    invoice.description.clone()
}

pub fn get_expiry_string(invoice: &Invoice) -> String {
    invoice.expiry.to_string()
}

pub fn get
    (invoice: &Invoice) -> String {
    invoice.timestamp.to_string()
}

pub fn
    get_min_final_cltv_expiry_string(invoice: &Invoice) -> String {
    invoice.min_final_cltv_expiry.to_string()
}

pub fn get_routes_string(invoice: &Invoice) -> String {
    invoice.routes.clone().join(",")
}

pub fn
    get_features_string(invoice: &Invoice) -> String {
    invoice.features.clone().join(",")
}

pub fn get_amount_string(invoice: &Invoice) -> String {
    invoice.amount.amount.to_string()
}

pub fn get_currency_string(invoice: &Invoice) -> String {
    invoice.amount.currency.to_string()
}

pub fn get_payee_string(invoice: &Invoice) -> String {
    invoice.payee.clone()
}

pub fn get_timestamp_string(invoice: &Invoice) -> String {
    invoice.timestamp.to_string()
}

pub fn get_min_final_cltv_expiry_string(invoice: &Invoice) -> String {
    invoice.min_final_cltv_expiry.to_string()
}

pub fn get_payment_hash_string(invoice: &Invoice) -> String {
    invoice.payment_hash.to_string()
}

pub fn get_payment_secret_string(invoice: &Invoice) -> String {
    invoice.payment_secret.clone()
}

pub fn get_description_string(invoice: &Invoice) -> String {
    invoice.description.clone()
}

pub fn get_expiry_string(invoice: &Invoice) -> String {
    invoice.expiry.to_string()
}

pub fn get_timestamp_string(invoice: &Invoice) -> String {
    invoice.timestamp.to_string()
}

pub fn get_min_final_cltv_expiry_string(invoice: &Invoice) -> String {
    invoice.min_final_cltv_expiry.to_string()
}

pub fn get_routes_string(invoice: &Invoice) -> String {
    invoice.routes.clone().join(",")
}

pub fn get_features_string(invoice: &Invoice) -> String {
    invoice.features.clone().join(",")
}

pub fn get_amount_string(invoice: &Invoice) -> String {
    invoice.amount.amount.to_string()
}

