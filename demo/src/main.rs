use druid::piet::{Text, TextLayout, TextLayoutBuilder};
use druid::widget::{Button, Container, Flex, Label, LabelText, Painter};
use druid::{
    AppLauncher, Color, Data, Insets, Lens, LocalizedString, Point, Rect, RenderContext, Widget,
    WidgetExt, WindowDesc,
};
use leftwm_layouts::layouts::Layouts;

const PRIMARY: Color = Color::rgb8(0x08, 0x0f, 0x0f);

const WINDOW_TITLE: LocalizedString<DemoState> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct DemoState {
    #[data(same_fn = "PartialEq::eq")]
    layouts: Layouts,
    current_layout: String,
    window_count: usize,
}

impl Default for DemoState {
    fn default() -> Self {
        let layouts = Layouts::default();
        let names = layouts.names();
        let name = names.get(0).unwrap();
        Self {
            layouts,
            current_layout: name.to_owned(),
            window_count: 3,
        }
    }
}

impl DemoState {
    fn current(&self) -> &leftwm_layouts::Layout {
        self.layouts.get(self.current_layout.as_str()).unwrap()
    }

    fn current_mut(&mut self) -> &mut leftwm_layouts::Layout {
        self.layouts.get_mut(self.current_layout.as_str()).unwrap()
    }

    fn add_window(&mut self) {
        self.window_count += 1
    }

    fn remove_window(&mut self) {
        let new_count = if self.window_count > 0 {
            self.window_count - 1
        } else {
            0
        };
        self.window_count = new_count
    }

    fn increase_main_width(&mut self) {
        self.current_mut().increase_main_size(9999)
    }

    fn decrease_main_width(&mut self) {
        self.current_mut().decrease_main_size()
    }

    fn increase_main_count(&mut self) {
        self.current_mut().increase_main_window_count()
    }

    fn decrease_main_count(&mut self) {
        self.current_mut().decrease_main_window_count()
    }

    fn toggle_flipped_horizontal(&mut self) {
        //self.current_mut().flipped = self.current().flipped.toggle_horizontal()
    }

    fn toggle_flipped_vertical(&mut self) {
        //self.current_mut().flipped = self.current().flipped.toggle_vertical()
    }

    fn toggle_balance_stacks(&mut self) {
        //self.current_mut().balance_stacks = !self.current().balance_stacks
    }

    fn change_reserve_space(&mut self) {
        //self.current_mut().reserve_column_space = match self.current().reserve_column_space {
        //    ReserveColumnSpace::None => ReserveColumnSpace::Reserve,
        //    ReserveColumnSpace::Reserve => ReserveColumnSpace::ReserveAndCenter,
        //    ReserveColumnSpace::ReserveAndCenter => ReserveColumnSpace::None,
        //};
    }

    fn rotate(&mut self) {
        self.current_mut().rotate(true);
    }
}

