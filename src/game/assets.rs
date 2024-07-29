use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    utils::HashMap,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HandleMap<ImageKey>>();
    app.init_resource::<HandleMap<ImageKey>>();

    app.register_type::<HandleMap<Scene3dKey>>();
    app.init_resource::<HandleMap<Scene3dKey>>();

    app.register_type::<HandleMap<SfxKey>>();
    app.init_resource::<HandleMap<SfxKey>>();

    app.register_type::<HandleMap<SoundtrackKey>>();
    app.init_resource::<HandleMap<SoundtrackKey>>();
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect, Debug)]
pub enum Scene3dKey {
    A,
    ALower,
    U,
    ULower,
    T,
    TLower,
    C,
    G,
    L,
    LLower,
    S,
    SLower,
    J,
    JLower,
    Period,
    W,
    WLower,
    P,
    PLower,
    O,
    OLower,
    K,
    KLower,
    I,
    ILower,
    Carret,
    N,
    NLower,
    E,
    ELower,
    M,
    MLower,
    Space,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Plus,
    Minus,
    Star,
    Slash,
    Underscore,
    Carrot,
    Comma,
    Question,
    Exclamation,
}

impl AssetKey for Scene3dKey {
    type Asset = Scene;
}
const PREFIX_3D: &str = "3d/text-3d";
const SUFFIX_3D: &str = ".glb#Scene0";
const LOWER_3D: &str = "-l";
const SYMBOL_3D: &str = "_";
impl FromWorld for HandleMap<Scene3dKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                Scene3dKey::Minus,
                asset_server.load(format!("{PREFIX_3D}{SYMBOL_3D}-{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::Underscore,
                asset_server.load(format!("{PREFIX_3D}{SYMBOL_3D}_{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::Carrot,
                asset_server.load(format!("{PREFIX_3D}{SYMBOL_3D}carrot{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::Comma,
                asset_server.load(format!("{PREFIX_3D}{SYMBOL_3D}comma{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::Exclamation,
                asset_server.load(format!("{PREFIX_3D}{SYMBOL_3D}exclamation{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::Period,
                asset_server.load(format!("{PREFIX_3D}{SYMBOL_3D}period{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::Plus,
                asset_server.load(format!("{PREFIX_3D}{SYMBOL_3D}plus{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::Question,
                asset_server.load(format!("{PREFIX_3D}{SYMBOL_3D}question{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::Slash,
                asset_server.load(format!("{PREFIX_3D}{SYMBOL_3D}slash{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::Star,
                asset_server.load(format!("{PREFIX_3D}{SYMBOL_3D}star{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::Digit0,
                asset_server.load(format!("{PREFIX_3D}-0{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::Digit1,
                asset_server.load(format!("{PREFIX_3D}-1{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::Digit2,
                asset_server.load(format!("{PREFIX_3D}-2{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::Digit3,
                asset_server.load(format!("{PREFIX_3D}-3{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::A,
                asset_server.load(format!("{PREFIX_3D}-A{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::ALower,
                asset_server.load(format!("{PREFIX_3D}-a{LOWER_3D}{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::C,
                asset_server.load(format!("{PREFIX_3D}-C{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::E,
                asset_server.load(format!("{PREFIX_3D}-E{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::ELower,
                asset_server.load(format!("{PREFIX_3D}-e{LOWER_3D}{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::G,
                asset_server.load(format!("{PREFIX_3D}-T{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::I,
                asset_server.load(format!("{PREFIX_3D}-I{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::ILower,
                asset_server.load(format!("{PREFIX_3D}-i{LOWER_3D}{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::J,
                asset_server.load(format!("{PREFIX_3D}-J{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::JLower,
                asset_server.load(format!("{PREFIX_3D}-j{LOWER_3D}{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::K,
                asset_server.load(format!("{PREFIX_3D}-K{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::KLower,
                asset_server.load(format!("{PREFIX_3D}-k{LOWER_3D}{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::L,
                asset_server.load(format!("{PREFIX_3D}-L{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::LLower,
                asset_server.load(format!("{PREFIX_3D}-l{LOWER_3D}{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::M,
                asset_server.load(format!("{PREFIX_3D}-M{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::MLower,
                asset_server.load(format!("{PREFIX_3D}-m{LOWER_3D}{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::N,
                asset_server.load(format!("{PREFIX_3D}-N{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::NLower,
                asset_server.load(format!("{PREFIX_3D}-n{LOWER_3D}{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::O,
                asset_server.load(format!("{PREFIX_3D}-O{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::OLower,
                asset_server.load(format!("{PREFIX_3D}-o{LOWER_3D}{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::P,
                asset_server.load(format!("{PREFIX_3D}-P{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::PLower,
                asset_server.load(format!("{PREFIX_3D}-p{LOWER_3D}{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::S,
                asset_server.load(format!("{PREFIX_3D}-S{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::SLower,
                asset_server.load(format!("{PREFIX_3D}-s{LOWER_3D}{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::T,
                asset_server.load(format!("{PREFIX_3D}-T{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::TLower,
                asset_server.load(format!("{PREFIX_3D}-t{LOWER_3D}{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::U,
                asset_server.load(format!("{PREFIX_3D}-U{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::ULower,
                asset_server.load(format!("{PREFIX_3D}-u{LOWER_3D}{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::W,
                asset_server.load(format!("{PREFIX_3D}-W{SUFFIX_3D}")),
            ),
            (
                Scene3dKey::WLower,
                asset_server.load(format!("{PREFIX_3D}-w{LOWER_3D}{SUFFIX_3D}")),
            ),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum ImageKey {
    Ducky,
    CodonChart,
}

impl AssetKey for ImageKey {
    type Asset = Image;
}

impl FromWorld for HandleMap<ImageKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                ImageKey::Ducky,
                asset_server.load_with_settings(
                    "images/ducky.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
            (
                ImageKey::CodonChart,
                asset_server.load_with_settings(
                    "images/Aminoacids_table.svg.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SfxKey {
    ButtonHover,
    ButtonPress,
    Step1,
    Step2,
    Step3,
    Step4,
}

impl AssetKey for SfxKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SfxKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SfxKey::ButtonHover,
                asset_server.load("audio/sfx/button_hover.ogg"),
            ),
            (
                SfxKey::ButtonPress,
                asset_server.load("audio/sfx/button_press.ogg"),
            ),
            (SfxKey::Step1, asset_server.load("audio/sfx/step1.ogg")),
            (SfxKey::Step2, asset_server.load("audio/sfx/step2.ogg")),
            (SfxKey::Step3, asset_server.load("audio/sfx/step3.ogg")),
            (SfxKey::Step4, asset_server.load("audio/sfx/step4.ogg")),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SoundtrackKey {
    Credits,
    Gameplay,
}

impl AssetKey for SoundtrackKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SoundtrackKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SoundtrackKey::Credits,
                asset_server.load("audio/soundtracks/Monkeys Spinning Monkeys.ogg"),
            ),
            (
                SoundtrackKey::Gameplay,
                asset_server.load("audio/soundtracks/Fluffing A Duck.ogg"),
            ),
        ]
        .into()
    }
}

pub trait AssetKey: Sized {
    type Asset: Asset;
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct HandleMap<K: AssetKey>(HashMap<K, Handle<K::Asset>>);

impl<K: AssetKey, T> From<T> for HandleMap<K>
where
    T: Into<HashMap<K, Handle<K::Asset>>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<K: AssetKey> HandleMap<K> {
    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}
