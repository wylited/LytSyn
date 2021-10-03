use crate::config::LytConfig;
use tui::{Frame, backend::CrosstermBackend};
use std::io::StdoutLock;

pub struct REngine<'a> {
    rect: &'a mut Frame<'a, CrosstermBackend<StdoutLock<'a>>>,
    config: LytConfig,
}

impl<'a> REngine<'a> {
    pub fn construct(rect: &'a mut Frame<'_, CrosstermBackend<StdoutLock<'a>>>, config: LytConfig) -> Result<REngine<'a>, std::io::Error>{
        Ok(REngine {
            rect,
            config,
        })
    }
}