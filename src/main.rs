extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};

fn unwrap_value(v: &Yaml) {
    match v {
        Yaml::Hash(v) => {
            for (k, vv) in v.iter() {
                match vv {
                    Yaml::String(_vv) => {
                        print!("\t\t {}: ", k.as_str().unwrap());
                    },
                    Yaml::Integer(_vv) => {
                        print!("\t\t {}: ", k.as_str().unwrap());
                    },
                    _ => {
                        println!("\t* {}:", k.as_str().unwrap());
                    }
                }
                unwrap_value(vv);
            }
        },
        Yaml::Array(v) => {
            for h in v.iter() {
                unwrap_value(h);
            }
        },
        Yaml::String(v) => {
            let tot: f32 = v.split("+").fold(0., |sum, s| sum + s.trim().parse::<f32>().unwrap());
            println!("{}", tot);
        },
        Yaml::Integer(v) => println!("{}", v),
        _ => (),
    }
}

fn main() -> Result<(), std::io::Error> {
    let doc = std::fs::read_to_string("finances.yaml")?;
    let data = YamlLoader::load_from_str(&doc).unwrap();
    let doc = &data[0];

    let map = doc.as_hash().unwrap();

    for (k, v) in map.iter() {
        println!("{}: ", k.as_str().unwrap());
        unwrap_value(v);
        println!();
    }

    Ok(())
}
