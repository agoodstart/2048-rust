use bevy::prelude::{Commands, Bundle, Reflect, Val, Size, Display};
use std::default;

#[derive (Bundle, Reflect)]
pub struct Wrapper {
    size: Size<Val>,
    
}

#[derive (Bundle, Reflect)]
pub struct FlexWrapper {
    pub size: bevy::math::Size<Val>,
    pub display: bevy::ui::Display,
}


impl Default for FlexWrapper { 
    fn default() -> Self {
        FlexWrapper { 
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            display: bevy::ui::Display::Flex,
        } 
    }
}