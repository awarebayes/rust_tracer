use iced::{
    button, scrollable, slider, text_input, Button, checkbox,
    Color, Column, Container, Element, HorizontalAlignment,
    Image, Length, Radio, Row, Sandbox, Scrollable, Settings,
    Slider, Space, Text, TextInput, 
};

pub struct Tour {
    steps: Steps,
}

struct S