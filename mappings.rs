use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Latin script mappings (UTF-8 to YUTF-8)
pub static LATIN_TO_YUTF8: Lazy<HashMap<char, char>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert('Ž', '@');  // 40
    map.insert('Š', '[');  // 5B
    map.insert('Đ', '\\'); // 5C
    map.insert('Ć', ']');  // 5D
    map.insert('Č', '^');  // 5E
    
    map.insert('ž', '`');  // 60
    map.insert('š', '{');  // 7B
    map.insert('đ', '|');  // 7C
    map.insert('ć', '}');  // 7D
    map.insert('č', '~');  // 7E
    map
});

/// Latin script mappings (YUTF-8 to UTF-8)
pub static YUTF8_TO_LATIN: Lazy<HashMap<char, char>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert('@', 'Ž');  // 40
    map.insert('[', 'Š');  // 5B
    map.insert('\\', 'Đ'); // 5C
    map.insert(']', 'Ć');  // 5D
    map.insert('^', 'Č');  // 5E
    
    map.insert('`', 'ž');  // 60
    map.insert('{', 'š');  // 7B
    map.insert('|', 'đ');  // 7C
    map.insert('}', 'ć');  // 7D
    map.insert('~', 'č');  // 7E
    map
});

/// Serbian Cyrillic mappings (UTF-8 to YUTF-8)
pub static SERBIAN_CYRILLIC_TO_YUTF8: Lazy<HashMap<char, char>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Capital Cyrillic letters (Serbian)
    map.insert('А', 'A');  // 41
    map.insert('Б', 'B');  // 42
    map.insert('Ц', 'C');  // 43
    map.insert('Д', 'D');  // 44
    map.insert('Е', 'E');  // 45
    map.insert('Ф', 'F');  // 46
    map.insert('Г', 'G');  // 47
    map.insert('Х', 'H');  // 48
    map.insert('И', 'I');  // 49
    map.insert('Ј', 'J');  // 4A
    map.insert('К', 'K');  // 4B
    map.insert('Л', 'L');  // 4C
    map.insert('М', 'M');  // 4D
    map.insert('Н', 'N');  // 4E
    map.insert('О', 'O');  // 4F
    map.insert('П', 'P');  // 50
    map.insert('Љ', 'Q');  // 51
    map.insert('Р', 'R');  // 52
    map.insert('С', 'S');  // 53
    map.insert('Т', 'T');  // 54
    map.insert('У', 'U');  // 55
    map.insert('В', 'V');  // 56
    map.insert('Њ', 'W');  // 57
    map.insert('Џ', 'X');  // 58
    map.insert('З', 'Z');  // 5A
    map.insert('Ш', '[');  // 5B
    map.insert('Ђ', '\\'); // 5C
    map.insert('Ћ', ']');  // 5D
    map.insert('Ч', '^');  // 5E
    map.insert('Ж', '@');  // 40

    // Lowercase Cyrillic letters (Serbian)
    map.insert('а', 'a');  // 61
    map.insert('б', 'b');  // 62
    map.insert('ц', 'c');  // 63
    map.insert('д', 'd');  // 64
    map.insert('е', 'e');  // 65
    map.insert('ф', 'f');  // 66
    map.insert('г', 'g');  // 67
    map.insert('х', 'h');  // 68
    map.insert('и', 'i');  // 69
    map.insert('ј', 'j');  // 6A
    map.insert('к', 'k');  // 6B
    map.insert('л', 'l');  // 6C
    map.insert('м', 'm');  // 6D
    map.insert('н', 'n');  // 6E
    map.insert('о', 'o');  // 6F
    map.insert('п', 'p');  // 70
    map.insert('љ', 'q');  // 71
    map.insert('р', 'r');  // 72
    map.insert('с', 's');  // 73
    map.insert('т', 't');  // 74
    map.insert('у', 'u');  // 75
    map.insert('в', 'v');  // 76
    map.insert('њ', 'w');  // 77
    map.insert('џ', 'x');  // 78
    map.insert('з', 'z');  // 7A
    map.insert('ш', '{');  // 7B
    map.insert('ђ', '|');  // 7C
    map.insert('ћ', '}');  // 7D
    map.insert('ч', '~');  // 7E
    map.insert('ж', '`');  // 60
    map
});

