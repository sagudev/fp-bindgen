use crate::types::*;

#[fp_bindgen_support::fp_import_signature]
pub fn import_fp_adjacently_tagged(arg: FpAdjacentlyTagged) -> FpAdjacentlyTagged;

#[fp_bindgen_support::fp_import_signature]
pub fn import_fp_enum(arg: FpVariantRenaming) -> FpVariantRenaming;

#[fp_bindgen_support::fp_import_signature]
pub fn import_fp_flatten(arg: FpFlatten) -> FpFlatten;

#[fp_bindgen_support::fp_import_signature]
pub fn import_fp_internally_tagged(arg: FpInternallyTagged) -> FpInternallyTagged;

#[fp_bindgen_support::fp_import_signature]
pub fn import_fp_struct(arg: FpPropertyRenaming) -> FpPropertyRenaming;

#[fp_bindgen_support::fp_import_signature]
pub fn import_fp_untagged(arg: FpUntagged) -> FpUntagged;

#[fp_bindgen_support::fp_import_signature]
pub fn import_generics(arg: StructWithGenerics<u64>) -> StructWithGenerics<u64>;

#[fp_bindgen_support::fp_import_signature]
pub fn import_multiple_primitives(arg1: i8, arg2: String) -> i64;

#[fp_bindgen_support::fp_import_signature]
pub fn import_primitive_bool(arg: bool) -> bool;

#[fp_bindgen_support::fp_import_signature]
pub fn import_primitive_f32(arg: f32) -> f32;

#[fp_bindgen_support::fp_import_signature]
pub fn import_primitive_f64(arg: f64) -> f64;

#[fp_bindgen_support::fp_import_signature]
pub fn import_primitive_i16(arg: i16) -> i16;

#[fp_bindgen_support::fp_import_signature]
pub fn import_primitive_i32(arg: i32) -> i32;

#[fp_bindgen_support::fp_import_signature]
pub fn import_primitive_i64(arg: i64) -> i64;

#[fp_bindgen_support::fp_import_signature]
pub fn import_primitive_i8(arg: i8) -> i8;

#[fp_bindgen_support::fp_import_signature]
pub fn import_primitive_u16(arg: u16) -> u16;

#[fp_bindgen_support::fp_import_signature]
pub fn import_primitive_u32(arg: u32) -> u32;

#[fp_bindgen_support::fp_import_signature]
pub fn import_primitive_u64(arg: u64) -> u64;

#[fp_bindgen_support::fp_import_signature]
pub fn import_primitive_u8(arg: u8) -> u8;

#[fp_bindgen_support::fp_import_signature]
pub fn import_serde_adjacently_tagged(arg: SerdeAdjacentlyTagged) -> SerdeAdjacentlyTagged;

#[fp_bindgen_support::fp_import_signature]
pub fn import_serde_enum(arg: SerdeVariantRenaming) -> SerdeVariantRenaming;

#[fp_bindgen_support::fp_import_signature]
pub fn import_serde_flatten(arg: SerdeFlatten) -> SerdeFlatten;

#[fp_bindgen_support::fp_import_signature]
pub fn import_serde_internally_tagged(arg: SerdeInternallyTagged) -> SerdeInternallyTagged;

#[fp_bindgen_support::fp_import_signature]
pub fn import_serde_struct(arg: SerdePropertyRenaming) -> SerdePropertyRenaming;

#[fp_bindgen_support::fp_import_signature]
pub fn import_serde_untagged(arg: SerdeUntagged) -> SerdeUntagged;

#[fp_bindgen_support::fp_import_signature]
pub fn import_string(arg: String) -> String;

#[fp_bindgen_support::fp_import_signature]
pub fn import_timestamp(arg: time::OffsetDateTime) -> time::OffsetDateTime;

#[fp_bindgen_support::fp_import_signature]
pub fn import_void_function();

/// Logs a message to the (development) console.
#[fp_bindgen_support::fp_import_signature]
pub fn log(message: String);

/// Example how a runtime could expose a `Fetch`-like function to plugins.
///
/// See `types/http.rs` for more info.
#[fp_bindgen_support::fp_import_signature]
pub async fn make_http_request(request: Request) -> HttpResult;
