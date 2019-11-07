mod app;
mod core;

pub use self::app::*;
pub use self::core::*;
use crate::documents::BuildXML;

#[derive(Debug)]
pub(crate) struct DocProps {
    app: AppProps,
    core: CoreProps,
}

impl DocProps {
    pub(crate) fn new(
        app_config: Option<AppPropsConfig>,
        core_config: Option<CorePropsConfig>,
    ) -> DocProps {
        let app = AppProps::new(app_config);
        let core = CoreProps::new(core_config);
        DocProps { app, core }
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
