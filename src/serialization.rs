macro_rules! bitflags_serde {
    ( $name:ident { $($variant:ident, )* }) => {
        impl ::serde::Serialize for $name::Flags {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::Serializer,
            {
                use ::serde::ser::SerializeSeq;

                // Serialize the bitflags as a sequence of strings.
                let mut state = serializer.serialize_seq(None)?;

                $( if self.contains($name::$variant) {
                    state.serialize_element(stringify!($variant))?;
                } )*

                state.end()
            }
        }

        impl ::serde::Deserialize for $name::Flags {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer,
            {
                struct Visitor;

                impl ::serde::de::Visitor for Visitor {
                    type Value = $name::Flags;

                    fn expecting(&self,
                        formatter: &mut ::std::fmt::Formatter)
                        -> ::std::fmt::Result
                    {
                        formatter.write_str("a sequence of valid variants of $name")
                    }

                    fn visit_seq<V>(self, mut visitor: V) ->
                        Result<$name::Flags, V::Error>
                    where V: ::serde::de::SeqVisitor,
                    {
                        use ::serde::de::Error;

                        let mut flag = $name::Flags::empty();
                        while let Some(variant) = visitor.visit()? {
                            let variant: String = variant;
                            flag |= match &*variant {
                                $(
                                    stringify!($variant) => $name::$variant,
                                )*
                                x => return Err(V::Error::invalid_value(
                                    ::serde::de::Unexpected::Str(x), &self)
                                ),
                            };
                        }
                        Ok(flag)
                    }
                }

                // Deserialize the enum from a string.
                deserializer.deserialize_seq(Visitor)
            }
        }
    }
}

macro_rules! enum_serde {
    ( $name:ident { $($variant:ident, )* }) => {
        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::Serializer,
            {
                // Serialize the enum as a string.
                serializer.serialize_str(match *self {
                    $( $name::$variant => stringify!($variant), )*
                })
            }
        }

        impl ::serde::Deserialize for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer,
            {
                struct Visitor;

                impl ::serde::de::Visitor for Visitor {
                    type Value = $name;

                    fn expecting(&self,
                        formatter: &mut ::std::fmt::Formatter)
                        -> ::std::fmt::Result
                    {
                        formatter.write_str("a valid variant for $name")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<$name, E>
                        where E: ::serde::de::Error,
                    {
                        match value {
                            $( stringify!($variant) => Ok($name::$variant), )*
                            x => Err(E::invalid_value(
                                ::serde::de::Unexpected::Str(x),
                                &self)),
                        }
                    }
                }

                // Deserialize the enum from a string.
                deserializer.deserialize_str(Visitor)
            }
        }
    }
}
