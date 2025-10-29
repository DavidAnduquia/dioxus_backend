use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::AppState;
use crate::services::socket_service::get_socket_service;

/// Estructura para manejar eventos de conexión de usuario
/// Equivalente a ConnectedUser en TypeScript
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedUser {
    pub identificador: String,
    pub user_id: i32,  // Cambiado de i64 a i32 para consistencia
    pub nombre: Option<String>,
}

/// Eventos de socket que el cliente puede enviar
/// Equivalente a los eventos manejados en socket.manager.ts
#[derive(Debug, Deserialize)]
#[serde(tag = "event", content = "data")]
pub enum SocketEvent {
    #[serde(rename = "user_connected")]
    UserConnected(ConnectedUser),
    #[serde(rename = "join_notifications")]
    JoinNotifications { user_id: i32 },
    #[serde(rename = "leave_notifications")]
    LeaveNotifications { user_id: i32 },
    #[serde(rename = "get_notification_status")]
    GetNotificationStatus { user_id: i32 },
    #[serde(rename = "solicitar_usuarios_conectados")]
    SolicitarUsuariosConectados,
}

/// Manejador principal de WebSocket
/// Equivalente a SocketManager en TypeScript
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(_state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket))
}

/// Maneja la conexión WebSocket individual
/// Equivalente a la lógica dentro de io.on('connection') en TypeScript
async fn handle_socket(mut socket: WebSocket) {
    let socket_id = uuid::Uuid::new_v4().to_string();
    tracing::info!("🔌 Nuevo cliente conectado: {}", socket_id);

    let socket_service = get_socket_service();
    
    // Estado local para rastrear el usuario conectado
    let current_user: Arc<RwLock<Option<i32>>> = Arc::new(RwLock::new(None));

    // Procesar mensajes del cliente
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            tracing::debug!("📨 Mensaje recibido: {}", text);
            
            // Intentar parsear el evento
            match serde_json::from_str::<SocketEvent>(&text) {
                Ok(event) => {
                    handle_socket_event(event, socket_service, &socket_id, &current_user).await;
                }
                Err(e) => {
                    tracing::warn!("⚠️  Error parseando evento: {}", e);
                }
            }
        } else if let Message::Close(_) = msg {
            tracing::info!("🔌 Cliente {} cerró la conexión", socket_id);
            break;
        }
    }

    // Limpiar la conexión
    if let Some(user_id) = *current_user.read().await {
        socket_service.remove_connection(user_id as i64, &socket_id).await;
    }
    
    tracing::info!("🔌 Cliente desconectado: {}", socket_id);
}

/// Maneja los diferentes eventos de socket
/// Equivalente a los socket.on() handlers en TypeScript
async fn handle_socket_event(
    event: SocketEvent,
    socket_service: &crate::services::socket_service::SocketService,
    socket_id: &str,
    current_user: &Arc<RwLock<Option<i32>>>,
) {
    match event {
        SocketEvent::UserConnected(user) => {
            tracing::info!("👤 Usuario conectado: {} (ID: {})", user.identificador, user.user_id);
            socket_service
                .add_connection(user.user_id as i64, socket_id)
                .await;
            
            // Guardar el user_id actual
            *current_user.write().await = Some(user.user_id);
        }
        SocketEvent::JoinNotifications { user_id } => {
            let room_name = format!("user_{}", user_id);
            tracing::info!("👤 Usuario {} se unió a la sala de notificaciones: {}", user_id, room_name);
            
            // En una implementación completa, aquí se manejarían las "rooms"
            // Por ahora solo registramos la acción
        }
        SocketEvent::LeaveNotifications { user_id } => {
            let room_name = format!("user_{}", user_id);
            tracing::info!("👤 Usuario {} salió de la sala de notificaciones: {}", user_id, room_name);
        }
        SocketEvent::GetNotificationStatus { user_id } => {
            let connection_info = socket_service.get_connection_info().await;
            tracing::info!(
                "📊 Estado de notificaciones para usuario {}: {} usuarios conectados",
                user_id,
                connection_info.connected_users
            );
            
            // En una implementación completa, aquí se enviaría la respuesta al cliente
            let user_room = format!("user_{}", user_id);
            let user_is_in_room = connection_info.rooms.iter().any(|r| r == &user_room);
            
            let _response = json!({
                "event": "notification_status",
                "data": {
                    "user_id": user_id,
                    "connected": true,
                    "total_connections": connection_info.connected_users,
                    "user_rooms": if user_is_in_room { vec![&user_room] } else { vec![] }
                }
            });
        }
        SocketEvent::SolicitarUsuariosConectados => {
            let connection_info = socket_service.get_connection_info().await;
            tracing::info!(
                "📊 Solicitud de usuarios conectados: {} usuarios activos",
                connection_info.connected_users
            );
            
            // En una implementación completa, aquí se enviaría la lista de usuarios conectados
            let _response = json!({
                "event": "usuarios_conectados",
                "data": {
                    "total_usuarios": connection_info.connected_users,
                    "total_rooms": connection_info.rooms.len(),
                    "timestamp": chrono::Utc::now().timestamp()
                }
            });
        }
    }
}
