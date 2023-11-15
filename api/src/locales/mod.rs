use fluent::{ FluentArgs, FluentResource };
use fluent_bundle::bundle::FluentBundle;
use intl_memoizer::concurrent::IntlLangMemoizer;
use unic_langid::LanguageIdentifier;
use std::fs;

pub fn load_localization(
    locale: &str
) -> FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer> {
    let ftl_path = format!("src/locales/{}/main.ftl", locale);
    let ftl_string = fs::read_to_string(ftl_path).expect("Failed to read FTL file");

    let ftl_resource = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

    let lang_id: LanguageIdentifier = locale.parse().expect("Parsing locale failed");

    let mut bundle = FluentBundle::new_concurrent(vec![lang_id]);
    bundle.add_resource(ftl_resource).expect("Failed to add FTL resource to the bundle.");

    bundle
}

pub fn get_localized_string(
    bundle: &FluentBundle<FluentResource, IntlLangMemoizer>,
    key: &str,
    args: Option<&FluentArgs>
) -> Option<String> {
    let message = bundle.get_message(key);

    if let Some(message) = message {
        let mut errors = vec![];
        let pattern = message.value().expect("Message has no value.");

        let formatted_pattern = bundle.format_pattern(pattern, args, &mut errors);
        if !errors.is_empty() {
            println!("Errors during message formatting: {:?}", errors);
        }

        Some(formatted_pattern.into())
    } else {
        None
    }
}
