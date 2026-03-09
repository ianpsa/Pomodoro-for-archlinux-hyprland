use gio::File;
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Button, Overlay, Video};

pub struct VideoOverlay {
    pub window: ApplicationWindow,
    pub video: Video,
    pub close_btn: Button,
}

impl VideoOverlay {
    pub fn new(app: &gtk4::Application, video_path: &str) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Break Time")
            .fullscreened(true)
            .modal(true)
            .build();

        let file = File::for_path(video_path);
        let video = Video::builder()
            .file(&file)
            .autoplay(true)
            .loop_(true)
            .build();

        let overlay = Overlay::builder().child(&video).build();

        let close_btn = Button::builder()
            .label("Voltar ao trabalho")
            .halign(gtk4::Align::Center)
            .valign(gtk4::Align::End)
            .margin_bottom(40)
            .css_classes(["overlay-button"])
            .build();

        overlay.add_overlay(&close_btn);

        window.set_child(Some(&overlay));

        Self {
            window,
            video,
            close_btn,
        }
    }
}
