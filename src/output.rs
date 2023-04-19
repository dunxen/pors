use crate::errors::Result;

/// Print a human-readable output to stdout
pub fn print(message: &str) -> Result<()> {
    print!("{message}");
    println!();

    Ok(())
}

// TODO: Print a machine-readable output (JSON) to stdout
// pub fn print_json(message: &str) -> Result<()> {
//     /// The output
//     #[derive(Serialize, Deserialize)]
//     struct JsonOutput {
//         /// The base message
//         message: String,
//     }

//     // Build the output
//     let output = JsonOutput {
//         message: message.to_owned(),
//     };

//     // Serialize it to stdout
//     serde_json::to_writer_pretty(std::io::stdout(), &output)?;

//     Ok(())
// }
