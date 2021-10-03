use rodio::OutputStream;

pub struct SEngine {
    pub stream: OutputStream,
    pub stream_handle: OutputStreamHandle,
    pub sink: Sink,
}

impl SEngine {
    pub fn create() -> self{
        self {
            stream: OutputStream::try_default().unwrap().0,
            stream_handle: OutputStream::try_default().unwrap().1,
            sink: Sink::try_new(&streamhandle).unwrap(),
        }
    }
}