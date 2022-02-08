// This file is generated by rust-protobuf 2.25.2. Do not edit
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
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `errors.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct WSError {
    // message fields
    pub code: ErrorCode,
    pub msg: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a WSError {
    fn default() -> &'a WSError {
        <WSError as ::protobuf::Message>::default_instance()
    }
}

impl WSError {
    pub fn new() -> WSError {
        ::std::default::Default::default()
    }

    // .ErrorCode code = 1;


    pub fn get_code(&self) -> ErrorCode {
        self.code
    }
    pub fn clear_code(&mut self) {
        self.code = ErrorCode::InternalError;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: ErrorCode) {
        self.code = v;
    }

    // string msg = 2;


    pub fn get_msg(&self) -> &str {
        &self.msg
    }
    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }
}

impl ::protobuf::Message for WSError {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.code, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.code != ErrorCode::InternalError {
            my_size += ::protobuf::rt::enum_size(1, self.code);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.code != ErrorCode::InternalError {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&self.code))?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> WSError {
        WSError::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ErrorCode>>(
                "code",
                |m: &WSError| { &m.code },
                |m: &mut WSError| { &mut m.code },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "msg",
                |m: &WSError| { &m.msg },
                |m: &mut WSError| { &mut m.msg },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<WSError>(
                "WSError",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static WSError {
        static instance: ::protobuf::rt::LazyV2<WSError> = ::protobuf::rt::LazyV2::INIT;
        instance.get(WSError::new)
    }
}

impl ::protobuf::Clear for WSError {
    fn clear(&mut self) {
        self.code = ErrorCode::InternalError;
        self.msg.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WSError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WSError {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ErrorCode {
    InternalError = 0,
    UnsupportedMessage = 1,
    Unauthorized = 2,
}

impl ::protobuf::ProtobufEnum for ErrorCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ErrorCode> {
        match value {
            0 => ::std::option::Option::Some(ErrorCode::InternalError),
            1 => ::std::option::Option::Some(ErrorCode::UnsupportedMessage),
            2 => ::std::option::Option::Some(ErrorCode::Unauthorized),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ErrorCode] = &[
            ErrorCode::InternalError,
            ErrorCode::UnsupportedMessage,
            ErrorCode::Unauthorized,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<ErrorCode>("ErrorCode", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for ErrorCode {
}

impl ::std::default::Default for ErrorCode {
    fn default() -> Self {
        ErrorCode::InternalError
    }
}

impl ::protobuf::reflect::ProtobufValue for ErrorCode {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cerrors.proto\";\n\x07WSError\x12\x1e\n\x04code\x18\x01\x20\x01(\
    \x0e2\n.ErrorCodeR\x04code\x12\x10\n\x03msg\x18\x02\x20\x01(\tR\x03msg*H\
    \n\tErrorCode\x12\x11\n\rInternalError\x10\0\x12\x16\n\x12UnsupportedMes\
    sage\x10\x01\x12\x10\n\x0cUnauthorized\x10\x02b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
