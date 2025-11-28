use std::collections::HashMap;
use std::num::NonZeroU32;
use std::str::FromStr;

/// A structured, strongly-typed postal address with all possible components
/// that libpostal can extract.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Address {
    /// House number (e.g., "781")
    pub house_number: Option<String>,

    /// Road/street name (e.g., "Franklin Ave")
    pub road: Option<String>,

    /// Unit/apartment number (e.g., "Apt 3B")
    pub unit: Option<String>,

    /// Building or complex name
    pub house: Option<String>,

    /// Floor/level
    pub level: Option<String>,

    /// Staircase identifier
    pub staircase: Option<String>,

    /// Entrance identifier
    pub entrance: Option<String>,

    /// P.O. Box number
    pub po_box: Option<String>,

    /// Postal/ZIP code
    pub postcode: Option<Postcode>,

    /// Suburb or neighborhood (e.g., "Crown Heights")
    pub suburb: Option<String>,

    /// City/town name (e.g., "Brooklyn", "NYC")
    pub city: Option<String>,

    /// City district
    pub city_district: Option<String>,

    /// Island name
    pub island: Option<String>,

    /// State/province/region (e.g., "NY")
    pub state: Option<State>,

    /// State district
    pub state_district: Option<String>,

    /// Country name (e.g., "USA")
    pub country: Option<Country>,

    /// Country region
    pub country_region: Option<String>,

    /// World region (e.g., "North America")
    pub world_region: Option<String>,

    /// Neighborhood name
    pub neighbourhood: Option<String>,

    /// Any additional categories or classifications
    pub category: Option<String>,

    /// Near location reference (e.g., "near Central Park")
    pub near: Option<String>,
}

/// Postal/ZIP code with validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Postcode(String);

impl Postcode {
    /// Create a new postcode. Returns None if empty.
    pub fn new(code: impl Into<String>) -> Option<Self> {
        let code = code.into();
        if code.trim().is_empty() {
            None
        } else {
            Some(Postcode(code))
        }
    }

    /// Get the postcode as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Postcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// State/province representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    /// US state code (e.g., "NY", "CA")
    UsStateCode(UsStateCode),
    /// Canadian province code (e.g., "ON", "BC")
    CanadianProvince(String),
    /// Other state/province/region name
    Other(String),
}

