use crate::com::ir::IRController;

/// Enumeration of every control available on original remote.
pub enum WimiusS27Control {
    ButtonPower,
    ButtonMenu,
    ButtonSource,
    ButtonDown,
    ButtonLeft,
    ButtonRight,
    ButtonUp,
    ButtonOK,
    ButtonHome,
    ButtonReturn,
    ButtonVolumeDown,
    ButtonVolumeUp,
    ButtonVolumeMute,
}

/// Remote controller interface.
pub struct WimiusS27RemoteControl<'a, C>
where
    C: IRController
{
    ir_controller: &'a C,
}

impl<'a, C> WimiusS27RemoteControl<'a, C> 
where 
    C: IRController
{
    pub fn new(
        ir_controller: &C,
    ) -> WimiusS27RemoteControl<'a, C> {
        WimiusS27RemoteControl {
            ir_controller: ir_controller,
        }
    }

    pub fn trigger(&self, control: WimiusS27Control) {
        // TODO: implement.
    }
}