/// Serbian Cyrillic mappings (YUTF-8 to UTF-8)
pub static YUTF8_TO_SERBIAN_CYRILLIC: Lazy<HashMap<char, char>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Map yutf8 to Cyrillic characters (Serbian)
    map.insert('A', 'А');  // 41
    map.insert('B', 'Б');  // 42
    map.insert('C', 'Ц');  // 43
    map.insert('D', 'Д');  // 44
    map.insert('E', 'Е');  // 45
    map.insert('F', 'Ф');  // 46
    map.insert('G', 'Г');  // 47
    map.insert('H', 'Х');  // 48
    map.insert('I', 'И');  // 49
    map.insert('J', 'Ј');  // 4A
    map.insert('K', 'К');  // 4B
    map.insert('L', 'Л');  // 4C
    map.insert('M', 'М');  // 4D
    map.insert('N', 'Н');  // 4E
    map.insert('O', 'О');  // 4F
    map.insert('P', 'П');  // 50
    map.insert('Q', 'Љ');  // 51
    map.insert('R', 'Р');  // 52
    map.insert('S', 'С');  // 53
    map.insert('T', 'Т');  // 54
    map.insert('U', 'У');  // 55
    map.insert('V', 'В');  // 56
    map.insert('W', 'Њ');  // 57
    map.insert('X', 'Џ');  // 58
    map.insert('Z', 'З');  // 5A
    map.insert('[', 'Ш');  // 5B
    map.insert('\\', 'Ђ'); // 5C
    map.insert(']', 'Ћ');  // 5D
    map.insert('^', 'Ч');  // 5E
    map.insert('@', 'Ж');  // 40

    // Lowercase letters
    map.insert('a', 'а');  // 61
    map.insert('b', 'б');  // 62
    map.insert('c', 'ц');  // 63
    map.insert('d', 'д');  // 64
    map.insert('e', 'е');  // 65
    map.insert('f', 'ф');  // 66
    map.insert('g', 'г');  // 67
    map.insert('h', 'х');  // 68
    map.insert('i', 'и');  // 69
    map.insert('j', 'ј');  // 6A
    map.insert('k', 'к');  // 6B
    map.insert('l', 'л');  // 6C
    map.insert('m', 'м');  // 6D
    map.insert('n', 'н');  // 6E
    map.insert('o', 'о');  // 6F
    map.insert('p', 'п');  // 70
    map.insert('q', 'љ');  // 71
    map.insert('r', 'р');  // 72
    map.insert('s', 'с');  // 73
    map.insert('t', 'т');  // 74
    map.insert('u', 'у');  // 75
    map.insert('v', 'в');  // 76
    map.insert('w', 'њ');  // 77
    map.insert('x', 'џ');  // 78
    map.insert('z', 'з');  // 7A
    map.insert('{', 'ш');  // 7B
    map.insert('|', 'ђ');  // 7C
    map.insert('}', 'ћ');  // 7D
    map.insert('~', 'ч');  // 7E
    map.insert('`', 'ж');  // 60
    map
});

/// Macedonian mappings (UTF-8 to YUTF-8)
pub static MACEDONIAN_TO_YUTF8: Lazy<HashMap<char, char>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Capital Cyrillic letters (Macedonian)
    map.insert('А', 'A');  // 41
    map.insert('Б', 'B');  // 42
    map.insert('Ц', 'C');  // 43
    map.insert('Д', 'D');  // 44
    map.insert('Е', 'E');  // 45
    map.insert('Ф', 'F');  // 46
    map.insert('Г', 'G');  // 47
    map.insert('Х', 'H');  // 48
    map.insert('И', 'I');  // 49
    map.insert('Ј', 'J');  // 4A
    map.insert('К', 'K');  // 4B
    map.insert('Л', 'L');  // 4C
    map.insert('М', 'M');  // 4D
    map.insert('Н', 'N');  // 4E
    map.insert('О', 'O');  // 4F
    map.insert('П', 'P');  // 50
    map.insert('Љ', 'Q');  // 51
    map.insert('Р', 'R');  // 52
    map.insert('С', 'S');  // 53
    map.insert('Т', 'T');  // 54
    map.insert('У', 'U');  // 55
    map.insert('В', 'V');  // 56
    map.insert('Њ', 'W');  // 57
    map.insert('Џ', 'X');  // 58
    map.insert('Ѕ', 'Y');  // 59 - Macedonian specific letter
    map.insert('З', 'Z');  // 5A
    map.insert('Ш', '[');  // 5B
    map.insert('Ѓ', '\\'); // 5C - Macedonian specific letter
    map.insert('Ќ', ']');  // 5D - Macedonian specific letter
    map.insert('Ч', '^');  // 5E
    map.insert('Ж', '@');  // 40

    // Lowercase Cyrillic letters (Macedonian)
    map.insert('а', 'a');  // 61
    map.insert('б', 'b');  // 62
    map.insert('ц', 'c');  // 63
    map.insert('д', 'd');  // 64
    map.insert('е', 'e');  // 65
    map.insert('ф', 'f');  // 66
    map.insert('г', 'g');  // 67
    map.insert('х', 'h');  // 68
    map.insert('и', 'i');  // 69
    map.insert('ј', 'j');  // 6A
    map.insert('к', 'k');  // 6B
    map.insert('л', 'l');  // 6C
    map.insert('м', 'm');  // 6D
    map.insert('н', 'n');  // 6E
    map.insert('о', 'o');  // 6F
    map.insert('п', 'p');  // 70
    map.insert('љ', 'q');  // 71
    map.insert('р', 'r');  // 72
    map.insert('с', 's');  // 73
    map.insert('т', 't');  // 74
    map.insert('у', 'u');  // 75
    map.insert('в', 'v');  // 76
    map.insert('њ', 'w');  // 77
    map.insert('џ', 'x');  // 78
    map.insert('ѕ', 'y');  // 79 - Macedonian specific letter
    map.insert('з', 'z');  // 7A
    map.insert('ш', '{');  // 7B
    map.insert('ѓ', '|');  // 7C - Macedonian specific letter
    map.insert('ќ', '}');  // 7D - Macedonian specific letter
    map.insert('ч', '~');  // 7E
    map.insert('ж', '`');  // 60
    map
});