fn main() {
    // create the initial app state
    let initial_state = DemoState::default();

    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((1280.0, 720.0));

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<DemoState> {
    Flex::row().with_child(controls()).with_flex_child(
        Container::new(layout_preview()).background(Color::BLACK),
        2.0,
    )
}

fn controls() -> impl Widget<DemoState> {
    let names = Layouts::default().names();

    let mut col = Flex::column();
    for key in names {
        let button = button(key.to_owned())
            .on_click(move |_ctx, data: &mut DemoState, _env| data.current_layout = key.to_owned());
        col.add_child(button)
    }

    let inc_main = button("IncreaseMainWidth")
        .on_click(move |_ctx, data: &mut DemoState, _env| data.increase_main_width());

    let dec_main = button("DecreaseMainWidth")
        .on_click(move |_ctx, data: &mut DemoState, _env| data.decrease_main_width());

    let add_window =
        button("AddWindow").on_click(move |_ctx, data: &mut DemoState, _env| data.add_window());

    let remove_window = button("RemoveWindow")
        .on_click(move |_ctx, data: &mut DemoState, _env| data.remove_window());

    let inc_main_count = button("IncreaseMainCount")
        .on_click(move |_ctx, data: &mut DemoState, _env| data.increase_main_count());

    let dec_main_count = button("DecreaseMainCount")
        .on_click(move |_ctx, data: &mut DemoState, _env| data.decrease_main_count());

    /*let flip_h = button(|data: &DemoState, _env: &_| {
        format!(
            "FlipHorziontal: {}",
            data.current().flipped.is_flipped_horizontal()
        )
    })
    .on_click(move |_ctx, data: &mut DemoState, _env| data.toggle_flipped_horizontal());

    let flip_v = button(|data: &DemoState, _env: &_| {
        format!(
            "FlipVertical: {}",
            data.current().flipped.is_flipped_vertical()
        )
    })
    .on_click(move |_ctx, data: &mut DemoState, _env| data.toggle_flipped_vertical());*/

    let rotation =
        button(|data: &DemoState, _env: &_| format!("Rotation: {:?}", data.current().rotate))
            .on_click(move |_ctx, data: &mut DemoState, _env| data.rotate());

    /*let balance_stacks = button(|data: &DemoState, _env: &_| {
        format!("BalanceStacks: {}", data.current().balance_stacks)
    })
    .on_click(move |_ctx, data: &mut DemoState, _env| data.toggle_balance_stacks());

    let reserve_space = button(|data: &DemoState, _env: &_| {
        format!(
            "ReserveColumnSpace: {:?}",
            data.current().reserve_column_space
        )
    })
    .on_click(move |_ctx, data: &mut DemoState, _env| data.change_reserve_space());*/

    let flex = Flex::column()
        .with_child(label("Layouts"))
        .with_child(col)
        .with_child(label("Modifiers"))
        .with_child(inc_main)
        .with_child(dec_main)
        .with_child(inc_main_count)
        .with_child(dec_main_count)
        .with_child(add_window)
        .with_child(remove_window)
        .with_child(rotation);
    /*.with_child(flip_h)
    .with_child(flip_v)
    .with_child(balance_stacks)
    .with_child(reserve_space)*/

    flex.fix_width(260.0).expand_height().background(PRIMARY)
}

fn layout_preview() -> impl Widget<DemoState> {
    Painter::new(|ctx, data: &DemoState, _| {
        let parent_size = ctx.size();
        let container = leftwm_layouts::geometry::Rect {
            x: 0,
            y: 0,
            w: parent_size.width as u32,
            h: parent_size.height as u32,
        };

        let layout = data.current().to_owned();

        //let layout: LayoutEnum = data.layout.into();
        let calcs = leftwm_layouts::apply(&layout, data.window_count, &container);
        let step = 1.0 / data.window_count as f64;
        let mut alpha = 1.0;
        calcs.into_iter().enumerate().for_each(|(i, o)| {
            let bg_color = Color::WHITE.with_alpha(alpha);
            let text_color = if alpha > 0.5 {
                Color::BLACK
            } else {
                Color::WHITE
            };
            alpha = alpha - step;

            let rect = Rect::new(
                o.x.into(),
                o.y.into(),
                (o.x + o.w as i32).into(),
                (o.y + o.h as i32).into(),
            );
            ctx.fill(rect, &bg_color);
            ctx.stroke(rect.inset(-0.5), &Color::WHITE, 1.0);
            let text = ctx.text();
            let font = text.font_family("monospace").unwrap();

            let text_layout = text
                .new_text_layout(format!("{}", i + 1))
                .text_color(text_color)
                .font(font, 22.0)
                .build()
                .unwrap();

            let center = o.center();

            let pos = Point {
                x: center.0 as f64 - (text_layout.size().width / 2.0),
                y: center.1 as f64 - (text_layout.size().height / 2.0),
            };

            ctx.draw_text(&text_layout, pos);
        })
    })
    .expand()
}

fn button(text: impl Into<LabelText<DemoState>>) -> impl Widget<DemoState> {
    Button::new(text).expand_width().padding(4.0)
}

fn label(text: impl Into<LabelText<DemoState>>) -> impl Widget<DemoState> {
    Label::new(text)
        .padding(Insets::new(0.0, 24.0, 0.0, 4.0))
        .expand_width()
}
