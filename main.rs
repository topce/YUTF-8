use std::env;
use std::io::{self, Read};
use once_cell::sync::Lazy;

mod mappings;
mod conversion;

use conversion::{convert_from_yutf8, convert_to_yutf8, ScriptType};

fn main() -> io::Result<()> {
    // Get input from either command line arguments or stdin
    let input = if env::args().len() > 1 {
        let args: Vec<String> = env::args().collect();
        if args.len() >= 3 {
            let mode = &args[1];
            let text = args[2..].join(" ");
            
            match mode.as_str() {
                "latin" => {
                    println!("Converting yutf8 to Serbian Latin/Croatian/Slovenian");
                    convert_from_yutf8(&text, ScriptType::Latin)
                },
                "cyrillic" => {
                    println!("Converting yutf8 to Serbian Cyrillic");
                    convert_from_yutf8(&text, ScriptType::SerbianCyrillic)
                },
                "macedonian" => {
                    println!("Converting yutf8 to Macedonian");
                    convert_from_yutf8(&text, ScriptType::Macedonian)
                },
                "to_latin" => {
                    println!("Converting UTF-8 Serbian Latin/Croatian/Slovenian to yutf8");
                    convert_to_yutf8(&text, ScriptType::Latin)
                },
                "to_cyrillic" => {
                    println!("Converting UTF-8 Serbian Cyrillic to yutf8");
                    convert_to_yutf8(&text, ScriptType::SerbianCyrillic)
                },
                "to_macedonian" => {
                    println!("Converting UTF-8 Macedonian to yutf8");
                    convert_to_yutf8(&text, ScriptType::Macedonian)
                },
                _ => {
                    println!("Unknown mode. Usage: program [latin|cyrillic|macedonian|to_latin|to_cyrillic|to_macedonian] text");
                    String::new()
                }
            }
        } else {
            println!("Usage: program [latin|cyrillic|macedonian|to_latin|to_cyrillic|to_macedonian] text");
            String::new()
        }
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        println!("No mode specified. Usage: program [latin|cyrillic|macedonian|to_latin|to_cyrillic|to_macedonian] text");
        String::new()
    };

    if !input.is_empty() {
        println!("{}", input);
        
        // Also show hexadecimal representation for UTF-8 bytes
        println!("\nUTF-8 Hex representation:");
        for byte in input.bytes() {
            print!("{:02X} ", byte);
        }
        println!();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversion::*;

    #[test]
    fn test_latin_roundtrip() {
        let original = "ABCČĆDĐEFGHIJKLMNOPQRSŠTUVWXYZŽ abcčćdđefghijklmnopqrsštuvwxyzž";
        let yutf8 = convert_to_yutf8(original, ScriptType::Latin);
        let converted_back = convert_from_yutf8(&yutf8, ScriptType::Latin);
        assert_eq!(original, converted_back);
    }

    #[test]
    fn test_serbian_cyrillic_roundtrip() {
        let original = "АБВГДЂЕЖЗИЈКЛЉМНЊОПРСТЋУФХЦЧЏШ абвгдђежзијклљмнњопрстћуфхцчџш";
        let yutf8 = convert_to_yutf8(original, ScriptType::SerbianCyrillic);
        let converted_back = convert_from_yutf8(&yutf8, ScriptType::SerbianCyrillic);
        assert_eq!(original, converted_back);
    }

    #[test]
    fn test_macedonian_roundtrip() {
        let original = "АБВГДЃЕЖЗЅИЈКЛЉМНЊОПРСТЌУФХЦЧЏШ абвгдѓежзѕијклљмнњопрстќуфхцчџш";
        let yutf8 = convert_to_yutf8(original, ScriptType::Macedonian);
        let converted_back = convert_from_yutf8(&yutf8, ScriptType::Macedonian);
        assert_eq!(original, converted_back);
    }

    #[test]
    fn test_latin_to_yutf8() {
        let input = "ČĆĐŠŽ čćđšž";
        let expected = "^]\\[@ ~}|{`";
        assert_eq!(convert_to_yutf8(input, ScriptType::Latin), expected);
    }

    #[test]
    fn test_serbian_cyrillic_to_yutf8() {
        let input = "ЉЊЏШЂЋЧ љњџшђћч";
        let expected = "QWX[\\]^ qwx{|}~";
        assert_eq!(convert_to_yutf8(input, ScriptType::SerbianCyrillic), expected);
    }

    #[test]
    fn test_macedonian_to_yutf8() {
        let input = "ЉЊЏЅШЃЌЧ љњџѕшѓќч";
        let expected = "QWXY[\\]^ qwxy{|}~";
        assert_eq!(convert_to_yutf8(input, ScriptType::Macedonian), expected);
    }

    #[test]
    fn test_yutf8_to_latin() {
        let input = "^]\\[@ ~}|{`";
        let expected = "ČĆĐŠŽ čćđšž";
        assert_eq!(convert_from_yutf8(input, ScriptType::Latin), expected);
    }

    #[test]
    fn test_yutf8_to_serbian_cyrillic() {
        let input = "QWX[\\]^ qwx{|}~";
        let expected = "ЉЊЏШЂЋЧ љњџшђћч";
        assert_eq!(convert_from_yutf8(input, ScriptType::SerbianCyrillic), expected);
    }

    #[test]
    fn test_yutf8_to_macedonian() {
        let input = "QWXY[\\]^ qwxy{|}~";
        let expected = "ЉЊЏЅШЃЌЧ љњџѕшѓќч";
        assert_eq!(convert_from_yutf8(input, ScriptType::Macedonian), expected);
    }

    #[test]
    fn test_unchanged_characters() {
        let input = "0123456789,.!? ";
        assert_eq!(convert_to_yutf8(input, ScriptType::Latin), input);
        assert_eq!(convert_to_yutf8(input, ScriptType::SerbianCyrillic), input);
        assert_eq!(convert_to_yutf8(input, ScriptType::Macedonian), input);
        assert_eq!(convert_from_yutf8(input, ScriptType::Latin), input);
        assert_eq!(convert_from_yutf8(input, ScriptType::SerbianCyrillic), input);
        assert_eq!(convert_from_yutf8(input, ScriptType::Macedonian), input);
    }
}


