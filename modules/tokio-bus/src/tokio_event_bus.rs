use asn_core_bus::{AsnBus, AsnBusRecvError, AsnTransmitter};
use asn_core_bus::{AsnBusSendError, AsnReceiver};
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;

/// Трансмиттер событий для Tokio EventBus
struct TokioTransmitter<E> {
    tx: Sender<E>,
}

/// Ресивер событий для Tokio EventBus
struct TokioReceiver<E> {
    rx: Receiver<E>,
}

impl<E> AsnTransmitter<E> for TokioTransmitter<E> {
    fn send_message(&self, m: E) -> Result<(), AsnBusSendError> {
        let result = self.tx.send(m);
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(AsnBusSendError::Closed),
        }
    }
}

impl<E> AsnReceiver<E> for TokioReceiver<E>
where
    E: Clone,
{
    fn get_message(&mut self) -> Result<E, AsnBusRecvError> {
        let result = self.rx.try_recv();
        match result {
            Ok(e) => Ok(e),
            Err(err) => match err {
                broadcast::error::TryRecvError::Empty => Err(AsnBusRecvError::Empty),
                broadcast::error::TryRecvError::Closed => Err(AsnBusRecvError::Closed),
                broadcast::error::TryRecvError::Lagged(n) => {
                    // Логгируем пропущенные сообщения и возвращаем пустое значение
                    eprintln!("Warning: Lagged {} messages", n);
                    Err(AsnBusRecvError::Empty)
                }
            },
        }
    }
}

/// Tokio EventBus реализация шины событий
pub struct TokioEventBus<E> {
    tx: Sender<E>,
}

impl<E> TokioEventBus<E>
where
    E: Clone,
{
    /// Создает новый Tokio EventBus с заданной ёмкостью канала
    fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel::<E>(capacity);
        TokioEventBus { tx }
    }
}

impl<E> AsnBus<E> for TokioEventBus<E>
where
    E: Clone,
{
    fn get_sender(&self) -> impl AsnTransmitter<E> {
        TokioTransmitter {
            tx: self.tx.clone(),
        }
    }

    fn get_receiver(&self) -> impl AsnReceiver<E> {
        TokioReceiver {
            rx: self.tx.subscribe(),
        }
    }
}

/// Фабричная функция для создания нового Tokio EventBus
pub fn new_tokio_bus<E: Clone>(capacity: usize) -> impl AsnBus<E> {
    TokioEventBus::new(capacity)
}
