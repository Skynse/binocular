use eframe;
use gstreamer::prelude::*;
struct App;

fn test() {
    // Initialize gstreamerreamer
    gstreamer::init().unwrap();

    // Build the pipeline
    let uri = "https://gstreamerreamer.freedesktop.org/data/media/sintel_trailer-480p.webm";
    let pipeline = gstreamer::parse_launch(&format!("playbin uri={}", uri)).unwrap();

    // Start playing
    pipeline
        .set_state(gstreamer::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    // Wait until error or EOS
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        use gstreamer::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        }
    }

    // Shutdown pipeline
    pipeline
        .set_state(gstreamer::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}
fn main() -> Result<(), eframe::Error> {
    test();
    let options = eframe::NativeOptions::default();

    eframe::run_native("Binocular", options, Box::new(|cc| Box::new(App)))
}
