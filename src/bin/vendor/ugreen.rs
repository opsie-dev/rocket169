use crate::com::ir::IRController;

/// Enumeration of every control available on original remote.
pub enum UGreenAW504Control {
    Button1,
    Button2,
    Button3,
    ButtonNext,
}

/// Remote controller interface.
pub struct UGreenAW504RemoteControl {
    ir_controller: &IRController,
}

impl UGreenAW504RemoteControl {
    pub fn new(
        ir_controller: &IRController,
    ) -> UGreenAW504RemoteControl {
        UGreenAW504RemoteControl {
            ir_controller: ir_controller,
        }
    }

    pub fn trigger(&self, control: UGreenAW504Control) {
        // TODO: implement.
    }
}
