
#[cfg(test)]
pub mod scene_test{
    use map_app::engine::structure::scenario::{Scenario};
    use termion::color;
    #[test]
    #[should_panic]
    pub fn neg_scene(){
        let _s: Scenario = Scenario::new(0,100,color::Rgb(0,0,0));
    }
}


#[cfg(test)]
pub mod screen_test{
    use map_app::engine::window::screen::{Screen,Panel};
    use termion::{
        terminal_size,
        screen::AlternateScreen,
        raw::IntoRawMode
    };
    use std::io::{
        stdout
    };
    #[test]
    pub fn windows_size_equal(){
        let d = match terminal_size(){
            Ok((w,h)) => (w,h),
            Err(e) => panic!("Error al crear Screen: {}",e)
        };
        let s_out = AlternateScreen::from(stdout().into_raw_mode().unwrap());
        let s = Screen::new(s_out, 20);
        let ((me_w,me_h),(ma_w,_ma_h)) = (s.get_sizes(Panel::Main),s.get_sizes(Panel::Menu));
        assert_eq!(d,(me_w+ma_w+3,me_h+2))
    }
}