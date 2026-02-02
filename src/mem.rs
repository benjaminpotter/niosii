pub(crate) struct ReadError {}
pub(crate) struct WriteError {}
pub(crate) struct Memory {}

impl Memory {
    pub(crate) fn new() -> Self {
        todo!()
    }

    pub(crate) fn read<A, W>(&self, addr: &A) -> Result<&W, ReadError> {
        todo!()
    }

    pub(crate) fn write<A, W>(&self, addr: &A, word: W) -> Result<W, WriteError> {
        todo!()
    }
}
