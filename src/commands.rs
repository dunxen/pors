use crate::{
    args::{Commands, DecodeInvoiceArgs, DecodeOfferArgs},
    output,
};

use lightning::offers::offer::Offer;
use lightning_invoice::SignedRawInvoice;
use miette::Report;

pub fn handle_command(command: &Option<Commands>) -> Result<(), Report> {
    match command {
        Some(Commands::DecodeInvoice(ref args)) => decode_invoice(args),
        Some(Commands::DecodeOffer(ref args)) => decode_offer(args),
        Some(Commands::FindPaymentPaths(ref _args)) => unimplemented!(),
        None => Ok(()),
    }
}

// Handlers
fn decode_invoice(args: &DecodeInvoiceArgs) -> Result<(), Report> {
    let inv = args.data.parse::<SignedRawInvoice>().unwrap();
    let raw_inv = inv.raw_invoice();
    let timestamp = inv.data.timestamp.clone().as_duration_since_epoch();
    let is_expired = true; //FIXME
    let verification_check = if inv.check_signature() {
        "\u{2705} Signature valid"
    } else {
        "\u{274C} Signature invalid"
    };
    let payment_hash = raw_inv
        .payment_hash()
        .map(|hash| hash.0.to_string())
        .unwrap_or("".into());
    let description = inv
        .description()
        .map(|desc| desc.to_string())
        .unwrap_or("None".to_string());
    let payee_pubkey = raw_inv
        .payee_pub_key()
        .map(|pk| pk.to_string())
        .unwrap_or("None".to_string());
    let min_final_cltv_delta = raw_inv
        .min_final_cltv_expiry_delta()
        .map(|a| format!("{} blocks", a.0))
        .unwrap_or("None".to_string());
    let message = format!(
        r#"
Invoice:

{verification_check}

Timestamp:              {timestamp:#?}
Is expired:             {is_expired:#?}
Payment hash:           {payment_hash}
Description:            {description}
Payee pubkey:           {payee_pubkey}
Min final CLTV delta:   {min_final_cltv_delta}
"#
    );
    output::print(&message)?;
    Ok(())
}

fn decode_offer(args: &DecodeOfferArgs) -> Result<(), Report> {
    let offer = args.data.parse::<Offer>().unwrap();
    let issuer = offer
        .issuer()
        .unwrap_or(lightning::util::string::PrintableString("Unknown"));
    let is_expired = offer.is_expired();
    let message = format!(
        r#"
Offer:

Issuer (unverified):    {issuer:#?}
Is expired:             {is_expired:#?}
"#
    );
    output::print(&message)?;
    Ok(())
}
