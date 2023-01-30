use crate::pb::{
    chat_server::{Chat, ChatServer},
    ChatMessage, GetMessageRequest, LoginRequest, NewChatMessage, SendMessageResponse, Token,
};
use futures::prelude::*;
use std::pin::Pin;
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{transport::Server, Request, Response, Status, Extensions};
use tracing::{info, log::warn};

pub struct ChatService {
    tx: broadcast::Sender<ChatMessage>,
}
pub type ChatResult<T> = Result<Response<T>, Status>;
const MAX_MESSAGE: usize = 1024;

#[tonic::async_trait]
impl Chat for ChatService {
    async fn login(&self, request: Request<LoginRequest>) -> ChatResult<Token> {
        let info = request.into_inner();
        info!("login:{info:?}");
        let token = info.into_token();
        Ok(Response::new(token))
    }

    async fn send_message(
        &self,
        request: Request<NewChatMessage>,
    ) -> ChatResult<SendMessageResponse> {
        let sender = get_username(request.extensions())?;
        let info = request.into_inner();
        info!("send_message:{info:?}");
        let msg = info.into_chat_message(sender);
        self.tx.send(msg).unwrap();
        Ok(Response::new(SendMessageResponse {}))
    }
    type GetMessageStream = Pin<Box<dyn Stream<Item = Result<ChatMessage, tonic::Status>> + Send>>;

    async fn get_message(
        &self,
        request: Request<GetMessageRequest>,
    ) -> ChatResult<Self::GetMessageStream> {
        let info = request.into_inner();
        info!("subscribe to message {info:?}");

        let mut rx = self.tx.subscribe();
        let (sender, receiver) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            //todo:filter out message i'm not interesting it
            while let Ok(msg) = rx.recv().await {
                if let Err(_) = sender.send(Ok(msg)) {
                    warn!("Failed to send. Sender might be closed");
                    return;
                }
            }
        });

        let stream = UnboundedReceiverStream::new(receiver);
        Ok(Response::new(Box::pin(stream)))
    }
}

impl Default for ChatService {
    fn default() -> Self {
        let (tx, _rx) = broadcast::channel(MAX_MESSAGE);
        Self { tx }
    }
}

pub async fn start() {
    let svc = ChatServer::with_interceptor(ChatService::default(), check_auth);
    let addr = "0.0.0.0:8080".parse().unwrap();
    info!("listening on http://{}", addr);
    Server::builder()
        .add_service(svc)
        .serve(addr)
        .await
        .unwrap();
}

fn check_auth(mut req: Request<()>) -> Result<Request<()>, Status> {
    let token = match req.metadata().get("authorization") {
        Some(v) => {
            let data = v
                .to_str()
                .map_err(|_| Status::new(tonic::Code::Unauthenticated, "Invalid token format"))?;

            Token::new(data.strip_prefix("Bearer").unwrap())
        }
        None => Token::default(),
    };
    req.extensions_mut().insert(token);
    Ok(req)
}

fn get_username(ext: &Extensions) -> Result<String, Status> {
    let token = ext.get::<Token>().ok_or(Status::unauthenticated("No token"))?;

    if token.is_valid() {
        Ok(token.into_username())
    } else {
        Err(Status::unauthenticated("Invalid token"))
    }
    
}
