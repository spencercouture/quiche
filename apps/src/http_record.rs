// This file is generated by rust-protobuf 3.3.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `http_record.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

// @@protoc_insertion_point(message:MahimahiProtobufs.HTTPMessage)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HTTPMessage {
    // message fields
    // @@protoc_insertion_point(field:MahimahiProtobufs.HTTPMessage.first_line)
    pub first_line: ::std::option::Option<::std::vec::Vec<u8>>,
    // @@protoc_insertion_point(field:MahimahiProtobufs.HTTPMessage.header)
    pub header: ::std::vec::Vec<HTTPHeader>,
    // @@protoc_insertion_point(field:MahimahiProtobufs.HTTPMessage.body)
    pub body: ::std::option::Option<::std::vec::Vec<u8>>,
    // special fields
    // @@protoc_insertion_point(special_field:MahimahiProtobufs.HTTPMessage.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HTTPMessage {
    fn default() -> &'a HTTPMessage {
        <HTTPMessage as ::protobuf::Message>::default_instance()
    }
}

impl HTTPMessage {
    pub fn new() -> HTTPMessage {
        ::std::default::Default::default()
    }

    // optional bytes first_line = 1;

    pub fn first_line(&self) -> &[u8] {
        match self.first_line.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_first_line(&mut self) {
        self.first_line = ::std::option::Option::None;
    }

    pub fn has_first_line(&self) -> bool {
        self.first_line.is_some()
    }

    // Param is passed by value, moved
    pub fn set_first_line(&mut self, v: ::std::vec::Vec<u8>) {
        self.first_line = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_first_line(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.first_line.is_none() {
            self.first_line = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.first_line.as_mut().unwrap()
    }

    // Take field
    pub fn take_first_line(&mut self) -> ::std::vec::Vec<u8> {
        self.first_line.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional bytes body = 3;

    pub fn body(&self) -> &[u8] {
        match self.body.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_body(&mut self) {
        self.body = ::std::option::Option::None;
    }

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: ::std::vec::Vec<u8>) {
        self.body = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.body.is_none() {
            self.body = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.body.as_mut().unwrap()
    }

    // Take field
    pub fn take_body(&mut self) -> ::std::vec::Vec<u8> {
        self.body.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "first_line",
            |m: &HTTPMessage| { &m.first_line },
            |m: &mut HTTPMessage| { &mut m.first_line },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "header",
            |m: &HTTPMessage| { &m.header },
            |m: &mut HTTPMessage| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "body",
            |m: &HTTPMessage| { &m.body },
            |m: &mut HTTPMessage| { &mut m.body },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HTTPMessage>(
            "HTTPMessage",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HTTPMessage {
    const NAME: &'static str = "HTTPMessage";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.first_line = ::std::option::Option::Some(is.read_bytes()?);
                },
                18 => {
                    self.header.push(is.read_message()?);
                },
                26 => {
                    self.body = ::std::option::Option::Some(is.read_bytes()?);
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
        if let Some(v) = self.first_line.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        for value in &self.header {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.body.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.first_line.as_ref() {
            os.write_bytes(1, v)?;
        }
        for v in &self.header {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if let Some(v) = self.body.as_ref() {
            os.write_bytes(3, v)?;
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

    fn new() -> HTTPMessage {
        HTTPMessage::new()
    }

    fn clear(&mut self) {
        self.first_line = ::std::option::Option::None;
        self.header.clear();
        self.body = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HTTPMessage {
        static instance: HTTPMessage = HTTPMessage {
            first_line: ::std::option::Option::None,
            header: ::std::vec::Vec::new(),
            body: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HTTPMessage {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HTTPMessage").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HTTPMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HTTPMessage {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:MahimahiProtobufs.HTTPHeader)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HTTPHeader {
    // message fields
    // @@protoc_insertion_point(field:MahimahiProtobufs.HTTPHeader.key)
    pub key: ::std::option::Option<::std::vec::Vec<u8>>,
    // @@protoc_insertion_point(field:MahimahiProtobufs.HTTPHeader.value)
    pub value: ::std::option::Option<::std::vec::Vec<u8>>,
    // special fields
    // @@protoc_insertion_point(special_field:MahimahiProtobufs.HTTPHeader.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HTTPHeader {
    fn default() -> &'a HTTPHeader {
        <HTTPHeader as ::protobuf::Message>::default_instance()
    }
}

impl HTTPHeader {
    pub fn new() -> HTTPHeader {
        ::std::default::Default::default()
    }

    // optional bytes key = 1;

    pub fn key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_key(&mut self) {
        self.key = ::std::option::Option::None;
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional bytes value = 2;

    pub fn value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "key",
            |m: &HTTPHeader| { &m.key },
            |m: &mut HTTPHeader| { &mut m.key },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "value",
            |m: &HTTPHeader| { &m.value },
            |m: &mut HTTPHeader| { &mut m.value },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HTTPHeader>(
            "HTTPHeader",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HTTPHeader {
    const NAME: &'static str = "HTTPHeader";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.key = ::std::option::Option::Some(is.read_bytes()?);
                },
                18 => {
                    self.value = ::std::option::Option::Some(is.read_bytes()?);
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
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(1, v)?;
        }
        if let Some(v) = self.value.as_ref() {
            os.write_bytes(2, v)?;
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

    fn new() -> HTTPHeader {
        HTTPHeader::new()
    }

    fn clear(&mut self) {
        self.key = ::std::option::Option::None;
        self.value = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HTTPHeader {
        static instance: HTTPHeader = HTTPHeader {
            key: ::std::option::Option::None,
            value: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HTTPHeader {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HTTPHeader").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HTTPHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HTTPHeader {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:MahimahiProtobufs.RequestResponse)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RequestResponse {
    // message fields
    // @@protoc_insertion_point(field:MahimahiProtobufs.RequestResponse.ip)
    pub ip: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:MahimahiProtobufs.RequestResponse.port)
    pub port: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:MahimahiProtobufs.RequestResponse.scheme)
    pub scheme: ::std::option::Option<::protobuf::EnumOrUnknown<request_response::Scheme>>,
    // @@protoc_insertion_point(field:MahimahiProtobufs.RequestResponse.request)
    pub request: ::protobuf::MessageField<HTTPMessage>,
    // @@protoc_insertion_point(field:MahimahiProtobufs.RequestResponse.response)
    pub response: ::protobuf::MessageField<HTTPMessage>,
    // special fields
    // @@protoc_insertion_point(special_field:MahimahiProtobufs.RequestResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RequestResponse {
    fn default() -> &'a RequestResponse {
        <RequestResponse as ::protobuf::Message>::default_instance()
    }
}

impl RequestResponse {
    pub fn new() -> RequestResponse {
        ::std::default::Default::default()
    }

    // optional string ip = 1;

    pub fn ip(&self) -> &str {
        match self.ip.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_ip(&mut self) {
        self.ip = ::std::option::Option::None;
    }

    pub fn has_ip(&self) -> bool {
        self.ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ip(&mut self, v: ::std::string::String) {
        self.ip = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ip(&mut self) -> &mut ::std::string::String {
        if self.ip.is_none() {
            self.ip = ::std::option::Option::Some(::std::string::String::new());
        }
        self.ip.as_mut().unwrap()
    }

    // Take field
    pub fn take_ip(&mut self) -> ::std::string::String {
        self.ip.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional uint32 port = 2;

    pub fn port(&self) -> u32 {
        self.port.unwrap_or(0)
    }

    pub fn clear_port(&mut self) {
        self.port = ::std::option::Option::None;
    }

    pub fn has_port(&self) -> bool {
        self.port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: u32) {
        self.port = ::std::option::Option::Some(v);
    }

    // optional .MahimahiProtobufs.RequestResponse.Scheme scheme = 3;

    pub fn scheme(&self) -> request_response::Scheme {
        match self.scheme {
            Some(e) => e.enum_value_or(request_response::Scheme::HTTP),
            None => request_response::Scheme::HTTP,
        }
    }

    pub fn clear_scheme(&mut self) {
        self.scheme = ::std::option::Option::None;
    }

    pub fn has_scheme(&self) -> bool {
        self.scheme.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scheme(&mut self, v: request_response::Scheme) {
        self.scheme = ::std::option::Option::Some(::protobuf::EnumOrUnknown::new(v));
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "ip",
            |m: &RequestResponse| { &m.ip },
            |m: &mut RequestResponse| { &mut m.ip },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "port",
            |m: &RequestResponse| { &m.port },
            |m: &mut RequestResponse| { &mut m.port },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "scheme",
            |m: &RequestResponse| { &m.scheme },
            |m: &mut RequestResponse| { &mut m.scheme },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, HTTPMessage>(
            "request",
            |m: &RequestResponse| { &m.request },
            |m: &mut RequestResponse| { &mut m.request },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, HTTPMessage>(
            "response",
            |m: &RequestResponse| { &m.response },
            |m: &mut RequestResponse| { &mut m.response },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RequestResponse>(
            "RequestResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RequestResponse {
    const NAME: &'static str = "RequestResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.ip = ::std::option::Option::Some(is.read_string()?);
                },
                16 => {
                    self.port = ::std::option::Option::Some(is.read_uint32()?);
                },
                24 => {
                    self.scheme = ::std::option::Option::Some(is.read_enum_or_unknown()?);
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.request)?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.response)?;
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
        if let Some(v) = self.ip.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.port {
            my_size += ::protobuf::rt::uint32_size(2, v);
        }
        if let Some(v) = self.scheme {
            my_size += ::protobuf::rt::int32_size(3, v.value());
        }
        if let Some(v) = self.request.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.response.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.ip.as_ref() {
            os.write_string(1, v)?;
        }
        if let Some(v) = self.port {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.scheme {
            os.write_enum(3, ::protobuf::EnumOrUnknown::value(&v))?;
        }
        if let Some(v) = self.request.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let Some(v) = self.response.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
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

    fn new() -> RequestResponse {
        RequestResponse::new()
    }

    fn clear(&mut self) {
        self.ip = ::std::option::Option::None;
        self.port = ::std::option::Option::None;
        self.scheme = ::std::option::Option::None;
        self.request.clear();
        self.response.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RequestResponse {
        static instance: RequestResponse = RequestResponse {
            ip: ::std::option::Option::None,
            port: ::std::option::Option::None,
            scheme: ::std::option::Option::None,
            request: ::protobuf::MessageField::none(),
            response: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RequestResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RequestResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RequestResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `RequestResponse`
pub mod request_response {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:MahimahiProtobufs.RequestResponse.Scheme)
    pub enum Scheme {
        // @@protoc_insertion_point(enum_value:MahimahiProtobufs.RequestResponse.Scheme.HTTP)
        HTTP = 1,
        // @@protoc_insertion_point(enum_value:MahimahiProtobufs.RequestResponse.Scheme.HTTPS)
        HTTPS = 2,
    }

    impl ::protobuf::Enum for Scheme {
        const NAME: &'static str = "Scheme";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<Scheme> {
            match value {
                1 => ::std::option::Option::Some(Scheme::HTTP),
                2 => ::std::option::Option::Some(Scheme::HTTPS),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<Scheme> {
            match str {
                "HTTP" => ::std::option::Option::Some(Scheme::HTTP),
                "HTTPS" => ::std::option::Option::Some(Scheme::HTTPS),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [Scheme] = &[
            Scheme::HTTP,
            Scheme::HTTPS,
        ];
    }

    impl ::protobuf::EnumFull for Scheme {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("RequestResponse.Scheme").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = match self {
                Scheme::HTTP => 0,
                Scheme::HTTPS => 1,
            };
            Self::enum_descriptor().value_by_index(index)
        }
    }

    // Note, `Default` is implemented although default value is not 0
    impl ::std::default::Default for Scheme {
        fn default() -> Self {
            Scheme::HTTP
        }
    }

    impl Scheme {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Scheme>("RequestResponse.Scheme")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11http_record.proto\x12\x11MahimahiProtobufs\"w\n\x0bHTTPMessage\x12\
    \x1d\n\nfirst_line\x18\x01\x20\x01(\x0cR\tfirstLine\x125\n\x06header\x18\
    \x02\x20\x03(\x0b2\x1d.MahimahiProtobufs.HTTPHeaderR\x06header\x12\x12\n\
    \x04body\x18\x03\x20\x01(\x0cR\x04body\"4\n\nHTTPHeader\x12\x10\n\x03key\
    \x18\x01\x20\x01(\x0cR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\x0cR\
    \x05value\"\x8d\x02\n\x0fRequestResponse\x12\x0e\n\x02ip\x18\x01\x20\x01\
    (\tR\x02ip\x12\x12\n\x04port\x18\x02\x20\x01(\rR\x04port\x12A\n\x06schem\
    e\x18\x03\x20\x01(\x0e2).MahimahiProtobufs.RequestResponse.SchemeR\x06sc\
    heme\x128\n\x07request\x18\x04\x20\x01(\x0b2\x1e.MahimahiProtobufs.HTTPM\
    essageR\x07request\x12:\n\x08response\x18\x05\x20\x01(\x0b2\x1e.Mahimahi\
    Protobufs.HTTPMessageR\x08response\"\x1d\n\x06Scheme\x12\x08\n\x04HTTP\
    \x10\x01\x12\t\n\x05HTTPS\x10\x02J\xd1\x07\n\x06\x12\x04\x02\0\x1c\x01\n\
    S\n\x01\x02\x12\x03\x02\x08\x192I\x20-*-mode:c++;\x20tab-width:\x204;\
    \x20indent-tabs-mode:\x20nil;\x20c-basic-offset:\x204\x20-*-\x20\n\n\n\
    \x02\x04\0\x12\x04\x04\0\x08\x01\n\n\n\x03\x04\0\x01\x12\x03\x04\x08\x13\
    \n\x0b\n\x04\x04\0\x02\0\x12\x03\x05\x04\"\n\x0c\n\x05\x04\0\x02\0\x04\
    \x12\x03\x05\x04\x0c\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x05\r\x12\n\x0c\
    \n\x05\x04\0\x02\0\x01\x12\x03\x05\x13\x1d\n\x0c\n\x05\x04\0\x02\0\x03\
    \x12\x03\x05\x20!\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x06\x04#\n\x0c\n\x05\
    \x04\0\x02\x01\x04\x12\x03\x06\x04\x0c\n\x0c\n\x05\x04\0\x02\x01\x06\x12\
    \x03\x06\r\x17\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x06\x18\x1e\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03\x06!\"\n\x0b\n\x04\x04\0\x02\x02\x12\x03\
    \x07\x04\x1c\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03\x07\x04\x0c\n\x0c\n\
    \x05\x04\0\x02\x02\x05\x12\x03\x07\r\x12\n\x0c\n\x05\x04\0\x02\x02\x01\
    \x12\x03\x07\x13\x17\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x07\x1a\x1b\n\
    \n\n\x02\x04\x01\x12\x04\n\0\r\x01\n\n\n\x03\x04\x01\x01\x12\x03\n\x08\
    \x12\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0b\x04\x1b\n\x0c\n\x05\x04\x01\
    \x02\0\x04\x12\x03\x0b\x04\x0c\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x0b\
    \r\x12\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x0b\x13\x16\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03\x0b\x19\x1a\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\
    \x0c\x04\x1d\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03\x0c\x04\x0c\n\x0c\n\
    \x05\x04\x01\x02\x01\x05\x12\x03\x0c\r\x12\n\x0c\n\x05\x04\x01\x02\x01\
    \x01\x12\x03\x0c\x13\x18\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x0c\x1b\
    \x1c\n\n\n\x02\x04\x02\x12\x04\x0f\0\x1c\x01\n\n\n\x03\x04\x02\x01\x12\
    \x03\x0f\x08\x17\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x10\x04\x1b\n\x0c\n\
    \x05\x04\x02\x02\0\x04\x12\x03\x10\x04\x0c\n\x0c\n\x05\x04\x02\x02\0\x05\
    \x12\x03\x10\r\x13\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x10\x14\x16\n\
    \x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x10\x19\x1a\n\x0b\n\x04\x04\x02\x02\
    \x01\x12\x03\x11\x04\x1d\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03\x11\x04\
    \x0c\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x11\r\x13\n\x0c\n\x05\x04\
    \x02\x02\x01\x01\x12\x03\x11\x14\x18\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\
    \x03\x11\x1b\x1c\n\x0c\n\x04\x04\x02\x04\0\x12\x04\x13\x04\x16\x05\n\x0c\
    \n\x05\x04\x02\x04\0\x01\x12\x03\x13\t\x0f\n\r\n\x06\x04\x02\x04\0\x02\0\
    \x12\x03\x14\x08\x11\n\x0e\n\x07\x04\x02\x04\0\x02\0\x01\x12\x03\x14\x08\
    \x0c\n\x0e\n\x07\x04\x02\x04\0\x02\0\x02\x12\x03\x14\x0f\x10\n\r\n\x06\
    \x04\x02\x04\0\x02\x01\x12\x03\x15\x08\x12\n\x0e\n\x07\x04\x02\x04\0\x02\
    \x01\x01\x12\x03\x15\x08\r\n\x0e\n\x07\x04\x02\x04\0\x02\x01\x02\x12\x03\
    \x15\x10\x11\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x18\x04\x1f\n\x0c\n\x05\
    \x04\x02\x02\x02\x04\x12\x03\x18\x04\x0c\n\x0c\n\x05\x04\x02\x02\x02\x06\
    \x12\x03\x18\r\x13\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03\x18\x14\x1a\n\
    \x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x18\x1d\x1e\n\x0b\n\x04\x04\x02\
    \x02\x03\x12\x03\x1a\x04%\n\x0c\n\x05\x04\x02\x02\x03\x04\x12\x03\x1a\
    \x04\x0c\n\x0c\n\x05\x04\x02\x02\x03\x06\x12\x03\x1a\r\x18\n\x0c\n\x05\
    \x04\x02\x02\x03\x01\x12\x03\x1a\x19\x20\n\x0c\n\x05\x04\x02\x02\x03\x03\
    \x12\x03\x1a#$\n\x0b\n\x04\x04\x02\x02\x04\x12\x03\x1b\x04&\n\x0c\n\x05\
    \x04\x02\x02\x04\x04\x12\x03\x1b\x04\x0c\n\x0c\n\x05\x04\x02\x02\x04\x06\
    \x12\x03\x1b\r\x18\n\x0c\n\x05\x04\x02\x02\x04\x01\x12\x03\x1b\x19!\n\
    \x0c\n\x05\x04\x02\x02\x04\x03\x12\x03\x1b$%\
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
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(HTTPMessage::generated_message_descriptor_data());
            messages.push(HTTPHeader::generated_message_descriptor_data());
            messages.push(RequestResponse::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(request_response::Scheme::generated_enum_descriptor_data());
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
