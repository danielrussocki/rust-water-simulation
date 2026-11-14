use std::path::Path;

use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size,
};
use water_simulation::{StartupDecision, startup_decision};

struct StartupView {
    lines: Vec<String>,
}

impl Render for StartupView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut content = div()
            .size_full()
            .bg(rgb(0x171717))
            .text_color(rgb(0xffffff))
            .p_6()
            .flex()
            .flex_col()
            .gap_2();
        for line in &self.lines {
            content = content.child(line.clone());
        }
        content
    }
}

fn main() {
    let (title, lines) = match startup_decision(Path::new("assets")) {
        StartupDecision::Ready => (
            "Recursos válidos".to_owned(),
            vec!["Recursos válidos. La simulación aún no está disponible.".to_owned()],
        ),
        StartupDecision::Blocked(errors) => {
            let names = errors.iter().map(|error| error.name).collect::<Vec<_>>();
            let mut lines =
                vec!["No se puede iniciar la simulación. Recursos ausentes o dañados:".to_owned()];
            lines.extend(
                errors
                    .iter()
                    .map(|error| format!("{}: {}", error.name, error.cause)),
            );
            lines.push("Corrija los recursos y vuelva a abrir la aplicación.".to_owned());
            (format!("Recursos inválidos: {}", names.join(", ")), lines)
        }
    };

    Application::new().run(move |cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.), px(400.)), cx);
        let result = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                window.set_window_title(&title);
                cx.new(|_| StartupView { lines })
            },
        );
        if let Err(error) = result {
            eprintln!("No se pudo abrir la ventana: {error:#}");
            cx.quit();
            return;
        }
        cx.activate(true);
    });
}
