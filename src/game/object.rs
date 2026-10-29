use mlua::{IntoLuaMulti, ObjectLike, Table, Value};

use crate::error::{Error, Result};

#[derive(Debug)]
pub struct Object {
    local: Table,
}

impl Object {
    pub fn new(local: Table) -> Self {
        Self { local }
    }

    pub fn emit(&self, name: &str, args: impl IntoLuaMulti) -> Result<()> {
        self.local
            .call_method(name, args)
            .map(|_: Value| ())
            .map_err(Error::from)
    }
}
