use iced::{
    button, scrollable, slider, text_input, Button, checkbox,
    Color, Column, Container, Element, HorizontalAlignment,
    Image, Length, Radio, Row, Sandbox, Scrollable, Settings,
    Slider, Space, Text, TextInput, 
};

use crate::gui::structs::*;

enum Step {
    Welcome,
    Slider {
        state: slider::State,
        value: u8,
    },
    RowsAndColumns {
        layout: Layout,
        spacing_slider: slider::State,
        spacing: u16,
    },
    Text {
        size_slider: slider::State,
        size: u16,
        color_sliders: [slider::State; 3],
        color: Color,
    },
    Radio {
        selection: Option<Language>,
    },
    Image {
        width: u16,
        slider: slider::State,
    },

    Scrollable,
    TextInput {
        value: String,
        is_secure: bool,
        state: text_input::State,
    },
    Debugger,
    End,
}

#[derive(Debug, Clone)]
pub enum StepMessage {
    SliderChanged(u8),
    LayoutChanged(u8),
    SpacingChanged(u16),
}