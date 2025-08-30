#[derive(Debug)]
pub enum AsnBusSendError {
    Full,
    Disconnected,
    Closed,
}

#[derive(Debug)]
pub enum AsnBusRecvError {
    Empty,
    Closed,
    Lagged(u64),
}

pub trait AsnTransmitter<M> {
    fn send_message(&self, m: M) -> Result<(), AsnBusSendError>;

    /// Sends a message asynchronously, waiting if the channel is full.
    ///
    /// This method will wait until there is space in the channel to send the message.
    ///
    /// # Arguments
    ///
    /// * `m` - The message to send
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the message was sent successfully
    /// * `Err(AsnBusSendError)` if there was an error sending the message
    fn send_message_async(
        &self,
        m: M,
    ) -> impl std::future::Future<Output = Result<(), AsnBusSendError>> + Send;
}

pub trait AsnReceiver<M> {
    fn get_message(&mut self) -> Result<M, AsnBusRecvError>;

    /// Waits asynchronously for a message to be available.
    ///
    /// This method will wait until a message is available in the channel.
    ///
    /// # Returns
    ///
    /// * `Ok(M)` with the received message
    /// * `Err(AsnBusRecvError)` if there was an error receiving the message
    fn wait_for_message(
        &mut self,
    ) -> impl std::future::Future<Output = Result<M, AsnBusRecvError>> + Send;
}

pub trait AsnBus<M> {
    fn get_sender(&self) -> impl AsnTransmitter<M>;
    fn get_receiver(&self) -> impl AsnReceiver<M>;
}
