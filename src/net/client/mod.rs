// use bytebuffer::ByteBuffer;
// use tokio::sync::mpsc::Receiver;
// use crate::net::codec::ProtocolStage;
//
// pub struct ClientListener {
//     incoming: Receiver<(i32, ByteBuffer)>,
//     stage: ProtocolStage
// }
//
// impl ClientListener {
//     pub fn new(incoming: Receiver<(i32, ByteBuffer)>) -> Self {
//         Self { incoming }
//     }
//
//     pub async fn listen(&mut self) {
//         loop {
//             let (id, data) = self.incoming.try_recv().unwrap();
//             match self.stage {
//                 ProtocolStage::Handshake => {
//
//                 }
//                 ProtocolStage::
//             }
//         }
//     }
// }