use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub head: Head,
    pub body: Body,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Head {
    pub status: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub no_similar_models_text: String,
    pub available_stores_text: String,
    pub available_store_text: String,
    #[serde(rename = "PickupMessage")]
    pub pickup_message: PickupMessage,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PickupMessage {
    pub stores: Vec<Store>,
    pub overlay_initiated_from_warm_start: bool,
    pub view_more_hours_link_text: String,
    pub stores_count: String,
    pub little: bool,
    pub location: String,
    pub not_available_nearby: String,
    pub not_available_near_one_store: String,
    #[serde(rename = "warmDudeWithAPU")]
    pub warm_dude_with_apu: bool,
    pub view_more_hours_vo_text: String,
    pub availability: Availability,
    pub view_details_text: String,
    pub availability_stores: String,
    pub recommended_products: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Store {
    pub store_email: String,
    pub store_name: String,
    pub reservation_url: String,
    pub make_reservation_url: String,
    pub state: String,
    pub store_image_url: String,
    pub country: String,
    pub city: String,
    pub store_number: String,
    pub parts_availability: PartsAvailability,
    pub phone_number: String,
    pub pickup_type_availability_text: String,
    pub address: Address,
    pub hours_url: String,
    pub directions_url: String,
    pub store_hours: StoreHours,
    pub storelatitude: f64,
    pub storelongitude: f64,
    pub storedistance: f64,
    pub store_distance_with_unit: String,
    pub store_distance_vo_text: String,
    pub retail_store: RetailStore,
    pub storelistnumber: i64,
    pub store_list_number: i64,
    pub pickup_options_details: PickupOptionsDetails,
    pub rank: i64,
    pub special_hours: Option<SpecialHours>,
}

type PartsAvailability = HashMap<String, Value>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mlu93LlA {
    pub store_pick_eligible: bool,
    pub store_search_enabled: bool,
    pub store_selection_enabled: bool,
    pub store_pickup_quote: String,
    #[serde(rename = "storePickupQuote2_0")]
    pub store_pickup_quote2_0: String,
    pub pickup_search_quote: String,
    pub store_pickup_label: String,
    pub part_number: String,
    pub purchase_option: String,
    pub cto_options: String,
    pub store_pickup_link_text: String,
    pub store_pickup_product_title: String,
    pub pickup_display: String,
    pub pickup_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mll53LlA {
    pub store_pick_eligible: bool,
    pub store_search_enabled: bool,
    pub store_selection_enabled: bool,
    pub store_pickup_quote: String,
    #[serde(rename = "storePickupQuote2_0")]
    pub store_pickup_quote2_0: String,
    pub pickup_search_quote: String,
    pub store_pickup_label: String,
    pub part_number: String,
    pub purchase_option: String,
    pub cto_options: String,
    pub store_pickup_link_text: String,
    pub store_pickup_product_title: String,
    pub pickup_display: String,
    pub pickup_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub address: String,
    pub address3: Option<String>,
    pub address2: String,
    pub postal_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreHours {
    pub store_hours_text: String,
    pub bopis_pickup_days: String,
    pub bopis_pickup_hours: String,
    pub hours: Vec<Hour>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hour {
    pub store_timings: String,
    pub store_days: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetailStore {
    pub store_number: String,
    pub store_unique_id: String,
    pub name: String,
    pub store_type_key: String,
    pub store_sub_type_key: String,
    pub store_type: String,
    pub phone_number: String,
    pub email: String,
    pub carrier_code: Value,
    pub location_type: Value,
    pub latitude: f64,
    pub longitude: f64,
    pub address: Address2,
    pub url_key: Value,
    pub directions_url: String,
    pub store_image_url: String,
    pub make_reservation_url: Value,
    pub hours_and_info_url: String,
    pub store_hours: Vec<StoreHour>,
    pub store_holidays: Vec<StoreHoliday>,
    pub secure_store_image_url: String,
    pub distance: f64,
    pub distance_unit: String,
    pub distance_with_unit: String,
    pub timezone: String,
    pub store_is_active: bool,
    pub last_updated: f64,
    pub last_fetched: i64,
    pub date_stamp: String,
    pub distance_separator: String,
    pub next_available_date: Value,
    pub store_holiday_look_ahead_window: i64,
    pub drive_distance_with_unit: Value,
    pub drive_distance_in_meters: Value,
    pub dynamic_attributes: DynamicAttributes,
    pub store_pickup_method_by_type: StorePickupMethodByType,
    pub store_timings: Value,
    pub available_now: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address2 {
    pub city: String,
    pub company_name: String,
    pub country_code: String,
    pub county: Value,
    pub district: Value,
    pub geo_code: Value,
    pub label: Value,
    pub language_code: String,
    pub mail_stop: Value,
    pub postal_code: String,
    pub province: Value,
    pub state: String,
    pub street: String,
    pub street2: Option<String>,
    pub street3: Value,
    pub suburb: Value,
    #[serde(rename = "type")]
    pub type_field: String,
    pub addr_source_type: Value,
    pub outside_city_flag: Value,
    pub daytime_phone_area_code: Value,
    pub evening_phone_area_code: Value,
    pub daytime_phone: String,
    pub full_phone_number: Value,
    pub evening_phone: Value,
    pub email_address: Value,
    pub first_name: Value,
    pub last_name: Value,
    pub suffix: Value,
    pub last_name_phonetic: Value,
    pub first_name_phonetic: Value,
    pub title: Value,
    pub business_address: bool,
    pub uuid: String,
    pub mobile_phone: Value,
    pub mobile_phone_area_code: Value,
    pub city_state_zip: Value,
    pub daytime_phone_selected: bool,
    pub middle_name: Value,
    pub residence_status: Value,
    pub move_in_date: Value,
    pub subscriber_id: Value,
    pub location_type: Value,
    pub carrier_code: Value,
    pub metadata: Metadata,
    pub verification_state: String,
    pub expiration: Value,
    pub mark_for_deletion: bool,
    pub primary_address: bool,
    pub full_evening_phone: Value,
    pub full_daytime_phone: String,
    pub correction_result: Value,
    pub two_line_address: String,
    pub address_verified: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreHour {
    pub store_days: String,
    pub vo_store_days: String,
    pub store_timings: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreHoliday {
    pub date: String,
    pub description: String,
    pub hours: String,
    pub comments: String,
    pub closed: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicAttributes {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorePickupMethodByType {
    #[serde(rename = "INSTORE")]
    pub instore: Instore,
    #[serde(rename = "CURBSIDE")]
    pub curbside: Option<Curbside>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instore {
    pub services: Vec<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub type_coordinate: TypeCoordinate,
    pub type_direction: TypeDirection,
    pub type_meetup_location: TypeMeetupLocation,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeCoordinate {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeDirection {
    pub direction_by_locale: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeMeetupLocation {
    pub meeting_location_by_locale: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Curbside {
    pub services: Vec<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub type_coordinate: TypeCoordinate2,
    pub type_direction: TypeDirection2,
    pub type_meetup_location: TypeMeetupLocation2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeCoordinate2 {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeDirection2 {
    pub direction_by_locale: DirectionByLocale,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectionByLocale {
    #[serde(rename = "en_US")]
    pub en_us: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeMeetupLocation2 {
    pub meeting_location_by_locale: MeetingLocationByLocale,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeetingLocationByLocale {
    #[serde(rename = "en_US")]
    pub en_us: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PickupOptionsDetails {
    pub what_to_expect_at_pickup: String,
    pub compare_pickup_options_link: String,
    pub pickup_options: Vec<PickupOption>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PickupOption {
    pub pickup_option_title: String,
    pub pickup_option_description: String,
    pub index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecialHours {
    pub special_hours_text: String,
    pub bopis_pickup_days: String,
    pub bopis_pickup_hours: String,
    pub special_hours_data: Vec<SpecialHoursDaum>,
    pub view_all_special_hours: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecialHoursDaum {
    pub special_days: String,
    pub special_timings: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Availability {
    pub is_coming_soon: bool,
}
