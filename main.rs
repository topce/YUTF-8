use std::collections::HashMap;
use std::env;
use std::io::{self, Read};

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
                    convert_yutf8_to_latin(&text)
                },
                "cyrillic" => {
                    println!("Converting yutf8 to Serbian Cyrillic");
                    convert_yutf8_to_serbian_cyrillic(&text)
                },
                "macedonian" => {
                    println!("Converting yutf8 to Macedonian");
                    convert_yutf8_to_macedonian(&text)
                },
                "to_latin" => {
                    println!("Converting UTF-8 Serbian Latin/Croatian/Slovenian to yutf8");
                    convert_latin_to_yutf8(&text)
                },
                "to_cyrillic" => {
                    println!("Converting UTF-8 Serbian Cyrillic to yutf8");
                    convert_serbian_cyrillic_to_yutf8(&text)
                },
                "to_macedonian" => {
                    println!("Converting UTF-8 Macedonian to yutf8");
                    convert_macedonian_to_yutf8(&text)
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

// Functions to convert from UTF-8 to yutf8

fn convert_latin_to_yutf8(text: &str) -> String {
    let mut mapping = HashMap::new();
    
    // Special Latin characters used in Serbian Latin/Croatian/Slovenian
    mapping.insert('Ž', '@');  // 40
    mapping.insert('Š', '[');  // 5B
    mapping.insert('Đ', '\\'); // 5C
    mapping.insert('Ć', ']');  // 5D
    mapping.insert('Č', '^');  // 5E
    
    mapping.insert('ž', '`');  // 60
    mapping.insert('š', '{');  // 7B
    mapping.insert('đ', '|');  // 7C
    mapping.insert('ć', '}');  // 7D
    mapping.insert('č', '~');  // 7E
    
    // Standard Latin letters remain unchanged
    
    // Process the input text
    text.chars().map(|c| {
        *mapping.get(&c).unwrap_or(&c)
    }).collect()
}

fn convert_serbian_cyrillic_to_yutf8(text: &str) -> String {
    let mut mapping = HashMap::new();
    
    // Capital Cyrillic letters (Serbian)
    mapping.insert('А', 'A');  // 41
    mapping.insert('Б', 'B');  // 42
    mapping.insert('Ц', 'C');  // 43
    mapping.insert('Д', 'D');  // 44
    mapping.insert('Е', 'E');  // 45
    mapping.insert('Ф', 'F');  // 46
    mapping.insert('Г', 'G');  // 47
    mapping.insert('Х', 'H');  // 48
    mapping.insert('И', 'I');  // 49
    mapping.insert('Ј', 'J');  // 4A
    mapping.insert('К', 'K');  // 4B
    mapping.insert('Л', 'L');  // 4C
    mapping.insert('М', 'M');  // 4D
    mapping.insert('Н', 'N');  // 4E
    mapping.insert('О', 'O');  // 4F
    mapping.insert('П', 'P');  // 50
    mapping.insert('Љ', 'Q');  // 51
    mapping.insert('Р', 'R');  // 52
    mapping.insert('С', 'S');  // 53
    mapping.insert('Т', 'T');  // 54
    mapping.insert('У', 'U');  // 55
    mapping.insert('В', 'V');  // 56
    mapping.insert('Њ', 'W');  // 57
    mapping.insert('Џ', 'X');  // 58
    mapping.insert('З', 'Z');  // 5A
    mapping.insert('Ш', '[');  // 5B
    mapping.insert('Ђ', '\\'); // 5C
    mapping.insert('Ћ', ']');  // 5D
    mapping.insert('Ч', '^');  // 5E
    mapping.insert('Ж', '@');  // 40

    // Lowercase Cyrillic letters (Serbian)
    mapping.insert('а', 'a');  // 61
    mapping.insert('б', 'b');  // 62
    mapping.insert('ц', 'c');  // 63
    mapping.insert('д', 'd');  // 64
    mapping.insert('е', 'e');  // 65
    mapping.insert('ф', 'f');  // 66
    mapping.insert('г', 'g');  // 67
    mapping.insert('х', 'h');  // 68
    mapping.insert('и', 'i');  // 69
    mapping.insert('ј', 'j');  // 6A
    mapping.insert('к', 'k');  // 6B
    mapping.insert('л', 'l');  // 6C
    mapping.insert('м', 'm');  // 6D
    mapping.insert('н', 'n');  // 6E
    mapping.insert('о', 'o');  // 6F
    mapping.insert('п', 'p');  // 70
    mapping.insert('љ', 'q');  // 71
    mapping.insert('р', 'r');  // 72
    mapping.insert('с', 's');  // 73
    mapping.insert('т', 't');  // 74
    mapping.insert('у', 'u');  // 75
    mapping.insert('в', 'v');  // 76
    mapping.insert('њ', 'w');  // 77
    mapping.insert('џ', 'x');  // 78
    mapping.insert('з', 'z');  // 7A
    mapping.insert('ш', '{');  // 7B
    mapping.insert('ђ', '|');  // 7C
    mapping.insert('ћ', '}');  // 7D
    mapping.insert('ч', '~');  // 7E
    mapping.insert('ж', '`');  // 60

    // Process the input text
    text.chars().map(|c| {
        *mapping.get(&c).unwrap_or(&c)
    }).collect()
}

fn convert_macedonian_to_yutf8(text: &str) -> String {
    let mut mapping = HashMap::new();
    
    // Capital Cyrillic letters (Macedonian)
    mapping.insert('А', 'A');  // 41
    mapping.insert('Б', 'B');  // 42
    mapping.insert('Ц', 'C');  // 43
    mapping.insert('Д', 'D');  // 44
    mapping.insert('Е', 'E');  // 45
    mapping.insert('Ф', 'F');  // 46
    mapping.insert('Г', 'G');  // 47
    mapping.insert('Х', 'H');  // 48
    mapping.insert('И', 'I');  // 49
    mapping.insert('Ј', 'J');  // 4A
    mapping.insert('К', 'K');  // 4B
    mapping.insert('Л', 'L');  // 4C
    mapping.insert('М', 'M');  // 4D
    mapping.insert('Н', 'N');  // 4E
    mapping.insert('О', 'O');  // 4F
    mapping.insert('П', 'P');  // 50
    mapping.insert('Љ', 'Q');  // 51
    mapping.insert('Р', 'R');  // 52
    mapping.insert('С', 'S');  // 53
    mapping.insert('Т', 'T');  // 54
    mapping.insert('У', 'U');  // 55
    mapping.insert('В', 'V');  // 56
    mapping.insert('Њ', 'W');  // 57
    mapping.insert('Џ', 'X');  // 58
    mapping.insert('Ѕ', 'Y');  // 59 - Macedonian specific letter
    mapping.insert('З', 'Z');  // 5A
    mapping.insert('Ш', '[');  // 5B
    mapping.insert('Ѓ', '\\'); // 5C - Macedonian specific letter
    mapping.insert('Ќ', ']');  // 5D - Macedonian specific letter
    mapping.insert('Ч', '^');  // 5E
    mapping.insert('Ж', '@');  // 40

    // Lowercase Cyrillic letters (Macedonian)
    mapping.insert('а', 'a');  // 61
    mapping.insert('б', 'b');  // 62
    mapping.insert('ц', 'c');  // 63
    mapping.insert('д', 'd');  // 64
    mapping.insert('е', 'e');  // 65
    mapping.insert('ф', 'f');  // 66
    mapping.insert('г', 'g');  // 67
    mapping.insert('х', 'h');  // 68
    mapping.insert('и', 'i');  // 69
    mapping.insert('ј', 'j');  // 6A
    mapping.insert('к', 'k');  // 6B
    mapping.insert('л', 'l');  // 6C
    mapping.insert('м', 'm');  // 6D
    mapping.insert('н', 'n');  // 6E
    mapping.insert('о', 'o');  // 6F
    mapping.insert('п', 'p');  // 70
    mapping.insert('љ', 'q');  // 71
    mapping.insert('р', 'r');  // 72
    mapping.insert('с', 's');  // 73
    mapping.insert('т', 't');  // 74
    mapping.insert('у', 'u');  // 75
    mapping.insert('в', 'v');  // 76
    mapping.insert('њ', 'w');  // 77
    mapping.insert('џ', 'x');  // 78
    mapping.insert('ѕ', 'y');  // 79 - Macedonian specific letter
    mapping.insert('з', 'z');  // 7A
    mapping.insert('ш', '{');  // 7B
    mapping.insert('ѓ', '|');  // 7C - Macedonian specific letter
    mapping.insert('ќ', '}');  // 7D - Macedonian specific letter
    mapping.insert('ч', '~');  // 7E
    mapping.insert('ж', '`');  // 60

    // Process the input text
    text.chars().map(|c| {
        *mapping.get(&c).unwrap_or(&c)
    }).collect()
}

// Inverse functions to convert from yutf8 to UTF-8

fn convert_yutf8_to_latin(text: &str) -> String {
    let mut mapping = HashMap::new();
    
    // Map yutf8 to special Latin characters
    mapping.insert('@', 'Ž');  // 40
    mapping.insert('[', 'Š');  // 5B
    mapping.insert('\\', 'Đ'); // 5C
    mapping.insert(']', 'Ć');  // 5D
    mapping.insert('^', 'Č');  // 5E
    
    mapping.insert('`', 'ž');  // 60
    mapping.insert('{', 'š');  // 7B
    mapping.insert('|', 'đ');  // 7C
    mapping.insert('}', 'ć');  // 7D
    mapping.insert('~', 'č');  // 7E
    
    // Process the input text
    text.chars().map(|c| {
        *mapping.get(&c).unwrap_or(&c)
    }).collect()
}

fn convert_yutf8_to_serbian_cyrillic(text: &str) -> String {
    let mut mapping = HashMap::new();
    
    // Map yutf8 to Cyrillic characters (Serbian)
    // Capital letters
    mapping.insert('A', 'А');  // 41
    mapping.insert('B', 'Б');  // 42
    mapping.insert('C', 'Ц');  // 43
    mapping.insert('D', 'Д');  // 44
    mapping.insert('E', 'Е');  // 45
    mapping.insert('F', 'Ф');  // 46
    mapping.insert('G', 'Г');  // 47
    mapping.insert('H', 'Х');  // 48
    mapping.insert('I', 'И');  // 49
    mapping.insert('J', 'Ј');  // 4A
    mapping.insert('K', 'К');  // 4B
    mapping.insert('L', 'Л');  // 4C
    mapping.insert('M', 'М');  // 4D
    mapping.insert('N', 'Н');  // 4E
    mapping.insert('O', 'О');  // 4F
    mapping.insert('P', 'П');  // 50
    mapping.insert('Q', 'Љ');  // 51
    mapping.insert('R', 'Р');  // 52
    mapping.insert('S', 'С');  // 53
    mapping.insert('T', 'Т');  // 54
    mapping.insert('U', 'У');  // 55
    mapping.insert('V', 'В');  // 56
    mapping.insert('W', 'Њ');  // 57
    mapping.insert('X', 'Џ');  // 58
    mapping.insert('Z', 'З');  // 5A
    mapping.insert('[', 'Ш');  // 5B
    mapping.insert('\\', 'Ђ'); // 5C
    mapping.insert(']', 'Ћ');  // 5D
    mapping.insert('^', 'Ч');  // 5E
    mapping.insert('@', 'Ж');  // 40

    // Lowercase letters
    mapping.insert('a', 'а');  // 61
    mapping.insert('b', 'б');  // 62
    mapping.insert('c', 'ц');  // 63
    mapping.insert('d', 'д');  // 64
    mapping.insert('e', 'е');  // 65
    mapping.insert('f', 'ф');  // 66
    mapping.insert('g', 'г');  // 67
    mapping.insert('h', 'х');  // 68
    mapping.insert('i', 'и');  // 69
    mapping.insert('j', 'ј');  // 6A
    mapping.insert('k', 'к');  // 6B
    mapping.insert('l', 'л');  // 6C
    mapping.insert('m', 'м');  // 6D
    mapping.insert('n', 'н');  // 6E
    mapping.insert('o', 'о');  // 6F
    mapping.insert('p', 'п');  // 70
    mapping.insert('q', 'љ');  // 71
    mapping.insert('r', 'р');  // 72
    mapping.insert('s', 'с');  // 73
    mapping.insert('t', 'т');  // 74
    mapping.insert('u', 'у');  // 75
    mapping.insert('v', 'в');  // 76
    mapping.insert('w', 'њ');  // 77
    mapping.insert('x', 'џ');  // 78
    mapping.insert('z', 'з');  // 7A
    mapping.insert('{', 'ш');  // 7B
    mapping.insert('|', 'ђ');  // 7C
    mapping.insert('}', 'ћ');  // 7D
    mapping.insert('~', 'ч');  // 7E
    mapping.insert('`', 'ж');  // 60

    // Process the input text
    text.chars().map(|c| {
        *mapping.get(&c).unwrap_or(&c)
    }).collect()
}

fn convert_yutf8_to_macedonian(text: &str) -> String {
    let mut mapping = HashMap::new();
    
    // Map yutf8 to Cyrillic characters (Macedonian)
    // Capital letters
    mapping.insert('A', 'А');  // 41
    mapping.insert('B', 'Б');  // 42
    mapping.insert('C', 'Ц');  // 43
    mapping.insert('D', 'Д');  // 44
    mapping.insert('E', 'Е');  // 45
    mapping.insert('F', 'Ф');  // 46
    mapping.insert('G', 'Г');  // 47
    mapping.insert('H', 'Х');  // 48
    mapping.insert('I', 'И');  // 49
    mapping.insert('J', 'Ј');  // 4A
    mapping.insert('K', 'К');  // 4B
    mapping.insert('L', 'Л');  // 4C
    mapping.insert('M', 'М');  // 4D
    mapping.insert('N', 'Н');  // 4E
    mapping.insert('O', 'О');  // 4F
    mapping.insert('P', 'П');  // 50
    mapping.insert('Q', 'Љ');  // 51
    mapping.insert('R', 'Р');  // 52
    mapping.insert('S', 'С');  // 53
    mapping.insert('T', 'Т');  // 54
    mapping.insert('U', 'У');  // 55
    mapping.insert('V', 'В');  // 56
    mapping.insert('W', 'Њ');  // 57
    mapping.insert('X', 'Џ');  // 58
    mapping.insert('Y', 'Ѕ');  // 59 - Macedonian specific letter
    mapping.insert('Z', 'З');  // 5A
    mapping.insert('[', 'Ш');  // 5B
    mapping.insert('\\', 'Ѓ'); // 5C - Macedonian specific letter
    mapping.insert(']', 'Ќ');  // 5D - Macedonian specific letter
    mapping.insert('^', 'Ч');  // 5E
    mapping.insert('@', 'Ж');  // 40

    // Lowercase letters
    mapping.insert('a', 'а');  // 61
    mapping.insert('b', 'б');  // 62
    mapping.insert('c', 'ц');  // 63
    mapping.insert('d', 'д');  // 64
    mapping.insert('e', 'е');  // 65
    mapping.insert('f', 'ф');  // 66
    mapping.insert('g', 'г');  // 67
    mapping.insert('h', 'х');  // 68
    mapping.insert('i', 'и');  // 69
    mapping.insert('j', 'ј');  // 6A
    mapping.insert('k', 'к');  // 6B
    mapping.insert('l', 'л');  // 6C
    mapping.insert('m', 'м');  // 6D
    mapping.insert('n', 'н');  // 6E
    mapping.insert('o', 'о');  // 6F
    mapping.insert('p', 'п');  // 70
    mapping.insert('q', 'љ');  // 71
    mapping.insert('r', 'р');  // 72
    mapping.insert('s', 'с');  // 73
    mapping.insert('t', 'т');  // 74
    mapping.insert('u', 'у');  // 75
    mapping.insert('v', 'в');  // 76
    mapping.insert('w', 'њ');  // 77
    mapping.insert('x', 'џ');  // 78
    mapping.insert('y', 'ѕ');  // 79 - Macedonian specific letter
    mapping.insert('z', 'з');  // 7A
    mapping.insert('{', 'ш');  // 7B
    mapping.insert('|', 'ѓ');  // 7C - Macedonian specific letter
    mapping.insert('}', 'ќ');  // 7D - Macedonian specific letter
    mapping.insert('~', 'ч');  // 7E
    mapping.insert('`', 'ж');  // 60

    // Process the input text
    text.chars().map(|c| {
        *mapping.get(&c).unwrap_or(&c)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latin_roundtrip() {
        let original = "ABCČĆDĐEFGHIJKLMNOPQRSŠTUVWXYZŽ abcčćdđefghijklmnopqrsštuvwxyzž";
        let yutf8 = convert_latin_to_yutf8(original);
        let converted_back = convert_yutf8_to_latin(&yutf8);
        assert_eq!(original, converted_back);
    }

    #[test]
    fn test_serbian_cyrillic_roundtrip() {
        let original = "АБВГДЂЕЖЗИЈКЛЉМНЊОПРСТЋУФХЦЧЏШ абвгдђежзијклљмнњопрстћуфхцчџш";
        let yutf8 = convert_serbian_cyrillic_to_yutf8(original);
        let converted_back = convert_yutf8_to_serbian_cyrillic(&yutf8);
        assert_eq!(original, converted_back);
    }

    #[test]
    fn test_macedonian_roundtrip() {
        let original = "АБВГДЃЕЖЗЅИЈКЛЉМНЊОПРСТЌУФХЦЧЏШ абвгдѓежзѕијклљмнњопрстќуфхцчџш";
        let yutf8 = convert_macedonian_to_yutf8(original);
        let converted_back = convert_yutf8_to_macedonian(&yutf8);
        assert_eq!(original, converted_back);
    }

    #[test]
    fn test_latin_to_yutf8() {
        let input = "ČĆĐŠŽ čćđšž";
        let expected = "^]\\[@ ~}|{`";
        assert_eq!(convert_latin_to_yutf8(input), expected);
    }

    #[test]
    fn test_serbian_cyrillic_to_yutf8() {
        let input = "ЉЊЏШЂЋЧ љњџшђћч";
        let expected = "QWX[\\]^ qwx{|}~";
        assert_eq!(convert_serbian_cyrillic_to_yutf8(input), expected);
    }

    #[test]
    fn test_macedonian_to_yutf8() {
        let input = "ЉЊЏЅШЃЌЧ љњџѕшѓќч";
        let expected = "QWXY[\\]^ qwxy{|}~";
        assert_eq!(convert_macedonian_to_yutf8(input), expected);
    }

    #[test]
    fn test_yutf8_to_latin() {
        let input = "^]\\[@ ~}|{`";
        let expected = "ČĆĐŠŽ čćđšž";
        assert_eq!(convert_yutf8_to_latin(input), expected);
    }

    #[test]
    fn test_yutf8_to_serbian_cyrillic() {
        let input = "QWX[\\]^ qwx{|}~";
        let expected = "ЉЊЏШЂЋЧ љњџшђћч";
        assert_eq!(convert_yutf8_to_serbian_cyrillic(input), expected);
    }

    #[test]
    fn test_yutf8_to_macedonian() {
        let input = "QWXY[\\]^ qwxy{|}~";
        let expected = "ЉЊЏЅШЃЌЧ љњџѕшѓќч";
        assert_eq!(convert_yutf8_to_macedonian(input), expected);
    }

    #[test]
    fn test_unchanged_characters() {
        let input = "0123456789,.!? ";
        assert_eq!(convert_latin_to_yutf8(input), input);
        assert_eq!(convert_serbian_cyrillic_to_yutf8(input), input);
        assert_eq!(convert_macedonian_to_yutf8(input), input);
        assert_eq!(convert_yutf8_to_latin(input), input);
        assert_eq!(convert_yutf8_to_serbian_cyrillic(input), input);
        assert_eq!(convert_yutf8_to_macedonian(input), input);
    }
}


