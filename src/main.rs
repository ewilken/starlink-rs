use prost::Message;

pub fn main() {
    // let file_descriptor_set_bytes = include_bytes!("../proto/dish.protoset");
    // let file_descriptor_set =
    //     prost_types::FileDescriptorSet::decode(&file_descriptor_set_bytes[..]).unwrap();

    // dbg!(&file_descriptor_set);

    // for fd in file_descriptor_set.file {
    //     let mut file = File::create(format!("proto/{}", fd.name.unwrap()))?;

    //     file.write_all(format!("syntax = \"{}\";\n\n", fd.syntax.unwrap()).as_bytes())?;
    //     file.write_all(format!("package {};\n\n", fd.package.unwrap()).as_bytes())?;

    //     file.flush()?;
    // }
}
