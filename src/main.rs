use std::{
    path::{Path, PathBuf},
    time::Instant,
};

use gpui::{
    AnyElement, App, Application, Bounds, Context, ObjectFit, Render, Window, WindowBounds,
    WindowOptions, canvas, div, fill, img, point, prelude::*, px, rgb, size,
};
use water_simulation::{
    StartupDecision,
    scene::{Scene, Water},
    startup_decision,
};

struct StartupView {
    lines: Vec<String>,
}

impl StartupView {
    fn content(&self) -> gpui::Div {
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

struct SceneView {
    scene: Scene,
    assets: PathBuf,
    last_frame: Instant,
}

impl SceneView {
    fn new(assets: PathBuf) -> Self {
        Self {
            scene: Scene::with_random(random_unit),
            assets,
            last_frame: Instant::now(),
        }
    }
}

impl SceneView {
    fn content(&mut self, window: &mut Window) -> gpui::Div {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;
        self.scene.update(elapsed);
        window.request_animation_frame();

        let water = WaterSurface::from_water(&self.scene.water);
        let sphere_center = self.scene.sphere_center;
        let environment = self.assets.join("zpos.jpg");
        let tiles = self.assets.join("tiles.jpg");
        div()
            .size_full()
            .relative()
            .overflow_hidden()
            .bg(rgb(0x111827))
            .child(
                img(environment)
                    .absolute()
                    .inset_0()
                    .size_full()
                    .object_fit(ObjectFit::Cover)
                    .opacity(0.42),
            )
            .child(
                img(tiles)
                    .absolute()
                    .bottom_0()
                    .left_0()
                    .right_0()
                    .h(px(180.))
                    .object_fit(ObjectFit::Cover)
                    .opacity(0.72),
            )
            .child(
                canvas(
                    move |bounds, _, _| (bounds, water, sphere_center),
                    |bounds, (scene_bounds, water, sphere_center), window, _| {
                        paint_scene(bounds, scene_bounds, &water, sphere_center, window);
                    },
                )
                .absolute()
                .inset_0()
                .size_full(),
            )
    }
}

enum Presentation {
    Startup(StartupView),
    Scene(SceneView),
}

struct WaterApp {
    presentation: Presentation,
}

impl Render for WaterApp {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let content: AnyElement = match &mut self.presentation {
            Presentation::Startup(view) => view.content().into_any_element(),
            Presentation::Scene(view) => view.content(window).into_any_element(),
        };
        content
    }
}

#[derive(Clone)]
struct WaterSurface {
    side: usize,
    heights: Vec<f32>,
}

impl WaterSurface {
    fn from_water(water: &Water) -> Self {
        const SIDE: usize = 48;
        let step = water.size() / SIDE;
        let heights = (0..SIDE)
            .flat_map(|y| {
                (0..SIDE).map(move |x| water.cell(x * step + step / 2, y * step + step / 2).0)
            })
            .collect();
        Self {
            side: SIDE,
            heights,
        }
    }
}

fn paint_scene(
    bounds: Bounds<gpui::Pixels>,
    _scene_bounds: Bounds<gpui::Pixels>,
    water: &WaterSurface,
    sphere_center: [f32; 3],
    window: &mut Window,
) {
    let pool_width = bounds.size.width * 0.72;
    let pool_height = bounds.size.height * 0.52;
    let pool_left = bounds.origin.x + (bounds.size.width - pool_width) * 0.14;
    let pool_top = bounds.origin.y + bounds.size.height * 0.27;
    let pool = Bounds {
        origin: point(pool_left, pool_top),
        size: size(pool_width, pool_height),
    };
    window.paint_quad(fill(pool, rgb(0x163c56)));

    let cell_width = pool_width / water.side as f32;
    let cell_height = pool_height / water.side as f32;
    for y in 0..water.side {
        for x in 0..water.side {
            let height = water.heights[y * water.side + x];
            let brightness = (120.0 + height * 8_000.0).clamp(35.0, 220.0) as u32;
            let blue = (brightness + 30).min(255);
            let color = (20 << 16) | (brightness << 8) | blue;
            window.paint_quad(fill(
                Bounds {
                    origin: point(
                        pool_left + cell_width * x as f32,
                        pool_top + cell_height * y as f32,
                    ),
                    size: size(cell_width + px(0.5), cell_height + px(0.5)),
                },
                rgb(color),
            ));
        }
    }

    let sphere_x = pool_left + (sphere_center[0] + 1.0) * 0.5 * pool_width;
    let sphere_y = pool_top + (sphere_center[2] + 1.0) * 0.5 * pool_height;
    let radius = pool_height * 0.10;
    window.paint_quad(
        fill(
            Bounds {
                origin: point(sphere_x - radius, sphere_y - radius),
                size: size(radius * 2.0, radius * 2.0),
            },
            rgb(0xd9e1e8),
        )
        .corner_radii(radius),
    );
}

fn random_unit() -> f32 {
    use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

    static STATE: AtomicU64 = AtomicU64::new(0x9e37_79b9_7f4a_7c15);
    let previous = STATE.fetch_add(0x9e37_79b9_7f4a_7c15, Relaxed);
    let mixed = (previous ^ (previous >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    let mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    ((mixed ^ (mixed >> 31)) >> 40) as f32 / (1_u32 << 24) as f32
}

fn main() {
    let assets = PathBuf::from("assets");
    let (title, lines, ready) = match startup_decision(Path::new("assets")) {
        StartupDecision::Ready => ("Simulación de agua".to_owned(), Vec::new(), true),
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
            (
                format!("Recursos inválidos: {}", names.join(", ")),
                lines,
                false,
            )
        }
    };

    Application::new().run(move |cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.), px(400.)), cx);
        let result = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            move |window, cx| {
                window.set_window_title(&title);
                if ready {
                    cx.new(|_| WaterApp {
                        presentation: Presentation::Scene(SceneView::new(assets)),
                    })
                } else {
                    cx.new(|_| WaterApp {
                        presentation: Presentation::Startup(StartupView { lines }),
                    })
                }
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
