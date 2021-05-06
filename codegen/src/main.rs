use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use prost::Message;
use prost_types::{DescriptorProto, FileDescriptorSet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_descriptor_set_bytes = include_bytes!("../static/device.protoset");
    let file_descriptor_set = FileDescriptorSet::decode(&file_descriptor_set_bytes[..]).unwrap();

    dbg!(&file_descriptor_set);

    for fd in file_descriptor_set.file {
        let name = fd.name.unwrap();

        let dir = format!("./proto/{}", Path::new(&name).parent().unwrap().to_str().unwrap());
        fs::create_dir_all(dir)?;

        let mut file = File::create(format!("./proto/{}", name))?;

        file.write_all(format!("syntax = \"{}\";\n\n", fd.syntax.unwrap()).as_bytes())?;
        file.write_all(format!("package {};\n\n", fd.package.unwrap()).as_bytes())?;

        if let Some(options) = fd.options {
            if let Some(go_package) = options.go_package {
                file.write_all(format!("option go_package = \"{}\";\n\n", go_package).as_bytes())?;
            }
        }

        for dep in &fd.dependency {
            file.write_all(format!("import \"{}\";\n", dep).as_bytes())?;
        }
        if fd.dependency.len() > 0 {
            file.write_all(b"\n")?;
        }

        for msg in fd.message_type {
            render_descriptor_proto(&mut file, &msg)?;
        }

        for e in fd.enum_type {
            file.write_all(format!("enum {} {{\n", e.name.unwrap()).as_bytes())?;
            for value in e.value {
                file.write_all(format!("\t{} = {};\n", value.name.unwrap(), value.number.unwrap()).as_bytes())?;
            }
            file.write_all(b"}\n\n")?;
        }

        file.flush()?;
    }

    Ok(())
}

fn render_descriptor_proto(file: &mut File, msg: &DescriptorProto) -> Result<(), Box<dyn std::error::Error>> {
    file.write_all(format!("message {} {{\n", msg.name.clone().unwrap()).as_bytes())?;
    for field in &msg.field {
        if field.oneof_index.is_none() {
            file.write_all(b"\t")?;
            if let Some(label) = field.label {
                file.write_all(format!("{} ", fmt_field_label(label)).as_bytes())?;
            }
            file.write_all(
                format!(
                    "{} {} = {}",
                    fmt_field_type(field.r#type.unwrap(), field.type_name.clone()),
                    field.name.clone().unwrap(),
                    field.number.unwrap()
                )
                .as_bytes(),
            )?;
            if let Some(json_name) = &field.json_name {
                file.write_all(format!(" [json_name=\"{}\"];\n", json_name).as_bytes())?;
            } else {
                file.write_all(b";\n")?;
            }
        }
    }

    for oneof in &msg.oneof_decl {
        file.write_all(format!("\toneof {} {{\n", oneof.name.clone().unwrap()).as_bytes())?;

        for field in &msg.field {
            if field.oneof_index.is_some() {
                file.write_all(
                    format!(
                        "\t\t{} {} = {}",
                        fmt_field_type(field.r#type.unwrap(), field.type_name.clone()),
                        field.name.clone().unwrap(),
                        field.number.unwrap()
                    )
                    .as_bytes(),
                )?;
                if let Some(json_name) = &field.json_name {
                    file.write_all(format!(" [json_name=\"{}\"];\n", json_name).as_bytes())?;
                } else {
                    file.write_all(b";\n")?;
                }
            }
        }

        if let Some(options) = &oneof.options {
            for _option in &options.uninterpreted_option {
                unimplemented!(); // TODO - check back here if this ever occours
            }
        }

        file.write_all(b"\t}\n")?;
    }

    for n_msg in &msg.nested_type {
        render_descriptor_proto(file, n_msg)?;
    }

    for n_e in &msg.enum_type {
        file.write_all(format!("\tenum {} {{\n", n_e.name.clone().unwrap()).as_bytes())?;
        for value in n_e.value.clone() {
            file.write_all(format!("\t\t{} = {};\n", value.name.unwrap(), value.number.unwrap()).as_bytes())?;
        }
        file.write_all(b"\t}\n")?;
    }

    file.write_all(b"}\n\n")?;

    Ok(())
}

// https://docs.rs/prost-types/0.7.0/src/prost_types/protobuf.rs.html#214
fn fmt_field_label(v: i32) -> String {
    match v {
        1 => "optional".into(),
        2 => "required".into(),
        3 => "repeated".into(),
        _ => unreachable!(),
    }
}

// https://docs.rs/prost-types/0.7.0/src/prost_types/protobuf.rs.html#178
fn fmt_field_type(v: i32, type_name: Option<String>) -> String {
    match v {
        1 => "double".into(),
        2 => "float".into(),
        3 => "int64".into(),
        4 => "uint64".into(),
        5 => "int32".into(),
        6 => "fixed64".into(),
        7 => "fixed32".into(),
        8 => "bool".into(),
        9 => "string".into(),
        10 => "group".into(),
        11 => type_name.unwrap(), // message
        12 => "bytes".into(),
        13 => "uint32".into(),
        14 => type_name.unwrap(), // enum
        15 => "sfixed32".into(),
        16 => "sfixed64".into(),
        17 => "sint32".into(),
        18 => "sint64".into(),
        _ => unreachable!(),
    }
}
