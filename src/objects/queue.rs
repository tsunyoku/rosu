use tokio::sync::Mutex;

pub struct PacketQueue {
    queue: Mutex<Vec<u8>>,
}

impl PacketQueue {
    pub fn new() -> Self {
        return Self {
            queue: Mutex::new(Vec::with_capacity(512)),
        };
    }

    #[inline(always)]
    pub async fn dequeue(&self) -> Vec<u8> {
        let mut queue = self.queue.lock().await;
        let queue_vec = queue.clone();

        queue.clear();
        return queue_vec;
    }

    pub async fn enqueue(&self, bytes: Vec<u8>) {
        self.queue.lock().await.extend(bytes);
    }
}