mod audio;
mod config;
mod timer;
mod ui;
mod utils;

use config::settings::UserConfig;
use gtk4::prelude::*;
use gtk4::{glib, Application};
use std::sync::atomic::{AtomicBool, Ordering};

static IS_PLAYING: AtomicBool = AtomicBool::new(false);

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.ianpsa.pomodoromind")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &gtk4::Application) {
    use gtk4::{glib, ApplicationWindow, GestureClick};
    use gtk4_layer_shell::{Edge, Layer, LayerShell};

    let settings = std::rc::Rc::new(std::cell::RefCell::new(UserConfig::load()));
    let initial_mins = settings.borrow().initial_minutes;

    let timer_logic = std::rc::Rc::new(std::cell::RefCell::new(
        timer::pomodoro::PomodoroTimer::new(initial_mins),
    ));
    let timer_ui = ui::widget::TimerWidget::new(timer_logic.clone());

    // caminho do vídeo selecionado (atualizado pelo file chooser)
    let selected_video_path = std::rc::Rc::new(std::cell::RefCell::new(
        settings.borrow().video_path.clone(),
    ));

    let overlay = ui::video_overlay::VideoOverlay::new(app, &settings.borrow().video_path);

    // aplicar valores iniciais do config na UI
    {
        let cfg = settings.borrow();

        timer_ui
            .video_volume_scale
            .set_value(cfg.video_volume * 100.0);
        timer_ui
            .video_volume_label
            .set_text(&format!("{}%", (cfg.video_volume * 100.0) as u32));
        timer_ui
            .brown_noise_volume_scale
            .set_value(cfg.brown_noise_volume * 100.0);
        timer_ui
            .brown_noise_volume_label
            .set_text(&format!("{}%", (cfg.brown_noise_volume * 100.0) as u32));

        let file_name = std::path::Path::new(&cfg.video_path)
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_else(|| cfg.video_path.clone());
        timer_ui.video_file_label.set_text(&file_name);

        audio::brown_noise::set_volume(cfg.brown_noise_volume);
    }

    let rest_remaining = std::rc::Rc::new(std::cell::RefCell::new(0u32));

    // loop principal do timer (1 segundo)
    glib::timeout_add_local(std::time::Duration::from_secs(1), {
        let timer_logic = timer_logic.clone();
        let timer_ui = timer_ui.clone();
        let overlay_win = overlay.window.clone();
        let video = overlay.video.clone();
        let rest_remaining = rest_remaining.clone();
        let settings = settings.clone();

        move || {
            let mut timer = timer_logic.borrow_mut();
            let mut rest_time = rest_remaining.borrow_mut();

            if overlay_win.is_visible() && *rest_time > 0 {
                *rest_time -= 1;
                if *rest_time == 0 {
                    overlay_win.set_visible(false);
                }
                return glib::ControlFlow::Continue;
            }

            timer_ui
                .time_label
                .set_text(&utils::format_time(timer.current_time));
            timer_ui.drawing_area.queue_draw();

            if matches!(timer.state, timer::pomodoro::TimerState::Play) {
                timer.tick();

                if matches!(timer.state, timer::pomodoro::TimerState::End) {
                    *rest_time = timer.initial_work_seconds / 5;

                    // resetar timer imediatamente ao terminar
                    timer.reset_to_base();
                    timer_ui.play_button.set_label("Pomodoro!");
                    timer_ui
                        .time_label
                        .set_text(&utils::format_time(timer.current_time));
                    timer_ui.drawing_area.queue_draw();

                    if let Some(stream) = video.media_stream() {
                        let headphones = audio::detector::AudioManager::are_headphones_connected();
                        stream.set_muted(!headphones);
                        stream.set_volume(settings.borrow().video_volume);
                    }
                    overlay_win.present();
                }
            }
            glib::ControlFlow::Continue
        }
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .decorated(false)
        .default_width(200)
        .default_height(250)
        .child(&timer_ui.container)
        .build();

    // botão config: abrir painel de configuração
    {
        let timer_ui_cfg = timer_ui.clone();
        timer_ui.config_button.connect_clicked(move |_| {
            timer_ui_cfg.top_bar.set_visible(false);
            timer_ui_cfg.time_label.set_visible(false);
            timer_ui_cfg.stack.set_visible_child_name("config");
        });
    }

    // botão voltar: fechar config sem salvar
    {
        let timer_ui_back = timer_ui.clone();
        timer_ui.config_back_button.connect_clicked(move |_| {
            timer_ui_back.stack.set_visible_child_name("timer");
            timer_ui_back.top_bar.set_visible(true);
            timer_ui_back.time_label.set_visible(true);
        });
    }

    // botão browse: abrir diálogo de seleção de arquivo
    {
        let video_file_label = timer_ui.video_file_label.clone();
        let selected_path = selected_video_path.clone();
        let window_browse = window.clone();
        timer_ui.video_browse_button.connect_clicked(move |_| {
            let dialog = gtk4::FileDialog::builder()
                .title("Select Video File")
                .modal(true)
                .build();

            let filters = gio::ListStore::new::<gtk4::FileFilter>();
            let video_filter = gtk4::FileFilter::new();
            video_filter.set_name(Some("Video Files"));
            video_filter.add_mime_type("video/*");
            filters.append(&video_filter);

            let all_filter = gtk4::FileFilter::new();
            all_filter.set_name(Some("All Files"));
            all_filter.add_pattern("*");
            filters.append(&all_filter);

            dialog.set_filters(Some(&filters));

            let label = video_file_label.clone();
            let path = selected_path.clone();
            dialog.open(
                Some(&window_browse),
                gio::Cancellable::NONE,
                move |result| {
                    if let Ok(file) = result {
                        if let Some(file_path) = file.path() {
                            let path_str = file_path.to_string_lossy().to_string();
                            let file_name = file_path
                                .file_name()
                                .map(|f| f.to_string_lossy().to_string())
                                .unwrap_or_else(|| path_str.clone());
                            label.set_text(&file_name);
                            *path.borrow_mut() = path_str;
                        }
                    }
                },
            );
        });
    }

    // botão salvar: gravar config e voltar ao timer
    {
        let timer_ui_save = timer_ui.clone();
        let settings_save = settings.clone();
        let selected_path_save = selected_video_path.clone();
        let overlay_video = overlay.video.clone();
        timer_ui.config_save_button.connect_clicked(move |_| {
            let video_volume = timer_ui_save.video_volume_scale.value() / 100.0;
            let brown_noise_volume = timer_ui_save.brown_noise_volume_scale.value() / 100.0;
            let video_path = selected_path_save.borrow().clone();

            {
                let mut cfg = settings_save.borrow_mut();
                cfg.video_volume = video_volume;
                cfg.brown_noise_volume = brown_noise_volume;
                cfg.video_path = video_path.clone();
                if let Err(e) = cfg.save() {
                    eprintln!("Failed to save config: {}", e);
                }
            }

            audio::brown_noise::set_volume(brown_noise_volume);

            let file = gio::File::for_path(&video_path);
            overlay_video.set_file(Some(&file));
            if let Some(stream) = overlay_video.media_stream() {
                stream.set_volume(video_volume);
            }

            timer_ui_save.stack.set_visible_child_name("timer");
            timer_ui_save.top_bar.set_visible(true);
            timer_ui_save.time_label.set_visible(true);
        });
    }

    // botão brown noise
    let brown_button = timer_ui.brown_noise.clone();
    let settings_bn = settings.clone();
    timer_ui.brown_noise.connect_clicked(move |_| {
        let playing = IS_PLAYING.fetch_xor(true, Ordering::SeqCst);
        audio::brown_noise::toggle();

        if playing {
            brown_button.set_label("Brown Noise: Off");
            brown_button.add_css_class("brown-noise-button-on");
        } else {
            brown_button.set_label("Brown Noise: On");
            brown_button.add_css_class("brown-noise-button-off");
            audio::brown_noise::set_volume(settings_bn.borrow().brown_noise_volume);
        }
    });

    // botão esconder (modo foco)
    let timer_ui_focus = timer_ui.clone();
    let window_focus = window.clone();
    timer_ui.hide_button.connect_clicked(move |_| {
        timer_ui_focus.stack.set_visible(false);
        timer_ui_focus.top_bar.set_visible(false);
        timer_ui_focus.container.add_css_class("focus-mode");
        window_focus.set_default_width(0);
        window_focus.set_default_height(0);
    });

    // restaurar UI ao clicar no label do tempo
    let gesture_restore = GestureClick::new();
    let timer_ui_restore = timer_ui.clone();
    let window_restore = window.clone();
    gesture_restore.connect_pressed(move |gesture, _, _, _| {
        let modifiers = gesture.current_event().map(|e| e.modifier_state());
        if let Some(state) = modifiers {
            if !state.contains(gtk4::gdk::ModifierType::SUPER_MASK) {
                timer_ui_restore.stack.set_visible(true);
                timer_ui_restore.top_bar.set_visible(true);
                timer_ui_restore.container.remove_css_class("focus-mode");
                window_restore.set_default_width(200);
                window_restore.set_default_height(250);
            }
        }
    });
    timer_ui.time_label.add_controller(gesture_restore);

    // layer shell (wayland overlay)
    if gtk4_layer_shell::is_supported() {
        window.init_layer_shell();
        window.set_layer(Layer::Overlay);
        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Right, true);
        window.set_margin(Edge::Top, 10);
        window.set_margin(Edge::Right, 20);
    }

    // mover janela entre os 4 cantos
    let move_gesture = GestureClick::new();
    let window_weak = window.downgrade();
    let current_pos = std::rc::Rc::new(std::cell::Cell::new(0));
    move_gesture.connect_pressed(move |_, _, _, _| {
        if let Some(win) = window_weak.upgrade() {
            let next = (current_pos.get() + 1) % 4;
            current_pos.set(next);

            win.set_anchor(Edge::Top, false);
            win.set_anchor(Edge::Bottom, false);
            win.set_anchor(Edge::Left, false);
            win.set_anchor(Edge::Right, false);

            let margin_y = 10;
            let margin_x = 20;

            match next {
                0 => {
                    // superior direito
                    win.set_anchor(Edge::Top, true);
                    win.set_anchor(Edge::Right, true);
                    win.set_margin(Edge::Top, margin_y);
                    win.set_margin(Edge::Right, margin_x);
                }
                1 => {
                    // inferior direito
                    win.set_anchor(Edge::Bottom, true);
                    win.set_anchor(Edge::Right, true);
                    win.set_margin(Edge::Bottom, margin_y);
                    win.set_margin(Edge::Right, margin_x);
                }
                2 => {
                    // inferior esquerdo
                    win.set_anchor(Edge::Bottom, true);
                    win.set_anchor(Edge::Left, true);
                    win.set_margin(Edge::Bottom, margin_y);
                    win.set_margin(Edge::Left, margin_x);
                }
                3 => {
                    // superior esquerdo
                    win.set_anchor(Edge::Top, true);
                    win.set_anchor(Edge::Left, true);
                    win.set_margin(Edge::Top, margin_y);
                    win.set_margin(Edge::Left, margin_x);
                }
                _ => {}
            }
        }
    });
    timer_ui.drawing_area.add_controller(move_gesture);

    // botões play/pause
    timer_ui.play_button.connect_clicked({
        let timer_logic = timer_logic.clone();
        let play_btn = timer_ui.play_button.clone();
        move |_| {
            let mut timer = timer_logic.borrow_mut();
            match timer.state {
                timer::pomodoro::TimerState::Play => {
                    timer.state = timer::pomodoro::TimerState::Pause;
                    play_btn.set_label("Pomodoro!");
                }
                _ => {
                    timer.state = timer::pomodoro::TimerState::Play;
                    play_btn.set_label("Pausar");
                }
            }
        }
    });

    // botões +5 / -5
    let adjust_btns = [(&timer_ui.plus_button, 300), (&timer_ui.minus_button, -300)];
    for (btn, offset) in adjust_btns {
        btn.connect_clicked({
            let timer_logic = timer_logic.clone();
            let timer_ui_clone = timer_ui.clone();
            move |_| {
                let mut timer = timer_logic.borrow_mut();
                if !matches!(timer.state, timer::pomodoro::TimerState::Play) {
                    if timer.current_time != timer.initial_work_seconds {
                        timer.reset_to_base();
                    }
                    timer.adjust_time(offset);
                    timer_ui_clone
                        .time_label
                        .set_text(&utils::format_time(timer.current_time));
                    timer_ui_clone.drawing_area.queue_draw();
                }
            }
        });
    }

    window.present();
}
