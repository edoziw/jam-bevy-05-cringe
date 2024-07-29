use crate::game::assets::Scene3dKey;

use super::Codon;

pub struct NucleobaseRnaToAminoChar {
    table: [[[char; 4]; 4]; 4],
}
impl NucleobaseRnaToAminoChar {
    pub const fn default() -> Self {
        NucleobaseRnaToAminoChar::new(TranslationMode::TokiPona)
    }
}

pub enum TranslationMode {
    Real,
    TokiPona,
}

impl NucleobaseRnaToAminoChar {
    pub const fn new(mode: TranslationMode) -> Self {
        let table = match mode {
            TranslationMode::Real => [
                //U
                [
                    // UUU - UUG
                    ['F', 'F', 'L', 'L'],
                    // UCU - UCG
                    ['S', 'S', 'S', 'S'],
                    // UAU - UAG
                    ['Y', 'Y', '.', '.'],
                    // UGU - UGG
                    ['C', 'C', '.', 'W'],
                ],
                // C
                [
                    // CUU - CUG
                    ['L', 'L', 'L', 'L'],
                    // CCU - CCG
                    ['P', 'P', 'P', 'P'],
                    // CAU - CAG
                    ['H', 'H', 'Q', 'Q'],
                    // CGU - CGG
                    ['R', 'R', 'R', 'R'],
                ],
                // A
                [
                    // AUU - AUG
                    ['I', 'I', 'I', 'M'],
                    // ACU - ACG
                    ['T', 'T', 'T', 'T'],
                    // AAU - AAG
                    ['N', 'N', 'K', 'K'],
                    // AGU - AGG
                    ['S', 'S', 'R', 'R'],
                ],
                // G
                [
                    // GUU - GUG
                    ['V', 'V', 'V', 'V'],
                    // GCU - GCG
                    ['A', 'A', 'A', 'A'],
                    // GAU - GAG
                    ['D', 'D', 'E', 'E'],
                    // GGU - GGG
                    ['G', 'G', 'G', 'G'],
                ],
            ],
            TranslationMode::TokiPona => [
                //U
                [
                    // UUU - UUG
                    ['U', 'u', 'L', 'l'],
                    // UCU - UCG
                    ['S', 's', 's', 's'],
                    // UAU - UAG
                    ['J', 'j', '.', '.'],
                    // UGU - UGG
                    ['s', 's', '.', 'W'],
                ],
                // C
                [
                    // CUU - CUG
                    ['l', 'l', 'l', 'l'],
                    // CCU - CCG
                    ['P', 'p', 'p', 'p'],
                    // CAU - CAG
                    ['u', 'u', 'O', 'o'],
                    // CGU - CGG
                    ['w', 'K', 'k', 'k'],
                ],
                // A
                [
                    // AUU - AUG
                    ['I', 'i', 'i', '^'],
                    // ACU - ACG
                    ['T', 't', 't', 't'],
                    // AAU - AAG
                    ['N', 'n', 'E', 'e'],
                    // AGU - AGG
                    ['s', 's', 'M', 'm'],
                ],
                // G
                [
                    // GUU - GUG
                    [' ', '0', '1', '2'],
                    // GCU - GCG
                    ['3', 'A', 'a', 'a'],
                    // GAU - GAG
                    ['+', '-', '*', '/'],
                    // GGU - GGG
                    ['_', ',', '?', '!'],
                ],
            ],
        };
        Self { table }
    }
    pub fn translation(&self, codon: &Codon) -> char {
        self.table[codon.i0()][codon.i1()][codon.i2()]
    }
    pub fn scene_3d_key(&self, c: char) -> Scene3dKey {
        match c {
            '-' => Scene3dKey::Minus,
            '_' => Scene3dKey::Underscore,
            '^' => Scene3dKey::Carrot,
            ',' => Scene3dKey::Comma,
            '!' => Scene3dKey::Exclamation,
            '.' => Scene3dKey::Period,
            '+' => Scene3dKey::Plus,
            '?' => Scene3dKey::Question,
            '/' => Scene3dKey::Slash,
            '*' => Scene3dKey::Star,
            '0' => Scene3dKey::Digit0,
            '1' => Scene3dKey::Digit1,
            '2' => Scene3dKey::Digit2,
            '3' => Scene3dKey::Digit3,
            'A' => Scene3dKey::A,
            'a' => Scene3dKey::ALower,
            'C' => Scene3dKey::C,
            'E' => Scene3dKey::E,
            'e' => Scene3dKey::ELower,
            'G' => Scene3dKey::G,
            'I' => Scene3dKey::I,
            'i' => Scene3dKey::ILower,
            'J' => Scene3dKey::J,
            'j' => Scene3dKey::JLower,
            'K' => Scene3dKey::K,
            'k' => Scene3dKey::KLower,
            'L' => Scene3dKey::L,
            'l' => Scene3dKey::LLower,
            'M' => Scene3dKey::M,
            'm' => Scene3dKey::MLower,
            'N' => Scene3dKey::N,
            'n' => Scene3dKey::NLower,
            'O' => Scene3dKey::O,
            'o' => Scene3dKey::OLower,
            'P' => Scene3dKey::P,
            'p' => Scene3dKey::PLower,
            'S' => Scene3dKey::S,
            's' => Scene3dKey::SLower,
            'T' => Scene3dKey::T,
            't' => Scene3dKey::TLower,
            'U' => Scene3dKey::U,
            'u' => Scene3dKey::ULower,
            'W' => Scene3dKey::W,
            'w' => Scene3dKey::WLower,

            _ => panic!("unexpected translation {}", c),
        }
    }
}

