
/// Wrapper around the bluer [session](bluer::Session) and [adapter](bluer::Adapter)
pub struct Session {
    pub session: bluer::Session,
    pub adapter: bluer::Adapter,
}
impl Session {
    /// Creates a new [Session](crate::Session)
    pub async fn new() -> bluer::Result<Self> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        Ok(Session { session, adapter })
    }
}