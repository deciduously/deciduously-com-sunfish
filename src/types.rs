use serde_derive::Deserialize;

/// Link icon do display in header
#[derive(Default, Deserialize, Debug)]
pub struct Hyperlink {
    pub name: String,
    pub target: String,
}

/// Link icon do display in header
#[derive(Default, Deserialize, Debug)]
pub struct CvLink {
    pub link: Hyperlink,
    pub icon: String,
}

/// Schema.org locality
#[derive(Default, Deserialize, Debug)]
pub struct Locality {
    pub name: String,
    pub postal_code: String,
    pub state: AddressRegion,
}

/// Schema.org addressRegion
#[derive(Default, Deserialize, Debug)]
pub struct AddressRegion {
    pub abbreviation: String,
    pub full_name: String,
    pub country: String,
}

/// Schema.org Postal Address
#[derive(Default, Deserialize, Debug)]
pub struct Address {
    pub street: String,
    pub line2: Option<String>,
    pub locality: Locality,
}

/// CV/Resume
#[derive(Default, Deserialize, Debug)]
pub struct CV {
    pub address: Address,
    pub first_name: String,
    pub last_name: String,
    pub subtitle: String,
    pub email: String,
    pub links: Vec<CvLink>,
}
