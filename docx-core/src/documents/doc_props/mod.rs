mod app;
mod core;

pub use self::app::*;
pub use self::core::*;

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
}
