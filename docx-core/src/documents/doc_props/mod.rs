mod app;
mod core;

pub use self::app::*;
pub use self::core::*;

use crate::documents::BuildXML;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocProps {
    pub app: AppProps,
    pub core: CoreProps,
}

impl DocProps {
    pub(crate) fn new(core_config: CorePropsConfig) -> DocProps {
        let app = AppProps::new();
        let core = CoreProps::new(core_config);
        DocProps { app, core }
    }

    pub fn created_at(mut self, date: &str) -> Self {
        self.core = self.core.created_at(date);
        self
    }

    pub fn updated_at(mut self, date: &str) -> Self {
        self.core = self.core.updated_at(date);
        self
    }

    pub(crate) fn build(&self) -> XMLDocProps {
        XMLDocProps {
            app: self.app.build(),
            core: self.core.build(),
        }
    }
}

#[derive(Debug)]
pub struct XMLDocProps {
    pub app: Vec<u8>,
    pub core: Vec<u8>,
}
