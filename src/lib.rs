use std::{collections::HashMap, sync::OnceLock};

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
            Self::InitializeError => write!(f, "环境变量初始化错误"),
            Self::NotInitialized => write!(f, "环境变量未初始化，请先调用 init()"),
            Self::Missing(name) => write!(f, "环境变量 '{}' 缺失且无默认值", name),
            Self::ParseError { name, raw_value } => {
                write!(f, "环境变量 '{}' 的值 '{}' 解析失败", name, raw_value)
            }
        }
    }
}
impl std::error::Error for EnvError {}

static ENV_CACHE: OnceLock<HashMap<String, String>> = OnceLock::new();

pub fn init() -> Result<(), EnvError> {
    #[cfg(feature = "dotenv")]
    let _ = dotenvy::dotenv();
    let map: HashMap<String, String> = std::env::vars().collect();
    ENV_CACHE.set(map).map_err(|_| EnvError::InitializeError)
}
pub fn get(key: impl EnvKey) -> &'static str {
    try_get(key).expect("读取环境变量失败")
}

pub fn try_get<K: EnvKey>(_key: K) -> Result<&'static str, EnvError> {
    let cache = ENV_CACHE.get().ok_or(EnvError::NotInitialized)?;
    let name = K::name();
    cache
        .get(name)
        .map(|v| v.as_str())
        .or_else(|| K::default())
        .ok_or_else(|| EnvError::Missing(name.to_string()))
}

pub fn take<R: std::str::FromStr>(key: impl EnvKey) -> R
where
    <R as std::str::FromStr>::Err: std::fmt::Debug,
{
    key.take()
}
pub fn try_take<R: std::str::FromStr>(key: impl EnvKey) -> Result<R, EnvError>
where
    <R as std::str::FromStr>::Err: std::fmt::Debug,
{
    key.try_take()
}
pub trait EnvKey {
    fn name() -> &'static str;
    fn default() -> Option<&'static str> {
        None
    }
    fn try_get(&self) -> Result<&str, EnvError> {
        let cache = ENV_CACHE.get().ok_or(EnvError::NotInitialized)?;
        let name = Self::name();

        cache
            .get(name)
            .map(|v| v.as_str())
            .or_else(|| Self::default())
            .ok_or_else(|| EnvError::Missing(name.to_string()))
    }
    fn get(&self) -> &str {
        self.try_get().expect("读取环境变量失败")
    }
    fn try_take<R: std::str::FromStr>(&self) -> Result<R, EnvError> {
        let name = Self::name();
        let raw = self.try_get()?;
        raw.parse::<R>().map_err(|_| EnvError::ParseError {
            name: name.to_string(),
            raw_value: raw.to_string(),
        })
    }
    fn take<R: std::str::FromStr>(&self) -> R
    where
        <R as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.try_take::<R>().expect("解析环境变量失败")
    }
}
#[macro_export]
macro_rules! define {
	{$($name:ident $(: $default:expr)?),* $(,)?} => {
        $(
			#[allow(non_camel_case_types)]
			#[allow(dead_code)]
			#[derive(Copy, Clone)]
            pub struct $name;
            impl $crate::EnvKey for $name {
                fn name() -> &'static str { stringify!($name) }
				$(
                    fn default() -> Option<&'static str> {
                        Some($default)
                    }
                )?
            }
        )*
    };
}
