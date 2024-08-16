use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct Country {
    #[schema(example = "Australia")]
    pub name: &'static str,
    #[schema(example = "Australia")]
    pub english_name: &'static str,
    #[schema(example = "au")]
    pub code: &'static str,
}

impl Country {
    pub const fn new(name: &'static str, english_name: &'static str, code: &'static str) -> Self {
        Country {
            name,
            english_name,
            code,
        }
    }
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct Region<'a> {
    #[schema(example = "Việt Nam")]
    pub language: &'static str,
    #[schema(example = "Vietnamese")]
    pub english_name: &'static str,
    #[schema(example = "vi")]
    pub language_code: &'static str,
    pub countries: &'a [&'a Country],
}

impl<'a> Region<'a> {
    pub const fn new(
        language: &'static str,
        english_name: &'static str,
        language_code: &'static str,
        countries: &'a [&'a Country],
    ) -> Self {
        Self {
            language,
            english_name,
            language_code,
            countries,
        }
    }
}

pub const SUPPORTED_LANGUAGES: &'static [&'static str] = &["en"];

pub fn validate_language_code<'a>(language_code: &'a str) -> Option<&'static str> {
    for c in SUPPORTED_LANGUAGES {
        if c.to_string() == language_code {
            return Some(c);
        }
    }

    None
}

pub fn validate_region_code<'a>(country_code: &'a str) -> Option<&'static Country> {
    for c in REGIONS {
        for country in c.countries {
            if country.code == country_code {
                return Some(country);
            }
        }
    }

    None
}

pub const REGIONS: &'static [Region] = &[
    // Region::new("Afrikaans", "Afrikaans", "af", ),
    // Region::new("shqip", "Albanian", "sq", "sq"),
    // Region::new("Հայ", "Armenian", "hy", "hy"),
    // Region::new("беларуская", "Belarusian", "be", "be"),
    // Region::new("বাংলা", "Bengali", "bn", "bn"),
    // Region::new("български", "Bulgarian", "bg", "bg"),
    // Region::new("Català", "Catalan", "ca", "ca"),
    // Region::new("普通话", "Chinese", "zh", "zh"),
    // Region::new("Hrvatski", "Croatian", "hr", "hr"),
    // Region::new("Čeština", "Czech", "cs", "cs"),
    // Region::new("Dansk", "Danish", "da", "da"),
    // Region::new("Nederlands", "Dutch", "nl", "nl"),
    Region::new(
        "English",
        "English",
        "en",
        &[
            &Country::new("Australia", "Australia", "au"),
            &Country::new("Belize", "Belize", "bz"),
            &Country::new("Canada", "Canada", "ca"),
            &Country::new("Caribbean", "Caribbean", "cb"),
            &Country::new("United Kingdom", "United Kingdom", "gb"),
            &Country::new("Ireland", "Ireland", "ie"),
            &Country::new("Jamaica", "Jamaica", "jm"),
            &Country::new("New Zealand", "New Zealand", "nz"),
            &Country::new("Philippines", "Philippines", "ph"),
            &Country::new("South Africa", "South Africa", "za"),
            &Country::new("United States", "United States", "us"),
        ],
    ),
    // Region::new("Eesti keel", "Estonian", "et", "et"),
    // Region::new("Filipino", "Filipino", "fil", "fil"),
    // Region::new("Suomi", "Finnish", "fi", "fi"),
    // Region::new("Français", "French", "fr", "fr"),
    // Region::new("Deutsch", "German", "de", "de"),
    // Region::new("Ελληνικά", "Greek", "el", "el"),
    // Region::new("ગુજરાતી", "Gujarati", "gu", "gu"),
    // Region::new("עברית", "Hebrew", "iw", "iw"),
    // Region::new("हिन्दी", "Hindi", "hi", "hi"),
    // Region::new("Magyar", "Hungarian", "hu", "hu"),
    // Region::new("Íslenska", "Icelandic", "is", "is"),
    // Region::new("Bahasa Indonesia", "Indonesian", "id", "id"),
    // Region::new("Italiano", "Italian", "it", "it"),
    Region::new(
        "日本語",
        "Japanese",
        "ja",
        &[&Country::new("日本", "Japan", "jp")],
    ),
    // Region::new("ಕನ್ನಡ", "Kannada", "kn", "kn"),
    // Region::new("ភាសាខ្មែរ", "Khmer", "km", "km"),
    // Region::new("한국어", "Korean", "ko", "ko"),
    // Region::new("ລາວ", "Lao", "lo", "la"),
    // Region::new("Latviešu", "Latvian", "lv", "lv"),
    // Region::new("Lietuvių", "Lithuanian", "lt", "lt"),
    // Region::new("Македонски", "Macedonian", "mk", "mk"),
    // Region::new("Bahasa Melayu", "Malay", "ms", "ms"),
    // Region::new("മലയാളം", "Malayalam", "ml", "ml"),
    // Region::new("मराठी", "Marathi", "mr", "mr"),
    // Region::new("नेपाली", "Nepali", "ne", "ne"),
    // Region::new("Norsk", "Norwegian", "no", "no"),
    // Region::new("فارسی", "Persian", "fa", "fa"),
    // Region::new("Polski", "Polish", "pl", "pl"),
    // Region::new("Português", "Portuguese", "pt", "pt"),
    // Region::new("ਪੰਜਾਬੀ", "Punjabi", "pa", "pa"),
    // Region::new("Română", "Romanian", "ro", "ro"),
    // Region::new("Русский", "Russian", "ru", "ru"),
    // Region::new("Српски", "Serbian", "sr", "rs"),
    // Region::new("Slovenčina", "Slovak", "sk", "sk"),
    // Region::new("Slovenščina", "Slovenian", "sl", "sl"),
    // Region::new("Español", "Spanish", "es", "es"),
    // Region::new("Svenska", "Swedish", "sv", "sv"),
    // Region::new("Tagalog", "Tagalog", "tl", "tl"),
    // Region::new("தமிழ்", "Tamil", "ta", "ta"),
    // Region::new("తెలుగు", "Telugu", "te", "te"),
    // Region::new("ไทย", "Thai", "th", "th"),
    // Region::new("Türkçe", "Turkish", "tr", "tr"),
    // Region::new("Українська", "Ukrainian", "uk", "ua"),
    Region::new(
        "Tiếng Việt",
        "Vietnamese",
        "vi",
        &[&Country::new("Việt Nam", "Vietnam", "vn")],
    ),
];
