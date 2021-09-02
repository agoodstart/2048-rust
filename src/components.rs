use bevy::prelude::{
    Commands, 
    Bundle, 
    Reflect, 
    Val, 
    Size, 
    Display, 
    Handle, 
    ColorMaterial,
    ResMut,
    Assets};

#[derive (Bundle, Reflect)]
pub struct Wrapper {
    size: Size<Val>,
    
}

#[derive (Bundle, Reflect)]
pub struct FlexWrapper {
    pub size: Size<Val>,
    pub display: Display,
    // pub background_color: Handle<ColorMaterial>,
}


impl Default for FlexWrapper { 
    fn default() -> Self {
        FlexWrapper { 
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            display: Display::Flex,
            // background_color: set_bg_color(materials).add(asset)
        } 
    }
}

fn set_bg_color(mut materials: ResMut<Assets<ColorMaterial>>) -> ResMut<'_, Assets<ColorMaterial>> {
    materials
}   