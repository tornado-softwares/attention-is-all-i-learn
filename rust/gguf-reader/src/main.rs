use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Read, Result};

pub fn read_string_le<R: Read>(reader: &mut R, len: usize) -> Result<String> {
    let mut buffer = vec![0; len];
    reader.read_exact(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer).to_string())
}

pub fn read_u32_le<R: Read>(reader: &mut R) -> io::Result<u32> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

pub fn read_u64_le<R: Read>(reader: &mut R) -> io::Result<u64> {
    let mut buf = [0; 8];
    reader.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}

pub fn read_u8_le<R: Read>(reader: &mut R) -> Result<u8> {
    let mut buffer = [0; 1];
    reader.read_exact(&mut buffer)?;
    Ok(buffer[0])
}

pub fn read_i8_le<R: Read>(reader: &mut R) -> Result<i8> {
    let mut buffer = [0; 1];
    reader.read_exact(&mut buffer)?;
    Ok(i8::from_le_bytes(buffer))
}

pub fn read_u16_le<R: Read>(reader: &mut R) -> Result<u16> {
    let mut buffer = [0; 2];
    reader.read_exact(&mut buffer)?;
    Ok(u16::from_le_bytes(buffer))
}

pub fn read_i16_le<R: Read>(reader: &mut R) -> Result<i16> {
    let mut buffer = [0; 2];
    reader.read_exact(&mut buffer)?;
    Ok(i16::from_le_bytes(buffer))
}

pub fn read_i32_le<R: Read>(reader: &mut R) -> Result<i32> {
    let mut buffer = [0; 4];
    reader.read_exact(&mut buffer)?;
    Ok(i32::from_le_bytes(buffer))
}

pub fn read_f32_le<R: Read>(reader: &mut R) -> Result<f32> {
    let mut buffer = [0; 4];
    reader.read_exact(&mut buffer)?;
    Ok(f32::from_le_bytes(buffer))
}

pub fn read_bool_le<R: Read>(reader: &mut R) -> Result<bool> {
    let value = read_u8_le(reader)?;
    Ok(value != 0)
}

pub fn read_i64_le<R: Read>(reader: &mut R) -> Result<i64> {
    let mut buffer = [0; 8];
    reader.read_exact(&mut buffer)?;
    Ok(i64::from_le_bytes(buffer))
}

pub fn read_f64_le<R: Read>(reader: &mut R) -> Result<f64> {
    let mut buffer = [0; 8];
    reader.read_exact(&mut buffer)?;
    Ok(f64::from_le_bytes(buffer))
}

