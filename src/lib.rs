use std::{collections::HashMap, fmt::Debug, str::FromStr, sync::OnceLock};

static ENV_CACHE: OnceLock<HashMap<String, String>> = OnceLock::new();

#[derive(Debug)]
pub enum EnvError {
    InitializeError,
    NotInitialized,
    Missing(String),
    ParseError { name: String, raw_value: String },
}

impl std::fmt::Display for EnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitializeError => {
                write!(f, "QEnv: Initialization error (possibly called twice)")
            }
            Self::NotInitialized => write!(f, "QEnv: Not initialized. Call qenv::init() first"),
            Self::Missing(name) => write!(f, "QEnv: Missing environment variable '{}'", name),
            Self::ParseError { name, raw_value } => {
                write!(
                    f,
                    "QEnv: Failed to parse '{}' with value '{}'",
                    name, raw_value
                )
            }
        }
    }
}
impl std::error::Error for EnvError {}

// --- 核心 Trait ---
pub trait EnvKey {
    fn name() -> &'static str;
    fn default() -> Option<&'static str> {
        None
    }
}
pub struct EnvVar<K: EnvKey>(pub K);

impl<K: EnvKey> EnvVar<K> {
    pub fn get(&self) -> &str {
        self.try_get().expect("QEnv: Get failed")
    }

    pub fn try_get(&self) -> Result<&str, EnvError> {
        let cache = ENV_CACHE.get().ok_or(EnvError::NotInitialized)?;
        let name = K::name();
        cache
            .get(name)
            .map(|v| v.as_str())
            .or_else(|| K::default())
            .ok_or_else(|| EnvError::Missing(name.to_string()))
    }

    pub fn take<R: FromStr>(&self) -> R
    where
        <R as FromStr>::Err: Debug,
    {
        self.try_take().expect("QEnv: Take failed")
    }

    pub fn try_take<R: FromStr>(&self) -> Result<R, EnvError>
    where
        <R as FromStr>::Err: Debug,
    {
        let raw = self.try_get()?;
        raw.parse::<R>().map_err(|_| EnvError::ParseError {
            name: K::name().to_string(),
            raw_value: raw.to_string(),
        })
    }
}

pub fn init() -> Result<(), EnvError> {
    #[cfg(feature = "dotenv")]
    let _ = dotenvy::dotenv();
    let map: HashMap<String, String> = std::env::vars().collect();
    ENV_CACHE.set(map).map_err(|_| EnvError::InitializeError)
}

#[macro_export]
macro_rules! define {
    ($($name:ident $(: $default:expr)?),* $(,)?) => {
        $(
			#[allow(non_snake_case)]
            mod $name {
                #[derive(Copy, Clone)]
                pub struct Tag;
                impl $crate::EnvKey for Tag {
                    fn name() -> &'static str { stringify!($name) }
                    $( fn default() -> Option<&'static str> { Some($default) } )?
                }
            }
            pub const $name: $crate::EnvVar<$name::Tag> = $crate::EnvVar($name::Tag);
        )*
    };
}
