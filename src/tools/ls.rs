use crate::args::LsArgs;

pub struct LsParser {
    args: LsArgs,
}

impl LsParser {
    pub fn new(args: LsArgs) -> Self {
        Self { args }
    }

    pub fn parse(&self) -> String {
        todo!()
    }
}
