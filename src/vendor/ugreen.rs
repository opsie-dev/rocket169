use log::debug;

/// Enumeration of every control available on original remote.
pub enum UGreenAW504Control {
    Button1,
    Button2,
    Button3,
    ButtonNext,
}

/// Remote controller interface.
pub struct UGreenAW504RemoteControl {}

impl UGreenAW504RemoteControl {
    pub fn new() -> UGreenAW504RemoteControl {
        UGreenAW504RemoteControl {}
    }

    pub fn trigger(&self, control: UGreenAW504Control) {
        match control {
            UGreenAW504Control::Button1 => {
                debug!("[UgreenAW504] 1 button triggered");
            }
            UGreenAW504Control::Button2 => {
                debug!("[UgreenAW504] 2 button triggered");
            }
            UGreenAW504Control::Button3 => {
                debug!("[UgreenAW504] 1 button triggered");
            }
            UGreenAW504Control::ButtonNext => {
                debug!("[UgreenAW504] Next button triggered");
            }
        }
    }
}
