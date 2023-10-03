/// MIT License
///
/// Copyright 2022 Alexey Kutepov <reximkut@gmail.com>
/// Copyright 2022 Matthias Zimmerman <matthias291999@gmail.com>
///
/// Code is converted from jello.py in JelloVM by Alexey Kutepov <@tsoding>
use std::{
    env,
    error::Error,
    fmt::{Display, Formatter},
    fs::File,
    io::Read,
    mem, vec,
};

#[macro_use]
extern crate bitflags;

extern crate num;
#[macro_use]
extern crate num_derive;

use num::FromPrimitive;

#[derive(Clone, Debug)]
pub enum CmdArg<'a> {
    LongOpt(&'a str),
    ShortOpt(&'a str),
    Seq(&'a str),
    Special(u8),
}

pub fn get_command_line_arguments<'a>(
    arguments: &'a Vec<String>,
) -> Result<Vec<CmdArg<'a>>, &'static str> {
    let mut output_args = vec![];
    output_args.reserve(arguments.len() - 1);
    if arguments.len() <= 1 {
        return Err("Not enough arguments");
    }
    for value in arguments.iter().skip(1) {
        let opt = value.split_once("--");
        if let Some(("", opt)) = opt {
            output_args.push(CmdArg::LongOpt(opt));
            continue;
        }
        let opt = value.split_once('-');
        if let Some(("", opt)) = opt {
            output_args.push(CmdArg::ShortOpt(opt));
            continue;
        }
        output_args.push(CmdArg::Seq(value));
    }
    Ok(output_args)
}

#[derive(Clone, Debug, PartialEq, Eq, FromPrimitive)]
#[repr(u8)]
pub enum ConstantTag {
    Utf8 = 1,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    Class = 7,
    String = 8,
    FieldRef = 9,
    MethodRef = 10,
    InterfaceMethodRef = 11,
    NameAndType = 12,
    MethodHandle = 15,
    MethodType = 16,
    Dynamic = 17,
    InvokeDynamic = 18,
    Module = 19,
    Package = 20,
}

impl Copy for ConstantTag {}

impl TryFrom<u8> for ConstantTag {
    type Error = Box<dyn Error>;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let out = FromPrimitive::from_u8(value);
        match out {
            Some(v) => Ok(v),
            _ => Err(Box::new(MyError::new("Failed to parse into enum".into()))),
        }
    }
}

#[derive(Debug)]
struct MyError {
    description: String,
}

impl MyError {
    fn new(description: String) -> MyError {
        MyError { description }
    }
}

impl Display for MyError {
    fn fmt(&self, w: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(w, "MyError {}", &self.description)
    }
}

impl Error for MyError {}

bitflags! {
    pub struct ClassAccessFlags: u16 {
        const PUBLIC = 0x0001;
        const FINAL = 0x0010;
        const SUPER = 0x0020;
        const INTERFACE = 0x0200;
        const ABSTRACT = 0x0400;
        const SYNTHETIC = 0x1000;
        const ANNOTATION = 0x2000;
        const ENUM = 0x4000;
        const MODULE = 0x8000;
    }
}

impl ClassAccessFlags {
    fn parse(f: &mut File) -> Option<Self> {
        Self::from_bits(parse_u2_raw(f))
    }
}

bitflags! {
    pub struct MethodAccessFlags: u16 {
        const PUBLIC = 0x0001;
        const PRIVATE = 0x0002;
        const PROTECTED = 0x0004;
        const STATIC = 0x0008;
        const FINAL = 0x0010;
        const SYNCHRONIZED = 0x0020;
        const BRIDGE = 0x0040;
        const VAR_ARGS = 0x0080;
        const NATIVE = 0x0100;
        const ABSTRACT = 0x0400;
        const STRICT = 0x0800;
        const SYNTHETIC = 0x1000;
    }
}

impl MethodAccessFlags {
    fn parse(f: &mut File) -> Option<Self> {
        Self::from_bits(parse_u2_raw(f))
    }
}

pub fn parse_u1_raw(f: &mut File) -> u8 {
    let mut value = [0];
    f.read_exact(&mut value).unwrap();
    value[0]
}

pub fn parse_u2_raw(f: &mut dyn Read) -> u16 {
    let mut value = [0; 2];
    f.read_exact(&mut value).unwrap();
    let v1 = value[1] as u16 + ((value[0] as u16) << 8);
    v1
}

