use conrod;
use rusttype;
use glium;

use app::App;
use glium::backend::glutin_backend::GlutinFacade;
use conrod::{widget, Positionable, Colorable, Widget, Labelable, Borderable, Sizeable};
use conrod::backend::winit::WinitWindow;

use std::cmp;

// Generate the widget identifiers.
widget_ids!(struct Ids {
    menu,
    resume,
    fullscreen,
    music_text,
    music_canvas,
    music_plus,
    music_minus,
    music_slider,
    effect_text,
    effect_canvas,
    effect_plus,
    effect_minus,
    effect_slider,
    donate,
    quit,
});

//"⇱Ξ≡⧉⊞⊟Χ≡↵"

static FONT: &'static [u8] = include_bytes!("../DejaVuSans.ttf");

pub struct Ui {
    image_map: conrod::image::Map<glium::texture::Texture2d>,
    ids: Ids,
    pub ui: conrod::Ui,
    renderer: conrod::backend::glium::Renderer,
    pub menu_state: bool,
}

impl Ui {
    pub fn new(window: &GlutinFacade) -> Self {
        // Ui
        let (w, h) = window.get_window().unwrap().get_inner_size_pixels().unwrap();
        let mut ui = conrod::UiBuilder::new([w as f64, h as f64]).build();

        let ids = Ids::new(ui.widget_id_generator());

        // Add a `Font` to the `Ui`'s `font::Map`.
        ui.fonts.insert(rusttype::FontCollection::from_bytes(FONT).into_font().unwrap());

        // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
        // for drawing to the glium `Surface`.
        let renderer = conrod::backend::glium::Renderer::new(window).unwrap();

        // The image map describing each of our widget->image mappings.
        let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

        Ui {
            image_map: image_map,
            ids: ids,
            ui: ui,
            renderer: renderer,
            menu_state: false,
        }
    }

