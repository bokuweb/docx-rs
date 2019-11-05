mod app;

pub use app::*;

pub(crate) struct DocProps {
    app: AppProps,
}

impl DocProps {
    pub(crate) fn new(appConfig: Option<AppPropsConfig>) -> DocProps {
        let app = AppProps::new(appConfig);
        DocProps { app }
    }
}
