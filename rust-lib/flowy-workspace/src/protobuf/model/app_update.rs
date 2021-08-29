// This file is generated by rust-protobuf 2.22.1. Do not edit
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
//! Generated file from `app_update.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct UpdateAppRequest {
    // message fields
    pub app_id: ::std::string::String,
    // message oneof groups
    pub one_of_name: ::std::option::Option<UpdateAppRequest_oneof_one_of_name>,
    pub one_of_desc: ::std::option::Option<UpdateAppRequest_oneof_one_of_desc>,
    pub one_of_color_style: ::std::option::Option<UpdateAppRequest_oneof_one_of_color_style>,
    pub one_of_is_trash: ::std::option::Option<UpdateAppRequest_oneof_one_of_is_trash>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UpdateAppRequest {
    fn default() -> &'a UpdateAppRequest {
        <UpdateAppRequest as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum UpdateAppRequest_oneof_one_of_name {
    name(::std::string::String),
}

#[derive(Clone,PartialEq,Debug)]
pub enum UpdateAppRequest_oneof_one_of_desc {
    desc(::std::string::String),
}

#[derive(Clone,PartialEq,Debug)]
pub enum UpdateAppRequest_oneof_one_of_color_style {
    color_style(super::app_create::ColorStyle),
}

#[derive(Clone,PartialEq,Debug)]
pub enum UpdateAppRequest_oneof_one_of_is_trash {
    is_trash(bool),
}

impl UpdateAppRequest {
    pub fn new() -> UpdateAppRequest {
        ::std::default::Default::default()
    }

    // string app_id = 1;


    pub fn get_app_id(&self) -> &str {
        &self.app_id
    }
    pub fn clear_app_id(&mut self) {
        self.app_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_app_id(&mut self, v: ::std::string::String) {
        self.app_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_app_id(&mut self) -> &mut ::std::string::String {
        &mut self.app_id
    }

    // Take field
    pub fn take_app_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.app_id, ::std::string::String::new())
    }

    // string name = 2;


    pub fn get_name(&self) -> &str {
        match self.one_of_name {
            ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_name::name(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_name(&mut self) {
        self.one_of_name = ::std::option::Option::None;
    }

    pub fn has_name(&self) -> bool {
        match self.one_of_name {
            ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_name::name(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.one_of_name = ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_name::name(v))
    }

    // Mutable pointer to the field.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_name::name(_)) = self.one_of_name {
        } else {
            self.one_of_name = ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_name::name(::std::string::String::new()));
        }
        match self.one_of_name {
            ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_name::name(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        if self.has_name() {
            match self.one_of_name.take() {
                ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_name::name(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // string desc = 3;


    pub fn get_desc(&self) -> &str {
        match self.one_of_desc {
            ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_desc::desc(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_desc(&mut self) {
        self.one_of_desc = ::std::option::Option::None;
    }

    pub fn has_desc(&self) -> bool {
        match self.one_of_desc {
            ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_desc::desc(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_desc(&mut self, v: ::std::string::String) {
        self.one_of_desc = ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_desc::desc(v))
    }

    // Mutable pointer to the field.
    pub fn mut_desc(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_desc::desc(_)) = self.one_of_desc {
        } else {
            self.one_of_desc = ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_desc::desc(::std::string::String::new()));
        }
        match self.one_of_desc {
            ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_desc::desc(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_desc(&mut self) -> ::std::string::String {
        if self.has_desc() {
            match self.one_of_desc.take() {
                ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_desc::desc(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // .ColorStyle color_style = 4;


    pub fn get_color_style(&self) -> &super::app_create::ColorStyle {
        match self.one_of_color_style {
            ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_color_style::color_style(ref v)) => v,
            _ => <super::app_create::ColorStyle as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_color_style(&mut self) {
        self.one_of_color_style = ::std::option::Option::None;
    }

    pub fn has_color_style(&self) -> bool {
        match self.one_of_color_style {
            ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_color_style::color_style(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_color_style(&mut self, v: super::app_create::ColorStyle) {
        self.one_of_color_style = ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_color_style::color_style(v))
    }

    // Mutable pointer to the field.
    pub fn mut_color_style(&mut self) -> &mut super::app_create::ColorStyle {
        if let ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_color_style::color_style(_)) = self.one_of_color_style {
        } else {
            self.one_of_color_style = ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_color_style::color_style(super::app_create::ColorStyle::new()));
        }
        match self.one_of_color_style {
            ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_color_style::color_style(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_color_style(&mut self) -> super::app_create::ColorStyle {
        if self.has_color_style() {
            match self.one_of_color_style.take() {
                ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_color_style::color_style(v)) => v,
                _ => panic!(),
            }
        } else {
            super::app_create::ColorStyle::new()
        }
    }

    // bool is_trash = 5;


    pub fn get_is_trash(&self) -> bool {
        match self.one_of_is_trash {
            ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_is_trash::is_trash(v)) => v,
            _ => false,
        }
    }
    pub fn clear_is_trash(&mut self) {
        self.one_of_is_trash = ::std::option::Option::None;
    }

    pub fn has_is_trash(&self) -> bool {
        match self.one_of_is_trash {
            ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_is_trash::is_trash(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_is_trash(&mut self, v: bool) {
        self.one_of_is_trash = ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_is_trash::is_trash(v))
    }
}

impl ::protobuf::Message for UpdateAppRequest {
    fn is_initialized(&self) -> bool {
        if let Some(UpdateAppRequest_oneof_one_of_color_style::color_style(ref v)) = self.one_of_color_style {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.app_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_name = ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_name::name(is.read_string()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_desc = ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_desc::desc(is.read_string()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_color_style = ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_color_style::color_style(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_is_trash = ::std::option::Option::Some(UpdateAppRequest_oneof_one_of_is_trash::is_trash(is.read_bool()?));
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
        if !self.app_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.app_id);
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_name {
            match v {
                &UpdateAppRequest_oneof_one_of_name::name(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_desc {
            match v {
                &UpdateAppRequest_oneof_one_of_desc::desc(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_color_style {
            match v {
                &UpdateAppRequest_oneof_one_of_color_style::color_style(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_is_trash {
            match v {
                &UpdateAppRequest_oneof_one_of_is_trash::is_trash(v) => {
                    my_size += 2;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.app_id.is_empty() {
            os.write_string(1, &self.app_id)?;
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_name {
            match v {
                &UpdateAppRequest_oneof_one_of_name::name(ref v) => {
                    os.write_string(2, v)?;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_desc {
            match v {
                &UpdateAppRequest_oneof_one_of_desc::desc(ref v) => {
                    os.write_string(3, v)?;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_color_style {
            match v {
                &UpdateAppRequest_oneof_one_of_color_style::color_style(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_is_trash {
            match v {
                &UpdateAppRequest_oneof_one_of_is_trash::is_trash(v) => {
                    os.write_bool(5, v)?;
                },
            };
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

    fn new() -> UpdateAppRequest {
        UpdateAppRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "app_id",
                |m: &UpdateAppRequest| { &m.app_id },
                |m: &mut UpdateAppRequest| { &mut m.app_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "name",
                UpdateAppRequest::has_name,
                UpdateAppRequest::get_name,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "desc",
                UpdateAppRequest::has_desc,
                UpdateAppRequest::get_desc,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::app_create::ColorStyle>(
                "color_style",
                UpdateAppRequest::has_color_style,
                UpdateAppRequest::get_color_style,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                "is_trash",
                UpdateAppRequest::has_is_trash,
                UpdateAppRequest::get_is_trash,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<UpdateAppRequest>(
                "UpdateAppRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static UpdateAppRequest {
        static instance: ::protobuf::rt::LazyV2<UpdateAppRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(UpdateAppRequest::new)
    }
}

impl ::protobuf::Clear for UpdateAppRequest {
    fn clear(&mut self) {
        self.app_id.clear();
        self.one_of_name = ::std::option::Option::None;
        self.one_of_desc = ::std::option::Option::None;
        self.one_of_color_style = ::std::option::Option::None;
        self.one_of_is_trash = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UpdateAppRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateAppRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UpdateAppParams {
    // message fields
    pub app_id: ::std::string::String,
    // message oneof groups
    pub one_of_name: ::std::option::Option<UpdateAppParams_oneof_one_of_name>,
    pub one_of_desc: ::std::option::Option<UpdateAppParams_oneof_one_of_desc>,
    pub one_of_color_style: ::std::option::Option<UpdateAppParams_oneof_one_of_color_style>,
    pub one_of_is_trash: ::std::option::Option<UpdateAppParams_oneof_one_of_is_trash>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UpdateAppParams {
    fn default() -> &'a UpdateAppParams {
        <UpdateAppParams as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum UpdateAppParams_oneof_one_of_name {
    name(::std::string::String),
}

#[derive(Clone,PartialEq,Debug)]
pub enum UpdateAppParams_oneof_one_of_desc {
    desc(::std::string::String),
}

#[derive(Clone,PartialEq,Debug)]
pub enum UpdateAppParams_oneof_one_of_color_style {
    color_style(super::app_create::ColorStyle),
}

#[derive(Clone,PartialEq,Debug)]
pub enum UpdateAppParams_oneof_one_of_is_trash {
    is_trash(bool),
}

impl UpdateAppParams {
    pub fn new() -> UpdateAppParams {
        ::std::default::Default::default()
    }

    // string app_id = 1;


    pub fn get_app_id(&self) -> &str {
        &self.app_id
    }
    pub fn clear_app_id(&mut self) {
        self.app_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_app_id(&mut self, v: ::std::string::String) {
        self.app_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_app_id(&mut self) -> &mut ::std::string::String {
        &mut self.app_id
    }

    // Take field
    pub fn take_app_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.app_id, ::std::string::String::new())
    }

    // string name = 2;


    pub fn get_name(&self) -> &str {
        match self.one_of_name {
            ::std::option::Option::Some(UpdateAppParams_oneof_one_of_name::name(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_name(&mut self) {
        self.one_of_name = ::std::option::Option::None;
    }

    pub fn has_name(&self) -> bool {
        match self.one_of_name {
            ::std::option::Option::Some(UpdateAppParams_oneof_one_of_name::name(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.one_of_name = ::std::option::Option::Some(UpdateAppParams_oneof_one_of_name::name(v))
    }

    // Mutable pointer to the field.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(UpdateAppParams_oneof_one_of_name::name(_)) = self.one_of_name {
        } else {
            self.one_of_name = ::std::option::Option::Some(UpdateAppParams_oneof_one_of_name::name(::std::string::String::new()));
        }
        match self.one_of_name {
            ::std::option::Option::Some(UpdateAppParams_oneof_one_of_name::name(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        if self.has_name() {
            match self.one_of_name.take() {
                ::std::option::Option::Some(UpdateAppParams_oneof_one_of_name::name(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // string desc = 3;


    pub fn get_desc(&self) -> &str {
        match self.one_of_desc {
            ::std::option::Option::Some(UpdateAppParams_oneof_one_of_desc::desc(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_desc(&mut self) {
        self.one_of_desc = ::std::option::Option::None;
    }

    pub fn has_desc(&self) -> bool {
        match self.one_of_desc {
            ::std::option::Option::Some(UpdateAppParams_oneof_one_of_desc::desc(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_desc(&mut self, v: ::std::string::String) {
        self.one_of_desc = ::std::option::Option::Some(UpdateAppParams_oneof_one_of_desc::desc(v))
    }

    // Mutable pointer to the field.
    pub fn mut_desc(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(UpdateAppParams_oneof_one_of_desc::desc(_)) = self.one_of_desc {
        } else {
            self.one_of_desc = ::std::option::Option::Some(UpdateAppParams_oneof_one_of_desc::desc(::std::string::String::new()));
        }
        match self.one_of_desc {
            ::std::option::Option::Some(UpdateAppParams_oneof_one_of_desc::desc(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_desc(&mut self) -> ::std::string::String {
        if self.has_desc() {
            match self.one_of_desc.take() {
                ::std::option::Option::Some(UpdateAppParams_oneof_one_of_desc::desc(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // .ColorStyle color_style = 4;


    pub fn get_color_style(&self) -> &super::app_create::ColorStyle {
        match self.one_of_color_style {
            ::std::option::Option::Some(UpdateAppParams_oneof_one_of_color_style::color_style(ref v)) => v,
            _ => <super::app_create::ColorStyle as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_color_style(&mut self) {
        self.one_of_color_style = ::std::option::Option::None;
    }

    pub fn has_color_style(&self) -> bool {
        match self.one_of_color_style {
            ::std::option::Option::Some(UpdateAppParams_oneof_one_of_color_style::color_style(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_color_style(&mut self, v: super::app_create::ColorStyle) {
        self.one_of_color_style = ::std::option::Option::Some(UpdateAppParams_oneof_one_of_color_style::color_style(v))
    }

    // Mutable pointer to the field.
    pub fn mut_color_style(&mut self) -> &mut super::app_create::ColorStyle {
        if let ::std::option::Option::Some(UpdateAppParams_oneof_one_of_color_style::color_style(_)) = self.one_of_color_style {
        } else {
            self.one_of_color_style = ::std::option::Option::Some(UpdateAppParams_oneof_one_of_color_style::color_style(super::app_create::ColorStyle::new()));
        }
        match self.one_of_color_style {
            ::std::option::Option::Some(UpdateAppParams_oneof_one_of_color_style::color_style(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_color_style(&mut self) -> super::app_create::ColorStyle {
        if self.has_color_style() {
            match self.one_of_color_style.take() {
                ::std::option::Option::Some(UpdateAppParams_oneof_one_of_color_style::color_style(v)) => v,
                _ => panic!(),
            }
        } else {
            super::app_create::ColorStyle::new()
        }
    }

    // bool is_trash = 5;


    pub fn get_is_trash(&self) -> bool {
        match self.one_of_is_trash {
            ::std::option::Option::Some(UpdateAppParams_oneof_one_of_is_trash::is_trash(v)) => v,
            _ => false,
        }
    }
    pub fn clear_is_trash(&mut self) {
        self.one_of_is_trash = ::std::option::Option::None;
    }

    pub fn has_is_trash(&self) -> bool {
        match self.one_of_is_trash {
            ::std::option::Option::Some(UpdateAppParams_oneof_one_of_is_trash::is_trash(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_is_trash(&mut self, v: bool) {
        self.one_of_is_trash = ::std::option::Option::Some(UpdateAppParams_oneof_one_of_is_trash::is_trash(v))
    }
}

impl ::protobuf::Message for UpdateAppParams {
    fn is_initialized(&self) -> bool {
        if let Some(UpdateAppParams_oneof_one_of_color_style::color_style(ref v)) = self.one_of_color_style {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.app_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_name = ::std::option::Option::Some(UpdateAppParams_oneof_one_of_name::name(is.read_string()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_desc = ::std::option::Option::Some(UpdateAppParams_oneof_one_of_desc::desc(is.read_string()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_color_style = ::std::option::Option::Some(UpdateAppParams_oneof_one_of_color_style::color_style(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_is_trash = ::std::option::Option::Some(UpdateAppParams_oneof_one_of_is_trash::is_trash(is.read_bool()?));
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
        if !self.app_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.app_id);
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_name {
            match v {
                &UpdateAppParams_oneof_one_of_name::name(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_desc {
            match v {
                &UpdateAppParams_oneof_one_of_desc::desc(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_color_style {
            match v {
                &UpdateAppParams_oneof_one_of_color_style::color_style(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_is_trash {
            match v {
                &UpdateAppParams_oneof_one_of_is_trash::is_trash(v) => {
                    my_size += 2;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.app_id.is_empty() {
            os.write_string(1, &self.app_id)?;
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_name {
            match v {
                &UpdateAppParams_oneof_one_of_name::name(ref v) => {
                    os.write_string(2, v)?;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_desc {
            match v {
                &UpdateAppParams_oneof_one_of_desc::desc(ref v) => {
                    os.write_string(3, v)?;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_color_style {
            match v {
                &UpdateAppParams_oneof_one_of_color_style::color_style(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_is_trash {
            match v {
                &UpdateAppParams_oneof_one_of_is_trash::is_trash(v) => {
                    os.write_bool(5, v)?;
                },
            };
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

    fn new() -> UpdateAppParams {
        UpdateAppParams::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "app_id",
                |m: &UpdateAppParams| { &m.app_id },
                |m: &mut UpdateAppParams| { &mut m.app_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "name",
                UpdateAppParams::has_name,
                UpdateAppParams::get_name,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "desc",
                UpdateAppParams::has_desc,
                UpdateAppParams::get_desc,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::app_create::ColorStyle>(
                "color_style",
                UpdateAppParams::has_color_style,
                UpdateAppParams::get_color_style,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                "is_trash",
                UpdateAppParams::has_is_trash,
                UpdateAppParams::get_is_trash,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<UpdateAppParams>(
                "UpdateAppParams",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static UpdateAppParams {
        static instance: ::protobuf::rt::LazyV2<UpdateAppParams> = ::protobuf::rt::LazyV2::INIT;
        instance.get(UpdateAppParams::new)
    }
}

impl ::protobuf::Clear for UpdateAppParams {
    fn clear(&mut self) {
        self.app_id.clear();
        self.one_of_name = ::std::option::Option::None;
        self.one_of_desc = ::std::option::Option::None;
        self.one_of_color_style = ::std::option::Option::None;
        self.one_of_is_trash = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UpdateAppParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateAppParams {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10app_update.proto\x1a\x10app_create.proto\"\xe9\x01\n\x10UpdateAppR\
    equest\x12\x15\n\x06app_id\x18\x01\x20\x01(\tR\x05appId\x12\x14\n\x04nam\
    e\x18\x02\x20\x01(\tH\0R\x04name\x12\x14\n\x04desc\x18\x03\x20\x01(\tH\
    \x01R\x04desc\x12.\n\x0bcolor_style\x18\x04\x20\x01(\x0b2\x0b.ColorStyle\
    H\x02R\ncolorStyle\x12\x1b\n\x08is_trash\x18\x05\x20\x01(\x08H\x03R\x07i\
    sTrashB\r\n\x0bone_of_nameB\r\n\x0bone_of_descB\x14\n\x12one_of_color_st\
    yleB\x11\n\x0fone_of_is_trash\"\xe8\x01\n\x0fUpdateAppParams\x12\x15\n\
    \x06app_id\x18\x01\x20\x01(\tR\x05appId\x12\x14\n\x04name\x18\x02\x20\
    \x01(\tH\0R\x04name\x12\x14\n\x04desc\x18\x03\x20\x01(\tH\x01R\x04desc\
    \x12.\n\x0bcolor_style\x18\x04\x20\x01(\x0b2\x0b.ColorStyleH\x02R\ncolor\
    Style\x12\x1b\n\x08is_trash\x18\x05\x20\x01(\x08H\x03R\x07isTrashB\r\n\
    \x0bone_of_nameB\r\n\x0bone_of_descB\x14\n\x12one_of_color_styleB\x11\n\
    \x0fone_of_is_trashJ\xcb\x06\n\x06\x12\x04\0\0\x10\x01\n\x08\n\x01\x0c\
    \x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x01\0\x1a\n\n\n\x02\x04\0\x12\
    \x04\x03\0\t\x01\n\n\n\x03\x04\0\x01\x12\x03\x03\x08\x18\n\x0b\n\x04\x04\
    \0\x02\0\x12\x03\x04\x04\x16\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x04\x04\
    \n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x04\x0b\x11\n\x0c\n\x05\x04\0\x02\
    \0\x03\x12\x03\x04\x14\x15\n\x0b\n\x04\x04\0\x08\0\x12\x03\x05\x04*\n\
    \x0c\n\x05\x04\0\x08\0\x01\x12\x03\x05\n\x15\n\x0b\n\x04\x04\0\x02\x01\
    \x12\x03\x05\x18(\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x05\x18\x1e\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x05\x1f#\n\x0c\n\x05\x04\0\x02\x01\
    \x03\x12\x03\x05&'\n\x0b\n\x04\x04\0\x08\x01\x12\x03\x06\x04*\n\x0c\n\
    \x05\x04\0\x08\x01\x01\x12\x03\x06\n\x15\n\x0b\n\x04\x04\0\x02\x02\x12\
    \x03\x06\x18(\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x06\x18\x1e\n\x0c\n\
    \x05\x04\0\x02\x02\x01\x12\x03\x06\x1f#\n\x0c\n\x05\x04\0\x02\x02\x03\
    \x12\x03\x06&'\n\x0b\n\x04\x04\0\x08\x02\x12\x03\x07\x04<\n\x0c\n\x05\
    \x04\0\x08\x02\x01\x12\x03\x07\n\x1c\n\x0b\n\x04\x04\0\x02\x03\x12\x03\
    \x07\x1f:\n\x0c\n\x05\x04\0\x02\x03\x06\x12\x03\x07\x1f)\n\x0c\n\x05\x04\
    \0\x02\x03\x01\x12\x03\x07*5\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x0789\
    \n\x0b\n\x04\x04\0\x08\x03\x12\x03\x08\x040\n\x0c\n\x05\x04\0\x08\x03\
    \x01\x12\x03\x08\n\x19\n\x0b\n\x04\x04\0\x02\x04\x12\x03\x08\x1c.\n\x0c\
    \n\x05\x04\0\x02\x04\x05\x12\x03\x08\x1c\x20\n\x0c\n\x05\x04\0\x02\x04\
    \x01\x12\x03\x08!)\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x08,-\n\n\n\x02\
    \x04\x01\x12\x04\n\0\x10\x01\n\n\n\x03\x04\x01\x01\x12\x03\n\x08\x17\n\
    \x0b\n\x04\x04\x01\x02\0\x12\x03\x0b\x04\x16\n\x0c\n\x05\x04\x01\x02\0\
    \x05\x12\x03\x0b\x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x0b\x0b\x11\
    \n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0b\x14\x15\n\x0b\n\x04\x04\x01\
    \x08\0\x12\x03\x0c\x04*\n\x0c\n\x05\x04\x01\x08\0\x01\x12\x03\x0c\n\x15\
    \n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x0c\x18(\n\x0c\n\x05\x04\x01\x02\
    \x01\x05\x12\x03\x0c\x18\x1e\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x0c\
    \x1f#\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x0c&'\n\x0b\n\x04\x04\x01\
    \x08\x01\x12\x03\r\x04*\n\x0c\n\x05\x04\x01\x08\x01\x01\x12\x03\r\n\x15\
    \n\x0b\n\x04\x04\x01\x02\x02\x12\x03\r\x18(\n\x0c\n\x05\x04\x01\x02\x02\
    \x05\x12\x03\r\x18\x1e\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\r\x1f#\n\
    \x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\r&'\n\x0b\n\x04\x04\x01\x08\x02\
    \x12\x03\x0e\x04<\n\x0c\n\x05\x04\x01\x08\x02\x01\x12\x03\x0e\n\x1c\n\
    \x0b\n\x04\x04\x01\x02\x03\x12\x03\x0e\x1f:\n\x0c\n\x05\x04\x01\x02\x03\
    \x06\x12\x03\x0e\x1f)\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03\x0e*5\n\
    \x0c\n\x05\x04\x01\x02\x03\x03\x12\x03\x0e89\n\x0b\n\x04\x04\x01\x08\x03\
    \x12\x03\x0f\x040\n\x0c\n\x05\x04\x01\x08\x03\x01\x12\x03\x0f\n\x19\n\
    \x0b\n\x04\x04\x01\x02\x04\x12\x03\x0f\x1c.\n\x0c\n\x05\x04\x01\x02\x04\
    \x05\x12\x03\x0f\x1c\x20\n\x0c\n\x05\x04\x01\x02\x04\x01\x12\x03\x0f!)\n\
    \x0c\n\x05\x04\x01\x02\x04\x03\x12\x03\x0f,-b\x06proto3\
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
