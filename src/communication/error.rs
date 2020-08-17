#[derive(ToPrimitive)]
pub enum ProcessError {
    ExtractFailed = 1,
    Unsupported = 2,
}
