use esp_hal::gpio::Input;

struct RemoteControllerButton {
    input: Input<'static>,
}

impl RemoteControllerButton {
    fn new(input: Input<'static>) -> RemoteControllerButton {
        RemoteControllerButton {
            input: input,
        }
    }
    fn poll(&self) -> bool {
        return self.input.is_high()
    }
}

pub(crate) struct RemoteControllerButtons {
    // Remote control source selector.
    apple: RemoteControllerButton,
    triangle: RemoteControllerButton,
    wimius: RemoteControllerButton,
    // Sound controls.
    sound_bass_up: RemoteControllerButton,
    sound_bass_down: RemoteControllerButton,
    sound_eq_reset: RemoteControllerButton,
    sound_mute: RemoteControllerButton,
    sound_source: RemoteControllerButton,
    sound_treble_up: RemoteControllerButton,
    sound_treble_down: RemoteControllerButton,
    sound_volume_up: RemoteControllerButton,
    sound_volume_down: RemoteControllerButton,
    // TV controls.
    tv_back: RemoteControllerButton,
    tv_home: RemoteControllerButton,
    tv_menu: RemoteControllerButton,
    tv_up: RemoteControllerButton,
    tv_down: RemoteControllerButton,
    tv_left: RemoteControllerButton,
    tv_right: RemoteControllerButton,
    tv_play_pause: RemoteControllerButton,
    // HDMI source controls.
    hdmi_source_1: RemoteControllerButton,
    hdmi_source_2: RemoteControllerButton,
    hdmi_source_3: RemoteControllerButton,
    // Light controls.
    light_on: RemoteControllerButton,
    light_off: RemoteControllerButton,
}

impl RemoteControllerButtons {
    pub(crate) fn new(
        input_apple: Input<'static>,
        input_triangle: Input<'static>,
        input_wimius: Input<'static>,
        input_sound_bass_up: Input<'static>,
        input_sound_bass_down: Input<'static>,
        input_sound_eq_reset: Input<'static>,
        input_sound_mute: Input<'static>,
        input_sound_source: Input<'static>,        
        input_sound_treble_up: Input<'static>,
        input_sound_treble_down: Input<'static>,
        input_sound_volume_up: Input<'static>,
        input_sound_volume_down: Input<'static>,
        input_tv_back: Input<'static>,
        input_tv_home: Input<'static>,
        input_tv_menu: Input<'static>,
        input_tv_up: Input<'static>,
        input_tv_down: Input<'static>,
        input_tv_left: Input<'static>,
        input_tv_right: Input<'static>,
        input_tv_play_pause: Input<'static>,
        input_hdmi_source_1: Input<'static>,
        input_hdmi_source_2: Input<'static>,
        input_hdmi_source_3: Input<'static>,
        input_light_on: Input<'static>,
        input_light_off: Input<'static>,
    ) -> RemoteControllerButtons {
        RemoteControllerButtons {
            apple: RemoteControllerButton::new(input_apple),
            triangle: RemoteControllerButton::new(input_triangle),
            wimius: RemoteControllerButton::new(input_wimius),
            sound_bass_down: RemoteControllerButton::new(input_sound_bass_down),
            sound_bass_up: RemoteControllerButton::new(input_sound_bass_up),
            sound_eq_reset: RemoteControllerButton::new(input_sound_eq_reset),
            sound_mute: RemoteControllerButton::new(input_sound_mute),
            sound_source: RemoteControllerButton::new(input_sound_source),
            sound_treble_down: RemoteControllerButton::new(input_sound_treble_down),
            sound_treble_up: RemoteControllerButton::new(input_sound_treble_up),
            sound_volume_down: RemoteControllerButton::new(input_sound_volume_down),
            sound_volume_up: RemoteControllerButton::new(input_sound_volume_up),
            tv_back: RemoteControllerButton::new(input_tv_back),
            tv_home: RemoteControllerButton::new(input_tv_home),
            tv_menu: RemoteControllerButton::new(input_tv_menu),
            tv_play_pause: RemoteControllerButton::new(input_tv_play_pause),
            tv_up: RemoteControllerButton::new(input_tv_up),
            tv_down: RemoteControllerButton::new(input_tv_down),
            tv_left: RemoteControllerButton::new(input_tv_left),
            tv_right: RemoteControllerButton::new(input_tv_right),
            hdmi_source_1: RemoteControllerButton::new(input_hdmi_source_1),
            hdmi_source_2: RemoteControllerButton::new(input_hdmi_source_2),
            hdmi_source_3: RemoteControllerButton::new(input_hdmi_source_3),
            light_off: RemoteControllerButton::new(input_light_off),
            light_on: RemoteControllerButton::new(input_light_on),
        }
    }
}
