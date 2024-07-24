mod translation;
use translation::NucleobaseRnaToAminoChar;
const TABLE: NucleobaseRnaToAminoChar = NucleobaseRnaToAminoChar::default();
/*
enum NucleobaseDna {
    Adenine,
    Thymine,
    Cytosine,
    Guanine,
}
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NucleobaseRna {
    Adenine,
    Uracil,
    Cytosine,
    Guanine,
}

impl NucleobaseRna {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'A' => Some(NucleobaseRna::Adenine),
            'U' => Some(NucleobaseRna::Uracil),
            'C' => Some(NucleobaseRna::Cytosine),
            'G' => Some(NucleobaseRna::Guanine),
            _ => None,
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            NucleobaseRna::Adenine => 'A',
            NucleobaseRna::Uracil => 'U',
            NucleobaseRna::Cytosine => 'C',
            NucleobaseRna::Guanine => 'G',
        }
    }
    pub fn to_index(&self) -> usize {
        match self {
            NucleobaseRna::Adenine => 2,
            NucleobaseRna::Uracil => 0,
            NucleobaseRna::Cytosine => 1,
            NucleobaseRna::Guanine => 3,
        }
    }
}

#[derive(Debug)]
struct Mrna {
    pub codons: Vec<Codon>,
}

impl Mrna {
    pub fn new(s: &str) -> Self {
        let mut codons = Vec::new();
        for i in (0..s.len()).step_by(3) {
            let b1 = s.chars().nth(i).unwrap();
            let b2 = s.chars().nth(i + 1).unwrap();
            let b3 = s.chars().nth(i + 2).unwrap();
            codons.push(Codon::new(b1, b2, b3));
        }
        Mrna { codons }
    }
    pub fn to_string_toki(&self) -> String {
        let mut s = String::new();
        for codon in &self.codons {
            s.push(TABLE.traslation(codon));
        }
        s
    }
}

#[derive(Debug)]
struct Codon {
    pub bases: (NucleobaseRna, NucleobaseRna, NucleobaseRna),
}

impl Codon {
    pub fn new(b1: char, b2: char, b3: char) -> Self {
        Codon {
            bases: (
                NucleobaseRna::from_char(b1).unwrap(),
                NucleobaseRna::from_char(b2).unwrap(),
                NucleobaseRna::from_char(b3).unwrap(),
            ),
        }
    }
    pub fn i0(&self) -> usize {
        self.bases.0.to_index()
    }
    pub fn i1(&self) -> usize {
        self.bases.1.to_index()
    }
    pub fn i2(&self) -> usize {
        self.bases.2.to_index()
    }
}

mod test {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_mrna() {
        let mrna = Mrna::new("AUGACUCAGCGAAUAGUUCCUCAGAACGCGUGA");
        assert_eq!(mrna.to_string_toki(), "^Toki Pona.");
    }
}
