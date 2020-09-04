

use std::collections::BTreeMap;
use crate::ametheed::assets::SerializableFormat;

/// A trait for all asset types that have their format types.
/// Use this as a bound for asset data types when used inside boxed format types intended for deserialization.
/// registered with `register_format_type` macro.
///
///  This trait should never be implemented manually. Use the `register_format_type` macro to register it correctly.
/// ```ignore
/// // this must be used exactly once per data type
/// amethyst_assets::register_format_type!(AudioData);
///
/// // this must be used for every Format type impl that can be deserialized dynamically
/// amethyst_assets::register_format!("WAV", AudioData as WavFormat);
/// impl Format<AudioData> for WavFormat {
///     fn name(&self) -> &'static str {
///         "WAV"
///     }

///     fn import_simple(&self, bytes: Vec<u8>) -> Result<AudioData, Error> {
///         Ok(AudioData(bytes))
///     }
/// }

/// impl SerializableFormat<AudioData> for WavFormat {}
/// ```
pub trait FormatRegisteredData: 'static {
    // Used by deserialization. This is a private API.
    #[doc(hidden)]
    type Registration;
    #[doc(hidden)]
    fn get_registration(
        name: &'static str,
        deserializer: DeserializeFn<dyn SerializableFormat<Self>>,
    ) -> Self::Registration;
    #[doc(hidden)]
    fn registry() -> &'static Registry<dyn SerializableFormat<Self>>;
}

// Not public API. Used by macros.
#[doc(hidden)]
pub type DeserializeFn<T> =
    fn(&mut dyn erased_serde::Deserializer<'_>) -> erased_serde::Result<Box<T>>;

// Not public API. Used by macros.
#[doc(hidden)]
pub struct Registry<T: ?Sized> {
    pub map: BTreeMap<&'static str, Option<DeserializeFn<T>>>,
    pub names: Vec<&'static str>,
}
