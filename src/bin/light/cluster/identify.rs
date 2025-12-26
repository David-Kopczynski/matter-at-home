use log::*;
use rs_matter_embassy::*;

matter::import!(Identify);

pub struct IdentifyHandler {
    dataver: matter::dm::Dataver,
    time: core::cell::Cell<u16>,
}
impl IdentifyHandler {
    pub fn new(dataver: matter::dm::Dataver) -> Self {
        Self {
            dataver,
            time: core::cell::Cell::new(0),
        }
    }

    pub const fn adapt(self) -> identify::HandlerAdaptor<Self> {
        identify::HandlerAdaptor(self)
    }
}

/// Cluster implementation for Identify
impl identify::ClusterHandler for IdentifyHandler {
    const CLUSTER: matter::dm::Cluster<'static> =
        identify::FULL_CLUSTER.with_attrs(matter::with!(required));

    fn dataver(&self) -> u32 {
        self.dataver.get()
    }
    fn dataver_changed(&self) {
        self.dataver.changed();
    }

    // ---------- Attributes ----------

    fn identify_time(
        &self,
        _ctx: impl matter::dm::ReadContext,
    ) -> Result<u16, matter::error::Error> {
        Ok(self.time.get())
    }
    fn identify_type(
        &self,
        _ctx: impl matter::dm::ReadContext,
    ) -> Result<identify::IdentifyTypeEnum, matter::error::Error> {
        Ok(identify::IdentifyTypeEnum::None)
    }
    fn set_identify_time(
        &self,
        _ctx: impl matter::dm::WriteContext,
        value: u16,
    ) -> Result<(), matter::error::Error> {
        self.time.set(value);

        Ok(())
    }

    // ---------- Commands ----------

    fn handle_identify(
        &self,
        _ctx: impl matter::dm::InvokeContext,
        request: identify::IdentifyRequest<'_>,
    ) -> Result<(), matter::error::Error> {
        // Fake identify in console
        if let Ok(time) = request.identify_time() {
            info!("Identify for {} seconds", time);

            Ok(())
        } else {
            Err(matter::error::ErrorCode::Failure.into())
        }
    }
    fn handle_trigger_effect(
        &self,
        _ctx: impl matter::dm::InvokeContext,
        _request: identify::TriggerEffectRequest<'_>,
    ) -> Result<(), matter::error::Error> {
        // Optional effect not implemented
        Err(matter::error::ErrorCode::InvalidCommand.into())
    }
}
