use crate::{
    casing::Casing,
    types::{
        Enum, EnumOptions, Field, Struct, StructOptions, TypeIdent, TypeMap, Variant, VariantAttrs,
    },
    Type,
};
use fp_bindgen_support::common::errors::FPGuestError;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    rc::Rc,
};

#[cfg(feature = "http-compat")]
mod http;
#[cfg(feature = "serde-bytes-compat")]
mod serde_bytes;
#[cfg(feature = "time-compat")]
mod time;

pub trait Serializable: 'static {
    /// The identifier of the type as defined in the protocol.
    fn ident() -> TypeIdent;

    /// The type definition.
    fn ty() -> Type;

    /// Whether this type is a primitive.
    fn is_primitive() -> bool {
        false
    }

    /// Collects the `Type` of this type and its dependencies.
    ///
    /// The default implementation is only suitable for types without
    /// dependencies.
    fn collect_types(types: &mut TypeMap) {
        types.entry(Self::ident()).or_insert_with(Self::ty);
    }
}

impl<T> Serializable for Box<T>
where
    T: Serializable,
{
    fn ident() -> TypeIdent {
        TypeIdent {
            name: "Box".to_owned(),
            generic_args: vec![TypeIdent::from("T")],
        }
    }

    fn ty() -> Type {
        Type::Container("Box".to_owned(), TypeIdent::from("T"))
    }

    fn collect_types(types: &mut TypeMap) {
        types.entry(Self::ident()).or_insert_with(Self::ty);
        T::collect_types(types);
    }
}

impl<K, V> Serializable for BTreeMap<K, V>
where
    K: Serializable,
    V: Serializable,
{
    fn ident() -> TypeIdent {
        TypeIdent {
            name: "BTreeMap".to_owned(),
            generic_args: vec![TypeIdent::from("K"), TypeIdent::from("V")],
        }
    }

    fn ty() -> Type {
        Type::Map(
            "BTreeMap".to_owned(),
            TypeIdent::from("K"),
            TypeIdent::from("V"),
        )
    }

    fn collect_types(types: &mut TypeMap) {
        types.entry(Self::ident()).or_insert_with(Self::ty);
        K::collect_types(types);
        V::collect_types(types);
    }
}

impl<T> Serializable for BTreeSet<T>
where
    T: Serializable,
{
    fn ident() -> TypeIdent {
        TypeIdent {
            name: "BTreeSet".to_owned(),
            generic_args: vec![TypeIdent::from("T")],
        }
    }

    fn ty() -> Type {
        Type::List("BTreeSet".to_owned(), TypeIdent::from("T"))
    }

    fn collect_types(types: &mut TypeMap) {
        types.entry(Self::ident()).or_insert_with(Self::ty);
        T::collect_types(types);
    }
}

impl<K, V> Serializable for HashMap<K, V>
where
    K: Serializable,
    V: Serializable,
{
    fn ident() -> TypeIdent {
        TypeIdent {
            name: "HashMap".to_owned(),
            generic_args: vec![TypeIdent::from("K"), TypeIdent::from("V")],
        }
    }

    fn ty() -> Type {
        Type::Map(
            "HashMap".to_owned(),
            TypeIdent::from("K"),
            TypeIdent::from("V"),
        )
    }

    fn collect_types(types: &mut TypeMap) {
        types.entry(Self::ident()).or_insert_with(Self::ty);
        K::collect_types(types);
        V::collect_types(types);
    }
}

impl<T> Serializable for HashSet<T>
where
    T: Serializable,
{
    fn ident() -> TypeIdent {
        TypeIdent {
            name: "HashSet".to_owned(),
            generic_args: vec![TypeIdent::from("T")],
        }
    }

    fn ty() -> Type {
        Type::List("HashSet".to_owned(), TypeIdent::from("T"))
    }

    fn collect_types(types: &mut TypeMap) {
        types.entry(Self::ident()).or_insert_with(Self::ty);
        T::collect_types(types);
    }
}

impl<T> Serializable for Option<T>
where
    T: Serializable,
{
    fn ident() -> TypeIdent {
        TypeIdent {
            name: "Option".to_owned(),
            generic_args: vec![TypeIdent::from("T")],
        }
    }

    fn ty() -> Type {
        Type::Container("Option".to_owned(), TypeIdent::from("T"))
    }

    fn collect_types(types: &mut TypeMap) {
        types.entry(Self::ident()).or_insert_with(Self::ty);
        T::collect_types(types);
    }
}

