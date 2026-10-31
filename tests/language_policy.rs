use paulinsider_web_foundations::language::{
    initial_language, preferred_supported_language, Language,
};

#[test]
fn unknown_and_other_regions_open_in_english() {
    for country in [
        None,
        Some(""),
        Some("XX"),
        Some("US"),
        Some("GB"),
        Some("DE"),
    ] {
        assert_eq!(initial_language(country, "fr,es;q=0.9"), Language::English);
    }
}

#[test]
fn france_and_spanish_speaking_regions_have_explicit_defaults() {
    assert_eq!(initial_language(Some("FR"), "en"), Language::French);
    for country in [
        "AR", "BO", "CL", "CO", "CR", "CU", "DO", "EC", "ES", "GQ", "GT", "HN", "MX", "NI", "PA",
        "PE", "PR", "PY", "SV", "UY", "VE",
    ] {
        assert_eq!(initial_language(Some(country), "en"), Language::Spanish);
    }
    assert_eq!(initial_language(Some(" mx "), "en"), Language::Spanish);
}

#[test]
fn canada_is_not_assumed_to_be_french_speaking() {
    assert_eq!(initial_language(Some("CA"), ""), Language::English);
    assert_eq!(
        initial_language(Some("CA"), "en-CA,fr;q=0.8"),
        Language::English
    );
    assert_eq!(
        initial_language(Some("CA"), "fr-CA,en;q=0.8"),
        Language::French
    );
    assert_eq!(
        initial_language(Some("CA"), "fr;q=0,en;q=1"),
        Language::English
    );
    assert_eq!(
        initial_language(Some("CA"), "es,fr;q=0.8"),
        Language::English
    );
}

#[test]
fn quality_and_header_order_resolve_supported_preferences() {
    for (header, expected) in [
        ("fr;q=0.5,en;q=0.9", Some(Language::English)),
        ("fr;q=0.9,en;q=0.9", Some(Language::French)),
        ("es-MX,fr;q=0.7", Some(Language::Spanish)),
        ("EN-us;q=0.2,FR-ca;q=1.000", Some(Language::French)),
        ("*,de,en;q=0.123", Some(Language::English)),
        ("en;q=0,fr;q=0,es;q=0", None),
    ] {
        assert_eq!(preferred_supported_language(header), expected, "{header}");
    }
}

#[test]
fn malformed_preferences_do_not_promote_an_invalid_language() {
    for header in [
        "fr;q=2",
        "fr;q=.9",
        "fr;q=-1",
        "fr;q=NaN",
        "fr;q=0.1234",
        "fr;q=1.001",
        "fr;q=0.5;q=1",
        "fr;other=1",
        "fr-",
        "fr-@",
        "fr\r\n",
    ] {
        assert_eq!(preferred_supported_language(header), None, "{header}");
    }
    assert_eq!(preferred_supported_language(&"f".repeat(4097)), None);
}

#[test]
fn paths_are_static_and_cannot_contain_a_supplied_redirect_target() {
    for (language, code, path) in [
        (Language::English, "en", "/en/"),
        (Language::French, "fr", "/fr/"),
        (Language::Spanish, "es", "/es/"),
    ] {
        assert_eq!(language.code(), code);
        assert_eq!(language.home_path(), path);
    }
    assert_eq!(
        initial_language(Some("https://example.invalid"), "fr"),
        Language::English
    );
}
