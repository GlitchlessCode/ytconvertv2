use nanoserde::DeJson;

use crate::data::license::License;

build_const::build_const!("licenses");

pub fn parse_licenses() -> Option<Vec<License>> {
    Vec::<License>::deserialize_json(LICENSES_STR).ok()
}
