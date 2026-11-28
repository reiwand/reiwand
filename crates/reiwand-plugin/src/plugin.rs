use crate::manifest::{Kind, Setting};
use crate::runner::{Call, run};
use crate::{Error, Installed, Limits, Net, Page};
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub enum Isolation {
    Inline,
    Process(PathBuf, Vec<String>),
}

impl Isolation {
    pub fn standard() -> Isolation {
        #[cfg(target_os = "android")]
        return Isolation::Inline;
        #[cfg(not(target_os = "android"))]
        Isolation::Process("/proc/self/exe".into(), vec![crate::HOST_ARG.into()])
    }
}

pub struct Plugin {
    pub installed: Installed,
    pub limits: Limits,
}

impl Plugin {
    pub fn new(installed: Installed) -> Plugin {
        Plugin { installed, limits: Limits::STANDARD }
    }

    pub fn settings(&self, given: &BTreeMap<String, String>) -> Result<Map<String, Value>, Error> {
        let declared = &self.installed.manifest.settings;
        if let Some(key) = given.keys().find(|k| !declared.iter().any(|s| &s.key == *k)) {
            return Err(Error::Setting(format!("the plugin has no setting {key}")));
        }
        let mut out = Map::new();
        for setting in declared {
            let text = given.get(&setting.key).ok_or_else(|| {
                Error::Setting(format!("the setting {} is missing: write its value", setting.key))
            })?;
            let value = parse(setting, text);
            setting.value(&value).map_err(|why| Error::Setting(format!("{}: {why}", setting.key)))?;
            out.insert(setting.key.clone(), value);
        }
        Ok(out)
    }

    pub fn list(
        &self,
        net: Arc<Net>,
        settings: Map<String, Value>,
        page: Option<String>,
        isolation: &Isolation,
    ) -> Result<Page, Error> {
        let (call, limits, modules) = (Call { settings, page }, self.limits, &self.installed.modules);
        let until = Instant::now() + limits.wall;
        let fetch = move |request: &_| net.fetch(request, until, limits.response);
        let value = match isolation {
            Isolation::Inline => run(modules, &limits, &call, Box::new(fetch))?,
            #[cfg(not(target_os = "android"))]
            Isolation::Process(program, args) => {
                crate::child::run_in(&(program.clone(), args.clone()), modules, &limits, &call, fetch)?
            }
            #[cfg(target_os = "android")]
            Isolation::Process(..) => return Err(Error::Host("the phone runs plugins inline".into())),
        };
        Page::read(&self.installed.manifest, &value, &limits)
    }
}

/// The value a form's text stands for: a number, a yes or no, or the text itself; `null` when the
/// text is none of what the setting takes, which `Setting::value` then refuses.
pub fn parse(setting: &Setting, text: &str) -> Value {
    match setting.kind {
        Kind::Bool => text.parse::<bool>().map(Value::from).unwrap_or(Value::Null),
        Kind::Int => text.parse::<i64>().map(Value::from).unwrap_or(Value::Null),
        Kind::Text | Kind::Choice => Value::from(text),
    }
}