impl State {
    pub fn as_str(&self) -> &str {
        match self {
            State::UsStateCode(code) => code.as_str(),
            State::CanadianProvince(s) | State::Other(s) => s.as_str(),
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// US state codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsStateCode {
    AL,
    AK,
    AZ,
    AR,
    CA,
    CO,
    CT,
    DE,
    FL,
    GA,
    HI,
    ID,
    IL,
    IN,
    IA,
    KS,
    KY,
    LA,
    ME,
    MD,
    MA,
    MI,
    MN,
    MS,
    MO,
    MT,
    NE,
    NV,
    NH,
    NJ,
    NM,
    NY,
    NC,
    ND,
    OH,
    OK,
    OR,
    PA,
    RI,
    SC,
    SD,
    TN,
    TX,
    UT,
    VT,
    VA,
    WA,
    WV,
    WI,
    WY,
    DC, // Washington D.C.
}

impl UsStateCode {
    pub fn as_str(&self) -> &str {
        match self {
            UsStateCode::AL => "AL",
            UsStateCode::AK => "AK",
            UsStateCode::AZ => "AZ",
            UsStateCode::AR => "AR",
            UsStateCode::CA => "CA",
            UsStateCode::CO => "CO",
            UsStateCode::CT => "CT",
            UsStateCode::DE => "DE",
            UsStateCode::FL => "FL",
            UsStateCode::GA => "GA",
            UsStateCode::HI => "HI",
            UsStateCode::ID => "ID",
            UsStateCode::IL => "IL",
            UsStateCode::IN => "IN",
            UsStateCode::IA => "IA",
            UsStateCode::KS => "KS",
            UsStateCode::KY => "KY",
            UsStateCode::LA => "LA",
            UsStateCode::ME => "ME",
            UsStateCode::MD => "MD",
            UsStateCode::MA => "MA",
            UsStateCode::MI => "MI",
            UsStateCode::MN => "MN",
            UsStateCode::MS => "MS",
            UsStateCode::MO => "MO",
            UsStateCode::MT => "MT",
            UsStateCode::NE => "NE",
            UsStateCode::NV => "NV",
            UsStateCode::NH => "NH",
            UsStateCode::NJ => "NJ",
            UsStateCode::NM => "NM",
            UsStateCode::NY => "NY",
            UsStateCode::NC => "NC",
            UsStateCode::ND => "ND",
            UsStateCode::OH => "OH",
            UsStateCode::OK => "OK",
            UsStateCode::OR => "OR",
            UsStateCode::PA => "PA",
            UsStateCode::RI => "RI",
            UsStateCode::SC => "SC",
            UsStateCode::SD => "SD",
            UsStateCode::TN => "TN",
            UsStateCode::TX => "TX",
            UsStateCode::UT => "UT",
            UsStateCode::VT => "VT",
            UsStateCode::VA => "VA",
            UsStateCode::WA => "WA",
            UsStateCode::WV => "WV",
            UsStateCode::WI => "WI",
            UsStateCode::WY => "WY",
            UsStateCode::DC => "DC",
        }
    }
}

impl FromStr for UsStateCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "AL" => Ok(UsStateCode::AL),
            "AK" => Ok(UsStateCode::AK),
            "AZ" => Ok(UsStateCode::AZ),
            "AR" => Ok(UsStateCode::AR),
            "CA" => Ok(UsStateCode::CA),
            "CO" => Ok(UsStateCode::CO),
            "CT" => Ok(UsStateCode::CT),
            "DE" => Ok(UsStateCode::DE),
            "FL" => Ok(UsStateCode::FL),
            "GA" => Ok(UsStateCode::GA),
            "HI" => Ok(UsStateCode::HI),
            "ID" => Ok(UsStateCode::ID),
            "IL" => Ok(UsStateCode::IL),
            "IN" => Ok(UsStateCode::IN),
            "IA" => Ok(UsStateCode::IA),
            "KS" => Ok(UsStateCode::KS),
            "KY" => Ok(UsStateCode::KY),
            "LA" => Ok(UsStateCode::LA),
            "ME" => Ok(UsStateCode::ME),
            "MD" => Ok(UsStateCode::MD),
            "MA" => Ok(UsStateCode::MA),
            "MI" => Ok(UsStateCode::MI),
            "MN" => Ok(UsStateCode::MN),
            "MS" => Ok(UsStateCode::MS),
            "MO" => Ok(UsStateCode::MO),
            "MT" => Ok(UsStateCode::MT),
            "NE" => Ok(UsStateCode::NE),
            "NV" => Ok(UsStateCode::NV),
            "NH" => Ok(UsStateCode::NH),
            "NJ" => Ok(UsStateCode::NJ),
            "NM" => Ok(UsStateCode::NM),
            "NY" => Ok(UsStateCode::NY),
            "NC" => Ok(UsStateCode::NC),
            "ND" => Ok(UsStateCode::ND),
            "OH" => Ok(UsStateCode::OH),
            "OK" => Ok(UsStateCode::OK),
            "OR" => Ok(UsStateCode::OR),
            "PA" => Ok(UsStateCode::PA),
            "RI" => Ok(UsStateCode::RI),
            "SC" => Ok(UsStateCode::SC),
            "SD" => Ok(UsStateCode::SD),
            "TN" => Ok(UsStateCode::TN),
            "TX" => Ok(UsStateCode::TX),
            "UT" => Ok(UsStateCode::UT),
            "VT" => Ok(UsStateCode::VT),
            "VA" => Ok(UsStateCode::VA),
            "WA" => Ok(UsStateCode::WA),
            "WV" => Ok(UsStateCode::WV),
            "WI" => Ok(UsStateCode::WI),
            "WY" => Ok(UsStateCode::WY),
            "DC" => Ok(UsStateCode::DC),
            _ => Err(()),
        }
    }
}

/// Country representation with ISO codes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Country {
    /// ISO 3166-1 alpha-2 code (e.g., "US", "CA", "GB")
    Iso2(String),
    /// ISO 3166-1 alpha-3 code (e.g., "USA", "CAN", "GBR")
    Iso3(String),
    /// Full country name
    Name(String),
}

impl Country {
    pub fn as_str(&self) -> &str {
        match self {
            Country::Iso2(s) | Country::Iso3(s) | Country::Name(s) => s.as_str(),
        }
    }

    /// Try to determine country type from string length and content
    pub fn from_string(s: String) -> Self {
        match s.len() {
            2 => Country::Iso2(s.to_uppercase()),
            3 => {
                // Could be ISO3 or abbreviation like "USA"
                if s.chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
                {
                    Country::Iso3(s.to_uppercase())
                } else {
                    Country::Name(s)
                }
            }
            _ => Country::Name(s),
        }
    }
}

