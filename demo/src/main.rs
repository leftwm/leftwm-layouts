use std::str::FromStr;

use druid::widget::{Align, Flex, Label, TextBox, Painter};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt, Rect, RenderContext, theme, PaintCtx, Color};
use leftwm_layouts::{Layout, Layouts, LayoutModifiers};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
    window_count: usize,
}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = HelloState {
        name: "World".into(),
        window_count: 3,
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<HelloState> {

    Painter::new(|ctx, _, env| {

        let parent_rect = ctx.size().to_rect();


        let mut modifiers = LayoutModifiers::default();
        modifiers.container_size = leftwm_layouts::geometry::Rect {
            x: parent_rect.x0 as i32,
            y: parent_rect.y0 as i32,
            w: (parent_rect.x1 - parent_rect.x0) as i32,
            h: (parent_rect.y1 - parent_rect.y0) as i32,
        };
        let layout = Layouts::from_str("MainAndVertStack");

        if let Ok(layout) = layout {
            let calcs = layout.get().apply(5, &modifiers);
            let mut master_count = layout.get().master_window_count(6, &modifiers);
            // println!("{:?}", calcs);
            calcs.into_iter()
                .filter(|o| o.is_some())
                .map(|o| o.unwrap())
                .for_each(|o| {
                    let rect = Rect::new(
                        o.x.into(), 
                        o.y.into(), 
                        (o.x + o.w).into(), 
                        (o.y + o.h).into(),
                    );
                    if master_count > 0 {
                        ctx.fill(rect, &env.get(theme::PRIMARY_LIGHT));
                        master_count = master_count - 1;
                    } else {
                        ctx.fill(rect, &env.get(theme::PRIMARY_DARK));
                    }
                    ctx.stroke(rect.inset(-0.5), &Color::WHITE, 1.0);
                })
            
        }
        
    })

    /*
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &HelloState, _env: &Env| format!("Hello {}!", data.name));
    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .with_placeholder("Who are we greeting?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::name);

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox);

    // center the two widgets in the available space
    Align::centered(layout)

    */
}