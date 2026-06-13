//! Estado compartilhado da aplicação, gerenciado pelo Tauri.

use std::collections::HashMap;
use std::sync::Mutex;

use sqlx::SqlitePool;
use tokio::sync::watch;

/// Estado global injetado nos commands.
pub struct AppState {
    pub db: SqlitePool,
    /// Canais de cancelamento ativos por operationId.
    pub cancel_senders: Mutex<HashMap<String, watch::Sender<bool>>>,
}

impl AppState {
    pub fn new(db: SqlitePool) -> Self {
        Self {
            db,
            cancel_senders: Mutex::new(HashMap::new()),
        }
    }

    /// Registra um canal de cancelamento para uma operação e retorna o receiver.
    pub fn register_cancel(&self, operation_id: &str) -> watch::Receiver<bool> {
        let (tx, rx) = watch::channel(false);
        if let Ok(mut map) = self.cancel_senders.lock() {
            map.insert(operation_id.to_string(), tx);
        }
        rx
    }

    /// Sinaliza cancelamento; retorna true se a operação existia.
    pub fn cancel(&self, operation_id: &str) -> bool {
        if let Ok(mut map) = self.cancel_senders.lock() {
            if let Some(tx) = map.remove(operation_id) {
                let _ = tx.send(true);
                return true;
            }
        }
        false
    }

    /// Remove o canal após a operação terminar.
    pub fn remove_cancel(&self, operation_id: &str) {
        if let Ok(mut map) = self.cancel_senders.lock() {
            map.remove(operation_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn state() -> AppState {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        AppState::new(pool)
    }

    #[tokio::test]
    async fn register_cancel_inicia_com_false() {
        let s = state().await;
        let rx = s.register_cancel("op-1");
        assert!(!*rx.borrow(), "receiver deve começar com false");
    }

    #[tokio::test]
    async fn cancel_sinaliza_true_e_retorna_true() {
        let s = state().await;
        let rx = s.register_cancel("op-2");
        let ok = s.cancel("op-2");
        assert!(ok, "cancel deve retornar true para operação existente");
        assert!(*rx.borrow(), "receiver deve ter sido sinalizado como true");
    }

    #[tokio::test]
    async fn cancel_retorna_false_para_operacao_inexistente() {
        let s = state().await;
        assert!(!s.cancel("nao-existe"));
    }

    #[tokio::test]
    async fn cancel_remove_entrada_do_mapa() {
        let s = state().await;
        s.register_cancel("op-3");
        s.cancel("op-3");
        // Segunda chamada deve retornar false — já foi removida.
        assert!(!s.cancel("op-3"));
    }

    #[tokio::test]
    async fn remove_cancel_nao_entra_em_panico_para_inexistente() {
        let s = state().await;
        s.remove_cancel("fantasma"); // não deve panic
    }

    #[tokio::test]
    async fn remove_cancel_remove_entrada() {
        let s = state().await;
        s.register_cancel("op-4");
        s.remove_cancel("op-4");
        // Após remoção, cancel deve retornar false.
        assert!(!s.cancel("op-4"));
    }
}
