//! Explicit, deterministic EN/FR/ES entry-language selection.
//!
//! Apply this policy only to entry URLs that do not already name a language.
//! The country must come from a trusted server-side provider, not from an
//! arbitrary request header supplied by a visitor.

/// A language supported by the public example.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Language {
    /// English, also the fallback when location is absent or ambiguous.
    English,
    /// French.
    French,
    /// Spanish.
    Spanish,
}

impl Language {
    /// Return the stable language code for a localized URL.
    pub const fn code(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::French => "fr",
            Self::Spanish => "es",
        }
    }

    /// Return a same-origin home path built solely from the supported enum.
    pub const fn home_path(self) -> &'static str {
        match self {
            Self::English => "/en/",
            Self::French => "/fr/",
            Self::Spanish => "/es/",
        }
    }
}

/// Choose an initial language for an entry URL without a language prefix.
///
/// France uses French. The listed Spanish-speaking countries use Spanish.
/// Canada uses French only when French is the preferred supported language.
/// All other cases, including an unknown country, use English.
///
/// This product policy is deliberately narrower than a general locale resolver.
/// It does not redirect explicit `/en/`, `/fr/` or `/es/` URLs; callers must
/// preserve those choices before invoking it.
pub fn initial_language(country: Option<&str>, accept_language: &str) -> Language {
    let country = country.unwrap_or("").trim();
    if country.eq_ignore_ascii_case("FR") {
        return Language::French;
    }
    const SPANISH_COUNTRIES: &[&str] = &[
        "AR", "BO", "CL", "CO", "CR", "CU", "DO", "EC", "ES", "GQ", "GT", "HN", "MX", "NI", "PA",
        "PE", "PR", "PY", "SV", "UY", "VE",
    ];
    if SPANISH_COUNTRIES
        .iter()
        .any(|code| country.eq_ignore_ascii_case(code))
    {
        return Language::Spanish;
    }
    if country.eq_ignore_ascii_case("CA")
        && preferred_supported_language(accept_language) == Some(Language::French)
    {
        return Language::French;
    }
    Language::English
}

/// Return the highest-quality supported language in an Accept-Language value.
///
/// Region subtags are accepted, ties preserve header order, `q=0` is excluded,
/// and malformed entries are ignored. Oversized headers return `None`.
/// Wildcards and languages outside EN/FR/ES do not select a language.
pub fn preferred_supported_language(header: &str) -> Option<Language> {
    if header.len() > 4096
        || header
            .bytes()
            .any(|byte| (byte < 32 && byte != b'\t') || byte == 127)
    {
        return None;
    }
    let mut best = None;
    for entry in header.split(',') {
        let Some((language, quality)) = parse_entry(entry) else {
            continue;
        };
        if quality > 0 && best.map_or(true, |(_, score)| quality > score) {
            best = Some((language, quality));
        }
    }
    best.map(|(language, _)| language)
}

fn parse_entry(entry: &str) -> Option<(Language, u16)> {
    let mut parts = entry.trim().split(';');
    let tag = parts.next()?.trim();
    let mut subtags = tag.split('-');
    let base = subtags.next()?;
    if subtags.any(|part| {
        part.is_empty() || part.len() > 8 || !part.bytes().all(|byte| byte.is_ascii_alphanumeric())
    }) {
        return None;
    }
    let language = if base.eq_ignore_ascii_case("en") {
        Language::English
    } else if base.eq_ignore_ascii_case("fr") {
        Language::French
    } else if base.eq_ignore_ascii_case("es") {
        Language::Spanish
    } else {
        return None;
    };
    let quality = match parts.next() {
        None => 1000,
        Some(parameter) => {
            let (name, value) = parameter.trim().split_once('=')?;
            if !name.trim().eq_ignore_ascii_case("q") {
                return None;
            }
            parse_quality(value.trim())?
        }
    };
    if parts.next().is_some() {
        return None;
    }
    Some((language, quality))
}

fn parse_quality(value: &str) -> Option<u16> {
    let (whole, fractional) = value.split_once('.').unwrap_or((value, ""));
    if !matches!(whole, "0" | "1")
        || fractional.len() > 3
        || !fractional.bytes().all(|byte| byte.is_ascii_digit())
    {
        return None;
    }
    if whole == "1" {
        return fractional.bytes().all(|byte| byte == b'0').then_some(1000);
    }
    let mut quality = 0;
    let mut multiplier = 100;
    for digit in fractional.bytes() {
        quality += u16::from(digit - b'0') * multiplier;
        multiplier /= 10;
    }
    Some(quality)
}
