use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use std::{collections::HashMap, fmt::Display, fs, path::PathBuf, process::Command};
use uuid::Uuid;

#[derive(Debug, Deserialize, PartialEq)]
struct Album {
    name: String,
    base: String,
    transformations: Option<HashMap<String, Vec<Transformation>>>,
    images: Vec<Image>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
enum Transformation {
    Size { width: u32, height: u32 },
    Normalize,
    Enhance,
    Unsharp { radius: u32 },
}

#[derive(Debug, Deserialize, PartialEq)]
struct Image {
    filename: String,
    transformations: Option<String>,
}

impl Image {
    pub fn to_string(&self) -> String {
        self.filename.clone()
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.filename)
    }
}

struct ParameterSet {
    input: String,
    output: String,
    transformations: Vec<Transformation>,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path of the configuration file
    #[clap(short, long, value_parser)]
    conf: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let document = String::from_utf8(fs::read(args.conf)?)?;
    let mut out_base = PathBuf::new();

    out_base.push("/tmp");
    out_base.push(format!("album_creator_{}", Uuid::new_v4().to_string()));

    let out_base = out_base
        .to_str()
        .ok_or(anyhow::anyhow!("could not generate output base path"))?;

    let album: Album = serde_json::from_str(&document).unwrap();
    let parameters = prepare_parameters(&album, &out_base)?;

    fs::create_dir(out_base)?;
    parameters
        .iter()
        .map(|parameter_set| {
            let mut com = Command::new("gm");

            com.arg("convert");

            for trans in &parameter_set.transformations {
                match trans {
                    Transformation::Enhance => com.arg("-enhance"),
                    Transformation::Normalize => com.arg("-normalize"),
                    Transformation::Size { width, height } => {
                        com.arg("-size").arg(format!("{}x{}", width, height))
                    }
                    Transformation::Unsharp { radius } => {
                        com.arg("-unsharp").arg(format!("{}", radius))
                    }
                };
            }

            com.arg(parameter_set.input.clone())
                .arg(parameter_set.output.clone())
                .output()?;
            Ok(())
        })
        .collect::<anyhow::Result<Vec<()>>>()?;
    Command::new("dolphin")
        .arg("--new-window")
        .arg(out_base)
        .output()?;
    parameters
        .iter()
        .map(|parameter_set| {
            fs::remove_file(parameter_set.output.clone())?;
            Ok(())
        })
        .collect::<anyhow::Result<Vec<()>>>()?;
    fs::remove_dir(out_base)?;

    Ok(())
}

fn prepare_parameters(album: &Album, out_base: &str) -> Result<Vec<ParameterSet>> {
    let max_len = get_max_length(album.images.len());
    let mut transformations: HashMap<String, Vec<Transformation>> = match &album.transformations {
        Some(trans) => (*trans).clone(),
        None => HashMap::new(),
    };

    if !transformations.contains_key("default") {
        transformations.insert(
            String::from("default"),
            vec![
                Transformation::Size {
                    width: 1920,
                    height: 1080,
                },
                Transformation::Normalize,
                Transformation::Enhance,
                Transformation::Unsharp { radius: 3 },
            ],
        );
    }
    album
        .images
        .iter()
        .enumerate()
        .map(|(idx, elem)| {
            let mut in_path = PathBuf::new();

            in_path.push(album.base.clone());
            in_path.push(elem.to_string());

            let in_path = in_path
                .to_str()
                .ok_or(anyhow::anyhow!("could not generate input path"))?;

            let mut out_path = PathBuf::new();

            out_path.push(out_base);
            out_path.push(format!(
                "{}_{}",
                pad_left((idx + 1).to_string(), max_len),
                elem
            ));

            let out_path = out_path
                .to_str()
                .ok_or(anyhow::anyhow!("could not generate output path"))?;

            Ok(ParameterSet {
                input: in_path.to_string(),
                output: out_path.to_string(),
                transformations: match &elem.transformations {
                    Some(trans) => transformations[trans].clone(),
                    None => transformations["default"].clone(),
                },
            })
        })
        .collect()
}

fn get_max_length(num: usize) -> usize {
    format!("{}", num).len()
}

fn pad_left(str: String, len: usize) -> String {
    let mut out = str;

    while out.len() < len {
        out = format!("0{}", out);
    }

    out
}

#[test]
fn get_max_length_1() {
    assert_eq!(get_max_length(5), 1);
    assert_eq!(get_max_length(14), 2);
    assert_eq!(get_max_length(3472), 4);
}

#[test]
fn prepare_parameters_1() {
    let mut trans = HashMap::new();

    trans.insert(
        String::from("default"),
        vec![Transformation::Normalize, Transformation::Enhance],
    );

    let album = Album {
        base: String::from("/temp_in/"),
        name: String::from("test name"),
        images: vec![Image {
            filename: String::from("image 1"),
            transformations: None,
        }],
        transformations: Some(trans),
    };

    let res = prepare_parameters(&album, "/tmp_out/").unwrap();

    assert_eq!(res.len(), 1);
    assert_eq!(res[0].transformations.len(), 2);
}
