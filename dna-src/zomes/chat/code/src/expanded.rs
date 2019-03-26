#![feature(prelude_import)]
#![no_std]
#![feature(try_from)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;
use hdk::error::ZomeApiResult;
use hdk::holochain_core_types::{
    cas::content::Address, error::HolochainError, hash::HashString, json::JsonString,
};
mod anchor {
    use hdk::entry_definition::ValidatingEntryType;
    use hdk::holochain_core_types::{
        cas::content::Address, dna::entry_types::Sharing, json::RawString,
    };
    pub fn anchor_definition() -> ValidatingEntryType {
        {
            let mut entry_type = hdk::holochain_core_types::dna::entry_types::EntryTypeDef::new();
            entry_type.description = String::from("");
            entry_type.sharing = Sharing::Public;
            match { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: To , other_entry_type : String :: from ( "%agent_id" ) , tag : String :: from ( "member_tag" ) , package_creator , validator , } } . link_type { $crate :: LinkDirection :: To => { entry_type . links_to . push ( $crate :: holochain_core_types :: dna :: entry_types :: LinksTo { target_type : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: To , other_entry_type : String :: from ( "%agent_id" ) , tag : String :: from ( "member_tag" ) , package_creator , validator , } } . other_entry_type , tag : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: To , other_entry_type : String :: from ( "%agent_id" ) , tag : String :: from ( "member_tag" ) , package_creator , validator , } } . tag , } ) ; } $crate :: LinkDirection :: From => { entry_type . linked_from . push ( $crate :: holochain_core_types :: dna :: entry_types :: LinkedFrom { base_type : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: To , other_entry_type : String :: from ( "%agent_id" ) , tag : String :: from ( "member_tag" ) , package_creator , validator , } } . other_entry_type , tag : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: To , other_entry_type : String :: from ( "%agent_id" ) , tag : String :: from ( "member_tag" ) , package_creator , validator , } } . tag , } ) ; } }
            match { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: To , other_entry_type : String :: from ( "public_stream" ) , tag : String :: from ( "public_stream" ) , package_creator , validator , } } . link_type { $crate :: LinkDirection :: To => { entry_type . links_to . push ( $crate :: holochain_core_types :: dna :: entry_types :: LinksTo { target_type : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: To , other_entry_type : String :: from ( "public_stream" ) , tag : String :: from ( "public_stream" ) , package_creator , validator , } } . other_entry_type , tag : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: To , other_entry_type : String :: from ( "public_stream" ) , tag : String :: from ( "public_stream" ) , package_creator , validator , } } . tag , } ) ; } $crate :: LinkDirection :: From => { entry_type . linked_from . push ( $crate :: holochain_core_types :: dna :: entry_types :: LinkedFrom { base_type : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: To , other_entry_type : String :: from ( "public_stream" ) , tag : String :: from ( "public_stream" ) , package_creator , validator , } } . other_entry_type , tag : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: To , other_entry_type : String :: from ( "public_stream" ) , tag : String :: from ( "public_stream" ) , package_creator , validator , } } . tag , } ) ; } }
            let package_creator = Box::new(|| hdk::ValidationPackageDefinition::Entry);
            let validator = Box :: new ( | entry : hdk :: holochain_core_types :: entry :: Entry , validation_data : hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _ctx = validation_data ; match entry { hdk :: holochain_core_types :: entry :: Entry :: App ( _ , app_entry_value ) => { let entry : RawString = :: std :: convert :: TryInto :: try_into ( app_entry_value ) ? ; let _name = entry ; { Ok ( ( ) ) } } _ => { Err ( String :: from ( "Schema validation failed" ) ) ? } } } ) ;
            hdk::entry_definition::ValidatingEntryType {
                name: hdk::holochain_core_types::entry::entry_type::EntryType::App(
                    hdk::holochain_core_types::entry::entry_type::AppEntryType::from(
                        "anchor".to_string(),
                    ),
                ),
                entry_type_definition: entry_type,
                package_creator,
                validator,
                links: <[_]>::into_vec(box [
                    {
                        let package_creator = Box::new(|| hdk::ValidationPackageDefinition::Entry);
                        let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ;
                        ::hdk::entry_definition::ValidatingLinkDefinition {
                            link_type: $crate::LinkDirection::To,
                            other_entry_type: String::from("%agent_id"),
                            tag: String::from("member_tag"),
                            package_creator,
                            validator,
                        }
                    },
                    {
                        let package_creator = Box::new(|| hdk::ValidationPackageDefinition::Entry);
                        let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ;
                        ::hdk::entry_definition::ValidatingLinkDefinition {
                            link_type: $crate::LinkDirection::To,
                            other_entry_type: String::from("public_stream"),
                            tag: String::from("public_stream"),
                            package_creator,
                            validator,
                        }
                    },
                ]),
            }
        }
    }
}
mod message {
    use hdk::holochain_core_types::dna::entry_types::Sharing;
    use hdk::holochain_core_types::error::HolochainError;
    use hdk::holochain_core_types::json::JsonString;
    use hdk::{self, entry_definition::ValidatingEntryType};
    pub struct Message {
        pub timestamp: u32,
        pub author: String,
        pub message_type: String,
        pub payload: String,
        pub meta: String,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_Message: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Message {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Message",
                    false as usize + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "timestamp",
                    &self.timestamp,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "author",
                    &self.author,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "message_type",
                    &self.message_type,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "payload",
                    &self.payload,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "meta",
                    &self.meta,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_Message: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Message {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 5",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "timestamp" => _serde::export::Ok(__Field::__field0),
                            "author" => _serde::export::Ok(__Field::__field1),
                            "message_type" => _serde::export::Ok(__Field::__field2),
                            "payload" => _serde::export::Ok(__Field::__field3),
                            "meta" => _serde::export::Ok(__Field::__field4),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"timestamp" => _serde::export::Ok(__Field::__field0),
                            b"author" => _serde::export::Ok(__Field::__field1),
                            b"message_type" => _serde::export::Ok(__Field::__field2),
                            b"payload" => _serde::export::Ok(__Field::__field3),
                            b"meta" => _serde::export::Ok(__Field::__field4),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<Message>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Message;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct Message")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Message with 5 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Message with 5 elements",
                                    ));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Message with 5 elements",
                                    ));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Message with 5 elements",
                                    ));
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct Message with 5 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(Message {
                            timestamp: __field0,
                            author: __field1,
                            message_type: __field2,
                            payload: __field3,
                            meta: __field4,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<u32> = _serde::export::None;
                        let mut __field1: _serde::export::Option<String> = _serde::export::None;
                        let mut __field2: _serde::export::Option<String> = _serde::export::None;
                        let mut __field3: _serde::export::Option<String> = _serde::export::None;
                        let mut __field4: _serde::export::Option<String> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "timestamp",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "author",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "message_type",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "payload",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "meta",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("timestamp") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("author") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("message_type") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("payload") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::export::Some(__field4) => __field4,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("meta") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(Message {
                            timestamp: __field0,
                            author: __field1,
                            message_type: __field2,
                            payload: __field3,
                            meta: __field4,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["timestamp", "author", "message_type", "payload", "meta"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Message",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<Message>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for Message {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                Message {
                    timestamp: ref __self_0_0,
                    author: ref __self_0_1,
                    message_type: ref __self_0_2,
                    payload: ref __self_0_3,
                    meta: ref __self_0_4,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Message");
                    let _ = debug_trait_builder.field("timestamp", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("author", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("message_type", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("payload", &&(*__self_0_3));
                    let _ = debug_trait_builder.field("meta", &&(*__self_0_4));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::clone::Clone for Message {
        #[inline]
        fn clone(&self) -> Message {
            match *self {
                Message {
                    timestamp: ref __self_0_0,
                    author: ref __self_0_1,
                    message_type: ref __self_0_2,
                    payload: ref __self_0_3,
                    meta: ref __self_0_4,
                } => Message {
                    timestamp: $crate::clone::Clone::clone(&(*__self_0_0)),
                    author: $crate::clone::Clone::clone(&(*__self_0_1)),
                    message_type: $crate::clone::Clone::clone(&(*__self_0_2)),
                    payload: $crate::clone::Clone::clone(&(*__self_0_3)),
                    meta: $crate::clone::Clone::clone(&(*__self_0_4)),
                },
            }
        }
    }
    impl<'a> From<&'a Message> for JsonString {
        fn from(v: &Message) -> JsonString {
            match ::serde_json::to_string(v) {
                Ok(s) => Ok(JsonString::from(s)),
                Err(e) => {
                    {
                        $crate::io::_eprint($crate::fmt::Arguments::new_v1(
                            &["Error serializing to JSON: ", "\n"],
                            &match (&e,) {
                                (arg0,) => [$crate::fmt::ArgumentV1::new(
                                    arg0,
                                    $crate::fmt::Debug::fmt,
                                )],
                            },
                        ));
                    };
                    Err(HolochainError::SerializationError(e.to_string()))
                }
            }
            .expect(&$crate::fmt::format($crate::fmt::Arguments::new_v1(
                &["could not Jsonify ", ": "],
                &match (&"Message", &v) {
                    (arg0, arg1) => [
                        $crate::fmt::ArgumentV1::new(arg0, $crate::fmt::Display::fmt),
                        $crate::fmt::ArgumentV1::new(arg1, $crate::fmt::Debug::fmt),
                    ],
                },
            )))
        }
    }
    impl From<Message> for JsonString {
        fn from(v: Message) -> JsonString {
            JsonString::from(&v)
        }
    }
    impl<'a> ::std::convert::TryFrom<&'a JsonString> for Message {
        type Error = HolochainError;
        fn try_from(json_string: &JsonString) -> Result<Self, Self::Error> {
            match ::serde_json::from_str(&String::from(json_string)) {
                Ok(d) => Ok(d),
                Err(e) => Err(HolochainError::SerializationError(e.to_string())),
            }
        }
    }
    impl ::std::convert::TryFrom<JsonString> for Message {
        type Error = HolochainError;
        fn try_from(json_string: JsonString) -> Result<Self, Self::Error> {
            Message::try_from(&json_string)
        }
    }
    impl Message {
        pub fn from_spec(spec: &MessageSpec, author: &String) -> Message {
            return Message {
                message_type: spec.message_type.clone(),
                payload: spec.payload.clone(),
                meta: spec.meta.clone(),
                author: author.to_owned(),
                timestamp: spec.timestamp.clone(),
            };
        }
    }
    pub struct MessageSpec {
        pub message_type: String,
        pub timestamp: u32,
        pub payload: String,
        pub meta: String,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_MessageSpec: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for MessageSpec {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "MessageSpec",
                    false as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "message_type",
                    &self.message_type,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "timestamp",
                    &self.timestamp,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "payload",
                    &self.payload,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "meta",
                    &self.meta,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_MessageSpec: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for MessageSpec {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 4",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "message_type" => _serde::export::Ok(__Field::__field0),
                            "timestamp" => _serde::export::Ok(__Field::__field1),
                            "payload" => _serde::export::Ok(__Field::__field2),
                            "meta" => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"message_type" => _serde::export::Ok(__Field::__field0),
                            b"timestamp" => _serde::export::Ok(__Field::__field1),
                            b"payload" => _serde::export::Ok(__Field::__field2),
                            b"meta" => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<MessageSpec>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MessageSpec;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct MessageSpec")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct MessageSpec with 4 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct MessageSpec with 4 elements",
                                    ));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct MessageSpec with 4 elements",
                                    ));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct MessageSpec with 4 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(MessageSpec {
                            message_type: __field0,
                            timestamp: __field1,
                            payload: __field2,
                            meta: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<String> = _serde::export::None;
                        let mut __field1: _serde::export::Option<u32> = _serde::export::None;
                        let mut __field2: _serde::export::Option<String> = _serde::export::None;
                        let mut __field3: _serde::export::Option<String> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "message_type",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "timestamp",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "payload",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "meta",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("message_type") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("timestamp") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("payload") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("meta") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(MessageSpec {
                            message_type: __field0,
                            timestamp: __field1,
                            payload: __field2,
                            meta: __field3,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["message_type", "timestamp", "payload", "meta"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "MessageSpec",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<MessageSpec>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for MessageSpec {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                MessageSpec {
                    message_type: ref __self_0_0,
                    timestamp: ref __self_0_1,
                    payload: ref __self_0_2,
                    meta: ref __self_0_3,
                } => {
                    let mut debug_trait_builder = f.debug_struct("MessageSpec");
                    let _ = debug_trait_builder.field("message_type", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("timestamp", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("payload", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("meta", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::clone::Clone for MessageSpec {
        #[inline]
        fn clone(&self) -> MessageSpec {
            match *self {
                MessageSpec {
                    message_type: ref __self_0_0,
                    timestamp: ref __self_0_1,
                    payload: ref __self_0_2,
                    meta: ref __self_0_3,
                } => MessageSpec {
                    message_type: $crate::clone::Clone::clone(&(*__self_0_0)),
                    timestamp: $crate::clone::Clone::clone(&(*__self_0_1)),
                    payload: $crate::clone::Clone::clone(&(*__self_0_2)),
                    meta: $crate::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    impl<'a> From<&'a MessageSpec> for JsonString {
        fn from(v: &MessageSpec) -> JsonString {
            match ::serde_json::to_string(v) {
                Ok(s) => Ok(JsonString::from(s)),
                Err(e) => {
                    {
                        $crate::io::_eprint($crate::fmt::Arguments::new_v1(
                            &["Error serializing to JSON: ", "\n"],
                            &match (&e,) {
                                (arg0,) => [$crate::fmt::ArgumentV1::new(
                                    arg0,
                                    $crate::fmt::Debug::fmt,
                                )],
                            },
                        ));
                    };
                    Err(HolochainError::SerializationError(e.to_string()))
                }
            }
            .expect(&$crate::fmt::format($crate::fmt::Arguments::new_v1(
                &["could not Jsonify ", ": "],
                &match (&"MessageSpec", &v) {
                    (arg0, arg1) => [
                        $crate::fmt::ArgumentV1::new(arg0, $crate::fmt::Display::fmt),
                        $crate::fmt::ArgumentV1::new(arg1, $crate::fmt::Debug::fmt),
                    ],
                },
            )))
        }
    }
    impl From<MessageSpec> for JsonString {
        fn from(v: MessageSpec) -> JsonString {
            JsonString::from(&v)
        }
    }
    impl<'a> ::std::convert::TryFrom<&'a JsonString> for MessageSpec {
        type Error = HolochainError;
        fn try_from(json_string: &JsonString) -> Result<Self, Self::Error> {
            match ::serde_json::from_str(&String::from(json_string)) {
                Ok(d) => Ok(d),
                Err(e) => Err(HolochainError::SerializationError(e.to_string())),
            }
        }
    }
    impl ::std::convert::TryFrom<JsonString> for MessageSpec {
        type Error = HolochainError;
        fn try_from(json_string: JsonString) -> Result<Self, Self::Error> {
            MessageSpec::try_from(&json_string)
        }
    }
    pub fn message_definition() -> ValidatingEntryType {
        {
            let mut entry_type = hdk::holochain_core_types::dna::entry_types::EntryTypeDef::new();
            entry_type.description = String::from("A generic message entry");
            entry_type.sharing = Sharing::Public;
            let package_creator = Box::new(|| hdk::ValidationPackageDefinition::Entry);
            let validator = Box :: new ( | entry : hdk :: holochain_core_types :: entry :: Entry , validation_data : hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _ctx = validation_data ; match entry { hdk :: holochain_core_types :: entry :: Entry :: App ( _ , app_entry_value ) => { let entry : Message = :: std :: convert :: TryInto :: try_into ( app_entry_value ) ? ; let _message = entry ; { Ok ( ( ) ) } } _ => { Err ( String :: from ( "Schema validation failed" ) ) ? } } } ) ;
            hdk::entry_definition::ValidatingEntryType {
                name: hdk::holochain_core_types::entry::entry_type::EntryType::App(
                    hdk::holochain_core_types::entry::entry_type::AppEntryType::from(
                        "message".to_string(),
                    ),
                ),
                entry_type_definition: entry_type,
                package_creator,
                validator,
                links: <[_]>::into_vec(box []),
            }
        }
    }
}
mod stream {
    use hdk::holochain_core_types::{cas::content::Address, dna::entry_types::Sharing};
    use hdk::{
        self, entry_definition::ValidatingEntryType, holochain_core_types::error::HolochainError,
        holochain_core_types::json::JsonString,
    };
    pub mod handlers {
        use crate::message;
        use crate::stream::Stream;
        use crate::utils;
        use hdk::error::ZomeApiResult;
        use hdk::holochain_core_types::{
            cas::content::Address, entry::Entry, hash::HashString, json::RawString,
        };
        use hdk::AGENT_ADDRESS;
        pub fn handle_create_stream(
            name: String,
            description: String,
            initial_members: Vec<Address>,
        ) -> ZomeApiResult<Address> {
            let stream = Stream { name, description };
            let entry = Entry::App("public_stream".into(), stream.into());
            let stream_address = hdk::commit_entry(&entry)?;
            utils::link_entries_bidir(&AGENT_ADDRESS, &stream_address, "member_of", "has_member")?;
            for member in initial_members {
                utils::link_entries_bidir(&member, &stream_address, "member_of", "has_member")?;
            }
            let anchor_entry =
                Entry::App("anchor".into(), RawString::from("public_streams").into());
            let anchor_address = hdk::commit_entry(&anchor_entry)?;
            hdk::link_entries(&anchor_address, &stream_address, "public_stream")?;
            Ok(stream_address)
        }
        pub fn handle_join_stream(stream_address: HashString) -> ZomeApiResult<()> {
            utils::link_entries_bidir(&AGENT_ADDRESS, &stream_address, "member_of", "has_member")?;
            Ok(())
        }
        pub fn handle_get_members(address: HashString) -> ZomeApiResult<Vec<Address>> {
            let all_member_ids = hdk::get_links(&address, "has_member")?
                .addresses()
                .to_owned();
            Ok(all_member_ids)
        }
        pub fn handle_get_messages(
            address: HashString,
        ) -> ZomeApiResult<utils::GetLinksLoadResult<message::Message>> {
            utils::get_links_and_load_type(&address, "message_in")
        }
        pub fn handle_post_message(
            stream_address: HashString,
            message_spec: message::MessageSpec,
        ) -> ZomeApiResult<()> {
            let message = message::Message::from_spec(&message_spec, &AGENT_ADDRESS.to_string());
            let message_entry = Entry::App("message".into(), message.into());
            let message_addr = hdk::commit_entry(&message_entry)?;
            hdk::link_entries(&stream_address, &message_addr, "message_in")?;
            Ok(())
        }
        pub fn handle_get_all_public_streams() -> ZomeApiResult<utils::GetLinksLoadResult<Stream>> {
            let anchor_entry =
                Entry::App("anchor".into(), RawString::from("public_streams").into());
            let anchor_address = hdk::entry_address(&anchor_entry)?;
            utils::get_links_and_load_type(&anchor_address, "public_stream")
        }
    }
    pub struct Stream {
        pub name: String,
        pub description: String,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_Stream: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Stream {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Stream",
                    false as usize + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "description",
                    &self.description,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_Stream: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Stream {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::export::Ok(__Field::__field0),
                            "description" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::export::Ok(__Field::__field0),
                            b"description" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<Stream>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Stream;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct Stream")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Stream with 2 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Stream with 2 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(Stream {
                            name: __field0,
                            description: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<String> = _serde::export::None;
                        let mut __field1: _serde::export::Option<String> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("name") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("description") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(Stream {
                            name: __field0,
                            description: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["name", "description"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Stream",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<Stream>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for Stream {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                Stream {
                    name: ref __self_0_0,
                    description: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Stream");
                    let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("description", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::clone::Clone for Stream {
        #[inline]
        fn clone(&self) -> Stream {
            match *self {
                Stream {
                    name: ref __self_0_0,
                    description: ref __self_0_1,
                } => Stream {
                    name: $crate::clone::Clone::clone(&(*__self_0_0)),
                    description: $crate::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    impl<'a> From<&'a Stream> for JsonString {
        fn from(v: &Stream) -> JsonString {
            match ::serde_json::to_string(v) {
                Ok(s) => Ok(JsonString::from(s)),
                Err(e) => {
                    {
                        $crate::io::_eprint($crate::fmt::Arguments::new_v1(
                            &["Error serializing to JSON: ", "\n"],
                            &match (&e,) {
                                (arg0,) => [$crate::fmt::ArgumentV1::new(
                                    arg0,
                                    $crate::fmt::Debug::fmt,
                                )],
                            },
                        ));
                    };
                    Err(HolochainError::SerializationError(e.to_string()))
                }
            }
            .expect(&$crate::fmt::format($crate::fmt::Arguments::new_v1(
                &["could not Jsonify ", ": "],
                &match (&"Stream", &v) {
                    (arg0, arg1) => [
                        $crate::fmt::ArgumentV1::new(arg0, $crate::fmt::Display::fmt),
                        $crate::fmt::ArgumentV1::new(arg1, $crate::fmt::Debug::fmt),
                    ],
                },
            )))
        }
    }
    impl From<Stream> for JsonString {
        fn from(v: Stream) -> JsonString {
            JsonString::from(&v)
        }
    }
    impl<'a> ::std::convert::TryFrom<&'a JsonString> for Stream {
        type Error = HolochainError;
        fn try_from(json_string: &JsonString) -> Result<Self, Self::Error> {
            match ::serde_json::from_str(&String::from(json_string)) {
                Ok(d) => Ok(d),
                Err(e) => Err(HolochainError::SerializationError(e.to_string())),
            }
        }
    }
    impl ::std::convert::TryFrom<JsonString> for Stream {
        type Error = HolochainError;
        fn try_from(json_string: JsonString) -> Result<Self, Self::Error> {
            Stream::try_from(&json_string)
        }
    }
    pub fn public_stream_definition() -> ValidatingEntryType {
        {
            let mut entry_type = hdk::holochain_core_types::dna::entry_types::EntryTypeDef::new();
            entry_type.description =
                String::from("A stream of which anyone can become a member and post");
            entry_type.sharing = Sharing::Public;
            match { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: To , other_entry_type : String :: from ( "%agent_id" ) , tag : String :: from ( "has_member" ) , package_creator , validator , } } . link_type { $crate :: LinkDirection :: To => { entry_type . links_to . push ( $crate :: holochain_core_types :: dna :: entry_types :: LinksTo { target_type : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: To , other_entry_type : String :: from ( "%agent_id" ) , tag : String :: from ( "has_member" ) , package_creator , validator , } } . other_entry_type , tag : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: To , other_entry_type : String :: from ( "%agent_id" ) , tag : String :: from ( "has_member" ) , package_creator , validator , } } . tag , } ) ; } $crate :: LinkDirection :: From => { entry_type . linked_from . push ( $crate :: holochain_core_types :: dna :: entry_types :: LinkedFrom { base_type : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: To , other_entry_type : String :: from ( "%agent_id" ) , tag : String :: from ( "has_member" ) , package_creator , validator , } } . other_entry_type , tag : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: To , other_entry_type : String :: from ( "%agent_id" ) , tag : String :: from ( "has_member" ) , package_creator , validator , } } . tag , } ) ; } }
            match { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: From , other_entry_type : String :: from ( "%agent_id" ) , tag : String :: from ( "member_of" ) , package_creator , validator , } } . link_type { $crate :: LinkDirection :: To => { entry_type . links_to . push ( $crate :: holochain_core_types :: dna :: entry_types :: LinksTo { target_type : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: From , other_entry_type : String :: from ( "%agent_id" ) , tag : String :: from ( "member_of" ) , package_creator , validator , } } . other_entry_type , tag : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: From , other_entry_type : String :: from ( "%agent_id" ) , tag : String :: from ( "member_of" ) , package_creator , validator , } } . tag , } ) ; } $crate :: LinkDirection :: From => { entry_type . linked_from . push ( $crate :: holochain_core_types :: dna :: entry_types :: LinkedFrom { base_type : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: From , other_entry_type : String :: from ( "%agent_id" ) , tag : String :: from ( "member_of" ) , package_creator , validator , } } . other_entry_type , tag : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: From , other_entry_type : String :: from ( "%agent_id" ) , tag : String :: from ( "member_of" ) , package_creator , validator , } } . tag , } ) ; } }
            match { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: To , other_entry_type : String :: from ( "message" ) , tag : String :: from ( "message_in" ) , package_creator , validator , } } . link_type { $crate :: LinkDirection :: To => { entry_type . links_to . push ( $crate :: holochain_core_types :: dna :: entry_types :: LinksTo { target_type : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: To , other_entry_type : String :: from ( "message" ) , tag : String :: from ( "message_in" ) , package_creator , validator , } } . other_entry_type , tag : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: To , other_entry_type : String :: from ( "message" ) , tag : String :: from ( "message_in" ) , package_creator , validator , } } . tag , } ) ; } $crate :: LinkDirection :: From => { entry_type . linked_from . push ( $crate :: holochain_core_types :: dna :: entry_types :: LinkedFrom { base_type : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: To , other_entry_type : String :: from ( "message" ) , tag : String :: from ( "message_in" ) , package_creator , validator , } } . other_entry_type , tag : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: To , other_entry_type : String :: from ( "message" ) , tag : String :: from ( "message_in" ) , package_creator , validator , } } . tag , } ) ; } }
            let package_creator = Box::new(|| hdk::ValidationPackageDefinition::Entry);
            let validator = Box :: new ( | entry : hdk :: holochain_core_types :: entry :: Entry , validation_data : hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _ctx = validation_data ; match entry { hdk :: holochain_core_types :: entry :: Entry :: App ( _ , app_entry_value ) => { let entry : Stream = :: std :: convert :: TryInto :: try_into ( app_entry_value ) ? ; let _stream = entry ; { Ok ( ( ) ) } } _ => { Err ( String :: from ( "Schema validation failed" ) ) ? } } } ) ;
            hdk::entry_definition::ValidatingEntryType {
                name: hdk::holochain_core_types::entry::entry_type::EntryType::App(
                    hdk::holochain_core_types::entry::entry_type::AppEntryType::from(
                        "public_stream".to_string(),
                    ),
                ),
                entry_type_definition: entry_type,
                package_creator,
                validator,
                links: <[_]>::into_vec(box [
                    {
                        let package_creator = Box::new(|| hdk::ValidationPackageDefinition::Entry);
                        let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ;
                        ::hdk::entry_definition::ValidatingLinkDefinition {
                            link_type: $crate::LinkDirection::To,
                            other_entry_type: String::from("%agent_id"),
                            tag: String::from("has_member"),
                            package_creator,
                            validator,
                        }
                    },
                    {
                        let package_creator = Box::new(|| hdk::ValidationPackageDefinition::Entry);
                        let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ;
                        ::hdk::entry_definition::ValidatingLinkDefinition {
                            link_type: $crate::LinkDirection::From,
                            other_entry_type: String::from("%agent_id"),
                            tag: String::from("member_of"),
                            package_creator,
                            validator,
                        }
                    },
                    {
                        let package_creator = Box::new(|| hdk::ValidationPackageDefinition::Entry);
                        let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ;
                        ::hdk::entry_definition::ValidatingLinkDefinition {
                            link_type: $crate::LinkDirection::To,
                            other_entry_type: String::from("message"),
                            tag: String::from("message_in"),
                            package_creator,
                            validator,
                        }
                    },
                ]),
            }
        }
    }
}
mod member {
    use hdk::entry_definition::ValidatingEntryType;
    use hdk::holochain_core_types::{
        cas::content::Address, dna::entry_types::Sharing, error::HolochainError, json::JsonString,
    };
    pub mod handlers {
        use crate::member::Profile;
        use crate::utils;
        use hdk::{
            error::{ZomeApiError, ZomeApiResult},
            holochain_core_types::{cas::content::Address, entry::Entry, json::RawString},
            AGENT_ADDRESS,
        };
        pub fn handle_register(name: String, avatar_url: String) -> ZomeApiResult<Address> {
            let anchor_entry =
                Entry::App("anchor".into(), RawString::from("member_directory").into());
            let anchor_address = hdk::commit_entry(&anchor_entry)?;
            hdk::link_entries(&anchor_address, &AGENT_ADDRESS, "member_tag")?;
            let profile_entry = Entry::App(
                "chat_profile".into(),
                Profile {
                    name,
                    avatar_url,
                    address: AGENT_ADDRESS.to_string().into(),
                }
                .into(),
            );
            let profile_addr = hdk::commit_entry(&profile_entry)?;
            hdk::link_entries(&AGENT_ADDRESS, &profile_addr, "profile")?;
            Ok(AGENT_ADDRESS.to_string().into())
        }
        pub fn handle_get_member_profile(agent_address: Address) -> ZomeApiResult<Profile> {
            utils::get_links_and_load_type(&agent_address, "profile")?
                .iter()
                .next()
                .ok_or(ZomeApiError::Internal(
                    "Agent does not have a profile registered".into(),
                ))
                .map(|elem: &utils::GetLinksLoadElement<Profile>| elem.entry.clone())
        }
        pub fn handle_get_my_member_profile() -> ZomeApiResult<Profile> {
            handle_get_member_profile(AGENT_ADDRESS.to_string().into())
        }
    }
    pub struct Member {
        pub address: Address,
        pub profile: Profile,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_Member: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Member {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Member",
                    false as usize + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "address",
                    &self.address,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "profile",
                    &self.profile,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_Member: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Member {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "address" => _serde::export::Ok(__Field::__field0),
                            "profile" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"address" => _serde::export::Ok(__Field::__field0),
                            b"profile" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<Member>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Member;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct Member")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<Address>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Member with 2 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<Profile>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Member with 2 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(Member {
                            address: __field0,
                            profile: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<Address> = _serde::export::None;
                        let mut __field1: _serde::export::Option<Profile> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "address",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Address>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "profile",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Profile>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("address") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("profile") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(Member {
                            address: __field0,
                            profile: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["address", "profile"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Member",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<Member>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for Member {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                Member {
                    address: ref __self_0_0,
                    profile: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Member");
                    let _ = debug_trait_builder.field("address", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("profile", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::clone::Clone for Member {
        #[inline]
        fn clone(&self) -> Member {
            match *self {
                Member {
                    address: ref __self_0_0,
                    profile: ref __self_0_1,
                } => Member {
                    address: $crate::clone::Clone::clone(&(*__self_0_0)),
                    profile: $crate::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    impl<'a> From<&'a Member> for JsonString {
        fn from(v: &Member) -> JsonString {
            match ::serde_json::to_string(v) {
                Ok(s) => Ok(JsonString::from(s)),
                Err(e) => {
                    {
                        $crate::io::_eprint($crate::fmt::Arguments::new_v1(
                            &["Error serializing to JSON: ", "\n"],
                            &match (&e,) {
                                (arg0,) => [$crate::fmt::ArgumentV1::new(
                                    arg0,
                                    $crate::fmt::Debug::fmt,
                                )],
                            },
                        ));
                    };
                    Err(HolochainError::SerializationError(e.to_string()))
                }
            }
            .expect(&$crate::fmt::format($crate::fmt::Arguments::new_v1(
                &["could not Jsonify ", ": "],
                &match (&"Member", &v) {
                    (arg0, arg1) => [
                        $crate::fmt::ArgumentV1::new(arg0, $crate::fmt::Display::fmt),
                        $crate::fmt::ArgumentV1::new(arg1, $crate::fmt::Debug::fmt),
                    ],
                },
            )))
        }
    }
    impl From<Member> for JsonString {
        fn from(v: Member) -> JsonString {
            JsonString::from(&v)
        }
    }
    impl<'a> ::std::convert::TryFrom<&'a JsonString> for Member {
        type Error = HolochainError;
        fn try_from(json_string: &JsonString) -> Result<Self, Self::Error> {
            match ::serde_json::from_str(&String::from(json_string)) {
                Ok(d) => Ok(d),
                Err(e) => Err(HolochainError::SerializationError(e.to_string())),
            }
        }
    }
    impl ::std::convert::TryFrom<JsonString> for Member {
        type Error = HolochainError;
        fn try_from(json_string: JsonString) -> Result<Self, Self::Error> {
            Member::try_from(&json_string)
        }
    }
    pub struct Profile {
        pub name: String,
        pub avatar_url: String,
        pub address: Address,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_Profile: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Profile {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Profile",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "avatar_url",
                    &self.avatar_url,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "address",
                    &self.address,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_Profile: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Profile {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 3",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::export::Ok(__Field::__field0),
                            "avatar_url" => _serde::export::Ok(__Field::__field1),
                            "address" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::export::Ok(__Field::__field0),
                            b"avatar_url" => _serde::export::Ok(__Field::__field1),
                            b"address" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<Profile>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Profile;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct Profile")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Profile with 3 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Profile with 3 elements",
                                    ));
                                }
                            };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<Address>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct Profile with 3 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(Profile {
                            name: __field0,
                            avatar_url: __field1,
                            address: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<String> = _serde::export::None;
                        let mut __field1: _serde::export::Option<String> = _serde::export::None;
                        let mut __field2: _serde::export::Option<Address> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "avatar_url",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "address",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Address>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("name") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("avatar_url") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("address") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(Profile {
                            name: __field0,
                            avatar_url: __field1,
                            address: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["name", "avatar_url", "address"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Profile",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<Profile>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for Profile {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                Profile {
                    name: ref __self_0_0,
                    avatar_url: ref __self_0_1,
                    address: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Profile");
                    let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("avatar_url", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("address", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::clone::Clone for Profile {
        #[inline]
        fn clone(&self) -> Profile {
            match *self {
                Profile {
                    name: ref __self_0_0,
                    avatar_url: ref __self_0_1,
                    address: ref __self_0_2,
                } => Profile {
                    name: $crate::clone::Clone::clone(&(*__self_0_0)),
                    avatar_url: $crate::clone::Clone::clone(&(*__self_0_1)),
                    address: $crate::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    impl<'a> From<&'a Profile> for JsonString {
        fn from(v: &Profile) -> JsonString {
            match ::serde_json::to_string(v) {
                Ok(s) => Ok(JsonString::from(s)),
                Err(e) => {
                    {
                        $crate::io::_eprint($crate::fmt::Arguments::new_v1(
                            &["Error serializing to JSON: ", "\n"],
                            &match (&e,) {
                                (arg0,) => [$crate::fmt::ArgumentV1::new(
                                    arg0,
                                    $crate::fmt::Debug::fmt,
                                )],
                            },
                        ));
                    };
                    Err(HolochainError::SerializationError(e.to_string()))
                }
            }
            .expect(&$crate::fmt::format($crate::fmt::Arguments::new_v1(
                &["could not Jsonify ", ": "],
                &match (&"Profile", &v) {
                    (arg0, arg1) => [
                        $crate::fmt::ArgumentV1::new(arg0, $crate::fmt::Display::fmt),
                        $crate::fmt::ArgumentV1::new(arg1, $crate::fmt::Debug::fmt),
                    ],
                },
            )))
        }
    }
    impl From<Profile> for JsonString {
        fn from(v: Profile) -> JsonString {
            JsonString::from(&v)
        }
    }
    impl<'a> ::std::convert::TryFrom<&'a JsonString> for Profile {
        type Error = HolochainError;
        fn try_from(json_string: &JsonString) -> Result<Self, Self::Error> {
            match ::serde_json::from_str(&String::from(json_string)) {
                Ok(d) => Ok(d),
                Err(e) => Err(HolochainError::SerializationError(e.to_string())),
            }
        }
    }
    impl ::std::convert::TryFrom<JsonString> for Profile {
        type Error = HolochainError;
        fn try_from(json_string: JsonString) -> Result<Self, Self::Error> {
            Profile::try_from(&json_string)
        }
    }
    pub fn profile_definition() -> ValidatingEntryType {
        {
            let mut entry_type = hdk::holochain_core_types::dna::entry_types::EntryTypeDef::new();
            entry_type.description = String::from("The data that chat has about a particular user");
            entry_type.sharing = Sharing::Public;
            match { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: From , other_entry_type : String :: from ( "%agent_id" ) , tag : String :: from ( "profile" ) , package_creator , validator , } } . link_type { $crate :: LinkDirection :: To => { entry_type . links_to . push ( $crate :: holochain_core_types :: dna :: entry_types :: LinksTo { target_type : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: From , other_entry_type : String :: from ( "%agent_id" ) , tag : String :: from ( "profile" ) , package_creator , validator , } } . other_entry_type , tag : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: From , other_entry_type : String :: from ( "%agent_id" ) , tag : String :: from ( "profile" ) , package_creator , validator , } } . tag , } ) ; } $crate :: LinkDirection :: From => { entry_type . linked_from . push ( $crate :: holochain_core_types :: dna :: entry_types :: LinkedFrom { base_type : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: From , other_entry_type : String :: from ( "%agent_id" ) , tag : String :: from ( "profile" ) , package_creator , validator , } } . other_entry_type , tag : { let package_creator = Box :: new ( | | { { hdk :: ValidationPackageDefinition :: Entry } } ) ; let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ; :: hdk :: entry_definition :: ValidatingLinkDefinition { link_type : $crate :: LinkDirection :: From , other_entry_type : String :: from ( "%agent_id" ) , tag : String :: from ( "profile" ) , package_creator , validator , } } . tag , } ) ; } }
            let package_creator = Box::new(|| hdk::ValidationPackageDefinition::Entry);
            let validator = Box :: new ( | entry : hdk :: holochain_core_types :: entry :: Entry , validation_data : hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _ctx = validation_data ; match entry { hdk :: holochain_core_types :: entry :: Entry :: App ( _ , app_entry_value ) => { let entry : Profile = :: std :: convert :: TryInto :: try_into ( app_entry_value ) ? ; let _profile = entry ; { Ok ( ( ) ) } } _ => { Err ( String :: from ( "Schema validation failed" ) ) ? } } } ) ;
            hdk::entry_definition::ValidatingEntryType {
                name: hdk::holochain_core_types::entry::entry_type::EntryType::App(
                    hdk::holochain_core_types::entry::entry_type::AppEntryType::from(
                        "chat_profile".to_string(),
                    ),
                ),
                entry_type_definition: entry_type,
                package_creator,
                validator,
                links: <[_]>::into_vec(box [{
                    let package_creator = Box::new(|| hdk::ValidationPackageDefinition::Entry);
                    let validator = Box :: new ( | source : Address , target : Address , validation_data : :: hdk :: holochain_wasm_utils :: holochain_core_types :: validation :: ValidationData | { let _base = source ; let _target = target ; let _ctx = validation_data ; { Ok ( ( ) ) } } ) ;
                    ::hdk::entry_definition::ValidatingLinkDefinition {
                        link_type: $crate::LinkDirection::From,
                        other_entry_type: String::from("%agent_id"),
                        tag: String::from("profile"),
                        package_creator,
                        validator,
                    }
                }]),
            }
        }
    }
}
mod utils {
    use core::convert::TryFrom;
    use hdk::{
        self,
        error::{ZomeApiError, ZomeApiResult},
        holochain_core_types::{
            entry::{AppEntryValue, Entry},
            hash::HashString,
        },
    };
    pub struct GetLinksLoadElement<T> {
        pub address: HashString,
        pub entry: T,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_GetLinksLoadElement: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<T> _serde::Serialize for GetLinksLoadElement<T>
        where
            T: _serde::Serialize,
        {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "GetLinksLoadElement",
                    false as usize + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "address",
                    &self.address,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "entry",
                    &self.entry,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_GetLinksLoadElement: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, T> _serde::Deserialize<'de> for GetLinksLoadElement<T>
        where
            T: _serde::Deserialize<'de>,
        {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "address" => _serde::export::Ok(__Field::__field0),
                            "entry" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"address" => _serde::export::Ok(__Field::__field0),
                            b"entry" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de, T>
                where
                    T: _serde::Deserialize<'de>,
                {
                    marker: _serde::export::PhantomData<GetLinksLoadElement<T>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
                where
                    T: _serde::Deserialize<'de>,
                {
                    type Value = GetLinksLoadElement<T>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(
                            __formatter,
                            "struct GetLinksLoadElement",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<HashString>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct GetLinksLoadElement with 2 elements",
                                ));
                            }
                        };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<T>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct GetLinksLoadElement with 2 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(GetLinksLoadElement {
                            address: __field0,
                            entry: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<HashString> = _serde::export::None;
                        let mut __field1: _serde::export::Option<T> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "address",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<HashString>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "entry",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<T>(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("address") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("entry") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(GetLinksLoadElement {
                            address: __field0,
                            entry: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["address", "entry"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "GetLinksLoadElement",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<GetLinksLoadElement<T>>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: $crate::fmt::Debug> $crate::fmt::Debug for GetLinksLoadElement<T> {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                GetLinksLoadElement {
                    address: ref __self_0_0,
                    entry: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("GetLinksLoadElement");
                    let _ = debug_trait_builder.field("address", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("entry", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: $crate::clone::Clone> $crate::clone::Clone for GetLinksLoadElement<T> {
        #[inline]
        fn clone(&self) -> GetLinksLoadElement<T> {
            match *self {
                GetLinksLoadElement {
                    address: ref __self_0_0,
                    entry: ref __self_0_1,
                } => GetLinksLoadElement {
                    address: $crate::clone::Clone::clone(&(*__self_0_0)),
                    entry: $crate::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    pub type GetLinksLoadResult<T> = Vec<GetLinksLoadElement<T>>;
    pub fn get_links_and_load<S: Into<String>>(
        base: &HashString,
        tag: S,
    ) -> ZomeApiResult<GetLinksLoadResult<Entry>> {
        let get_links_result = hdk::get_links(base, tag)?;
        Ok(get_links_result
            .addresses()
            .iter()
            .map(|address| {
                hdk::get_entry(&address.to_owned()).map(|entry: Option<Entry>| {
                    GetLinksLoadElement {
                        address: address.to_owned(),
                        entry: entry.unwrap(),
                    }
                })
            })
            .filter_map(Result::ok)
            .collect())
    }
    pub fn get_links_and_load_type<S: Into<String>, R: TryFrom<AppEntryValue>>(
        base: &HashString,
        tag: S,
    ) -> ZomeApiResult<GetLinksLoadResult<R>> {
        let link_load_results = get_links_and_load(base, tag)?;
        Ok(link_load_results
            .iter()
            .map(|get_links_result| match get_links_result.entry.clone() {
                Entry::App(_, entry_value) => {
                    let entry = R::try_from(entry_value).map_err(|_| {
                        ZomeApiError::Internal(
                            "Could not convert get_links result to requested type".to_string(),
                        )
                    })?;
                    Ok(GetLinksLoadElement::<R> {
                        entry: entry,
                        address: get_links_result.address.clone(),
                    })
                }
                _ => Err(ZomeApiError::Internal(
                    "get_links did not return an app entry".to_string(),
                )),
            })
            .filter_map(Result::ok)
            .collect())
    }
    pub fn link_entries_bidir<S: Into<String>>(
        a: &HashString,
        b: &HashString,
        tag_a_b: &str,
        tag_b_a: S,
    ) -> ZomeApiResult<()> {
        hdk::link_entries(a, b, tag_a_b)?;
        hdk::link_entries(b, a, tag_b_a)?;
        Ok(())
    }
}
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn zome_setup(zd: &mut $crate::meta::ZomeDefinition) {
    zd.define(message::message_definition());
    zd.define(stream::public_stream_definition());
    zd.define(member::profile_definition());
    zd.define(anchor::anchor_definition());
}
#[no_mangle]
pub extern "C" fn genesis(
    encoded_allocation_of_input: hdk::holochain_core_types::error::RibosomeEncodingBits,
) -> hdk::holochain_core_types::error::RibosomeEncodingBits {
    let maybe_allocation = $crate :: holochain_wasm_utils :: memory :: allocation :: WasmAllocation :: try_from_ribosome_encoding ( encoded_allocation_of_input ) ;
    let allocation = match maybe_allocation {
        Ok(allocation) => allocation,
        Err(allocation_error) => {
            return hdk::holochain_core_types::error::RibosomeEncodedValue::from(allocation_error)
                .into();
        }
    };
    let init = $crate::global_fns::init_global_memory(allocation);
    if init.is_err() {
        return $crate::holochain_wasm_utils::memory::ribosome::return_code_for_allocation_result(
            init,
        )
        .into();
    }
    fn execute() -> Result<(), String> {
        {
            Ok(())
        }
    }
    match execute() {
        Ok(_) => hdk::holochain_core_types::error::RibosomeEncodedValue::Success.into(),
        Err(e) => {
            $crate::holochain_wasm_utils::memory::ribosome::return_code_for_allocation_result(
                $crate::global_fns::write_json(
                    $crate::holochain_wasm_utils::holochain_core_types::json::RawString::from(e),
                ),
            )
            .into()
        }
    }
}
use std::collections::HashMap;
#[no_mangle]
#[allow(unused_imports)]
pub fn __list_traits() -> $crate::holochain_core_types::dna::zome::ZomeTraits {
    use std::collections::BTreeMap;
    use $crate::holochain_core_types::dna::fn_declarations::{
        FnDeclaration, FnParameter, TraitFns,
    };
    let return_value: $crate::holochain_core_types::dna::zome::ZomeTraits = {
        let mut traitfns_map = BTreeMap::new();
        {
            let mut traitfns = TraitFns::new();
            traitfns.functions = <[_]>::into_vec(box [
                "register".into(),
                "create_stream".into(),
                "join_stream".into(),
                "get_all_public_streams".into(),
                "get_members".into(),
                "get_member_profile".into(),
                "get_my_member_profile".into(),
                "post_message".into(),
                "get_messages".into(),
            ]);
            traitfns_map.insert("hc_public".into(), traitfns);
        }
        traitfns_map
    };
    return_value
}
#[no_mangle]
#[allow(unused_imports)]
pub fn __list_functions() -> $crate::holochain_core_types::dna::zome::ZomeFnDeclarations {
    use $crate::holochain_core_types::dna::fn_declarations::{FnDeclaration, FnParameter};
    let return_value: $crate::holochain_core_types::dna::zome::ZomeFnDeclarations = {
        <[_]>::into_vec(box [
            FnDeclaration {
                name: "register".into(),
                inputs: <[_]>::into_vec(box [
                    FnParameter::new("name", "String"),
                    FnParameter::new("avatar_url", "String"),
                ]),
                outputs: <[_]>::into_vec(box [FnParameter::new(
                    "result",
                    "ZomeApiResult<Address>",
                )]),
            },
            FnDeclaration {
                name: "create_stream".into(),
                inputs: <[_]>::into_vec(box [
                    FnParameter::new("name", "String"),
                    FnParameter::new("description", "String"),
                    FnParameter::new("initial_members", "Vec<Address>"),
                ]),
                outputs: <[_]>::into_vec(box [FnParameter::new(
                    "result",
                    "ZomeApiResult<Address>",
                )]),
            },
            FnDeclaration {
                name: "join_stream".into(),
                inputs: <[_]>::into_vec(box [FnParameter::new("stream_address", "HashString")]),
                outputs: <[_]>::into_vec(box [FnParameter::new("result", "ZomeApiResult<()>")]),
            },
            FnDeclaration {
                name: "get_all_public_streams".into(),
                inputs: <[_]>::into_vec(box []),
                outputs: <[_]>::into_vec(box [FnParameter::new(
                    "result",
                    "ZomeApiResult<utils::GetLinksLoadResult<stream::Stream>>",
                )]),
            },
            FnDeclaration {
                name: "get_members".into(),
                inputs: <[_]>::into_vec(box [FnParameter::new("stream_address", "HashString")]),
                outputs: <[_]>::into_vec(box [FnParameter::new(
                    "result",
                    "ZomeApiResult<Vec<Address>>",
                )]),
            },
            FnDeclaration {
                name: "get_member_profile".into(),
                inputs: <[_]>::into_vec(box [FnParameter::new("agent_address", "HashString")]),
                outputs: <[_]>::into_vec(box [FnParameter::new(
                    "result",
                    "ZomeApiResult<member::Profile>",
                )]),
            },
            FnDeclaration {
                name: "get_my_member_profile".into(),
                inputs: <[_]>::into_vec(box []),
                outputs: <[_]>::into_vec(box [FnParameter::new(
                    "result",
                    "ZomeApiResult<member::Profile>",
                )]),
            },
            FnDeclaration {
                name: "post_message".into(),
                inputs: <[_]>::into_vec(box [
                    FnParameter::new("stream_address", "HashString"),
                    FnParameter::new("message", "message::MessageSpec"),
                ]),
                outputs: <[_]>::into_vec(box [FnParameter::new("result", "ZomeApiResult<()>")]),
            },
            FnDeclaration {
                name: "get_messages".into(),
                inputs: <[_]>::into_vec(box [FnParameter::new("address", "HashString")]),
                outputs: <[_]>::into_vec(box [FnParameter::new(
                    "result",
                    "ZomeApiResult<utils::GetLinksLoadResult<message::Message>>",
                )]),
            },
        ])
    };
    return_value
}
#[no_mangle]
pub extern "C" fn __install_panic_handler() -> () {
    use std::panic;
    use $crate::{api::debug, holochain_core_types::json::RawString};
    panic::set_hook(Box::new(move |info| {
        let _ = debug(RawString::from(
            info.payload().downcast_ref::<String>().unwrap().clone(),
        ));
        let _ = if let Some(location) = info.location() {
            debug(RawString::from($crate::fmt::format(
                $crate::fmt::Arguments::new_v1(
                    &["panic occurred in file \'", "\' at line "],
                    &match (&location.file(), &location.line()) {
                        (arg0, arg1) => [
                            $crate::fmt::ArgumentV1::new(arg0, $crate::fmt::Display::fmt),
                            $crate::fmt::ArgumentV1::new(arg1, $crate::fmt::Display::fmt),
                        ],
                    },
                ),
            )))
        } else {
            debug(RawString::from($crate::fmt::format(
                $crate::fmt::Arguments::new_v1(
                    &["panic occurred but can\'t get location information..."],
                    &match () {
                        () => [],
                    },
                ),
            )))
        };
    }));
}
#[no_mangle]
pub extern "C" fn register(
    encoded_allocation_of_input: hdk::holochain_core_types::error::RibosomeEncodingBits,
) -> hdk::holochain_core_types::error::RibosomeEncodingBits {
    let maybe_allocation = $crate :: holochain_wasm_utils :: memory :: allocation :: WasmAllocation :: try_from_ribosome_encoding ( encoded_allocation_of_input ) ;
    let allocation = match maybe_allocation {
        Ok(allocation) => allocation,
        Err(allocation_error) => {
            return hdk::holochain_core_types::error::RibosomeEncodedValue::from(allocation_error)
                .into();
        }
    };
    let init = $crate::global_fns::init_global_memory(allocation);
    if init.is_err() {
        return $crate::holochain_wasm_utils::memory::ribosome::return_code_for_allocation_result(
            init,
        )
        .into();
    }
    struct InputStruct {
        name: String,
        avatar_url: String,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_InputStruct: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for InputStruct {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::export::Ok(__Field::__field0),
                            "avatar_url" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::export::Ok(__Field::__field0),
                            b"avatar_url" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<InputStruct>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = InputStruct;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct InputStruct")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct InputStruct with 2 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct InputStruct with 2 elements",
                                    ));
                                }
                            };
                        _serde::export::Ok(InputStruct {
                            name: __field0,
                            avatar_url: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<String> = _serde::export::None;
                        let mut __field1: _serde::export::Option<String> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "avatar_url",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("name") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("avatar_url") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(InputStruct {
                            name: __field0,
                            avatar_url: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["name", "avatar_url"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "InputStruct",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<InputStruct>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_InputStruct: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for InputStruct {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "InputStruct",
                    false as usize + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "avatar_url",
                    &self.avatar_url,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for InputStruct {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                InputStruct {
                    name: ref __self_0_0,
                    avatar_url: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("InputStruct");
                    let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("avatar_url", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl<'a> From<&'a InputStruct> for JsonString {
        fn from(v: &InputStruct) -> JsonString {
            match ::serde_json::to_string(v) {
                Ok(s) => Ok(JsonString::from(s)),
                Err(e) => {
                    {
                        $crate::io::_eprint($crate::fmt::Arguments::new_v1(
                            &["Error serializing to JSON: ", "\n"],
                            &match (&e,) {
                                (arg0,) => [$crate::fmt::ArgumentV1::new(
                                    arg0,
                                    $crate::fmt::Debug::fmt,
                                )],
                            },
                        ));
                    };
                    Err(HolochainError::SerializationError(e.to_string()))
                }
            }
            .expect(&$crate::fmt::format($crate::fmt::Arguments::new_v1(
                &["could not Jsonify ", ": "],
                &match (&"InputStruct", &v) {
                    (arg0, arg1) => [
                        $crate::fmt::ArgumentV1::new(arg0, $crate::fmt::Display::fmt),
                        $crate::fmt::ArgumentV1::new(arg1, $crate::fmt::Debug::fmt),
                    ],
                },
            )))
        }
    }
    impl From<InputStruct> for JsonString {
        fn from(v: InputStruct) -> JsonString {
            JsonString::from(&v)
        }
    }
    impl<'a> ::std::convert::TryFrom<&'a JsonString> for InputStruct {
        type Error = HolochainError;
        fn try_from(json_string: &JsonString) -> Result<Self, Self::Error> {
            match ::serde_json::from_str(&String::from(json_string)) {
                Ok(d) => Ok(d),
                Err(e) => Err(HolochainError::SerializationError(e.to_string())),
            }
        }
    }
    impl ::std::convert::TryFrom<JsonString> for InputStruct {
        type Error = HolochainError;
        fn try_from(json_string: JsonString) -> Result<Self, Self::Error> {
            InputStruct::try_from(&json_string)
        }
    }
    let input: InputStruct = {
        let maybe_input =
            $crate::holochain_wasm_utils::memory::ribosome::load_ribosome_encoded_json(
                encoded_allocation_of_input,
            );
        match maybe_input { Ok ( input ) => input , Err ( hc_err ) => return $crate :: holochain_wasm_utils :: memory :: ribosome :: return_code_for_allocation_result ( $crate :: global_fns :: write_json ( hc_err ) ) . into ( ) , }
    };
    fn execute(params: InputStruct) -> ZomeApiResult<Address> {
        let InputStruct { name, avatar_url } = params;
        member::handlers::handle_register(name, avatar_url)
    }
    $crate::holochain_wasm_utils::memory::ribosome::return_code_for_allocation_result(
        $crate::global_fns::write_json(execute(input)),
    )
    .into()
}
#[no_mangle]
pub extern "C" fn create_stream(
    encoded_allocation_of_input: hdk::holochain_core_types::error::RibosomeEncodingBits,
) -> hdk::holochain_core_types::error::RibosomeEncodingBits {
    let maybe_allocation = $crate :: holochain_wasm_utils :: memory :: allocation :: WasmAllocation :: try_from_ribosome_encoding ( encoded_allocation_of_input ) ;
    let allocation = match maybe_allocation {
        Ok(allocation) => allocation,
        Err(allocation_error) => {
            return hdk::holochain_core_types::error::RibosomeEncodedValue::from(allocation_error)
                .into();
        }
    };
    let init = $crate::global_fns::init_global_memory(allocation);
    if init.is_err() {
        return $crate::holochain_wasm_utils::memory::ribosome::return_code_for_allocation_result(
            init,
        )
        .into();
    }
    struct InputStruct {
        name: String,
        description: String,
        initial_members: Vec<Address>,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_InputStruct: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for InputStruct {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 3",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::export::Ok(__Field::__field0),
                            "description" => _serde::export::Ok(__Field::__field1),
                            "initial_members" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::export::Ok(__Field::__field0),
                            b"description" => _serde::export::Ok(__Field::__field1),
                            b"initial_members" => _serde::export::Ok(__Field::__field2),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<InputStruct>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = InputStruct;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct InputStruct")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct InputStruct with 3 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct InputStruct with 3 elements",
                                    ));
                                }
                            };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<Vec<Address>>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct InputStruct with 3 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(InputStruct {
                            name: __field0,
                            description: __field1,
                            initial_members: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<String> = _serde::export::None;
                        let mut __field1: _serde::export::Option<String> = _serde::export::None;
                        let mut __field2: _serde::export::Option<Vec<Address>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "initial_members",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<Address>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("name") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("description") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("initial_members") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(InputStruct {
                            name: __field0,
                            description: __field1,
                            initial_members: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["name", "description", "initial_members"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "InputStruct",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<InputStruct>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_InputStruct: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for InputStruct {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "InputStruct",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "description",
                    &self.description,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "initial_members",
                    &self.initial_members,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for InputStruct {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                InputStruct {
                    name: ref __self_0_0,
                    description: ref __self_0_1,
                    initial_members: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("InputStruct");
                    let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("description", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("initial_members", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl<'a> From<&'a InputStruct> for JsonString {
        fn from(v: &InputStruct) -> JsonString {
            match ::serde_json::to_string(v) {
                Ok(s) => Ok(JsonString::from(s)),
                Err(e) => {
                    {
                        $crate::io::_eprint($crate::fmt::Arguments::new_v1(
                            &["Error serializing to JSON: ", "\n"],
                            &match (&e,) {
                                (arg0,) => [$crate::fmt::ArgumentV1::new(
                                    arg0,
                                    $crate::fmt::Debug::fmt,
                                )],
                            },
                        ));
                    };
                    Err(HolochainError::SerializationError(e.to_string()))
                }
            }
            .expect(&$crate::fmt::format($crate::fmt::Arguments::new_v1(
                &["could not Jsonify ", ": "],
                &match (&"InputStruct", &v) {
                    (arg0, arg1) => [
                        $crate::fmt::ArgumentV1::new(arg0, $crate::fmt::Display::fmt),
                        $crate::fmt::ArgumentV1::new(arg1, $crate::fmt::Debug::fmt),
                    ],
                },
            )))
        }
    }
    impl From<InputStruct> for JsonString {
        fn from(v: InputStruct) -> JsonString {
            JsonString::from(&v)
        }
    }
    impl<'a> ::std::convert::TryFrom<&'a JsonString> for InputStruct {
        type Error = HolochainError;
        fn try_from(json_string: &JsonString) -> Result<Self, Self::Error> {
            match ::serde_json::from_str(&String::from(json_string)) {
                Ok(d) => Ok(d),
                Err(e) => Err(HolochainError::SerializationError(e.to_string())),
            }
        }
    }
    impl ::std::convert::TryFrom<JsonString> for InputStruct {
        type Error = HolochainError;
        fn try_from(json_string: JsonString) -> Result<Self, Self::Error> {
            InputStruct::try_from(&json_string)
        }
    }
    let input: InputStruct = {
        let maybe_input =
            $crate::holochain_wasm_utils::memory::ribosome::load_ribosome_encoded_json(
                encoded_allocation_of_input,
            );
        match maybe_input { Ok ( input ) => input , Err ( hc_err ) => return $crate :: holochain_wasm_utils :: memory :: ribosome :: return_code_for_allocation_result ( $crate :: global_fns :: write_json ( hc_err ) ) . into ( ) , }
    };
    fn execute(params: InputStruct) -> ZomeApiResult<Address> {
        let InputStruct {
            name,
            description,
            initial_members,
        } = params;
        stream::handlers::handle_create_stream(name, description, initial_members)
    }
    $crate::holochain_wasm_utils::memory::ribosome::return_code_for_allocation_result(
        $crate::global_fns::write_json(execute(input)),
    )
    .into()
}
#[no_mangle]
pub extern "C" fn join_stream(
    encoded_allocation_of_input: hdk::holochain_core_types::error::RibosomeEncodingBits,
) -> hdk::holochain_core_types::error::RibosomeEncodingBits {
    let maybe_allocation = $crate :: holochain_wasm_utils :: memory :: allocation :: WasmAllocation :: try_from_ribosome_encoding ( encoded_allocation_of_input ) ;
    let allocation = match maybe_allocation {
        Ok(allocation) => allocation,
        Err(allocation_error) => {
            return hdk::holochain_core_types::error::RibosomeEncodedValue::from(allocation_error)
                .into();
        }
    };
    let init = $crate::global_fns::init_global_memory(allocation);
    if init.is_err() {
        return $crate::holochain_wasm_utils::memory::ribosome::return_code_for_allocation_result(
            init,
        )
        .into();
    }
    struct InputStruct {
        stream_address: HashString,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_InputStruct: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for InputStruct {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "stream_address" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"stream_address" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<InputStruct>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = InputStruct;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct InputStruct")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<HashString>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct InputStruct with 1 element",
                                ));
                            }
                        };
                        _serde::export::Ok(InputStruct {
                            stream_address: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<HashString> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "stream_address",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<HashString>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("stream_address") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(InputStruct {
                            stream_address: __field0,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["stream_address"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "InputStruct",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<InputStruct>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_InputStruct: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for InputStruct {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "InputStruct",
                    false as usize + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "stream_address",
                    &self.stream_address,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for InputStruct {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                InputStruct {
                    stream_address: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("InputStruct");
                    let _ = debug_trait_builder.field("stream_address", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl<'a> From<&'a InputStruct> for JsonString {
        fn from(v: &InputStruct) -> JsonString {
            match ::serde_json::to_string(v) {
                Ok(s) => Ok(JsonString::from(s)),
                Err(e) => {
                    {
                        $crate::io::_eprint($crate::fmt::Arguments::new_v1(
                            &["Error serializing to JSON: ", "\n"],
                            &match (&e,) {
                                (arg0,) => [$crate::fmt::ArgumentV1::new(
                                    arg0,
                                    $crate::fmt::Debug::fmt,
                                )],
                            },
                        ));
                    };
                    Err(HolochainError::SerializationError(e.to_string()))
                }
            }
            .expect(&$crate::fmt::format($crate::fmt::Arguments::new_v1(
                &["could not Jsonify ", ": "],
                &match (&"InputStruct", &v) {
                    (arg0, arg1) => [
                        $crate::fmt::ArgumentV1::new(arg0, $crate::fmt::Display::fmt),
                        $crate::fmt::ArgumentV1::new(arg1, $crate::fmt::Debug::fmt),
                    ],
                },
            )))
        }
    }
    impl From<InputStruct> for JsonString {
        fn from(v: InputStruct) -> JsonString {
            JsonString::from(&v)
        }
    }
    impl<'a> ::std::convert::TryFrom<&'a JsonString> for InputStruct {
        type Error = HolochainError;
        fn try_from(json_string: &JsonString) -> Result<Self, Self::Error> {
            match ::serde_json::from_str(&String::from(json_string)) {
                Ok(d) => Ok(d),
                Err(e) => Err(HolochainError::SerializationError(e.to_string())),
            }
        }
    }
    impl ::std::convert::TryFrom<JsonString> for InputStruct {
        type Error = HolochainError;
        fn try_from(json_string: JsonString) -> Result<Self, Self::Error> {
            InputStruct::try_from(&json_string)
        }
    }
    let input: InputStruct = {
        let maybe_input =
            $crate::holochain_wasm_utils::memory::ribosome::load_ribosome_encoded_json(
                encoded_allocation_of_input,
            );
        match maybe_input { Ok ( input ) => input , Err ( hc_err ) => return $crate :: holochain_wasm_utils :: memory :: ribosome :: return_code_for_allocation_result ( $crate :: global_fns :: write_json ( hc_err ) ) . into ( ) , }
    };
    fn execute(params: InputStruct) -> ZomeApiResult<()> {
        let InputStruct { stream_address } = params;
        stream::handlers::handle_join_stream(stream_address)
    }
    $crate::holochain_wasm_utils::memory::ribosome::return_code_for_allocation_result(
        $crate::global_fns::write_json(execute(input)),
    )
    .into()
}
#[no_mangle]
pub extern "C" fn get_all_public_streams(
    encoded_allocation_of_input: hdk::holochain_core_types::error::RibosomeEncodingBits,
) -> hdk::holochain_core_types::error::RibosomeEncodingBits {
    let maybe_allocation = $crate :: holochain_wasm_utils :: memory :: allocation :: WasmAllocation :: try_from_ribosome_encoding ( encoded_allocation_of_input ) ;
    let allocation = match maybe_allocation {
        Ok(allocation) => allocation,
        Err(allocation_error) => {
            return hdk::holochain_core_types::error::RibosomeEncodedValue::from(allocation_error)
                .into();
        }
    };
    let init = $crate::global_fns::init_global_memory(allocation);
    if init.is_err() {
        return $crate::holochain_wasm_utils::memory::ribosome::return_code_for_allocation_result(
            init,
        )
        .into();
    }
    struct InputStruct {}
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_InputStruct: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for InputStruct {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 0",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<InputStruct>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = InputStruct;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct InputStruct")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        _: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        _serde::export::Ok(InputStruct {})
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        _serde::export::Ok(InputStruct {})
                    }
                }
                const FIELDS: &'static [&'static str] = &[];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "InputStruct",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<InputStruct>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_InputStruct: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for InputStruct {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "InputStruct",
                    false as usize,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for InputStruct {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                InputStruct {} => {
                    let mut debug_trait_builder = f.debug_struct("InputStruct");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl<'a> From<&'a InputStruct> for JsonString {
        fn from(v: &InputStruct) -> JsonString {
            match ::serde_json::to_string(v) {
                Ok(s) => Ok(JsonString::from(s)),
                Err(e) => {
                    {
                        $crate::io::_eprint($crate::fmt::Arguments::new_v1(
                            &["Error serializing to JSON: ", "\n"],
                            &match (&e,) {
                                (arg0,) => [$crate::fmt::ArgumentV1::new(
                                    arg0,
                                    $crate::fmt::Debug::fmt,
                                )],
                            },
                        ));
                    };
                    Err(HolochainError::SerializationError(e.to_string()))
                }
            }
            .expect(&$crate::fmt::format($crate::fmt::Arguments::new_v1(
                &["could not Jsonify ", ": "],
                &match (&"InputStruct", &v) {
                    (arg0, arg1) => [
                        $crate::fmt::ArgumentV1::new(arg0, $crate::fmt::Display::fmt),
                        $crate::fmt::ArgumentV1::new(arg1, $crate::fmt::Debug::fmt),
                    ],
                },
            )))
        }
    }
    impl From<InputStruct> for JsonString {
        fn from(v: InputStruct) -> JsonString {
            JsonString::from(&v)
        }
    }
    impl<'a> ::std::convert::TryFrom<&'a JsonString> for InputStruct {
        type Error = HolochainError;
        fn try_from(json_string: &JsonString) -> Result<Self, Self::Error> {
            match ::serde_json::from_str(&String::from(json_string)) {
                Ok(d) => Ok(d),
                Err(e) => Err(HolochainError::SerializationError(e.to_string())),
            }
        }
    }
    impl ::std::convert::TryFrom<JsonString> for InputStruct {
        type Error = HolochainError;
        fn try_from(json_string: JsonString) -> Result<Self, Self::Error> {
            InputStruct::try_from(&json_string)
        }
    }
    let input: InputStruct = {
        let maybe_input =
            $crate::holochain_wasm_utils::memory::ribosome::load_ribosome_encoded_json(
                encoded_allocation_of_input,
            );
        match maybe_input { Ok ( input ) => input , Err ( hc_err ) => return $crate :: holochain_wasm_utils :: memory :: ribosome :: return_code_for_allocation_result ( $crate :: global_fns :: write_json ( hc_err ) ) . into ( ) , }
    };
    fn execute(params: InputStruct) -> ZomeApiResult<utils::GetLinksLoadResult<stream::Stream>> {
        let InputStruct {} = params;
        stream::handlers::handle_get_all_public_streams()
    }
    $crate::holochain_wasm_utils::memory::ribosome::return_code_for_allocation_result(
        $crate::global_fns::write_json(execute(input)),
    )
    .into()
}
#[no_mangle]
pub extern "C" fn get_members(
    encoded_allocation_of_input: hdk::holochain_core_types::error::RibosomeEncodingBits,
) -> hdk::holochain_core_types::error::RibosomeEncodingBits {
    let maybe_allocation = $crate :: holochain_wasm_utils :: memory :: allocation :: WasmAllocation :: try_from_ribosome_encoding ( encoded_allocation_of_input ) ;
    let allocation = match maybe_allocation {
        Ok(allocation) => allocation,
        Err(allocation_error) => {
            return hdk::holochain_core_types::error::RibosomeEncodedValue::from(allocation_error)
                .into();
        }
    };
    let init = $crate::global_fns::init_global_memory(allocation);
    if init.is_err() {
        return $crate::holochain_wasm_utils::memory::ribosome::return_code_for_allocation_result(
            init,
        )
        .into();
    }
    struct InputStruct {
        stream_address: HashString,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_InputStruct: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for InputStruct {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "stream_address" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"stream_address" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<InputStruct>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = InputStruct;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct InputStruct")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<HashString>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct InputStruct with 1 element",
                                ));
                            }
                        };
                        _serde::export::Ok(InputStruct {
                            stream_address: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<HashString> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "stream_address",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<HashString>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("stream_address") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(InputStruct {
                            stream_address: __field0,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["stream_address"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "InputStruct",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<InputStruct>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_InputStruct: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for InputStruct {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "InputStruct",
                    false as usize + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "stream_address",
                    &self.stream_address,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for InputStruct {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                InputStruct {
                    stream_address: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("InputStruct");
                    let _ = debug_trait_builder.field("stream_address", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl<'a> From<&'a InputStruct> for JsonString {
        fn from(v: &InputStruct) -> JsonString {
            match ::serde_json::to_string(v) {
                Ok(s) => Ok(JsonString::from(s)),
                Err(e) => {
                    {
                        $crate::io::_eprint($crate::fmt::Arguments::new_v1(
                            &["Error serializing to JSON: ", "\n"],
                            &match (&e,) {
                                (arg0,) => [$crate::fmt::ArgumentV1::new(
                                    arg0,
                                    $crate::fmt::Debug::fmt,
                                )],
                            },
                        ));
                    };
                    Err(HolochainError::SerializationError(e.to_string()))
                }
            }
            .expect(&$crate::fmt::format($crate::fmt::Arguments::new_v1(
                &["could not Jsonify ", ": "],
                &match (&"InputStruct", &v) {
                    (arg0, arg1) => [
                        $crate::fmt::ArgumentV1::new(arg0, $crate::fmt::Display::fmt),
                        $crate::fmt::ArgumentV1::new(arg1, $crate::fmt::Debug::fmt),
                    ],
                },
            )))
        }
    }
    impl From<InputStruct> for JsonString {
        fn from(v: InputStruct) -> JsonString {
            JsonString::from(&v)
        }
    }
    impl<'a> ::std::convert::TryFrom<&'a JsonString> for InputStruct {
        type Error = HolochainError;
        fn try_from(json_string: &JsonString) -> Result<Self, Self::Error> {
            match ::serde_json::from_str(&String::from(json_string)) {
                Ok(d) => Ok(d),
                Err(e) => Err(HolochainError::SerializationError(e.to_string())),
            }
        }
    }
    impl ::std::convert::TryFrom<JsonString> for InputStruct {
        type Error = HolochainError;
        fn try_from(json_string: JsonString) -> Result<Self, Self::Error> {
            InputStruct::try_from(&json_string)
        }
    }
    let input: InputStruct = {
        let maybe_input =
            $crate::holochain_wasm_utils::memory::ribosome::load_ribosome_encoded_json(
                encoded_allocation_of_input,
            );
        match maybe_input { Ok ( input ) => input , Err ( hc_err ) => return $crate :: holochain_wasm_utils :: memory :: ribosome :: return_code_for_allocation_result ( $crate :: global_fns :: write_json ( hc_err ) ) . into ( ) , }
    };
    fn execute(params: InputStruct) -> ZomeApiResult<Vec<Address>> {
        let InputStruct { stream_address } = params;
        stream::handlers::handle_get_members(stream_address)
    }
    $crate::holochain_wasm_utils::memory::ribosome::return_code_for_allocation_result(
        $crate::global_fns::write_json(execute(input)),
    )
    .into()
}
#[no_mangle]
pub extern "C" fn get_member_profile(
    encoded_allocation_of_input: hdk::holochain_core_types::error::RibosomeEncodingBits,
) -> hdk::holochain_core_types::error::RibosomeEncodingBits {
    let maybe_allocation = $crate :: holochain_wasm_utils :: memory :: allocation :: WasmAllocation :: try_from_ribosome_encoding ( encoded_allocation_of_input ) ;
    let allocation = match maybe_allocation {
        Ok(allocation) => allocation,
        Err(allocation_error) => {
            return hdk::holochain_core_types::error::RibosomeEncodedValue::from(allocation_error)
                .into();
        }
    };
    let init = $crate::global_fns::init_global_memory(allocation);
    if init.is_err() {
        return $crate::holochain_wasm_utils::memory::ribosome::return_code_for_allocation_result(
            init,
        )
        .into();
    }
    struct InputStruct {
        agent_address: HashString,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_InputStruct: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for InputStruct {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "agent_address" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"agent_address" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<InputStruct>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = InputStruct;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct InputStruct")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<HashString>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct InputStruct with 1 element",
                                ));
                            }
                        };
                        _serde::export::Ok(InputStruct {
                            agent_address: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<HashString> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "agent_address",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<HashString>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("agent_address") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(InputStruct {
                            agent_address: __field0,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["agent_address"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "InputStruct",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<InputStruct>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_InputStruct: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for InputStruct {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "InputStruct",
                    false as usize + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "agent_address",
                    &self.agent_address,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for InputStruct {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                InputStruct {
                    agent_address: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("InputStruct");
                    let _ = debug_trait_builder.field("agent_address", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl<'a> From<&'a InputStruct> for JsonString {
        fn from(v: &InputStruct) -> JsonString {
            match ::serde_json::to_string(v) {
                Ok(s) => Ok(JsonString::from(s)),
                Err(e) => {
                    {
                        $crate::io::_eprint($crate::fmt::Arguments::new_v1(
                            &["Error serializing to JSON: ", "\n"],
                            &match (&e,) {
                                (arg0,) => [$crate::fmt::ArgumentV1::new(
                                    arg0,
                                    $crate::fmt::Debug::fmt,
                                )],
                            },
                        ));
                    };
                    Err(HolochainError::SerializationError(e.to_string()))
                }
            }
            .expect(&$crate::fmt::format($crate::fmt::Arguments::new_v1(
                &["could not Jsonify ", ": "],
                &match (&"InputStruct", &v) {
                    (arg0, arg1) => [
                        $crate::fmt::ArgumentV1::new(arg0, $crate::fmt::Display::fmt),
                        $crate::fmt::ArgumentV1::new(arg1, $crate::fmt::Debug::fmt),
                    ],
                },
            )))
        }
    }
    impl From<InputStruct> for JsonString {
        fn from(v: InputStruct) -> JsonString {
            JsonString::from(&v)
        }
    }
    impl<'a> ::std::convert::TryFrom<&'a JsonString> for InputStruct {
        type Error = HolochainError;
        fn try_from(json_string: &JsonString) -> Result<Self, Self::Error> {
            match ::serde_json::from_str(&String::from(json_string)) {
                Ok(d) => Ok(d),
                Err(e) => Err(HolochainError::SerializationError(e.to_string())),
            }
        }
    }
    impl ::std::convert::TryFrom<JsonString> for InputStruct {
        type Error = HolochainError;
        fn try_from(json_string: JsonString) -> Result<Self, Self::Error> {
            InputStruct::try_from(&json_string)
        }
    }
    let input: InputStruct = {
        let maybe_input =
            $crate::holochain_wasm_utils::memory::ribosome::load_ribosome_encoded_json(
                encoded_allocation_of_input,
            );
        match maybe_input { Ok ( input ) => input , Err ( hc_err ) => return $crate :: holochain_wasm_utils :: memory :: ribosome :: return_code_for_allocation_result ( $crate :: global_fns :: write_json ( hc_err ) ) . into ( ) , }
    };
    fn execute(params: InputStruct) -> ZomeApiResult<member::Profile> {
        let InputStruct { agent_address } = params;
        member::handlers::handle_get_member_profile(agent_address)
    }
    $crate::holochain_wasm_utils::memory::ribosome::return_code_for_allocation_result(
        $crate::global_fns::write_json(execute(input)),
    )
    .into()
}
#[no_mangle]
pub extern "C" fn get_my_member_profile(
    encoded_allocation_of_input: hdk::holochain_core_types::error::RibosomeEncodingBits,
) -> hdk::holochain_core_types::error::RibosomeEncodingBits {
    let maybe_allocation = $crate :: holochain_wasm_utils :: memory :: allocation :: WasmAllocation :: try_from_ribosome_encoding ( encoded_allocation_of_input ) ;
    let allocation = match maybe_allocation {
        Ok(allocation) => allocation,
        Err(allocation_error) => {
            return hdk::holochain_core_types::error::RibosomeEncodedValue::from(allocation_error)
                .into();
        }
    };
    let init = $crate::global_fns::init_global_memory(allocation);
    if init.is_err() {
        return $crate::holochain_wasm_utils::memory::ribosome::return_code_for_allocation_result(
            init,
        )
        .into();
    }
    struct InputStruct {}
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_InputStruct: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for InputStruct {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 0",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<InputStruct>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = InputStruct;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct InputStruct")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        _: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        _serde::export::Ok(InputStruct {})
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        _serde::export::Ok(InputStruct {})
                    }
                }
                const FIELDS: &'static [&'static str] = &[];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "InputStruct",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<InputStruct>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_InputStruct: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for InputStruct {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "InputStruct",
                    false as usize,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for InputStruct {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                InputStruct {} => {
                    let mut debug_trait_builder = f.debug_struct("InputStruct");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl<'a> From<&'a InputStruct> for JsonString {
        fn from(v: &InputStruct) -> JsonString {
            match ::serde_json::to_string(v) {
                Ok(s) => Ok(JsonString::from(s)),
                Err(e) => {
                    {
                        $crate::io::_eprint($crate::fmt::Arguments::new_v1(
                            &["Error serializing to JSON: ", "\n"],
                            &match (&e,) {
                                (arg0,) => [$crate::fmt::ArgumentV1::new(
                                    arg0,
                                    $crate::fmt::Debug::fmt,
                                )],
                            },
                        ));
                    };
                    Err(HolochainError::SerializationError(e.to_string()))
                }
            }
            .expect(&$crate::fmt::format($crate::fmt::Arguments::new_v1(
                &["could not Jsonify ", ": "],
                &match (&"InputStruct", &v) {
                    (arg0, arg1) => [
                        $crate::fmt::ArgumentV1::new(arg0, $crate::fmt::Display::fmt),
                        $crate::fmt::ArgumentV1::new(arg1, $crate::fmt::Debug::fmt),
                    ],
                },
            )))
        }
    }
    impl From<InputStruct> for JsonString {
        fn from(v: InputStruct) -> JsonString {
            JsonString::from(&v)
        }
    }
    impl<'a> ::std::convert::TryFrom<&'a JsonString> for InputStruct {
        type Error = HolochainError;
        fn try_from(json_string: &JsonString) -> Result<Self, Self::Error> {
            match ::serde_json::from_str(&String::from(json_string)) {
                Ok(d) => Ok(d),
                Err(e) => Err(HolochainError::SerializationError(e.to_string())),
            }
        }
    }
    impl ::std::convert::TryFrom<JsonString> for InputStruct {
        type Error = HolochainError;
        fn try_from(json_string: JsonString) -> Result<Self, Self::Error> {
            InputStruct::try_from(&json_string)
        }
    }
    let input: InputStruct = {
        let maybe_input =
            $crate::holochain_wasm_utils::memory::ribosome::load_ribosome_encoded_json(
                encoded_allocation_of_input,
            );
        match maybe_input { Ok ( input ) => input , Err ( hc_err ) => return $crate :: holochain_wasm_utils :: memory :: ribosome :: return_code_for_allocation_result ( $crate :: global_fns :: write_json ( hc_err ) ) . into ( ) , }
    };
    fn execute(params: InputStruct) -> ZomeApiResult<member::Profile> {
        let InputStruct {} = params;
        member::handlers::handle_get_my_member_profile()
    }
    $crate::holochain_wasm_utils::memory::ribosome::return_code_for_allocation_result(
        $crate::global_fns::write_json(execute(input)),
    )
    .into()
}
#[no_mangle]
pub extern "C" fn post_message(
    encoded_allocation_of_input: hdk::holochain_core_types::error::RibosomeEncodingBits,
) -> hdk::holochain_core_types::error::RibosomeEncodingBits {
    let maybe_allocation = $crate :: holochain_wasm_utils :: memory :: allocation :: WasmAllocation :: try_from_ribosome_encoding ( encoded_allocation_of_input ) ;
    let allocation = match maybe_allocation {
        Ok(allocation) => allocation,
        Err(allocation_error) => {
            return hdk::holochain_core_types::error::RibosomeEncodedValue::from(allocation_error)
                .into();
        }
    };
    let init = $crate::global_fns::init_global_memory(allocation);
    if init.is_err() {
        return $crate::holochain_wasm_utils::memory::ribosome::return_code_for_allocation_result(
            init,
        )
        .into();
    }
    struct InputStruct {
        stream_address: HashString,
        message: message::MessageSpec,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_InputStruct: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for InputStruct {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "stream_address" => _serde::export::Ok(__Field::__field0),
                            "message" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"stream_address" => _serde::export::Ok(__Field::__field0),
                            b"message" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<InputStruct>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = InputStruct;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct InputStruct")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<HashString>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct InputStruct with 2 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            message::MessageSpec,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct InputStruct with 2 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(InputStruct {
                            stream_address: __field0,
                            message: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<HashString> = _serde::export::None;
                        let mut __field1: _serde::export::Option<message::MessageSpec> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "stream_address",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<HashString>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "message",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            message::MessageSpec,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("stream_address") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("message") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(InputStruct {
                            stream_address: __field0,
                            message: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["stream_address", "message"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "InputStruct",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<InputStruct>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_InputStruct: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for InputStruct {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "InputStruct",
                    false as usize + 1 + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "stream_address",
                    &self.stream_address,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "message",
                    &self.message,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for InputStruct {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                InputStruct {
                    stream_address: ref __self_0_0,
                    message: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("InputStruct");
                    let _ = debug_trait_builder.field("stream_address", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("message", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl<'a> From<&'a InputStruct> for JsonString {
        fn from(v: &InputStruct) -> JsonString {
            match ::serde_json::to_string(v) {
                Ok(s) => Ok(JsonString::from(s)),
                Err(e) => {
                    {
                        $crate::io::_eprint($crate::fmt::Arguments::new_v1(
                            &["Error serializing to JSON: ", "\n"],
                            &match (&e,) {
                                (arg0,) => [$crate::fmt::ArgumentV1::new(
                                    arg0,
                                    $crate::fmt::Debug::fmt,
                                )],
                            },
                        ));
                    };
                    Err(HolochainError::SerializationError(e.to_string()))
                }
            }
            .expect(&$crate::fmt::format($crate::fmt::Arguments::new_v1(
                &["could not Jsonify ", ": "],
                &match (&"InputStruct", &v) {
                    (arg0, arg1) => [
                        $crate::fmt::ArgumentV1::new(arg0, $crate::fmt::Display::fmt),
                        $crate::fmt::ArgumentV1::new(arg1, $crate::fmt::Debug::fmt),
                    ],
                },
            )))
        }
    }
    impl From<InputStruct> for JsonString {
        fn from(v: InputStruct) -> JsonString {
            JsonString::from(&v)
        }
    }
    impl<'a> ::std::convert::TryFrom<&'a JsonString> for InputStruct {
        type Error = HolochainError;
        fn try_from(json_string: &JsonString) -> Result<Self, Self::Error> {
            match ::serde_json::from_str(&String::from(json_string)) {
                Ok(d) => Ok(d),
                Err(e) => Err(HolochainError::SerializationError(e.to_string())),
            }
        }
    }
    impl ::std::convert::TryFrom<JsonString> for InputStruct {
        type Error = HolochainError;
        fn try_from(json_string: JsonString) -> Result<Self, Self::Error> {
            InputStruct::try_from(&json_string)
        }
    }
    let input: InputStruct = {
        let maybe_input =
            $crate::holochain_wasm_utils::memory::ribosome::load_ribosome_encoded_json(
                encoded_allocation_of_input,
            );
        match maybe_input { Ok ( input ) => input , Err ( hc_err ) => return $crate :: holochain_wasm_utils :: memory :: ribosome :: return_code_for_allocation_result ( $crate :: global_fns :: write_json ( hc_err ) ) . into ( ) , }
    };
    fn execute(params: InputStruct) -> ZomeApiResult<()> {
        let InputStruct {
            stream_address,
            message,
        } = params;
        stream::handlers::handle_post_message(stream_address, message)
    }
    $crate::holochain_wasm_utils::memory::ribosome::return_code_for_allocation_result(
        $crate::global_fns::write_json(execute(input)),
    )
    .into()
}
#[no_mangle]
pub extern "C" fn get_messages(
    encoded_allocation_of_input: hdk::holochain_core_types::error::RibosomeEncodingBits,
) -> hdk::holochain_core_types::error::RibosomeEncodingBits {
    let maybe_allocation = $crate :: holochain_wasm_utils :: memory :: allocation :: WasmAllocation :: try_from_ribosome_encoding ( encoded_allocation_of_input ) ;
    let allocation = match maybe_allocation {
        Ok(allocation) => allocation,
        Err(allocation_error) => {
            return hdk::holochain_core_types::error::RibosomeEncodedValue::from(allocation_error)
                .into();
        }
    };
    let init = $crate::global_fns::init_global_memory(allocation);
    if init.is_err() {
        return $crate::holochain_wasm_utils::memory::ribosome::return_code_for_allocation_result(
            init,
        )
        .into();
    }
    struct InputStruct {
        address: HashString,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_InputStruct: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for InputStruct {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "address" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"address" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<InputStruct>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = InputStruct;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct InputStruct")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<HashString>(
                            &mut __seq,
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct InputStruct with 1 element",
                                ));
                            }
                        };
                        _serde::export::Ok(InputStruct { address: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<HashString> = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "address",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<HashString>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("address") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(InputStruct { address: __field0 })
                    }
                }
                const FIELDS: &'static [&'static str] = &["address"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "InputStruct",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<InputStruct>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_InputStruct: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for InputStruct {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "InputStruct",
                    false as usize + 1,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "address",
                    &self.address,
                ) {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl $crate::fmt::Debug for InputStruct {
        fn fmt(&self, f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
            match *self {
                InputStruct {
                    address: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("InputStruct");
                    let _ = debug_trait_builder.field("address", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl<'a> From<&'a InputStruct> for JsonString {
        fn from(v: &InputStruct) -> JsonString {
            match ::serde_json::to_string(v) {
                Ok(s) => Ok(JsonString::from(s)),
                Err(e) => {
                    {
                        $crate::io::_eprint($crate::fmt::Arguments::new_v1(
                            &["Error serializing to JSON: ", "\n"],
                            &match (&e,) {
                                (arg0,) => [$crate::fmt::ArgumentV1::new(
                                    arg0,
                                    $crate::fmt::Debug::fmt,
                                )],
                            },
                        ));
                    };
                    Err(HolochainError::SerializationError(e.to_string()))
                }
            }
            .expect(&$crate::fmt::format($crate::fmt::Arguments::new_v1(
                &["could not Jsonify ", ": "],
                &match (&"InputStruct", &v) {
                    (arg0, arg1) => [
                        $crate::fmt::ArgumentV1::new(arg0, $crate::fmt::Display::fmt),
                        $crate::fmt::ArgumentV1::new(arg1, $crate::fmt::Debug::fmt),
                    ],
                },
            )))
        }
    }
    impl From<InputStruct> for JsonString {
        fn from(v: InputStruct) -> JsonString {
            JsonString::from(&v)
        }
    }
    impl<'a> ::std::convert::TryFrom<&'a JsonString> for InputStruct {
        type Error = HolochainError;
        fn try_from(json_string: &JsonString) -> Result<Self, Self::Error> {
            match ::serde_json::from_str(&String::from(json_string)) {
                Ok(d) => Ok(d),
                Err(e) => Err(HolochainError::SerializationError(e.to_string())),
            }
        }
    }
    impl ::std::convert::TryFrom<JsonString> for InputStruct {
        type Error = HolochainError;
        fn try_from(json_string: JsonString) -> Result<Self, Self::Error> {
            InputStruct::try_from(&json_string)
        }
    }
    let input: InputStruct = {
        let maybe_input =
            $crate::holochain_wasm_utils::memory::ribosome::load_ribosome_encoded_json(
                encoded_allocation_of_input,
            );
        match maybe_input { Ok ( input ) => input , Err ( hc_err ) => return $crate :: holochain_wasm_utils :: memory :: ribosome :: return_code_for_allocation_result ( $crate :: global_fns :: write_json ( hc_err ) ) . into ( ) , }
    };
    fn execute(params: InputStruct) -> ZomeApiResult<utils::GetLinksLoadResult<message::Message>> {
        let InputStruct { address } = params;
        stream::handlers::handle_get_messages(address)
    }
    $crate::holochain_wasm_utils::memory::ribosome::return_code_for_allocation_result(
        $crate::global_fns::write_json(execute(input)),
    )
    .into()
}