#[derive(Debug)]
enum Value {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    F32(f32),
    Bool(bool),
    String(String),
    Array(Vec<Value>),
    U64(u64),
    I64(i64),
    F64(f64),
}
 
 
fn main() -> Result<()> {
    let mut file = File::open(
        "/home/tornado-softwares/Bureau/tornado/models/gguf/Oxy-1-micro-Qwen-1.5B-F16.gguf",
    )?;

    let magic = read_u32_le(&mut file)?;
    let filetype_version = read_u32_le(&mut file)?;
    let tensor_count = read_u64_le(&mut file)?;
    let metadata_kv_count = read_u64_le(&mut file)?;
    let mut metadata = HashMap::new();

    println!("Magic : {:#X}", magic);
    println!("GGUF version: {filetype_version}");
    println!("Tensor count: {tensor_count}");
    println!("Metadata kv count: {metadata_kv_count}\n");

    for i in 0..metadata_kv_count {
        let key_len = read_u64_le(&mut file)? as usize;
        let key = read_string_le(&mut file, key_len)?;
        let value_type = read_u32_le(&mut file)?;
        let value = match value_type {
            0 => Value::U8(read_u8_le(&mut file)?),
            1 => Value::I8(read_i8_le(&mut file)?),
            2 => Value::U16(read_u16_le(&mut file)?),
            3 => Value::I16(read_i16_le(&mut file)?),
            4 => Value::U32(read_u32_le(&mut file)?),
            5 => Value::I32(read_i32_le(&mut file)?),
            6 => Value::F32(read_f32_le(&mut file)?),
            7 => Value::Bool(read_bool_le(&mut file)?),
            8 => {
                let str_len = read_u64_le(&mut file)? as usize;
                Value::String(read_string_le(&mut file, str_len)?)
            }
            9 => {
                let elem_type = read_u32_le(&mut file)?;
                let count = read_u64_le(&mut file)?;
                let mut elements = Vec::new();

                for _ in 0..count {
                    elements.push(match elem_type {
                        0 => Value::U8(read_u8_le(&mut file)?),
                        1 => Value::I8(read_i8_le(&mut file)?),
                        2 => Value::U16(read_u16_le(&mut file)?),
                        3 => Value::I16(read_i16_le(&mut file)?),
                        4 => Value::U32(read_u32_le(&mut file)?),
                        5 => Value::I32(read_i32_le(&mut file)?),
                        6 => Value::F32(read_f32_le(&mut file)?),
                        7 => Value::Bool(read_bool_le(&mut file)?),
                        8 => {
                            let str_len = read_u64_le(&mut file)? as usize;
                            Value::String(read_string_le(&mut file, str_len)?)
                        }
                        10 => Value::U64(read_u64_le(&mut file)?),
                        11 => Value::I64(read_i64_le(&mut file)?),
                        12 => Value::F64(read_f64_le(&mut file)?),
                        _ => panic!("Unsupported array element type in array"),
                    });
                }
                Value::Array(elements)
            }
            10 => Value::U64(read_u64_le(&mut file)?),
            11 => Value::I64(read_i64_le(&mut file)?),
            12 => Value::F64(read_f64_le(&mut file)?),
            _ => panic!("Unknown value type: {}", value_type),
        };
        if key.contains("tokenizer") {
            println!("Metadata {i} : {key} = flemme", i = i + 1);
        } else {
            println!("Metadata {i} : {key} = {value:?}", i = i + 1);
        }
        metadata.insert(key, value);
    }

    let quant = match metadata.get("general.file_type") {
        Some(Value::U32(0)) => "ALL_F32",
        Some(Value::U32(1)) => "MOSTLY_F16",
        Some(Value::U32(2)) => "MOSTLY_Q4_0",
        Some(Value::U32(3)) => "MOSTLY_Q4_1",
        Some(Value::U32(4)) => "MOSTLY_Q4_1_SOME_F16",
        Some(Value::U32(5)) => "MOSTLY_Q4_2",
        Some(Value::U32(6)) => "MOSTLY_Q4_3",
        Some(Value::U32(7)) => "MOSTLY_Q8_0",
        Some(Value::U32(8)) => "MOSTLY_Q5_0",
        Some(Value::U32(9)) => "MOSTLY_Q5_1",
        Some(Value::U32(10)) => "MOSTLY_Q2_K",
        Some(Value::U32(11)) => "MOSTLY_Q3_K_S",
        Some(Value::U32(12)) => "MOSTLY_Q3_K_M",
        Some(Value::U32(13)) => "MOSTLY_Q3_K_L",
        Some(Value::U32(14)) => "MOSTLY_Q4_K_S",
        Some(Value::U32(15)) => "MOSTLY_Q4_K_M",
        Some(Value::U32(16)) => "MOSTLY_Q5_K_S",
        Some(Value::U32(17)) => "MOSTLY_Q5_K_M",
        Some(Value::U32(18)) => "MOSTLY_Q6_K",
        _ => "UNKNOWN",
    };

    println!("\nQuantisation : {quant}\n");
    let  mut params = 0;

    for i in 0..tensor_count {
        let name_len = read_u64_le(&mut file)? as usize;
        let name = read_string_le(&mut file, name_len)?;
        let dimensions = read_u32_le(&mut file)?;
        let mut shape = [1; 4];
        let mut dimensions_data = HashMap::new(); 
        for y in 0..dimensions {
            shape[y as usize] = read_u64_le(&mut file)?;
            dimensions_data.insert(y, shape[y as usize]);
        }

        let tensor_type = read_u32_le(&mut file)?;
        let offset = read_u64_le(&mut file)?;

        let (block_size, type_size) = match tensor_type {
            0 => (1, 4),
            1 => (1, 2),
            2 => (32, 2 + 16),
            3 => (32, 2 + 16),
            4 => (32, 2 + 4 + 16),
            5 => (32, 2 + 2 + 16),
            6 => (32, 2 + 16),
            7 => (32, 4 + 16),
            8 => (32, 2 + 16),
            9 => (32, 2 + 16),
            10 => (256, 2 + 2 + 12 + 128),
            11 => (256, 2 + 2 + 12 + 32 + 128),
            12 => (256, 32 + 128),
            _ => (1, 0),
        };
        let quant = match tensor_type {
            0 => "ALL_F32",
            1 => "MOSTLY_F16",
            2 => "MOSTLY_Q4_0",
            3 => "MOSTLY_Q4_1",
            4 => "MOSTLY_Q4_1_SOME_F16",
            5 => "MOSTLY_Q4_2",
            6 => "MOSTLY_Q4_3",
            7 => "MOSTLY_Q8_0",
            8 => "MOSTLY_Q5_0",
            9 => "MOSTLY_Q5_1",
            10 => "MOSTLY_Q2_K",
            11 => "MOSTLY_Q3_K_S",
            12 => "MOSTLY_Q3_K_M",
            13 => "MOSTLY_Q3_K_L",
            14 => "MOSTLY_Q4_K_S",
            15 => "MOSTLY_Q4_K_M",
            16 => "MOSTLY_Q5_K_S",
            17 => "MOSTLY_Q5_K_M",
            18 => "MOSTLY_Q6_K",
            _ => "UNKNOWN",
        };

        let parameters = shape[0] * shape[1] * shape[2] * shape[3];
        let size = parameters * type_size / block_size;
        params+= parameters;
        println!(
            "Tensor Data {i} : {name} {quant} : dimensions={dimensions_data:?} tensor_type={tensor_type} block_size={block_size} parameters={parameters} size={size} offset={offset}",
            i = i + 1
        )
    }
    println!("{params}");

    Ok(())
}
