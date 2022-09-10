use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use std::{fs, path::PathBuf, process::Command};
use uuid::Uuid;

#[derive(Debug, Deserialize, PartialEq)]
struct Album {
    name: String,
    base: String,
    images: Vec<String>,
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
    let filelist = prepare_filelists(&album, &out_base)?;

    fs::create_dir(out_base)?;
    filelist
        .iter()
        .map(|(in_file, out_file)| {
            Command::new("gm")
                .arg("convert")
                .arg("-size")
                .arg("1920x1080")
                .arg("-normalize")
                .arg("-enhance")
                .arg("-unsharp")
                .arg("3")
                .arg(in_file)
                .arg(out_file)
                .output()?;
            Ok(())
        })
        .collect::<anyhow::Result<Vec<()>>>()?;
    Command::new("dolphin")
        .arg("--new-window")
        .arg(out_base)
        .output()?;
    filelist
        .iter()
        .map(|(_, out_file)| {
            fs::remove_file(out_file)?;
            Ok(())
        })
        .collect::<anyhow::Result<Vec<()>>>()?;
    fs::remove_dir(out_base)?;

    Ok(())
}

fn prepare_filelists(album: &Album, out_base: &str) -> Result<Vec<(String, String)>> {
    let max_len = get_max_length(album.images.len());

    album
        .images
        .iter()
        .enumerate()
        .map(|(idx, elem)| {
            let mut in_path = PathBuf::new();

            in_path.push(album.base.clone());
            in_path.push(elem);

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

            Ok((in_path.to_string(), out_path.to_string()))

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