pub fn parse_u4_raw(f: &mut dyn Read) -> u32 {
    let mut value = [0; 4];
    f.read_exact(&mut value).unwrap();
    value[3] as u32
        + ((value[2] as u32) << 8)
        + ((value[1] as u32) << 16)
        + ((value[0] as u32) << 24)
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaValue {
    FakePrintStream,
    Integer { value: i32 },
    Double { value: f64 },
    Float { value: f32 },
    Long { value: i64 },
    String { value: String },
    ClassInstance { index: i32 },
}

#[derive(Clone, Debug)]
pub struct ParsedClass {
    pub magic: u32,
    pub minor: u16,
    pub major: u16,
    pub constant_pool: Vec<Option<Constant>>,
    pub access_flags: ClassAccessFlags,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<ParsedInterface>,
    pub fields: Vec<ParsedField>,
    pub methods: Vec<ParsedMethod>,
    pub attributes: Vec<ParsedAttribute>,
    pub class_id: i32,
}

fn parse_interfaces(f: &mut File, interfaces_count: u16) -> Vec<ParsedInterface> {
    (0..interfaces_count)
        .map(|_| ParsedInterface::parse(f))
        .collect()
}

impl ParsedClass {
    fn matches_name(&self, name_index: u16, needle_name: &str) -> bool {
        if let Constant::Utf8 { value, .. } = self.cp_as_ref(name_index) {
            needle_name == *value
        } else {
            false
        }
    }

    fn matches_descriptor(&self, descriptor_index: u16, needle_name: &str) -> bool {
        if let Constant::Utf8 { value, .. } = self.cp_as_ref(descriptor_index) {
            needle_name == *value
        } else {
            false
        }
    }

    fn find_methods_by_name(&self, name: &str, descriptor: &str) -> Vec<&ParsedMethod> {
        self.methods
            .iter()
            .filter(|method| {
                self.matches_name(method.name_index, name)
                    && self.matches_descriptor(method.descriptor_index, descriptor)
            })
            .collect()
    }

    fn cp_as_ref(&self, index: u16) -> &Constant {
        self.cp(index).as_ref().unwrap()
    }

    fn cp(&self, index: u16) -> &Option<Constant> {
        &self.constant_pool[(index - 1) as usize]
    }

    fn parse(program: &mut Program, f: &mut File) -> Self {
        let magic = parse_u4_raw(f);
        let minor = parse_u2_raw(f);
        let major = parse_u2_raw(f);
        let constant_pool = parse_constant_pool(program, f);
        let access_flags = ClassAccessFlags::parse(f).unwrap();
        let this_class = parse_u2_raw(f);
        let super_class = parse_u2_raw(f);
        let interfaces_count = parse_u2_raw(f);
        let interfaces = parse_interfaces(f, interfaces_count);
        let fields_count = parse_u2_raw(f);
        let fields = (0..fields_count).map(|_| ParsedField::parse(f)).collect();
        let methods_count = parse_u2_raw(f);
        let methods = (0..methods_count).map(|_| ParsedMethod::parse(f)).collect();
        let attributes_count = parse_u2_raw(f);
        let attributes = parse_attributes(f, attributes_count);
        let class_id = program.global_class_count;
        program.global_class_count += 1;
        Self {
            magic,
            minor,
            major,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
            class_id,
        }
    }

    #[cfg(test)]
    fn with_constant_pool(pool: Vec<Option<Constant>>) -> Self {
        Self {
            magic: 0,
            minor: 0,
            major: 0,
            constant_pool: pool,
            access_flags: ClassAccessFlags { bits: 0 },
            this_class: 0,
            super_class: 0,
            interfaces: vec![],
            fields: vec![],
            methods: vec![],
            attributes: vec![],
            class_id: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ParsedInterface {
    pub interface_index: u16,
}

impl ParsedInterface {
    fn parse(f: &mut File) -> ParsedInterface {
        let interface_index = parse_u2_raw(f);
        ParsedInterface { interface_index }
    }
}

#[derive(Clone, Debug)]
pub struct ParsedField {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<ParsedAttribute>,
}

impl ParsedField {
    fn parse(f: &mut File) -> ParsedField {
        let access_flags = parse_u2_raw(f);
        let name_index = parse_u2_raw(f);
        let descriptor_index = parse_u2_raw(f);
        let attributes_count = parse_u2_raw(f);
        let attributes = parse_attributes(f, attributes_count);
        ParsedField {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Constant {
    // CONSTANT_Utf8_info { .. }
    Utf8 {
        value: String,
    },
    // CONSTANT_Integer_info { .. }
    Integer {
        value: i32,
    },
    // CONSTANT_Float_info { .. }
    Float {
        value: f32,
    },
    // CONSTANT_Methodref_info { .. }
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    // CONSTANT_Class_info { .. }
    Class {
        name_index: u16,
    },
    // CONSTANT_NameAndType_info { .. }
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    // CONSTANT_Fieldref_info { .. }
    FieldRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    // CONSTANT_String_info { .. }
    String {
        string_index: u16,
    },
    // CONSTANT_Long_info { .. }
    Long {
        value: i64,
    },
    // CONSTANT_Double_info { .. }
    Double {
        value: f64,
    },
    // CONSTANT_InterfaceMethodref_info { .. }
    InterfaceMethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    // CONSTANT_MethodHandle_info { .. }
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    // CONSTANT_MethodType_info { .. }
    MethodType {
        descriptor_index: u16,
    },
    // CONSTANT_Dynamic_info { .. }
    /// class file format >= 55.0
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    // CONSTANT_InvokeDynamic_info { .. }
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    // CONSTANT_Module_info { .. }
    /// class file format >= 53.0
    Module {
        name_index: u16,
    },
    // CONSTANT_Package_info { .. }
    /// class file format >= 53.0
    Package {
        name_index: u16,
    },
}

#[cfg(test)]
impl Constant {
    fn str(&self) -> &str {
        match self {
            Self::Utf8 { value } => value,
            _ => panic!("{self:?}"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ParsedMethod {
    pub access_flags: MethodAccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<ParsedAttribute>,
}

impl ParsedMethod {
    // method_info { .. }
    fn parse(f: &mut File) -> ParsedMethod {
        // u2             access_flags;
        // u2             name_index;
        // u2             descriptor_index;
        // u2             attributes_count;
        // attribute_info attributes[attributes_count];
        let access_flags = MethodAccessFlags::parse(f).unwrap();
        let name_index = parse_u2_raw(f);
        let descriptor_index = parse_u2_raw(f);
        let attributes_count = parse_u2_raw(f);
        let attributes = parse_attributes(f, attributes_count);
        ParsedMethod {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ParsedAttribute {
    pub attribute_name_index: u16,
    pub info: Vec<u8>,
}

impl ParsedAttribute {
    // attribute_info { .. }
    fn parse(f: &mut dyn Read) -> ParsedAttribute {
        // u2 attribute_name_index;
        // u4 attribute_length;
        // u1 info[attribute_length];
        let attribute_name_index = parse_u2_raw(f);
        let attribute_length = parse_u4_raw(f);
        let info = parse_vec_u8(f, attribute_length as u64);
        ParsedAttribute {
            attribute_name_index,
            info,
        }
    }

    pub fn lookup(&self, clazz: &ParsedClass) -> AttributeDescription {
        let attr_name = clazz.cp(self.attribute_name_index);
        if let Some(Constant::Utf8 { value }) = attr_name {
            return AttributeDescription {
                attribute_name: value.into(),
                info: self.info.clone(),
            };
        }
        AttributeDescription {
            attribute_name: "".into(),
            info: vec![],
        }
    }
}

#[derive(Debug)]
pub struct AttributeDescription {
    pub attribute_name: String,
    pub info: Vec<u8>,
}

// attribute_info { .. }
fn parse_attributes(f: &mut dyn Read, attributes_count: u16) -> Vec<ParsedAttribute> {
    (0..attributes_count)
        .map(|_| ParsedAttribute::parse(f))
        .collect::<Vec<ParsedAttribute>>()
}

fn parse_vec_u8(f: &mut dyn Read, length: u64) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];
    f.take(length as u64).read_to_end(&mut bytes).unwrap();
    bytes
}

fn parse_constant_pool_item(inc_size: &mut usize, f: &mut File) -> Constant {
    let tag_raw = parse_u1_raw(f);
    let tag = ConstantTag::try_from(tag_raw).unwrap();
    match tag {
        ConstantTag::Utf8 => {
            let length = parse_u2_raw(f);
            let vec = parse_vec_u8(f, length.into());
            // TODO: Parse from java_utf8 instead of utf8
            let value = String::from_utf8(vec).unwrap();
            Constant::Utf8 { value }
        }
        ConstantTag::Integer => {
            let value = parse_u4_raw(f);
            let value = value as i32;
            Constant::Integer { value }
        }
        ConstantTag::Float => {
            let value = parse_u4_raw(f);
            let value = f32::from_bits(value);
            Constant::Float { value }
        }
        ConstantTag::Long => {
            *inc_size = 2;
            let high_bytes = parse_u4_raw(f);
            let low_bytes = parse_u4_raw(f);
            let value = ((high_bytes as u64) << 32) + (low_bytes as u64);
            let value = value as i64;
            Constant::Long { value }
        }
        ConstantTag::Double => {
            *inc_size = 2;
            let high_bytes = parse_u4_raw(f);
            let low_bytes = parse_u4_raw(f);
            let bits = ((high_bytes as u64) << 32) + (low_bytes as u64);
            let value = f64::from_bits(bits);
            Constant::Double { value }
        }
        ConstantTag::Class => Constant::Class {
            name_index: parse_u2_raw(f),
        },
        ConstantTag::String => Constant::String {
            string_index: parse_u2_raw(f),
        },
        ConstantTag::FieldRef => Constant::FieldRef {
            class_index: parse_u2_raw(f),
            name_and_type_index: parse_u2_raw(f),
        },
        ConstantTag::MethodRef => Constant::MethodRef {
            class_index: parse_u2_raw(f),
            name_and_type_index: parse_u2_raw(f),
        },
        ConstantTag::InterfaceMethodRef => Constant::InterfaceMethodRef {
            class_index: parse_u2_raw(f),
            name_and_type_index: parse_u2_raw(f),
        },
        // CONSTANT_NameAndType_info { .. }
        ConstantTag::NameAndType => Constant::NameAndType {
            // u1 tag;
            // u2 name_index;
            // u2 descriptor_index;
            name_index: parse_u2_raw(f),
            descriptor_index: parse_u2_raw(f),
        },
        ConstantTag::MethodHandle => Constant::MethodHandle {
            reference_kind: parse_u1_raw(f),
            reference_index: parse_u2_raw(f),
        },
        ConstantTag::MethodType => Constant::MethodType {
            descriptor_index: parse_u2_raw(f),
        },
        ConstantTag::Dynamic => Constant::Dynamic {
            bootstrap_method_attr_index: parse_u2_raw(f),
            name_and_type_index: parse_u2_raw(f),
        },
        ConstantTag::InvokeDynamic => Constant::InvokeDynamic {
            bootstrap_method_attr_index: parse_u2_raw(f),
            name_and_type_index: parse_u2_raw(f),
        },
        ConstantTag::Module => Constant::Module {
            name_index: parse_u2_raw(f),
        },
        ConstantTag::Package => Constant::Package {
            name_index: parse_u2_raw(f),
        },
    }
}

struct Program {
    print_debug_info: bool,
    is_verbose: bool,
    print_overloads: bool,
    lookup_attr: bool,
    global_class_count: i32,
}

fn parse_constant_pool(program: &Program, f: &mut File) -> Vec<Option<Constant>> {
    let cp_count = usize::from(parse_u2_raw(f));
    let mut ret: Vec<Option<Constant>> = vec![];
    let is_printing_verbose = program.print_debug_info && program.is_verbose;
    if is_printing_verbose {
        println!("cp_count: {}", cp_count);
        println!("Constant Pool:");
    }
    let mut index = 0;
    while index < cp_count - 1 {
        let mut item_size = 1;
        let item = parse_constant_pool_item(&mut item_size, f);
        if is_printing_verbose {
            println!("  #{} = {:?}", index + 1, item);
        }
        ret.insert(index, Some(item));
        match item_size {
            1 => (),
            2 => ret.push(None),
            _ => panic!("Unhandled item_size '{}'", item_size),
        }
        index += item_size;
    }
    ret
}

fn parse_class_file(program: &mut Program, file_path: &str) -> ParsedClass {
    let mut f = File::open(file_path).unwrap();
    ParsedClass::parse(program, &mut f)
}

fn get_code_attrib(clazz: &ParsedClass, attributes: &[ParsedAttribute]) -> CodeInfo {
    let code_attrib_vec = find_attributes_by_name(clazz, attributes, "Code");
    assert_eq!(
        code_attrib_vec.len(),
        1,
        "found only one \"Code\" attribute"
    );
    let code_attrib = code_attrib_vec[0];
    CodeInfo::parse(&mut &code_attrib.info[..])
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let (program, args) = (args.first().unwrap(), args.split_at(1).1);
    if args.is_empty() {
        println!("Usage: {program} <path/to/Main.class>");
        println!("ERROR: no path to Main.class was provided");
        std::process::exit(1);
    }
    let file_path = &args[0];
    let mut program = Program {
        global_class_count: 0,
        print_debug_info: false,
        is_verbose: false,
        print_overloads: false,
        lookup_attr: false,
    };
    let clazz = parse_class_file(&mut program, file_path);
    let main_overloads =
        clazz.find_methods_by_name("main", &format!("({}L{};)V", "[", "java/lang/String"));
    if (program.print_debug_info && program.is_verbose) || program.print_overloads {
        println!("{:?}", main_overloads);
    }
    let mut runtime = Runtime {
        locals: vec![],
        stack: vec![],
    };
    let init_overloads = clazz.find_methods_by_name("<init>", "()V");
    let code_attrib: CodeInfo = get_code_attrib(&clazz, &init_overloads[0].attributes);
    runtime.locals.push(JavaValue::ClassInstance {
        index: clazz.class_id,
    });
    execute_code(&mut runtime, &clazz, &code_attrib.code);
    runtime.locals.clear();
    if main_overloads.len() == 0 {
        println!("no overloads found for main");
        std::process::exit(1);
    }
    let code_attrib = get_code_attrib(&clazz, &main_overloads[0].attributes);
    if program.lookup_attr {
        let attributes = &code_attrib.attributes;
        let attr = attributes[0].lookup(&clazz);
        println!("{:?}", attr);
    }
    execute_code(&mut runtime, &clazz, &code_attrib.code);
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum Opcode {
    nop,
    // spell:words aconst
    aconst_null,
    // spell:words iconst
    iconst_m1,
    iconst_0,
    iconst_1,
    iconst_2,
    iconst_3,
    iconst_4,
    iconst_5,
    // spell:words lconst
    lconst_0,
    lconst_1,
    // spell:words fconst
    fconst_0,
    fconst_1,
    fconst_2,
    // spell:words dconst
    dconst_0,
    dconst_1,
    // spell:words sipush
    bipush,
    sipush,
    ldc,
    ldc_w,
    ldc2_w,
    // spell:words iload
    iload,
    // spell:words lload
    lload,
    // spell:words fload
    fload,
    // spell:words dload
    dload,
    // spell:words aload
    aload,
    iload_0,
    iload_1,
    iload_2,
    iload_3,
    lload_0,
    lload_1,
    lload_2,
    lload_3,
    fload_0,
    fload_1,
    fload_2,
    fload_3,
    dload_0,
    dload_1,
    dload_2,
    dload_3,
    aload_0,
    aload_1,
    aload_2,
    aload_3,
    // spell:words iaload
    iaload,
    // spell:words laload
    laload,
    // spell:words faload
    faload,
    // spell:words daload
    daload,
    // spell:words aaload
    aaload,
    // spell:words baload
    baload,
    // spell:words caload
    caload,
    // spell:words saload
    saload,
    // spell:words istore
    istore,
    // spell:words lstore
    lstore,
    // spell:words fstore
    fstore,
    // spell:words dstore
    dstore,
    // spell:words astore
    astore,
    istore_0,
    istore_1,
    istore_2,
    istore_3,
    lstore_0,
    lstore_1,
    lstore_2,
    lstore_3,
    fstore_0,
    fstore_1,
    fstore_2,
    fstore_3,
    dstore_0,
    dstore_1,
    dstore_2,
    dstore_3,
    astore_0,
    astore_1,
    astore_2,
    astore_3,
    // spell:words iastore
    iastore,
    // spell:words lastore
    lastore,
    // spell:words fastore
    fastore,
    // spell:words dastore
    dastore,
    // spell:words aastore
    aastore,
    // spell:words bastore
    bastore,
    // spell:words castore
    castore,
    // spell:words sastore
    sastore,
    pop,
    pop2,
    dup,
    dup_x1,
    dup_x2,
    dup2,
    dup2_x1,
    dup2_x2,
    swap,
    // spell:words iadd
    iadd,
    // spell:words ladd
    ladd,
    // spell:words fadd
    fadd,
    // spell:words dadd
    dadd,
    // spell:words isub
    isub,
    // spell:words lsub
    lsub,
    // spell:words fsub
    fsub,
    // spell:words dsub
    dsub,
    // spell:words imul
    imul,
    // spell:words lmul
    lmul,
    // spell:words fmul
    fmul,
    // spell:words dmul
    dmul,
    // spell:words idiv
    idiv,
    // spell:words ldiv
    ldiv,
    // spell:words fdiv
    fdiv,
    // spell:words ddiv
    ddiv,
    // spell:words irem
    irem,
    // spell:words lrem
    lrem,
    // spell:words frem
    frem,
    // spell:words drem
    drem,
    // spell:words ineg
    ineg,
    // spell:words lneg
    lneg,
    // spell:words fneg
    fneg,
    // spell:words dneg
    dneg,
    // spell:words ishl
    ishl,
    // spell:words lshl
    lshl,
    // spell:words ishr
    ishr,
    // spell:words lshr
    lshr,
    // spell:words iushr
    iushr,
    // spell:words lushr
    lushr,
    // spell:words iand
    iand,
    land,
    ior,
    lor,
    // spell:words ixor
    ixor,
    // spell:words lxor
    lxor,
    // spell:words iinc
    iinc,
    i2l,
    i2f,
    i2d,
    l2i,
    l2f,
    l2d,
    f2i,
    f2l,
    f2d,
    d2i,
    d2l,
    d2f,
    i2b,
    i2c,
    i2s,
    // spell:words lcmp
    lcmp,
    // spell:words fcmpl
    fcmpl,
    // spell:words fcmpg
    fcmpg,
    // spell:words dcmpl
    dcmpl,
    // spell:words dcmpg
    dcmpg,
    // spell:words ifeq
    ifeq,
    // spell:words ifne
    ifne,
    // spell:words iflt
    iflt,
    // spell:words ifge
    ifge,
    // spell:words ifgt
    ifgt,
    // spell:words ifle
    ifle,
    // spell:words icmpeq
    if_icmpeq,
    // spell:words icmpne
    if_icmpne,
    // spell:words icmplt
    if_icmplt,
    // spell:words icmpge
    if_icmpge,
    // spell:words icmpgt
    if_icmpgt,
    // spell:words icmple
    if_icmple,
    // spell:words acmpeq
    if_acmpeq,
    // spell:words acmpne
    if_acmpne,
    goto,
    jsr,
    ret,
    // spell:words tableswitch
    tableswitch,
    // spell:words lookupswitch
    lookupswitch,
    // spell:words ireturn
    ireturn,
    // spell:words lreturn
    lreturn,
    // spell:words freturn
    freturn,
    // spell:words dreturn
    dreturn,
    // spell:words areturn
    areturn,
    return_,
    getstatic,
    // spell:words putstatic
    putstatic,
    // spell:words getfield
    getfield,
    // spell:words putfield
    putfield,
    invokevirtual,
    // spell:words invokespecial
    invokespecial,
    // spell:words invokestatic
    invokestatic,
    // spell:words invokeinterface
    invokeinterface,
    // spell:words invokedynamic
    invokedynamic,
    new,
    // spell:words newarray
    newarray,
    // spell:words anewarray
    anewarray,
    arraylength,
    // spell:words athrow
    athrow,
    // spell:words checkcast
    checkcast,
    // spell:words instanceof
    instanceof,
    // spell:words monitorenter
    monitorenter,
    // spell:words monitorexit
    monitorexit,
    wide,
    // spell:words multianewarray
    multianewarray,
    // spell:words ifnull
    ifnull,
    // spell:words ifnonnull
    ifnonnull,
    goto_w,
    jsr_w,
    breakpoint,
    // spell:words impdep
    impdep1 = 0xfe,
    impdep2 = 0xff,
}

fn get_cp_string(clazz: &ParsedClass, index: u16) -> String {
    let pool_item = clazz.cp(index);
    let Some(Constant::Utf8 { value }) = pool_item else {
        panic!("get_pool_string not utf8");
    };
    return value.into();
}

struct Runtime {
    locals: Vec<JavaValue>,
    stack: Vec<JavaValue>,
}

fn java_intrinsic_println_value(value: &JavaValue) {
    if let JavaValue::String { value } = value {
        println!("{}", value);
    } else if let JavaValue::Integer { value } = value {
        println!("{}", value);
    } else if let &JavaValue::Double { value } = value {
        let ten = 10f64;
        let minus_one = -1.0;
        if value >= 1e7 {
            let exp = value.log10().floor();
            println!("{:?}E{}", value / (ten.powf(exp)), exp as i32);
        } else if value >= 1e-7 && value < 1e-3 {
            let exp = value.log10().floor();
            println!("{:?}E{}", value / (ten.powf(exp)), exp as i32);
        } else if value <= -1e-7 && value > -1e-3 {
            let exp = (value * minus_one).log10().floor();
            println!("{:?}E{}", value / (ten.powf(exp)), exp as i32);
        } else if value <= -1e7 {
            let exp = (value * minus_one).log10().floor();
            println!("{:?}E{}", value / (ten.powf(exp)), exp as i32);
        } else {
            println!("{:?}", value);
        }
    } else if let &JavaValue::Float { value } = value {
        let ten = 10f32;
        let minus_one = -1f32;
        if false {
        } else if value >= 1e7 {
            let exp = value.log10().floor();
            println!("{:?}E{}", value / (ten.powf(exp)), exp as i32);
        } else if value >= 1e-7 && value < 1e-3 {
            let exp = value.log10().floor();
            println!("{:?}E{}", value / (ten.powf(exp)), exp as i32);
        } else if value <= -1e-7 && value > -1e-3 {
            let exp = (value * minus_one).log10().floor();
            println!("{:?}E{}", value / (ten.powf(exp)), exp as i32);
        } else if value <= -1e7 {
            let exp = (value * minus_one).log10().floor();
            println!("{:?}E{}", value / (ten.powf(exp)), exp as i32);
        } else {
            println!("{:?}", value);
        }
    } else if let JavaValue::Long { value } = value {
        println!("{}", value);
    } else {
        panic!("intrinsic println not implemented for type {value:?}");
    }
}

fn execute_instruction(
    runtime: &mut Runtime,
    opcode: &Opcode,
    index: &mut usize,
    code: &[u8],
    clazz: &ParsedClass,
) {
    let stack = &mut runtime.stack;
    if let Opcode::invokevirtual = opcode {
        let index = parse_u2_vec(index, code);
        let methodref = clazz.cp(index);
        let Some(Constant::MethodRef {
            class_index,
            name_and_type_index,
        }) = methodref else {panic!();};
        let name_of_class = get_name_of_class(clazz, *class_index);
        let name_of_member = get_name_of_member(clazz, *name_and_type_index);
        if !(name_of_class == "java/io/PrintStream" && name_of_member == "println") {
            panic!("Unknown method {name_of_member} in class {name_of_class} in invokevirtual instruction");
        };
        let n = stack.len();
        if stack.len() < 2 {
            panic!("RuntimeError({name_of_class}/{name_of_member} expects 2 arguments, but provided {n})");
        }
        let obj = &stack[stack.len() - 2];
        let dsc = mem::discriminant(obj);
        if mem::discriminant(&JavaValue::FakePrintStream) != dsc {
            panic!("Unsupported stream type {dsc:?}");
        };
        java_intrinsic_println_value(&stack[stack.len() - 1]);
    } else if let Opcode::getstatic = opcode {
        let index = parse_u2_vec(index, &code);
        let fieldref = clazz.cp(index);
        let Some(Constant::FieldRef {
            class_index,
            name_and_type_index,
        }) = fieldref else {panic!();};

        let name_of_class = get_name_of_class(clazz, *class_index);
        let name_of_member = get_name_of_member(clazz, *name_and_type_index);
        if name_of_class == "java/lang/System" && name_of_member == "out" {
            stack.push(JavaValue::FakePrintStream);
            return;
        } else {
            panic!("Unsupported member {name_of_class}/{name_of_member} in getstatic instruction");
        }
    } else if let Opcode::ldc = opcode {
        let index = parse_u1_vec(index, &code);
        let pool_item = clazz.cp(index.into());
        let value = if let Some(Constant::String { string_index }) = pool_item {
            let value = get_cp_string(clazz, *string_index);
            JavaValue::String { value }
        } else if let &Some(Constant::Float { value }) = pool_item {
            JavaValue::Float { value }
        } else if let &Some(Constant::Integer { value }) = pool_item {
            JavaValue::Integer { value }
        } else {
            panic!("ldc: value={:?}", pool_item);
        };
        stack.push(value);
    } else if let Opcode::ldc2_w = opcode {
        let pool_index = parse_u2_vec(index, &code);
        let pool_item = clazz.cp(pool_index);
        if let &Some(Constant::Double { value }) = pool_item {
            stack.push(JavaValue::Double { value });
        } else if let &Some(Constant::Long { value }) = pool_item {
            stack.push(JavaValue::Long { value });
        } else {
            panic!("ldc2_w: value={:?}", pool_item);
        }
    } else
    // int push
    if let Opcode::bipush = opcode {
        let byte = parse_u1_vec(index, &code);
        stack.push(JavaValue::Integer { value: byte.into() });
    } else
    // double
    if let Opcode::dconst_0 = opcode {
        stack.push(JavaValue::Double { value: 0.0 });
    } else if let Opcode::dconst_1 = opcode {
        stack.push(JavaValue::Double { value: 1.0 });
    } else
    // float
    if let Opcode::fconst_0 = opcode {
        stack.push(JavaValue::Float { value: 0.0 });
    } else if let Opcode::fconst_1 = opcode {
        stack.push(JavaValue::Float { value: 1.0 });
    } else if let Opcode::fconst_2 = opcode {
        stack.push(JavaValue::Float { value: 2.0 });
    } else
    // Return
    if let Opcode::return_ = opcode {
        return;
    } else
    // int
    if let Opcode::iconst_m1 = opcode {
        stack.push(JavaValue::Integer { value: -1 });
    } else if let Opcode::iconst_0 = opcode {
        stack.push(JavaValue::Integer { value: 0 });
    } else if let Opcode::iconst_1 = opcode {
        stack.push(JavaValue::Integer { value: 1 });
    } else if let Opcode::iconst_2 = opcode {
        stack.push(JavaValue::Integer { value: 2 });
    } else if let Opcode::iconst_3 = opcode {
        stack.push(JavaValue::Integer { value: 3 });
    } else if let Opcode::iconst_4 = opcode {
        stack.push(JavaValue::Integer { value: 4 });
    } else if let Opcode::iconst_5 = opcode {
        stack.push(JavaValue::Integer { value: 5 });
    } else
    // nop
    if let Opcode::nop = opcode {
        // this block left intentionally empty
    } else
    // sipush
    if let Opcode::sipush = opcode {
        let byte = parse_u2_vec(index, &code) as i16;
        stack.push(JavaValue::Integer { value: byte.into() });
    } else
    // long
    if let Opcode::lconst_0 = opcode {
        stack.push(JavaValue::Long { value: 0 });
    } else if let Opcode::lconst_1 = opcode {
        stack.push(JavaValue::Long { value: 1 });
    } else if let Opcode::aload_0 = opcode {
        stack.push(runtime.locals[0].clone());
    } else if let Opcode::invokespecial = opcode {
        let pool_index = parse_u2_vec(index, &code);
        let pool_item = clazz.cp_as_ref(pool_index);
        let &Constant::MethodRef {
            class_index,
            name_and_type_index,
        } = pool_item else {panic!();};
        let name_of_class = get_name_of_class(clazz, class_index);
        let name_of_member = get_name_of_member(clazz, name_and_type_index);
        if name_of_class == "java/lang/Object" && name_of_member == "<init>" {
            // Skip <init> for java/lang/Object
            return;
        } else {
            panic!("Unsupported member {name_of_class}/{name_of_member}");
        }
    } else {
        panic!("Unknown opcode {:?}", opcode);
    }
}

fn execute_code(runtime: &mut Runtime, clazz: &ParsedClass, code: &[u8]) {
    let mut index = 0;
    while index < code.len() {
        let raw_opcode = parse_u1_vec(&mut index, &code);
        let opt_opcode = FromPrimitive::from_u8(raw_opcode);
        if let None = opt_opcode {
            println!("opcode: {:?} = {:x}", opt_opcode, raw_opcode)
        };
        let opcode = opt_opcode.unwrap();
        execute_instruction(runtime, &opcode, &mut index, code, &clazz);
    }
}

#[derive(Clone, Debug)]
pub enum ConstantPrint<'a> {
    NameAndType { name: &'a str, descriptor: &'a str },
    Class { name: &'a str },
}

#[cfg(test)]
impl<'a> ConstantPrint<'a> {
    fn new(class: &'a ParsedClass, name_and_type_index: u16) -> Self {
        let pool_item = class.cp(name_and_type_index).as_ref().unwrap();
        match pool_item {
            Constant::NameAndType {
                name_index,
                descriptor_index,
            } => {
                let name = class.cp_as_ref(*name_index).str();
                let descriptor = class.cp_as_ref(*descriptor_index).str();
                ConstantPrint::NameAndType { name, descriptor }
            }
            Constant::Class { name_index } => {
                let name = class.cp_as_ref(*name_index).str();
                ConstantPrint::Class { name }
            }
            _ => {
                dbg!(pool_item);
                panic!()
            }
        }
    }

    fn class(&self) -> Option<&str> {
        match self {
            &ConstantPrint::Class { name } => Some(name),
            _ => panic!("invalid constant formatter {self:?}"),
        }
    }

    fn name_and_type(&self) -> Option<(String, String)> {
        match self {
            &ConstantPrint::NameAndType { name, descriptor } => {
                Some((name.into(), descriptor.into()))
            }
            _ => panic!("invalid constant formatter {self:?}"),
        }
    }
}

fn get_name_of_member(clazz: &ParsedClass, name_and_type_index: u16) -> String {
    let pool_item = clazz.cp(name_and_type_index);
    let Some(Constant::NameAndType { name_index, .. }) = pool_item else {panic!()};
    let Some(Constant::Utf8 { value }) = clazz.cp(*name_index) else {panic!()};
    return value.into();
}

fn get_name_of_class(clazz: &ParsedClass, class_index: u16) -> String {
    let pool_item = clazz.cp(class_index);
    let Some(Constant::Class { name_index, .. }) = pool_item else {panic!()};
    let Some(Constant::Utf8 { value }) = clazz.cp(*name_index) else {panic!()};
    return value.into();
}

fn parse_u2_vec(index: &mut usize, code: &[u8]) -> u16 {
    let value = &code[*index..*index + 2];
    let value = [value[1], value[0]];
    let value = (value[0] as u16) + ((value[1] as u16) << 8);
    *index += 2;
    value
}

fn parse_u1_vec(index: &mut usize, code: &[u8]) -> u8 {
    let value = code[*index];
    *index += 1;
    value
}

pub struct CodeInfo {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionInfo>,
    pub attributes: Vec<ParsedAttribute>,
}

impl CodeInfo {
    // Code_attribute { .. }
    pub(crate) fn parse(f: &mut dyn Read) -> Self {
        // u2 attribute_name_index;
        // u4 attribute_length;
        // u2 max_stack;
        // u2 max_locals;
        // u4 code_length;
        // u1 code[code_length];
        // u2 exception_table_length;
        // {   u2 start_pc;
        //     u2 end_pc;
        //     u2 handler_pc;
        //     u2 catch_type;
        // } exception_table[exception_table_length];
        // u2 attributes_count;
        // attribute_info attributes[attributes_count];
        let max_stack = parse_u2_raw(f);
        let max_locals = parse_u2_raw(f);
        let code_length = parse_u4_raw(f);
        let code = parse_vec_u8(f, code_length as u64);
        let exception_table = parse_exception_table(f);
        let attributes_count = parse_u2_raw(f);
        let attributes = parse_attributes(f, attributes_count);
        Self {
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        }
    }
}

pub struct ExceptionInfo {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

impl ExceptionInfo {
    fn parse(f: &mut dyn Read) -> ExceptionInfo {
        // u2 start_pc;
        // u2 end_pc;
        // u2 handler_pc;
        // u2 catch_type;
        let start_pc = parse_u2_raw(f);
        let end_pc = parse_u2_raw(f);
        let handler_pc = parse_u2_raw(f);
        let catch_type = parse_u2_raw(f);
        ExceptionInfo {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        }
    }
}

fn parse_exception_table(f: &mut dyn Read) -> Vec<ExceptionInfo> {
    let exception_table_length = parse_u2_raw(f);
    (0..exception_table_length)
        .map(|_| ExceptionInfo::parse(f))
        .collect()
}

fn find_attributes_by_name<'a>(
    clazz: &ParsedClass,
    attributes: &'a [ParsedAttribute],
    name: &str,
) -> Vec<&'a ParsedAttribute> {
    let iter = attributes.into_iter();
    let iter = iter.filter(|&attr| {
        let Some(Constant::Utf8 { value, .. }) = clazz.cp(attr.attribute_name_index) else {
            return false;
        };
        name == value
    });
    iter.collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn constant_get_str() {
        let c_val: Constant = Constant::Utf8 {
            value: "str".into(),
        };
        assert_eq!(c_val.str(), "str");
    }

    #[test]
    fn constant_print() {
        let mut box_list = vec![];
        box_list.push(Box::new(ParsedClass::with_constant_pool(vec![
            Some(Constant::Class { name_index: 2 }),
            Some(Constant::Utf8 {
                value: "str1".into(),
            }),
        ])));
        let clazz1 = box_list.last().unwrap().as_ref();
        let const_print1 = ConstantPrint::new(clazz1, 1);
        assert_eq!(const_print1.class().unwrap(), "str1");

        box_list.push(Box::new(ParsedClass::with_constant_pool(vec![
            Some(Constant::NameAndType {
                name_index: 2,
                descriptor_index: 3,
            }),
            Some(Constant::Utf8 {
                value: "str1".into(),
            }),
            Some(Constant::Utf8 {
                value: "str2".into(),
            }),
        ])));
        let clazz2 = box_list.last().unwrap().as_ref();
        let const_print2 = ConstantPrint::new(clazz2, 1);
        let obj2 = const_print2.name_and_type().unwrap();
        assert_eq!(obj2.0, "str1".to_owned());
        assert_eq!(obj2.1, "str2".to_owned());
    }
}
