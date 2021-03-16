use std::env;

extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};

fn main() -> Result<(), std::io::Error> {
    let fin_file = env::var("FIN_FILE").unwrap_or("finances.yaml".to_string());

    let doc = std::fs::read_to_string(fin_file)?;
    let data = YamlLoader::load_from_str(&doc).unwrap();
    let doc = &data[0];

    let map = doc.as_hash().unwrap();

    for (k, v) in map.iter() {
        println!("{}: ", k.as_str().unwrap());
        let sum = unwrap_values(v);
        println!("== {}", sum);
        println!();
    }

    Ok(())
}

fn unwrap_values(v: &Yaml) -> f32 {
    match v {
        Yaml::Hash(v) => {
            let mut sum = 0.;
            for (k, vv) in v.iter() {
                match vv {
                    Yaml::String(_vv) => {
                        print!("\t\t {}: ", k.as_str().unwrap());
                    }
                    Yaml::Integer(_vv) => {
                        print!("\t\t {}: ", k.as_str().unwrap());
                    }
                    _ => {
                        println!("\t* {}:", k.as_str().unwrap());
                    }
                }
                sum += unwrap_values(vv);
            }
            sum
        }
        Yaml::Array(v) => {
            let mut sub_sum = 0.;
            for h in v.iter() {
                sub_sum += unwrap_values(h);
            }
            println!("\t= {}", sub_sum);
            sub_sum
        }
        Yaml::String(v) => {
            let tot: f32 = v
                .split("+")
                .fold(0., |sum, s| sum + s.trim().parse::<f32>().unwrap());
            println!("{}", tot);
            tot
        }
        Yaml::Integer(v) => {
            println!("{}", v);
            *v as f32
        }
        _ => 0.,
    }
}
