#[derive(Debug)]
pub(crate) enum DictLabelType {
    Short, // high bit is 0
    Long,  // high bits are 10
    Same,  // high bits are 11
}