impl std::fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Address {
    /// Convert from the libpostal HashMap format to a structured Address
    pub fn from_parsed(parsed: HashMap<String, String>) -> Self {
        let mut addr = Address::default();

        for (key, value) in parsed {
            match key.as_str() {
                "house_number" => addr.house_number = Some(value),
                "road" => addr.road = Some(value),
                "unit" => addr.unit = Some(value),
                "house" => addr.house = Some(value),
                "level" => addr.level = Some(value),
                "staircase" => addr.staircase = Some(value),
                "entrance" => addr.entrance = Some(value),
                "po_box" => addr.po_box = Some(value),
                "postcode" => addr.postcode = Postcode::new(value),
                "suburb" => addr.suburb = Some(value),
                "city" => addr.city = Some(value),
                "city_district" => addr.city_district = Some(value),
                "island" => addr.island = Some(value),
                "state" => {
                    addr.state =
                        Some(if let Ok(us_state) = UsStateCode::from_str(&value) {
                            State::UsStateCode(us_state)
                        } else if value.len() == 2
                            && value.chars().all(|c| c.is_ascii_alphabetic())
                        {
                            State::CanadianProvince(value.to_uppercase())
                        } else {
                            State::Other(value)
                        })
                }
                "state_district" => addr.state_district = Some(value),
                "country" => addr.country = Some(Country::from_string(value)),
                "country_region" => addr.country_region = Some(value),
                "world_region" => addr.world_region = Some(value),
                "neighbourhood" => addr.neighbourhood = Some(value),
                "category" => addr.category = Some(value),
                "near" => addr.near = Some(value),
                _ => {}
            }
        }

        addr
    }

    /// Get a single-line representation of the address
    pub fn to_single_line(&self) -> String {
        let mut parts = Vec::new();

        if let Some(ref num) = self.house_number {
            parts.push(num.clone());
        }
        if let Some(ref road) = self.road {
            parts.push(road.clone());
        }
        if let Some(ref unit) = self.unit {
            parts.push(format!("#{}", unit));
        }
        if let Some(ref city) = self.city {
            parts.push(city.clone());
        }
        if let Some(ref state) = self.state {
            parts.push(state.to_string());
        }
        if let Some(ref postcode) = self.postcode {
            parts.push(postcode.to_string());
        }
        if let Some(ref country) = self.country {
            parts.push(country.to_string());
        }

        parts.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_from_parsed() {
        let mut map = HashMap::new();
        map.insert("house_number".to_string(), "781".to_string());
        map.insert("road".to_string(), "Franklin Ave".to_string());
        map.insert("city".to_string(), "Brooklyn".to_string());
        map.insert("state".to_string(), "NY".to_string());
        map.insert("postcode".to_string(), "11216".to_string());
        map.insert("country".to_string(), "USA".to_string());

        let addr = Address::from_parsed(map);

        assert_eq!(addr.house_number, Some("781".to_string()));
        assert_eq!(addr.road, Some("Franklin Ave".to_string()));
        assert_eq!(addr.city, Some("Brooklyn".to_string()));
        assert!(matches!(
            addr.state,
            Some(State::UsStateCode(UsStateCode::NY))
        ));
        assert_eq!(addr.postcode.as_ref().map(|p| p.as_str()), Some("11216"));
        assert!(matches!(addr.country, Some(Country::Iso3(_))));
    }

    #[test]
    fn test_us_state_code_parsing() {
        assert_eq!(UsStateCode::from_str("ny"), Ok(UsStateCode::NY));
        assert_eq!(UsStateCode::from_str("NY"), Ok(UsStateCode::NY));
        assert_eq!(UsStateCode::from_str("ca"), Ok(UsStateCode::CA));
        assert!(UsStateCode::from_str("XX").is_err());
    }

    #[test]
    fn test_single_line_formatting() {
        let addr = Address {
            house_number: Some("123".to_string()),
            road: Some("Main St".to_string()),
            city: Some("Springfield".to_string()),
            state: Some(State::UsStateCode(UsStateCode::IL)),
            postcode: Postcode::new("62701"),
            ..Default::default()
        };

        let line = addr.to_single_line();
        assert!(line.contains("123"));
        assert!(line.contains("Main St"));
        assert!(line.contains("Springfield"));
        assert!(line.contains("IL"));
        assert!(line.contains("62701"));
    }
}
