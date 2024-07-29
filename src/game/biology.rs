pub mod translation;
use crate::game::cells::text::spawn_acid;
use bevy::prelude::*;
use translation::NucleobaseRnaToAminoChar;

pub const CODON_TABLE: NucleobaseRnaToAminoChar = NucleobaseRnaToAminoChar::default();

pub fn plugin(app: &mut App) {
    app.init_resource::<CodonState>();
    app.init_resource::<Mrna>();
    app.init_resource::<LogTextRes>();
    app.add_systems(Startup, spawn_log);
    app.add_systems(Update, (build_codon, update_log));
}

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct CodonState {
    bases: Vec<NucleobaseRna>,
}
impl Default for CodonState {
    fn default() -> Self {
        CodonState { bases: Vec::new() }
    }
}
impl CodonState {
    pub fn remove_head_codon(&mut self) -> Option<Codon> {
        if self.bases.len() < 3 {
            return None;
        }
        Some(Codon::from_bases((
            self.bases.remove(0),
            self.bases.remove(0),
            self.bases.remove(0),
        )))
    }
    pub fn push(&mut self, base: NucleobaseRna) {
        self.bases.push(base);
    }
}
#[derive(Resource)]
pub struct LogTextRes {
    pub text: String,
}
impl Default for LogTextRes {
    fn default() -> Self {
        LogTextRes {
            text: "Press a triplet of (U, C, A, G) to build a codon of an mRNA sequence"
                .to_string(),
        }
    }
}
#[derive(Component)]
pub struct Log {}
pub fn spawn_log(mut commands: Commands, help_text: Res<LogTextRes>) {
    commands.spawn((
        TextBundle::from_section(
            help_text.text.clone(),
            TextStyle {
                font_size: 20.,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(100.0),
            ..default()
        }),
        Log {},
    ));
}

pub fn update_log(mut log_q: Query<&mut Text, With<Log>>, help_text: Res<LogTextRes>) {
    for mut text in log_q.iter_mut() {
        text.sections[0].value = help_text.text.clone();
    }
}

pub fn build_codon(
    commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut codon_state: ResMut<CodonState>,
    mut mrna: ResMut<Mrna>,
    mut log_res: ResMut<LogTextRes>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyU) {
        codon_state.push(NucleobaseRna::Uracil)
    }
    if keyboard_input.just_pressed(KeyCode::KeyC) {
        codon_state.push(NucleobaseRna::Cytosine)
    }
    if keyboard_input.just_pressed(KeyCode::KeyA) {
        codon_state.push(NucleobaseRna::Adenine)
    }
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        codon_state.push(NucleobaseRna::Guanine)
    }
    let mut codon_option = codon_state.remove_head_codon();
    while codon_option.is_some() {
        let codon = codon_option.unwrap();
        codon_option = codon_state.remove_head_codon();
        let stop_codon = CODON_TABLE.translation(&codon) == '.' && mrna.has_start_codon;
        mrna.push(codon);
        if stop_codon {
            spawn_acid(&commands, &mrna, &mut log_res);
        }
    }
}
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

#[derive(Resource, Debug, Clone, PartialEq)]
pub struct Mrna {
    codons: Vec<Codon>,
    has_start_codon: bool,
}

impl Default for Mrna {
    fn default() -> Self {
        Mrna {
            codons: Vec::new(),
            has_start_codon: false,
        }
    }
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
        Mrna {
            codons,
            has_start_codon: s.contains("AUG"),
        }
    }
    pub fn to_string_toki(&self) -> String {
        let mut s = String::new();
        for codon in &self.codons {
            s.push(CODON_TABLE.translation(codon));
        }
        s
    }
    pub fn push(&mut self, codon: Codon) {
        if codon.bases
            == (
                NucleobaseRna::Adenine,
                NucleobaseRna::Uracil,
                NucleobaseRna::Guanine,
            )
        {
            self.has_start_codon = true;
        }
        self.codons.push(codon);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Codon {
    pub bases: (NucleobaseRna, NucleobaseRna, NucleobaseRna),
}

impl Codon {
    pub fn from_bases(bases: (NucleobaseRna, NucleobaseRna, NucleobaseRna)) -> Self {
        Codon { bases }
    }
    pub fn new(b1: char, b2: char, b3: char) -> Self {
        Codon {
            bases: (
                NucleobaseRna::from_char(b1).unwrap(),
                NucleobaseRna::from_char(b2).unwrap(),
                NucleobaseRna::from_char(b3).unwrap(),
            ),
        }
    }
    pub fn from_index(i: usize) -> Self {
        let b1 = base_from_last_index(i / 16);
        let b2 = base_from_last_index(i / 4);
        let b3 = base_from_last_index(i);
        Codon {
            bases: (b1, b2, b3),
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

fn base_from_last_index(i: usize) -> NucleobaseRna {
    match i % 4 {
        0 => NucleobaseRna::Uracil,
        1 => NucleobaseRna::Cytosine,
        2 => NucleobaseRna::Adenine,
        3 => NucleobaseRna::Guanine,
        _ => panic!(),
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
