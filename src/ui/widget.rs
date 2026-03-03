use gtk4::prelude::*;
use gtk4::{
    Box, Button, CssProvider, DrawingArea, Label, Orientation, Scale, Stack,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use std::cell::RefCell;
use std::f64::consts::{FRAC_PI_2, PI};
use std::rc::Rc;

use crate::timer::pomodoro::PomodoroTimer;

#[derive(Clone)]
pub struct TimerWidget {
    pub container: Box,
    pub stack: Stack,
    pub drawing_area: DrawingArea,
    pub time_label: Label,
    pub play_button: Button,
    pub plus_button: Button,
    pub minus_button: Button,
    pub controls_box: Box,
    pub top_bar: Box,
    pub hide_button: Button,
    pub config_button: Button,
    pub brown_noise: Button,
    pub config_panel: Box,
    pub video_file_label: Label,
    pub video_browse_button: Button,
    pub video_volume_scale: Scale,
    pub video_volume_label: Label,
    pub brown_noise_volume_scale: Scale,
    pub brown_noise_volume_label: Label,
    pub config_save_button: Button,
    pub config_back_button: Button,
}

impl TimerWidget {
    pub fn new(timer_logic: Rc<RefCell<PomodoroTimer>>) -> Self {
        let provider = CssProvider::new();
        provider.load_from_data("
            window { background: none; background-color: transparent; }

            .timer-container {
                background-color: rgba(20, 20, 20, 0.85);
                border-radius: 25px;
                padding: 15px;
                transition: background-color 0.5s ease;
            }

            .focus-mode {
                background-color: rgba(0, 0, 0, 0.05);
                padding: 5px;
            }

            .timer-label { font-size: 24px; font-weight: bold; color: white; }

            .hide-button { background: none; border: none; color: rgba(255, 255, 255, 0.3); }

            .config-button { font-size: 30px; background: none; border: none; color: rgba(255, 255, 255, 0.3); }

            brown-noise-button-on { font-size: 10px; font-weight: bold; color: rgba(220, 85, 11, 0.5); border: none; }

            brown-noise-button-off { font-size: 10px; font-weight: bold; color: rgba(255, 255, 255, 0.3); border: none; }

            .config-panel { padding: 10px; }

            .config-title { font-size: 18px; font-weight: bold; color: white; }

            .config-section-label { font-size: 12px; color: rgba(255, 255, 255, 0.7); margin-top: 8px; }

            .config-file-label {
                font-size: 11px;
                color: white;
                padding: 4px 8px;
                background-color: rgba(255, 255, 255, 0.1);
                border-radius: 4px;
            }

            .config-browse-btn { font-size: 11px; padding: 4px 8px; }

            .config-save-btn { margin-top: 10px; padding: 6px 16px; font-weight: bold; }

            .config-back-btn { background: none; border: none; color: rgba(255, 255, 255, 0.3); font-size: 18px; }

            .volume-value { font-size: 11px; color: rgba(255, 255, 255, 0.6); min-width: 35px; }

            .config-scale trough { background-color: rgba(255, 255, 255, 0.15); border-radius: 4px; min-height: 6px; }

            .config-scale trough highlight { background-color: rgb(255, 128, 0); border-radius: 4px; min-height: 6px; }

            .config-scale slider { background-color: rgb(255, 128, 0); border-radius: 50%; min-width: 14px; min-height: 14px; margin: -4px; }
            ");

        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().expect("Could not connect to display"),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // container principal
        let container = Box::new(Orientation::Vertical, 5);
        container.add_css_class("timer-container");

        // barra superior
        let top_bar = Box::new(Orientation::Horizontal, 0);

        let config_button: Button = Button::builder()
            .label("⚙")
            .css_classes(["config-button"])
            .halign(gtk4::Align::Start)
            .build();

        let spacer = gtk4::Box::new(Orientation::Horizontal, 0);
        spacer.set_hexpand(true);

        let hide_button = Button::builder()
            .label("×")
            .css_classes(["hide-button"])
            .halign(gtk4::Align::End)
            .build();

        let brown_noise: Button = Button::builder()
            .label("Brown Noise: Off")
            .halign(gtk4::Align::Center)
            .build();

        top_bar.append(&config_button);
        top_bar.append(&spacer);
        top_bar.append(&brown_noise);
        top_bar.append(&hide_button);
        container.append(&top_bar);

        // desenho do círculo
        let drawing_area = DrawingArea::builder()
            .content_width(180)
            .content_height(180)
            .build();

        let time_label = Label::builder()
            .label("25:00")
            .css_classes(["timer-label"])
            .build();

        let play_button = Button::with_label("Pomodoro!");
        let plus_button = Button::with_label("+5");
        let minus_button = Button::with_label("-5");

        let controls_box = Box::new(Orientation::Horizontal, 8);
        controls_box.set_halign(gtk4::Align::Center);
        controls_box.append(&minus_button);
        controls_box.append(&play_button);
        controls_box.append(&plus_button);

        // página do timer dentro do stack
        let timer_page = Box::new(Orientation::Vertical, 5);
        timer_page.append(&drawing_area);
        timer_page.append(&controls_box);

        // painel de configuração
        let config_panel = Box::new(Orientation::Vertical, 6);
        config_panel.add_css_class("config-panel");
        config_panel.set_vexpand(true);

        // cabeçalho do config
        let config_header = Box::new(Orientation::Horizontal, 4);
        let config_back_button = Button::builder()
            .label("←")
            .css_classes(["config-back-btn"])
            .halign(gtk4::Align::Start)
            .build();
        let config_title = Label::builder()
            .label("Settings")
            .css_classes(["config-title"])
            .hexpand(true)
            .halign(gtk4::Align::Center)
            .build();
        let header_spacer = Label::new(Some(""));
        header_spacer.set_width_request(30);
        config_header.append(&config_back_button);
        config_header.append(&config_title);
        config_header.append(&header_spacer);
        config_panel.append(&config_header);

        // volume do vídeo
        let video_vol_section = Label::builder()
            .label("Video Volume")
            .css_classes(["config-section-label"])
            .halign(gtk4::Align::Start)
            .build();
        config_panel.append(&video_vol_section);

        let video_vol_row = Box::new(Orientation::Horizontal, 6);
        let video_volume_scale = Scale::with_range(Orientation::Horizontal, 0.0, 100.0, 1.0);
        video_volume_scale.set_value(100.0);
        video_volume_scale.set_hexpand(true);
        video_volume_scale.add_css_class("config-scale");
        let video_volume_label = Label::builder()
            .label("100%")
            .css_classes(["volume-value"])
            .build();
        video_vol_row.append(&video_volume_scale);
        video_vol_row.append(&video_volume_label);
        config_panel.append(&video_vol_row);

        // volume do brown noise
        let bn_vol_section = Label::builder()
            .label("Brown Noise Volume")
            .css_classes(["config-section-label"])
            .halign(gtk4::Align::Start)
            .build();
        config_panel.append(&bn_vol_section);

        let bn_vol_row = Box::new(Orientation::Horizontal, 6);
        let brown_noise_volume_scale = Scale::with_range(Orientation::Horizontal, 0.0, 100.0, 1.0);
        brown_noise_volume_scale.set_value(60.0);
        brown_noise_volume_scale.set_hexpand(true);
        brown_noise_volume_scale.add_css_class("config-scale");
        let brown_noise_volume_label = Label::builder()
            .label("60%")
            .css_classes(["volume-value"])
            .build();
        bn_vol_row.append(&brown_noise_volume_scale);
        bn_vol_row.append(&brown_noise_volume_label);
        config_panel.append(&bn_vol_row);

        // seleção de vídeo relaxante
        let video_section_label = Label::builder()
            .label("Select a Relaxing Video")
            .css_classes(["config-section-label"])
            .halign(gtk4::Align::Center)
            .build();
        config_panel.append(&video_section_label);

        let video_file_row = Box::new(Orientation::Horizontal, 6);
        let video_file_label = Label::builder()
            .label("relax.mp4")
            .css_classes(["config-file-label"])
            .hexpand(true)
            .halign(gtk4::Align::Start)
            .ellipsize(gtk4::pango::EllipsizeMode::Start)
            .max_width_chars(16)
            .build();
        let video_browse_button = Button::builder()
            .label("Browse")
            .css_classes(["config-browse-btn"])
            .build();
        video_file_row.append(&video_file_label);
        video_file_row.append(&video_browse_button);
        config_panel.append(&video_file_row);

        // botão salvar
        let config_save_button = Button::builder()
            .label("Save")
            .css_classes(["config-save-btn"])
            .halign(gtk4::Align::Center)
            .build();
        config_panel.append(&config_save_button);

        // stack para alternar entre timer e config
        let stack = Stack::new();
        stack.set_hhomogeneous(false);
        stack.set_vhomogeneous(false);
        stack.add_named(&timer_page, Some("timer"));
        stack.add_named(&config_panel, Some("config"));
        stack.set_visible_child_name("timer");
        container.append(&time_label);
        container.append(&stack);

        // atualizar labels de volume ao mover os sliders
        {
            let label = video_volume_label.clone();
            video_volume_scale.connect_value_changed(move |scale| {
                label.set_text(&format!("{}%", scale.value() as u32));
            });
        }
        {
            let label = brown_noise_volume_label.clone();
            brown_noise_volume_scale.connect_value_changed(move |scale| {
                label.set_text(&format!("{}%", scale.value() as u32));
            });
        }

        // desenho do círculo
        let timer_logic_clone = timer_logic.clone();
        drawing_area.set_draw_func(move |_area, cr, width, height| {
            let timer = timer_logic_clone.borrow();

            let total_seconds = timer.initial_work_seconds as f64;
            let percentage = if total_seconds > 0.0 {
                timer.current_time as f64 / total_seconds
            } else {
                0.0
            };

            let center_x = (width / 2) as f64;
            let center_y = (height / 2) as f64;
            let radius = center_x.min(center_y) - 10.0;

            cr.set_operator(gtk4::cairo::Operator::Clear);
            cr.paint().expect("Clear failed");
            cr.set_operator(gtk4::cairo::Operator::Over);

            cr.set_source_rgba(0.2, 0.2, 0.2, 0.4);
            cr.set_line_width(8.0);
            cr.arc(center_x, center_y, radius, 0.0, 2.0 * PI);
            let _ = cr.stroke();

            let start_angle = -FRAC_PI_2;
            let end_angle = start_angle + (percentage * 2.0 * PI);

            cr.set_source_rgb(1.0, 0.5, 0.0);
            cr.set_line_width(10.0);
            cr.set_line_cap(gtk4::cairo::LineCap::Round);
            cr.arc(center_x, center_y, radius, start_angle, end_angle);
            let _ = cr.stroke();
        });

        Self {
            container,
            stack,
            drawing_area,
            time_label,
            play_button,
            plus_button,
            minus_button,
            controls_box,
            top_bar,
            hide_button,
            config_button,
            brown_noise,
            config_panel,
            video_file_label,
            video_browse_button,
            video_volume_scale,
            video_volume_label,
            brown_noise_volume_scale,
            brown_noise_volume_label,
            config_save_button,
            config_back_button,
        }
    }
}
