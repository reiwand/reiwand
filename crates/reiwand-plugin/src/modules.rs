use rquickjs::loader::{ImportAttributes, Loader, Resolver};
use rquickjs::module::Declared;
use rquickjs::{Ctx, Error, Module, Result};
use std::collections::BTreeMap;

#[derive(Clone)]
pub(crate) struct Modules(pub(crate) BTreeMap<String, String>);

fn joined(base: &str, name: &str) -> String {
    let mut parts: Vec<&str> = base.rsplit_once('/').map_or(Vec::new(), |(dir, _)| dir.split('/').collect());
    for part in name.split('/') {
        match part {
            "." | "" => {}
            ".." => {
                parts.pop();
            }
            part => parts.push(part),
        }
    }
    parts.join("/")
}

impl Resolver for Modules {
    fn resolve<'js>(
        &mut self,
        _: &Ctx<'js>,
        base: &str,
        name: &str,
        _: Option<ImportAttributes<'js>>,
    ) -> Result<String> {
        let path = match name.starts_with("./") || name.starts_with("../") {
            true => joined(base, name),
            false => name.to_owned(),
        };
        match self.0.contains_key(&path) {
            true => Ok(path),
            false => Err(Error::new_resolving_message(
                base,
                name,
                "a plugin imports only its own modules, by a relative path that ends in .ts",
            )),
        }
    }
}

impl Loader for Modules {
    fn load<'js>(
        &mut self,
        ctx: &Ctx<'js>,
        name: &str,
        _: Option<ImportAttributes<'js>>,
    ) -> Result<Module<'js, Declared>> {
        let source = self.0.get(name).ok_or_else(|| Error::new_loading(name))?;
        Module::declare(ctx.clone(), name, source.as_str())
    }
}