impl<T> Serializable for Rc<T>
where
    T: Serializable,
{
    fn ident() -> TypeIdent {
        TypeIdent {
            name: "Rc".to_owned(),
            generic_args: vec![TypeIdent::from("T")],
        }
    }

    fn ty() -> Type {
        Type::Container("Rc".to_owned(), TypeIdent::from("T"))
    }

    fn collect_types(types: &mut TypeMap) {
        types.entry(Self::ident()).or_insert_with(Self::ty);
        T::collect_types(types);
    }
}

impl<T, E> Serializable for Result<T, E>
where
    T: Serializable,
    E: Serializable,
{
    fn ident() -> TypeIdent {
        TypeIdent {
            name: "Result".to_owned(),
            generic_args: vec![TypeIdent::from("T"), TypeIdent::from("E")],
        }
    }

    fn ty() -> Type {
        Type::Enum(Enum {
            ident: Self::ident(),
            variants: vec![
                Variant {
                    name: "Ok".to_owned(),
                    ty: Type::Tuple(vec![TypeIdent::from("T")]),
                    doc_lines: vec![" Represents a succesful result.".to_owned()],
                    attrs: VariantAttrs::default(),
                },
                Variant {
                    name: "Err".to_owned(),
                    ty: Type::Tuple(vec![TypeIdent::from("E")]),
                    doc_lines: vec![" Represents an error.".to_owned()],
                    attrs: VariantAttrs::default(),
                },
            ],
            doc_lines: vec![
                " A result that can be either successful (`Ok)` or represent an error (`Err`)."
                    .to_owned(),
            ],
            options: EnumOptions::default(),
        })
    }

    fn collect_types(types: &mut TypeMap) {
        types.entry(Self::ident()).or_insert_with(Self::ty);
        T::collect_types(types);
        E::collect_types(types);
    }
}

impl Serializable for String {
    fn ident() -> TypeIdent {
        TypeIdent::from("String")
    }

    fn ty() -> Type {
        Type::String
    }
}

impl<T> Serializable for Vec<T>
where
    T: Serializable,
{
    fn ident() -> TypeIdent {
        TypeIdent {
            name: "Vec".to_owned(),
            generic_args: vec![TypeIdent::from("T")],
        }
    }

    fn ty() -> Type {
        Type::List("Vec".to_owned(), TypeIdent::from("T"))
    }

    fn collect_types(types: &mut TypeMap) {
        types.entry(Self::ident()).or_insert_with(Self::ty);
        T::collect_types(types);
    }
}

impl Serializable for FPGuestError {
    fn ident() -> TypeIdent {
        TypeIdent {
            name: "FPGuestError".to_owned(),
            generic_args: vec![],
        }
    }

    fn ty() -> Type {
        Type::Enum(Enum {
            ident: Self::ident(),
            variants: vec![
                Variant {
                    name: "SerdeError".to_owned(),
                    ty: Type::Struct(Struct {
                        ident: TypeIdent::from("SerdeError"),
                        fields: vec![
                            Field {
                                name: "path".to_owned(),
                                ty: String::ident(),
                                doc_lines: vec!["Path to the failed field that failed to serde".to_owned()],
                                attrs: Default::default(),
                            },
                            Field {
                                name: "message".to_owned(),
                                ty: String::ident(),
                                doc_lines: vec![],
                                attrs: Default::default(),
                            }
                        ],
                        doc_lines: Default::default(),
                        options: StructOptions {
                            field_casing: Casing::SnakeCase,
                            ..Default::default()
                        },
                    }),
                    doc_lines: vec!["Deserialization of data failed, possible mismatch between guest and runtime protocol version".to_owned()],
                    attrs: Default::default(),
                },
                Variant {
                    name: "InvalidFatPtr".to_owned(),
                    ty: Type::Unit,
                    doc_lines: vec!["Received an invalid `FatPtr`".to_owned()],
                    attrs: Default::default(),
                }
            ],
            doc_lines: Default::default(),
            options: EnumOptions {
                variant_casing: Casing::SnakeCase,
                content_prop_name: None,
                tag_prop_name: Some("type".to_owned()),
                untagged: false,
                native_modules: BTreeMap::from([
                    ("rust_wasmer_runtime".to_owned(), "fp_bindgen_support::common::errors".to_owned()),
                    ("rust_plugin".to_owned(), "fp_bindgen_support::common::errors".to_owned())
                ])
            },
        })
    }
}