/// Macedonian mappings (YUTF-8 to UTF-8)
pub static YUTF8_TO_MACEDONIAN: Lazy<HashMap<char, char>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Map yutf8 to Cyrillic characters (Macedonian)
    map.insert('A', 'А');  // 41
    map.insert('B', 'Б');  // 42
    map.insert('C', 'Ц');  // 43
    map.insert('D', 'Д');  // 44
    map.insert('E', 'Е');  // 45
    map.insert('F', 'Ф');  // 46
    map.insert('G', 'Г');  // 47
    map.insert('H', 'Х');  // 48
    map.insert('I', 'И');  // 49
    map.insert('J', 'Ј');  // 4A
    map.insert('K', 'К');  // 4B
    map.insert('L', 'Л');  // 4C
    map.insert('M', 'М');  // 4D
    map.insert('N', 'Н');  // 4E
    map.insert('O', 'О');  // 4F
    map.insert('P', 'П');  // 50
    map.insert('Q', 'Љ');  // 51
    map.insert('R', 'Р');  // 52
    map.insert('S', 'С');  // 53
    map.insert('T', 'Т');  // 54
    map.insert('U', 'У');  // 55
    map.insert('V', 'В');  // 56
    map.insert('W', 'Њ');  // 57
    map.insert('X', 'Џ');  // 58
    map.insert('Y', 'Ѕ');  // 59 - Macedonian specific letter
    map.insert('Z', 'З');  // 5A
    map.insert('[', 'Ш');  // 5B
    map.insert('\\', 'Ѓ'); // 5C - Macedonian specific letter
    map.insert(']', 'Ќ');  // 5D - Macedonian specific letter
    map.insert('^', 'Ч');  // 5E
    map.insert('@', 'Ж');  // 40

    // Lowercase letters
    map.insert('a', 'а');  // 61
    map.insert('b', 'б');  // 62
    map.insert('c', 'ц');  // 63
    map.insert('d', 'д');  // 64
    map.insert('e', 'е');  // 65
    map.insert('f', 'ф');  // 66
    map.insert('g', 'г');  // 67
    map.insert('h', 'х');  // 68
    map.insert('i', 'и');  // 69
    map.insert('j', 'ј');  // 6A
    map.insert('k', 'к');  // 6B
    map.insert('l', 'л');  // 6C
    map.insert('m', 'м');  // 6D
    map.insert('n', 'н');  // 6E
    map.insert('o', 'о');  // 6F
    map.insert('p', 'п');  // 70
    map.insert('q', 'љ');  // 71
    map.insert('r', 'р');  // 72
    map.insert('s', 'с');  // 73
    map.insert('t', 'т');  // 74
    map.insert('u', 'у');  // 75
    map.insert('v', 'в');  // 76
    map.insert('w', 'њ');  // 77
    map.insert('x', 'џ');  // 78
    map.insert('y', 'ѕ');  // 79 - Macedonian specific letter
    map.insert('z', 'з');  // 7A
    map.insert('{', 'ш');  // 7B
    map.insert('|', 'ѓ');  // 7C - Macedonian specific letter
    map.insert('}', 'ќ');  // 7D - Macedonian specific letter
    map.insert('~', 'ч');  // 7E
    map.insert('`', 'ж');  // 60
    map
});
