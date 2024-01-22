use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

use anyhow::Result;
use clap::Parser;
use clap::{Args, Subcommand};
use std::fs;
use std::str::FromStr;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 将隐秘信息编码进PNG图片
    Encode(EncodeArgs),
    /// 将隐秘信息从PNG图片中解码
    Decode(DecodeArgs),
    /// 将隐秘信息从PNG图片中删除
    Remove(RemoveArgs),
}

#[derive(Args)]
pub struct EncodeArgs {
    /// 文件的路径
    #[arg(short, long, required = true)]
    file_path: String,
    /// 合法的自定义的类型
    #[arg(short, long, default_value = "ruSt")]
    chunk_type: String,
    /// 需要编码的信息
    #[arg(short, long, required = true)]
    message: String,
    /// 输出的文件路径
    #[arg(short, long)]
    output_file: Option<String>,
}

#[derive(Args)]
pub struct DecodeArgs {
    /// 文件的路径
    #[arg(short, long, required = true)]
    file_path: String,
    #[arg(short, long, default_value = "ruSt")]
    /// 合法的自定义的类型
    chunk_type: String,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// 文件的路径
    #[arg(short, long, required = true)]
    file_path: String,
    #[arg(short, long, default_value = "ruSt")]
    /// 合法的自定义的类型
    chunk_type: String,
}

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: &EncodeArgs) -> Result<()> {
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let data: Vec<u8> = args.message.bytes().collect();
    let chunk = Chunk::new(chunk_type, data);

    let contents = fs::read(&args.file_path)?;
    let mut png = Png::try_from(&contents[..])?;
    png.append_chunk(chunk);

    if let Some(output_file) = &args.output_file {
        fs::write(output_file, png.as_bytes())?;
    } else {
        fs::write(&args.file_path, png.as_bytes())?;
    }

    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: &DecodeArgs) -> Result<()> {
    let image = fs::read(&args.file_path)?;
    let png = Png::try_from(&image[..])?;
    if let Some(chunk) = png.chunk_by_type(&args.chunk_type) {
        let de = chunk.data_as_string()?;
        println!("🍺你的隐藏信息是：{}", de);
    } else {
        println!("🔥没有发现类型：{}的隐藏信息", args.chunk_type)
    }
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: &RemoveArgs) -> Result<()> {
    let image = fs::read(&args.file_path)?;
    let mut png = Png::try_from(&image[..])?;
    png.remove_chunk(&args.chunk_type)?;
    fs::write(&args.file_path, png.as_bytes())?;
    Ok(())
}
