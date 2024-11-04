use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Export {
    pub encrypted: bool,
    pub collections: Vec<Collection>,
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub id: String,
    pub organization_id: Option<String>,
    pub name: String,
    pub external_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordHistory {
    pub last_used_date: String,
    pub password: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomField {
    pub name: String,
    pub value: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub linked_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub password_history: Option<Vec<PasswordHistory>>,
    pub revision_date: String,
    pub creation_date: String,
    pub deleted_date: Option<String>,
    pub id: String,
    pub organization_id: Option<String>,
    pub folder_id: Option<String>,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub reprompt: i64,
    pub name: String,
    pub notes: String,
    pub favorite: bool,
    pub secure_note: Option<SecureNote>,
    pub collection_ids: Vec<String>,
    pub identity: Option<Identity>,
    pub card: Option<Card>,
    pub login: Option<Login>,
    #[serde(default)]
    pub fields: Vec<CustomField>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureNote {
    #[serde(rename = "type")]
    pub type_field: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub address3: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub company: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub ssn: Option<String>,
    pub username: Option<String>,
    pub passport_number: Option<String>,
    pub license_number: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub cardholder_name: Option<String>,
    pub brand: Option<String>,
    pub number: Option<String>,
    pub exp_month: Option<String>,
    pub exp_year: Option<String>,
    pub code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Login {
    #[serde(default)]
    pub uris: Vec<Uri>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub totp: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uri {
    #[serde(rename = "match")]
    pub match_field: Option<String>,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,
    pub value: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub linked_id: Option<String>,
}
