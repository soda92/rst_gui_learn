#![windows_subsystem = "windows"]
use glow::HasContext;
use imgui::{Context, WindowFlags, sys, Ui, Condition, ColorPreview, ColorEditInputMode};
use imgui_glow_renderer::AutoRenderer;
use sdl2_01::SdlPlatform;
use sdl2::{
    event::Event,
    video::{GLProfile, Window},
};

// Create a new glow context.
fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

fn main() {
    /* initialize SDL and its video subsystem */
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    /* hint SDL to initialize an OpenGL 3.3 core profile context */
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(GLProfile::Core);

    /* create a new window, be sure to call opengl method on the builder when using glow! */
    let window = video_subsystem
        .window("Hello imgui-rs!", 600, 400)
        .allow_highdpi()
        .opengl()
        .position_centered()
        .resizable()
        .maximized()
        .build()
        .unwrap();

    /* create a new OpenGL context and make it current */
    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();

    /* enable vsync to cap framerate */
    window.subsystem().gl_set_swap_interval(1).unwrap();

    /* create new glow and imgui contexts */
    let gl = glow_context(&window);

    /* create context */
    let mut imgui = Context::create();

    /* disable creation of files on disc */
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    /* setup platform and renderer, and fonts to imgui */
    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    /* create platform and renderer */
    let mut platform = SdlPlatform::init(&mut imgui);
    let mut renderer = AutoRenderer::initialize(gl, &mut imgui).unwrap();

    /* start main loop */
    let mut event_pump = sdl.event_pump().unwrap();

    let mut run = true;
    let mut state = State::default();

    'main: loop {
        for event in event_pump.poll_iter() {
            /* pass all events to imgui platfrom */
            platform.handle_event(&mut imgui, &event);

            if let Event::Quit { .. } = event {
                break 'main;
            }
        }

        /* call prepare_frame before calling imgui.new_frame() */
        platform.prepare_frame(&mut imgui, &window, &event_pump);

        let ui = imgui.new_frame();

        let viewport = unsafe { *sys::igGetMainViewport() };
        let pos = viewport.Pos;
        let size = viewport.Size;

        /* create imgui UI here */
        ui.show_demo_window(&mut true);

        example_selector(&mut run, ui, &mut state);
        match state.example {
            1 => example_1(ui, &mut state),
            2 => example_2(ui),
            3 => example_3(ui),
            _ => ()
        }

        ui.window("Your first window!")
        .flags(WindowFlags::NO_DECORATION | WindowFlags::NO_SAVED_SETTINGS | WindowFlags::NO_TITLE_BAR)
        .size([size.x, size.y], imgui::Condition::FirstUseEver)
        .position([pos.x, pos.y], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.text("Hello world!");
        });

        /* render */
        let draw_data = imgui.render();

        unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
        renderer.render(draw_data).unwrap();

        window.gl_swap_window();
    }
}


fn example_selector(run: &mut bool, ui: &mut Ui, state: &mut State) {
    let w = ui
        .window("Color button examples")
        .opened(run)
        .position([20.0, 20.0], Condition::Appearing)
        .size([700.0, 100.0], Condition::Appearing)
        .resizable(false);
    w.build(|| {
        let ex1 = ui.radio_button("Example 1: Basics", &mut state.example, 1);
        let ex2 = ui.radio_button("Example 2: Alpha component", &mut state.example, 2);
        let ex3 = ui.radio_button("Example 3: Input format", &mut state.example, 3);
        if ex1 || ex2 || ex3 {
            state.reset();
        }
    });
}

fn example_1(ui: &Ui, state: &mut State) {
    let w = ui
        .window("Example 1: Basics")
        .size([700.0, 300.0], Condition::Appearing)
        .position([20.0, 140.0], Condition::Appearing);
    w.build(|| {
        ui.text_wrapped(
            "Color button is a widget that displays a color value as a clickable rectangle. \
             It also supports a tooltip with detailed information about the color value. \
             Try hovering over and clicking these buttons!",
        );
        ui.text(state.notify_text);

        ui.text("This button is black:");
        if ui.color_button("Black color", [0.0, 0.0, 0.0, 1.0]) {
            state.notify_text = "*** Black button was clicked";
        }

        ui.text("This button is red:");
        if ui.color_button("Red color", [1.0, 0.0, 0.0, 1.0]) {
            state.notify_text = "*** Red button was clicked";
        }

        ui.text("This button is BIG because it has a custom size:");
        if ui
            .color_button_config("Green color", [0.0, 1.0, 0.0, 1.0])
            .size([100.0, 50.0])
            .build()
        {
            state.notify_text = "*** BIG button was clicked";
        }

        ui.text("This button doesn't use the tooltip at all:");
        if ui
            .color_button_config("No tooltip", [0.0, 0.0, 1.0, 1.0])
            .tooltip(false)
            .build()
        {
            state.notify_text = "*** No tooltip button was clicked";
        }
    });
}

fn example_2(ui: &Ui) {
    let w = ui
        .window("Example 2: Alpha component")
        .size([700.0, 320.0], Condition::Appearing)
        .position([20.0, 140.0], Condition::Appearing);
    w.build(|| {
        ui.text_wrapped(
            "The displayed color is passed to the button as four float values between \
             0.0 - 1.0 (RGBA). If you don't care about the alpha component, it can be \
             disabled and it won't show up in the tooltip",
        );

        ui.text("This button ignores the alpha component:");
        ui.color_button_config("Red color", [1.0, 0.0, 0.0, 0.5])
            .alpha(false)
            .build();

        ui.spacing();
        ui.spacing();
        ui.spacing();
        ui.text_wrapped(
            "If you *do* care about the alpha component, you can choose how it's \
             displayed in the button and the tooltip",
        );

        ui.separator();
        ui.text_wrapped("ColorPreview::Opaque (default) doesn't show the alpha component at all");
        ui.color_button_config("Red + ColorPreview::Opaque", [1.0, 0.0, 0.0, 0.5])
            .preview(ColorPreview::Opaque)
            .build();

        ui.separator();
        ui.text_wrapped(
            "ColorPreview::HalfAlpha divides the color area into two halves and uses a \
             checkerboard pattern in one half to illustrate the alpha component",
        );
        ui.color_button_config("Red + ColorPreview::HalfAlpha", [1.0, 0.0, 0.0, 0.5])
            .preview(ColorPreview::HalfAlpha)
            .build();

        ui.separator();
        ui.text_wrapped(
            "ColorPreview::Alpha uses a checkerboard pattern in the entire color area to \
             illustrate the alpha component",
        );
        ui.color_button_config("Red + ColorPreview::Alpha", [1.0, 0.0, 0.0, 0.5])
            .preview(ColorPreview::Alpha)
            .build();
    });
}

fn example_3(ui: &Ui) {
    let w = ui
        .window("Example 3: Input format")
        .size([700.0, 320.0], Condition::Appearing)
        .position([20.0, 140.0], Condition::Appearing);
    w.build(|| {
        ui.text("This button interprets the input value [1.0, 0.0, 0.0, 1.0] as RGB(A) (default):");
        ui.color_button("RGBA red", [1.0, 0.0, 0.0, 1.0]);

        ui.separator();
        ui.text("This button interprets the input value [1.0, 0.0, 0.0, 1.0] as HSV(A):");
        ui.color_button_config("HSVA black", [1.0, 0.0, 0.0, 1.0])
            .input_mode(ColorEditInputMode::HSV)
            .build();
    });
}

#[derive(Default)]
struct State {
    example: u32,
    notify_text: &'static str,
}

impl State {
    fn reset(&mut self) {
        self.notify_text = "";
    }
}

