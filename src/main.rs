use std::collections::BTreeMap;
use clap::{App,Arg};

#[derive(Default, Debug, Clone)]
pub struct KeyDirEntry {
    file_id: u32,
    data_entry_position: u32,
}
fn main() {
    let n = 1_500_000_000_u32;
    let matches = App::new("Btree Benchmarking")
        .version("1.0")
        .author("")
        .about("")
        .arg(Arg::with_name("ds")
            .short('d')
            .long("data")
            .value_name("DATA")
            .takes_value(true))
        .get_matches();

    if let Some(ds) = matches.value_of("ds") {
        if ds == "vec" {
            let mut btree = BTreeMap::new();
            for i in 0..n{
                let i = i as u8 % u8::MAX;
                let key = [i;32];
                let value = KeyDirEntry::default();
                btree.insert(key,value);
            }
        }else {
            let mut btree = BTreeMap::new();
            for i in 0..n {
                let i = i as u8 % u8::MAX;
                let key = [i;32];
                let value = KeyDirEntry::default();
                btree.insert(key.to_vec(),value);
            }
        }
    }


}