mod test {
    #[allow(unused_imports)]
    use super::*;
    struct CodonAcids {
        codon: Codon,
        expected_real: char,
        expected_toki: char,
    }
    impl CodonAcids {
        pub fn new(codon: Codon, expected_real: char, expected_toki: char) -> Self {
            Self {
                codon,
                expected_real,
                expected_toki,
            }
        }
    }

    #[test]
    fn test_traslations() {
        let table_real: NucleobaseRnaToAminoChar =
            NucleobaseRnaToAminoChar::new(TranslationMode::Real);
        let table_toki: NucleobaseRnaToAminoChar =
            NucleobaseRnaToAminoChar::new(TranslationMode::TokiPona);
        let values: [CodonAcids; 64] = [
            CodonAcids::new(Codon::new('U', 'U', 'U'), 'F', 'U'),
            CodonAcids::new(Codon::new('U', 'U', 'C'), 'F', 'u'),
            CodonAcids::new(Codon::new('U', 'U', 'A'), 'L', 'L'),
            CodonAcids::new(Codon::new('U', 'U', 'G'), 'L', 'l'),
            CodonAcids::new(Codon::new('U', 'C', 'U'), 'S', 'S'),
            CodonAcids::new(Codon::new('U', 'C', 'C'), 'S', 's'),
            CodonAcids::new(Codon::new('U', 'C', 'A'), 'S', 's'),
            CodonAcids::new(Codon::new('U', 'C', 'G'), 'S', 's'),
            CodonAcids::new(Codon::new('U', 'A', 'U'), 'Y', 'J'),
            CodonAcids::new(Codon::new('U', 'A', 'C'), 'Y', 'j'),
            CodonAcids::new(Codon::new('U', 'A', 'A'), '.', '.'),
            CodonAcids::new(Codon::new('U', 'A', 'G'), '.', '.'),
            CodonAcids::new(Codon::new('U', 'G', 'U'), 'C', 's'),
            CodonAcids::new(Codon::new('U', 'G', 'C'), 'C', 's'),
            CodonAcids::new(Codon::new('U', 'G', 'A'), '.', '.'),
            CodonAcids::new(Codon::new('U', 'G', 'G'), 'W', 'W'),
            CodonAcids::new(Codon::new('C', 'U', 'U'), 'L', 'l'),
            CodonAcids::new(Codon::new('C', 'U', 'C'), 'L', 'l'),
            CodonAcids::new(Codon::new('C', 'U', 'A'), 'L', 'l'),
            CodonAcids::new(Codon::new('C', 'U', 'G'), 'L', 'l'),
            CodonAcids::new(Codon::new('C', 'C', 'U'), 'P', 'P'),
            CodonAcids::new(Codon::new('C', 'C', 'C'), 'P', 'p'),
            CodonAcids::new(Codon::new('C', 'C', 'A'), 'P', 'p'),
            CodonAcids::new(Codon::new('C', 'C', 'G'), 'P', 'p'),
            CodonAcids::new(Codon::new('C', 'A', 'U'), 'H', 'u'),
            CodonAcids::new(Codon::new('C', 'A', 'C'), 'H', 'u'),
            CodonAcids::new(Codon::new('C', 'A', 'A'), 'Q', 'O'),
            CodonAcids::new(Codon::new('C', 'A', 'G'), 'Q', 'o'),
            CodonAcids::new(Codon::new('C', 'G', 'U'), 'R', 'w'),
            CodonAcids::new(Codon::new('C', 'G', 'C'), 'R', 'K'),
            CodonAcids::new(Codon::new('C', 'G', 'A'), 'R', 'k'),
            CodonAcids::new(Codon::new('C', 'G', 'G'), 'R', 'k'),
            CodonAcids::new(Codon::new('A', 'U', 'U'), 'I', 'I'),
            CodonAcids::new(Codon::new('A', 'U', 'C'), 'I', 'i'),
            CodonAcids::new(Codon::new('A', 'U', 'A'), 'I', 'i'),
            CodonAcids::new(Codon::new('A', 'U', 'G'), 'M', '^'),
            CodonAcids::new(Codon::new('A', 'C', 'U'), 'T', 'T'),
            CodonAcids::new(Codon::new('A', 'C', 'C'), 'T', 't'),
            CodonAcids::new(Codon::new('A', 'C', 'A'), 'T', 't'),
            CodonAcids::new(Codon::new('A', 'C', 'G'), 'T', 't'),
            CodonAcids::new(Codon::new('A', 'A', 'U'), 'N', 'N'),
            CodonAcids::new(Codon::new('A', 'A', 'C'), 'N', 'n'),
            CodonAcids::new(Codon::new('A', 'A', 'A'), 'K', 'E'),
            CodonAcids::new(Codon::new('A', 'A', 'G'), 'K', 'e'),
            CodonAcids::new(Codon::new('A', 'G', 'U'), 'S', 's'),
            CodonAcids::new(Codon::new('A', 'G', 'C'), 'S', 's'),
            CodonAcids::new(Codon::new('A', 'G', 'A'), 'R', 'M'),
            CodonAcids::new(Codon::new('A', 'G', 'G'), 'R', 'm'),
            CodonAcids::new(Codon::new('G', 'U', 'U'), 'V', ' '),
            CodonAcids::new(Codon::new('G', 'U', 'C'), 'V', '0'),
            CodonAcids::new(Codon::new('G', 'U', 'A'), 'V', '1'),
            CodonAcids::new(Codon::new('G', 'U', 'G'), 'V', '2'),
            CodonAcids::new(Codon::new('G', 'C', 'U'), 'A', '3'),
            CodonAcids::new(Codon::new('G', 'C', 'C'), 'A', 'A'),
            CodonAcids::new(Codon::new('G', 'C', 'A'), 'A', 'a'),
            CodonAcids::new(Codon::new('G', 'C', 'G'), 'A', 'a'),
            CodonAcids::new(Codon::new('G', 'A', 'U'), 'D', '+'),
            CodonAcids::new(Codon::new('G', 'A', 'C'), 'D', '-'),
            CodonAcids::new(Codon::new('G', 'A', 'A'), 'E', '*'),
            CodonAcids::new(Codon::new('G', 'A', 'G'), 'E', '/'),
            CodonAcids::new(Codon::new('G', 'G', 'U'), 'G', '_'),
            CodonAcids::new(Codon::new('G', 'G', 'C'), 'G', ','),
            CodonAcids::new(Codon::new('G', 'G', 'A'), 'G', '?'),
            CodonAcids::new(Codon::new('G', 'G', 'G'), 'G', '!'),
        ];
        for e in values.iter() {
            let result = table_real.translation(&e.codon);
            let expected_char = e.expected_real;
            assert_eq!(
                result, expected_char,
                "we are testing translation to real acids with {:?}, expecting acid {}, got {}",
                e.codon, expected_char, result
            );
            let result = table_toki.translation(&e.codon);
            let expected_char = e.expected_toki;
            assert_eq!(
                result, expected_char,
                "we are testing translation to toki acids with {:?}, expecting acid {}, got {}",
                e.codon, expected_char, result
            );
        }
    }
}
