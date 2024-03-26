use tokio::sync::mpsc::channel;

/// The chat thread is I/O heavy and spends most of its time awaiting new messages.
/// It is therefore most appropriate to run this thread on the tokio runtime.
pub async fn chat_thread() {
    
}