    pub fn update(&mut self, window: &GlutinFacade, app: &mut App) {
        let (w,h) = window.get_inner_size().unwrap();
        let size = cmp::min(w, h) / 20;
        let border_size = size as f64 /8.;
        let width = size as f64 * 1.5 * 8.;
        let height = size as f64 * 1.5;
        let number_of_line = 6usize;

        if self.menu_state {
            // Instantiate all widgets in the GUI.
            let ui = &mut self.ui.set_widgets();

            for _ in widget::Button::new()
                .label("↵")
                .top_right_with_margin_on(ui.window, size as f64 / 2.)
                .border(size as f64 / 8.)
                .w_h(height, height)
                .label_font_size(size)
                .set(self.ids.menu, ui)
            {
                self.menu_state = false;
            }

            for _ in widget::Button::new()
                .label("resume")
                .down(h as f64/2. - (number_of_line+1) as f64 / 2. * (height + border_size*2.))
                .align_middle_x_of(ui.window)
                .border(border_size)
                .h(height)
                .w(width)
                .label_font_size(size)
                .set(self.ids.resume, ui)
            {
                self.menu_state = false;
            }

            for _ in widget::Button::new()
                .down(border_size)
                .label("toggle fullscreen")
                .border(border_size)
                .h(height)
                .w(width)
                .label_font_size(size)
                .set(self.ids.fullscreen, ui)
            {
                app.toggle_fullscreen();
            }

            widget::Canvas::new()
                .color(conrod::color::WHITE)
                .align_middle_x_of(self.ids.fullscreen)
                .down(border_size)
                .h(height)
                .w(width - height * 2. - border_size * 2.)
                .border(border_size)
                .set(self.ids.music_canvas, ui);

            widget::Slider::new(0.5, 0., 1.)
                .enabled(false)
                .graphics_for(self.ids.music_canvas)
                .color(conrod::color::LIGHT_BLUE)
                .align_middle_x_of(self.ids.fullscreen)
                .align_middle_y_of(self.ids.music_canvas)
                .enabled(false)
                .h(height - border_size * 2.)
                .w(width - height * 2. - border_size * 4.)
                .border(0.)
                .border_color(conrod::color::WHITE)
                .set(self.ids.music_slider, ui);

            for _ in widget::Button::new()
                .label("+")
                .align_middle_y_of(self.ids.music_slider)
                .align_right_of(self.ids.fullscreen)
                .h(height)
                .w(height)
                .border(border_size)
                .label_font_size(size)
                .set(self.ids.music_plus, ui)
            {
                unimplemented!();
            }

            for _ in widget::Button::new()
                .label("−")
                .left(size as f64 * 0.5)
                .align_middle_y_of(self.ids.music_slider)
                .align_left_of(self.ids.fullscreen)
                .h(height)
                .w(height)
                .border(border_size)
                .label_font_size(size)
                .set(self.ids.music_minus, ui)
            {
                unimplemented!();
            }

            widget::Text::new("music")
                .align_middle_y_of(self.ids.music_slider)
                .align_middle_x_of(self.ids.fullscreen)
                .w(width - height * 2. - border_size * 2.)
                .justify(conrod::text::Justify::Center)
                .font_size(size)
                .set(self.ids.music_text, ui);

            widget::Canvas::new()
                .color(conrod::color::WHITE)
                .align_middle_x_of(self.ids.fullscreen)
                .down(border_size*3.)
                .h(height)
                .w(width - height * 2. - border_size * 2.)
                .border(border_size)
                .set(self.ids.effect_canvas, ui);

            widget::Slider::new(0.5, 0., 1.)
                .enabled(false)
                .graphics_for(self.ids.effect_canvas)
                .color(conrod::color::LIGHT_BLUE)
                .align_middle_x_of(self.ids.fullscreen)
                .align_middle_y_of(self.ids.effect_canvas)
                .enabled(false)
                .h(height - border_size * 2.)
                .w(width - height * 2. - border_size * 4.)
                .border(0.)
                .border_color(conrod::color::WHITE)
                .set(self.ids.effect_slider, ui);

            for _ in widget::Button::new()
                .label("+")
                .align_middle_y_of(self.ids.effect_slider)
                .align_right_of(self.ids.fullscreen)
                .h(height)
                .w(height)
                .border(border_size)
                .label_font_size(size)
                .set(self.ids.effect_plus, ui)
            {
                unimplemented!();
            }

            for _ in widget::Button::new()
                .label("−")
                .left(size as f64 * 0.5)
                .align_middle_y_of(self.ids.effect_slider)
                .align_left_of(self.ids.fullscreen)
                .h(height)
                .w(height)
                .border(border_size)
                .label_font_size(size)
                .set(self.ids.effect_minus, ui)
            {
                unimplemented!();
            }

            widget::Text::new("effect")
                .align_middle_y_of(self.ids.effect_slider)
                .align_middle_x_of(self.ids.fullscreen)
                .w(width - height * 2. - border_size * 2.)
                .justify(conrod::text::Justify::Center)
                .font_size(size)
                .set(self.ids.effect_text, ui);

            for _ in widget::Button::new()
                .down(border_size*3.)
                .align_middle_x_of(self.ids.fullscreen)
                .label("donate")
                .border(border_size)
                .h(height)
                .w(width)
                .label_font_size(size)
                .set(self.ids.donate, ui)
            {
                unimplemented!();
            }

            for _ in widget::Button::new()
                .down(border_size)
                .align_middle_x_of(self.ids.fullscreen)
                .label("quit")
                .border(border_size)
                .h(height)
                .w(width)
                .label_font_size(size)
                .set(self.ids.quit, ui)
            {
                app.quit();
            }
        } else {
            // Instantiate all widgets in the GUI.
            let ui = &mut self.ui.set_widgets();

            for _ in widget::Button::new()
                .label("≡")
                .top_right_with_margin_on(ui.window, size as f64 / 2.)
                .border(size as f64 / 8.)
                .w_h(height, height)
                .label_font_size(size)
                .set(self.ids.menu, ui)
            {
                self.menu_state = true;
            }

            for _ in widget::Button::new()
                .label("⇱")
                .left(border_size*2.)
                .border(size as f64 / 8.)
                .w_h(height, height)
                .label_font_size(size)
                .set(self.ids.fullscreen, ui)
            {
                app.toggle_fullscreen();
            }
        }

        // Render UI
        self.renderer.fill(window, self.ui.draw(), &self.image_map);
    }

    pub fn draw(&mut self, window: &GlutinFacade, target: &mut glium::Frame) {
        self.renderer.draw(window, target, &self.image_map).unwrap();
    }
}
