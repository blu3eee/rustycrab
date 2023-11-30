use crate::{ twilightrs::commands::context::ParsedArg, utilities::app_error::BoxedError };

pub fn split_trigger_and_value(
    command_args: Vec<ParsedArg>
) -> Result<(String, String), BoxedError> {
    let args = (match command_args.first() {
        Some(&ParsedArg::Text(ref args)) => Ok(args),
        _ => Err("invalid command"),
    })?;

    // validate the url
    Ok(
        if let Some(idx) = args.find("|") {
            let (trigger_part, response_part) = args.split_at(idx);
            (trigger_part.trim().to_string(), response_part[1..].trim().to_string())
        } else {
            (args.trim().to_string(), "".to_string())
        }
    )
}
