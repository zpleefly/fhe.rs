// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `rq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:fhers.Rq)
pub struct Rq {
    // message fields
    // @@protoc_insertion_point(field:fhers.Rq.representation)
    pub representation: ::protobuf::EnumOrUnknown<rq::Representation>,
    // @@protoc_insertion_point(field:fhers.Rq.degree)
    pub degree: u32,
    // @@protoc_insertion_point(field:fhers.Rq.coefficients)
    pub coefficients: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:fhers.Rq.allow_variable_time)
    pub allow_variable_time: bool,
    // special fields
    // @@protoc_insertion_point(special_field:fhers.Rq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Rq {
    fn default() -> &'a Rq {
        <Rq as ::protobuf::Message>::default_instance()
    }
}

impl Rq {
    pub fn new() -> Rq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "representation",
            |m: &Rq| { &m.representation },
            |m: &mut Rq| { &mut m.representation },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "degree",
            |m: &Rq| { &m.degree },
            |m: &mut Rq| { &mut m.degree },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "coefficients",
            |m: &Rq| { &m.coefficients },
            |m: &mut Rq| { &mut m.coefficients },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "allow_variable_time",
            |m: &Rq| { &m.allow_variable_time },
            |m: &mut Rq| { &mut m.allow_variable_time },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Rq>(
            "Rq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Rq {
    const NAME: &'static str = "Rq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.representation = is.read_enum_or_unknown()?;
                },
                16 => {
                    self.degree = is.read_uint32()?;
                },
                26 => {
                    self.coefficients = is.read_bytes()?;
                },
                32 => {
                    self.allow_variable_time = is.read_bool()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.representation != ::protobuf::EnumOrUnknown::new(rq::Representation::UNKNOWN) {
            my_size += ::protobuf::rt::int32_size(1, self.representation.value());
        }
        if self.degree != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.degree);
        }
        if !self.coefficients.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.coefficients);
        }
        if self.allow_variable_time != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.representation != ::protobuf::EnumOrUnknown::new(rq::Representation::UNKNOWN) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.representation))?;
        }
        if self.degree != 0 {
            os.write_uint32(2, self.degree)?;
        }
        if !self.coefficients.is_empty() {
            os.write_bytes(3, &self.coefficients)?;
        }
        if self.allow_variable_time != false {
            os.write_bool(4, self.allow_variable_time)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Rq {
        Rq::new()
    }

    fn clear(&mut self) {
        self.representation = ::protobuf::EnumOrUnknown::new(rq::Representation::UNKNOWN);
        self.degree = 0;
        self.coefficients.clear();
        self.allow_variable_time = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Rq {
        static instance: Rq = Rq {
            representation: ::protobuf::EnumOrUnknown::from_i32(0),
            degree: 0,
            coefficients: ::std::vec::Vec::new(),
            allow_variable_time: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Rq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Rq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Rq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Rq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `Rq`
pub mod rq {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:fhers.Rq.Representation)
    pub enum Representation {
        // @@protoc_insertion_point(enum_value:fhers.Rq.Representation.UNKNOWN)
        UNKNOWN = 0,
        // @@protoc_insertion_point(enum_value:fhers.Rq.Representation.POWERBASIS)
        POWERBASIS = 1,
        // @@protoc_insertion_point(enum_value:fhers.Rq.Representation.NTT)
        NTT = 2,
        // @@protoc_insertion_point(enum_value:fhers.Rq.Representation.NTTSHOUP)
        NTTSHOUP = 3,
    }

    impl ::protobuf::Enum for Representation {
        const NAME: &'static str = "Representation";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<Representation> {
            match value {
                0 => ::std::option::Option::Some(Representation::UNKNOWN),
                1 => ::std::option::Option::Some(Representation::POWERBASIS),
                2 => ::std::option::Option::Some(Representation::NTT),
                3 => ::std::option::Option::Some(Representation::NTTSHOUP),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [Representation] = &[
            Representation::UNKNOWN,
            Representation::POWERBASIS,
            Representation::NTT,
            Representation::NTTSHOUP,
        ];
    }

    impl ::protobuf::EnumFull for Representation {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("Rq.Representation").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for Representation {
        fn default() -> Self {
            Representation::UNKNOWN
        }
    }

    impl Representation {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Representation>("Rq.Representation")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x08rq.proto\x12\x05fhers\"\xf8\x01\n\x02Rq\x12@\n\x0erepresentation\
    \x18\x01\x20\x01(\x0e2\x18.fhers.Rq.RepresentationR\x0erepresentation\
    \x12\x16\n\x06degree\x18\x02\x20\x01(\rR\x06degree\x12\"\n\x0ccoefficien\
    ts\x18\x03\x20\x01(\x0cR\x0ccoefficients\x12.\n\x13allow_variable_time\
    \x18\x04\x20\x01(\x08R\x11allowVariableTime\"D\n\x0eRepresentation\x12\
    \x0b\n\x07UNKNOWN\x10\0\x12\x0e\n\nPOWERBASIS\x10\x01\x12\x07\n\x03NTT\
    \x10\x02\x12\x0c\n\x08NTTSHOUP\x10\x03J\xe8\x03\n\x06\x12\x04\0\0\x0f\
    \x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\0\x0e\n\
    \n\n\x02\x04\0\x12\x04\x04\0\x0f\x01\n\n\n\x03\x04\0\x01\x12\x03\x04\x08\
    \n\n\x0c\n\x04\x04\0\x04\0\x12\x04\x05\x04\n\x05\n\x0c\n\x05\x04\0\x04\0\
    \x01\x12\x03\x05\t\x17\n\r\n\x06\x04\0\x04\0\x02\0\x12\x03\x06\x08\x14\n\
    \x0e\n\x07\x04\0\x04\0\x02\0\x01\x12\x03\x06\x08\x0f\n\x0e\n\x07\x04\0\
    \x04\0\x02\0\x02\x12\x03\x06\x12\x13\n\r\n\x06\x04\0\x04\0\x02\x01\x12\
    \x03\x07\x08\x17\n\x0e\n\x07\x04\0\x04\0\x02\x01\x01\x12\x03\x07\x08\x12\
    \n\x0e\n\x07\x04\0\x04\0\x02\x01\x02\x12\x03\x07\x15\x16\n\r\n\x06\x04\0\
    \x04\0\x02\x02\x12\x03\x08\x08\x10\n\x0e\n\x07\x04\0\x04\0\x02\x02\x01\
    \x12\x03\x08\x08\x0b\n\x0e\n\x07\x04\0\x04\0\x02\x02\x02\x12\x03\x08\x0e\
    \x0f\n\r\n\x06\x04\0\x04\0\x02\x03\x12\x03\t\x08\x15\n\x0e\n\x07\x04\0\
    \x04\0\x02\x03\x01\x12\x03\t\x08\x10\n\x0e\n\x07\x04\0\x04\0\x02\x03\x02\
    \x12\x03\t\x13\x14\n\x0b\n\x04\x04\0\x02\0\x12\x03\x0b\x04&\n\x0c\n\x05\
    \x04\0\x02\0\x06\x12\x03\x0b\x04\x12\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x0b\x13!\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0b$%\n\x0b\n\x04\x04\0\
    \x02\x01\x12\x03\x0c\x04\x16\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x0c\
    \x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0c\x0b\x11\n\x0c\n\x05\x04\
    \0\x02\x01\x03\x12\x03\x0c\x14\x15\n\x0b\n\x04\x04\0\x02\x02\x12\x03\r\
    \x04\x1b\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\r\x04\t\n\x0c\n\x05\x04\0\
    \x02\x02\x01\x12\x03\r\n\x16\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\r\x19\
    \x1a\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x0e\x04!\n\x0c\n\x05\x04\0\x02\
    \x03\x05\x12\x03\x0e\x04\x08\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x0e\t\
    \x1c\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x0e\x1f\x20b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Rq::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(rq::Representation::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
