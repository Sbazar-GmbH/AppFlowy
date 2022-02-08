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
//! Generated file from `share.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct ExportRequest {
    // message fields
    pub doc_id: ::std::string::String,
    pub export_type: ExportType,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ExportRequest {
    fn default() -> &'a ExportRequest {
        <ExportRequest as ::protobuf::Message>::default_instance()
    }
}

impl ExportRequest {
    pub fn new() -> ExportRequest {
        ::std::default::Default::default()
    }

    // string doc_id = 1;


    pub fn get_doc_id(&self) -> &str {
        &self.doc_id
    }
    pub fn clear_doc_id(&mut self) {
        self.doc_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_doc_id(&mut self, v: ::std::string::String) {
        self.doc_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_doc_id(&mut self) -> &mut ::std::string::String {
        &mut self.doc_id
    }

    // Take field
    pub fn take_doc_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.doc_id, ::std::string::String::new())
    }

    // .ExportType export_type = 2;


    pub fn get_export_type(&self) -> ExportType {
        self.export_type
    }
    pub fn clear_export_type(&mut self) {
        self.export_type = ExportType::Text;
    }

    // Param is passed by value, moved
    pub fn set_export_type(&mut self, v: ExportType) {
        self.export_type = v;
    }
}

impl ::protobuf::Message for ExportRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.doc_id)?;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.export_type, 2, &mut self.unknown_fields)?
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
        if !self.doc_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.doc_id);
        }
        if self.export_type != ExportType::Text {
            my_size += ::protobuf::rt::enum_size(2, self.export_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.doc_id.is_empty() {
            os.write_string(1, &self.doc_id)?;
        }
        if self.export_type != ExportType::Text {
            os.write_enum(2, ::protobuf::ProtobufEnum::value(&self.export_type))?;
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

    fn new() -> ExportRequest {
        ExportRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "doc_id",
                |m: &ExportRequest| { &m.doc_id },
                |m: &mut ExportRequest| { &mut m.doc_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ExportType>>(
                "export_type",
                |m: &ExportRequest| { &m.export_type },
                |m: &mut ExportRequest| { &mut m.export_type },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ExportRequest>(
                "ExportRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ExportRequest {
        static instance: ::protobuf::rt::LazyV2<ExportRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ExportRequest::new)
    }
}

impl ::protobuf::Clear for ExportRequest {
    fn clear(&mut self) {
        self.doc_id.clear();
        self.export_type = ExportType::Text;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExportRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExportRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ExportData {
    // message fields
    pub data: ::std::string::String,
    pub export_type: ExportType,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ExportData {
    fn default() -> &'a ExportData {
        <ExportData as ::protobuf::Message>::default_instance()
    }
}

impl ExportData {
    pub fn new() -> ExportData {
        ::std::default::Default::default()
    }

    // string data = 1;


    pub fn get_data(&self) -> &str {
        &self.data
    }
    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::string::String) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::string::String {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.data, ::std::string::String::new())
    }

    // .ExportType export_type = 2;


    pub fn get_export_type(&self) -> ExportType {
        self.export_type
    }
    pub fn clear_export_type(&mut self) {
        self.export_type = ExportType::Text;
    }

    // Param is passed by value, moved
    pub fn set_export_type(&mut self, v: ExportType) {
        self.export_type = v;
    }
}

impl ::protobuf::Message for ExportData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.data)?;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.export_type, 2, &mut self.unknown_fields)?
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
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.data);
        }
        if self.export_type != ExportType::Text {
            my_size += ::protobuf::rt::enum_size(2, self.export_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.data.is_empty() {
            os.write_string(1, &self.data)?;
        }
        if self.export_type != ExportType::Text {
            os.write_enum(2, ::protobuf::ProtobufEnum::value(&self.export_type))?;
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

    fn new() -> ExportData {
        ExportData::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "data",
                |m: &ExportData| { &m.data },
                |m: &mut ExportData| { &mut m.data },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ExportType>>(
                "export_type",
                |m: &ExportData| { &m.export_type },
                |m: &mut ExportData| { &mut m.export_type },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ExportData>(
                "ExportData",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ExportData {
        static instance: ::protobuf::rt::LazyV2<ExportData> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ExportData::new)
    }
}

impl ::protobuf::Clear for ExportData {
    fn clear(&mut self) {
        self.data.clear();
        self.export_type = ExportType::Text;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExportData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExportData {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ExportType {
    Text = 0,
    Markdown = 1,
    Link = 2,
}

impl ::protobuf::ProtobufEnum for ExportType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ExportType> {
        match value {
            0 => ::std::option::Option::Some(ExportType::Text),
            1 => ::std::option::Option::Some(ExportType::Markdown),
            2 => ::std::option::Option::Some(ExportType::Link),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ExportType] = &[
            ExportType::Text,
            ExportType::Markdown,
            ExportType::Link,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<ExportType>("ExportType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for ExportType {
}

impl ::std::default::Default for ExportType {
    fn default() -> Self {
        ExportType::Text
    }
}

impl ::protobuf::reflect::ProtobufValue for ExportType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bshare.proto\"T\n\rExportRequest\x12\x15\n\x06doc_id\x18\x01\x20\
    \x01(\tR\x05docId\x12,\n\x0bexport_type\x18\x02\x20\x01(\x0e2\x0b.Export\
    TypeR\nexportType\"N\n\nExportData\x12\x12\n\x04data\x18\x01\x20\x01(\tR\
    \x04data\x12,\n\x0bexport_type\x18\x02\x20\x01(\x0e2\x0b.ExportTypeR\nex\
    portType*.\n\nExportType\x12\x08\n\x04Text\x10\0\x12\x0c\n\x08Markdown\
    \x10\x01\x12\x08\n\x04Link\x10\x02b\x06proto3\
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
