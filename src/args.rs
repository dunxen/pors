use clap::{
    builder::{PossibleValuesParser, TypedValueParser},
    Args, Parser, Subcommand, ValueEnum,
};
use tracing::level_filters::LevelFilter;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(args_conflicts_with_subcommands = true)]
#[clap(arg_required_else_help = true)]
pub struct CliArgs {
    /// Log level
    #[clap(long)]
    #[clap(default_value_t = LevelFilter::WARN)]
    #[clap(value_parser = PossibleValuesParser::new(["off", "error", "warn", "info", "debug", "trace"]).map(|s| s.parse::<LevelFilter>().expect("possible values are valid")))]
    #[clap(help_heading = "GLOBAL OPTIONS", global = true)]
    pub verbose: LevelFilter,

    /// The format of the output
    #[clap(long, value_enum)]
    #[clap(default_value_t = OutputFormat::Human)]
    #[clap(help_heading = "GLOBAL OPTIONS", global = true)]
    pub output_format: OutputFormat,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Style of output we should produce
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable output
    Human,
    /// Machine-readable JSON output
    Json,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Decodes a BOLT11 payment request
    DecodeInvoice(DecodeInvoiceArgs),
    /// Decode a BOLT12 offer
    DecodeOffer(DecodeOfferArgs),
    /// Find candidate paths given an invoice
    FindPaymentPaths(FindPaymentPathsArgs),
}

#[derive(Args)]
pub struct DecodeInvoiceArgs {
    pub data: String,
}

#[derive(Args)]
pub struct DecodeOfferArgs {
    pub data: String,
}

#[derive(Args)]
pub struct FindPaymentPathsArgs {
    pub data: String,
}
