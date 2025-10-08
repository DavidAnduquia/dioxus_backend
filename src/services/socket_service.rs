use std::sync::OnceLock;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Estructura para manejar conexiones de WebSocket
/// Equivalente a SocketService en TypeScript
#[derive(Clone)]
#[allow(dead_code)]
pub struct SocketService {
    connections: Arc<RwLock<HashMap<i64, Vec<String>>>>, // user_id -> vec of socket_ids
}

impl SocketService {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Registra una nueva conexión de socket para un usuario
    pub async fn add_connection(&self, user_id: i64, socket_id: String) {
        tracing::info!("👤 Usuario {} conectado con socket {}", user_id, socket_id);
        let mut connections = self.connections.write().await;
        connections
            .entry(user_id)
            .or_insert_with(Vec::new)
            .push(socket_id);
    }

    /// Elimina una conexión de socket
    pub async fn remove_connection(&self, user_id: i64, socket_id: &str) {
        let mut connections = self.connections.write().await;
        if let Some(sockets) = connections.get_mut(&user_id) {
            sockets.retain(|id| id != socket_id);
            if sockets.is_empty() {
                connections.remove(&user_id);
            }
        }
        tracing::info!("👤 Usuario {} desconectado del socket {}", user_id, socket_id);
    }

    /// Emite una notificación a un usuario específico
    /// Equivalente a emitNotificationToUser en TypeScript
    #[allow(dead_code)]
    pub async fn emit_notification_to_user(&self, user_id: i64, notification: Value) {
        let connections = self.connections.read().await;
        if let Some(sockets) = connections.get(&user_id) {
            tracing::info!(
                "📤 Emitiendo notificación al usuario {} ({} conexiones)",
                user_id,
                sockets.len()
            );
            // Aquí se enviaría el mensaje a través de los sockets
            // Por ahora solo registramos la acción
            for socket_id in sockets {
                tracing::debug!("  → Socket {}: {:?}", socket_id, notification);
            }
        } else {
            tracing::warn!("⚠️  Usuario {} no tiene conexiones activas", user_id);
        }
    }

    /// Emite una notificación a múltiples usuarios
    /// Equivalente a emitNotificationToUsers en TypeScript
    #[allow(dead_code)]
    pub async fn emit_notification_to_users(&self, user_ids: Vec<i64>, notification: Value) {
        for user_id in user_ids {
            self.emit_notification_to_user(user_id, notification.clone())
                .await;
        }
    }

    /// Emite una notificación broadcast a todos los usuarios conectados
    /// Equivalente a emitNotificationBroadcast en TypeScript
    #[allow(dead_code)]
    pub async fn emit_notification_broadcast(&self, notification: Value) {
        let connections = self.connections.read().await;
        let total_users = connections.len();
        tracing::info!("📢 Broadcast de notificación a {} usuarios", total_users);
        
        for (user_id, sockets) in connections.iter() {
            for socket_id in sockets {
                tracing::debug!(
                    "  → Usuario {} Socket {}: {:?}",
                    user_id,
                    socket_id,
                    notification
                );
            }
        }
    }

    /// Verifica si el servicio de sockets está disponible
    /// Equivalente a isAvailable en TypeScript
    #[allow(dead_code)]
    pub async fn is_available(&self) -> bool {
        true // En Rust siempre está disponible si la instancia existe
    }

    /// Obtiene información de conexión
    /// Equivalente a getConnectionInfo en TypeScript
    #[allow(dead_code)]
    pub async fn get_connection_info(&self) -> ConnectionInfo {
        let connections = self.connections.read().await;
        let connected_users = connections.len();
        let rooms: Vec<String> = connections
            .keys()
            .map(|user_id| format!("user_{}", user_id))
            .collect();

        ConnectionInfo {
            connected_users,
            rooms,
        }
    }

    /// Obtiene el número total de conexiones activas
    #[allow(dead_code)]
    pub async fn get_total_connections(&self) -> usize {
        let connections = self.connections.read().await;
        connections.values().map(|v| v.len()).sum()
    }
}

impl Default for SocketService {
    fn default() -> Self {
        Self::new()
    }
}

/// Información de conexión del servidor de sockets
#[derive(Debug, Clone, serde::Serialize)]
pub struct ConnectionInfo {
    pub connected_users: usize,
    pub rooms: Vec<String>,
}

/// Instancia singleton global del SocketService
/// Equivalente a socketService = SocketService.getInstance() en TypeScript
pub static SOCKET_SERVICE: OnceLock<SocketService> = OnceLock::new();

/// Función helper para obtener la instancia global
pub fn get_socket_service() -> &'static SocketService {
    SOCKET_SERVICE.get_or_init(|| SocketService::new())
}
