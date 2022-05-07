use druid::piet::{Text, TextLayout, TextLayoutBuilder};
use druid::widget::{Button, Container, Flex, LabelText, Painter, RadioGroup};
use druid::{
    AppLauncher, Color, Data, Lens, LocalizedString, Point, Rect, RenderContext, Widget, WidgetExt,
    WindowDesc,
};
use leftwm_layouts::geometry::{Flipped, ReserveColumnSpace, Rotation, Size};
use leftwm_layouts::{LayoutEnum, LayoutModifiers, LayoutOptions};

const PRIMARY: Color = Color::rgb8(0x08, 0x0f, 0x0f);

const WINDOW_TITLE: LocalizedString<DemoState> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct DemoState {
    layout: LayoutOption,
    window_count: usize,
    master_width_percentage: f32,
    master_window_count: usize,
    max_column_width: Option<u32>,
    reserve_space: bool,

    #[data(same_fn = "PartialEq::eq")]
    reserve_column_space: ReserveOption,

    #[data(same_fn = "PartialEq::eq")]
    flipped: Flipped,

    #[data(same_fn = "PartialEq::eq")]
    rotation: Rotation,
}

impl Default for DemoState {
    fn default() -> Self {
        Self {
            layout: LayoutOption::MainAndVertStack,
            window_count: 5,
            master_width_percentage: 60.0,
            master_window_count: 1,
            max_column_width: None,
            flipped: Flipped::default(),
            rotation: Rotation::default(),
            reserve_space: false,
            reserve_column_space: ReserveOption::None,
        }
    }
}

impl DemoState {
    fn add_window(&mut self) {
        self.window_count += 1;
    }

    fn remove_window(&mut self) {
        let new_count = if self.window_count > 0 {
            self.window_count - 1
        } else {
            0
        };
        self.window_count = new_count;
    }

    fn increase_master_width(&mut self) {
        let new_width = self.master_width_percentage + 5.0;
        if new_width > 100.0 {
            self.master_width_percentage = 100.0;
        } else {
            self.master_width_percentage = new_width;
        }
    }

    fn decrease_master_width(&mut self) {
        let new_width = self.master_width_percentage - 5.0;
        if new_width < 0.0 {
            self.master_width_percentage = 0.0;
        } else {
            self.master_width_percentage = new_width;
        }
    }

    fn increase_master_count(&mut self) {
        self.master_window_count += 1;
    }

    fn decrease_master_count(&mut self) {
        let new_count = if self.master_window_count > 0 {
            self.master_window_count - 1
        } else {
            0
        };
        self.master_window_count = new_count;
    }

    fn toggle_flipped_horizontal(&mut self) {
        self.flipped = Flipped::toggle_horizontal(&self.flipped)
    }

    fn toggle_flipped_vertical(&mut self) {
        self.flipped = Flipped::toggle_vertical(&self.flipped)
    }

    fn toggle_reserve_space(&mut self) {
        self.reserve_space = !self.reserve_space
    }

    fn rotate(&mut self) {
        self.rotation = match self.rotation {
            Rotation::North => Rotation::East,
            Rotation::East => Rotation::South,
            Rotation::South => Rotation::West,
            Rotation::West => Rotation::North,
        }
    }
}

impl From<&DemoState> for LayoutModifiers {
    fn from(value: &DemoState) -> Self {
        LayoutModifiers {
            main_size: Size::Percentage(value.master_width_percentage),
            main_window_count: value.master_window_count,
            reserve_column_space: value.reserve_column_space.into(),
            ..Default::default()
        }
    }
}

impl From<&DemoState> for LayoutOptions {
    fn from(value: &DemoState) -> Self {
        LayoutOptions {
            flipped: value.flipped,
            rotation: value.rotation,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, Data, PartialEq)]
enum LayoutOption {
    Monocle,
    MainAndVertStack,
    MainAndHorizontalStack,
    CenterMain,
    Grid,
    EvenHorizontal,
    Fibonacci,
    Dwindle,
}

#[derive(Debug, Clone, Copy, Data, PartialEq)]
enum ReserveOption {
    None,
    Reserve,
    ReserveAndCenter,
}

impl From<ReserveOption> for ReserveColumnSpace {
    fn from(option: ReserveOption) -> Self {
        match option {
            ReserveOption::None => ReserveColumnSpace::None,
            ReserveOption::Reserve => ReserveColumnSpace::Reserve,
            ReserveOption::ReserveAndCenter => ReserveColumnSpace::ReserveAndCenter,
        }
    }
}

impl From<LayoutOption> for LayoutEnum {
    fn from(option: LayoutOption) -> Self {
        match option {
            LayoutOption::Monocle => Self::Monocle,
            LayoutOption::MainAndVertStack => Self::MainAndVertStack,
            LayoutOption::CenterMain => Self::CenterMain,
            LayoutOption::Fibonacci => Self::Fibonacci,
            LayoutOption::MainAndHorizontalStack => Self::MainAndHorizontalStack,
            LayoutOption::Grid => Self::Grid,
            LayoutOption::EvenHorizontal => Self::EvenHorizontal,
            LayoutOption::Dwindle => Self::Dwindle,
        }
    }
}

//impl Into<LayoutEnum> for LayoutOption {
//    fn into(self) -> LayoutEnum {
//        match self {
//            Self::Monocle => LayoutEnum::Monocle,
//            Self::MainAndVertStack => LayoutEnum::MainAndVertStack,
//        }
//    }
//}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((1280.0, 720.0));

    // create the initial app state
    let initial_state = DemoState::default();

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
    let selector = RadioGroup::new(vec![
        ("Monocle", LayoutOption::Monocle),
        ("MainAndVertStack", LayoutOption::MainAndVertStack),
        (
            "MainAndHorizontalStack",
            LayoutOption::MainAndHorizontalStack,
        ),
        ("Grid", LayoutOption::Grid),
        ("EvenHorizontal", LayoutOption::EvenHorizontal),
        ("CenterMain", LayoutOption::CenterMain),
        ("Fibonacci", LayoutOption::Fibonacci),
        ("Dwindle", LayoutOption::Dwindle),
    ])
    .lens(DemoState::layout);

    let inc_master = button("IncreaseMainWidth")
        .on_click(move |_ctx, data: &mut DemoState, _env| data.increase_master_width());

    let dec_master = button("DecreaseMainWidth")
        .on_click(move |_ctx, data: &mut DemoState, _env| data.decrease_master_width());

    let add_window =
        button("AddWindow").on_click(move |_ctx, data: &mut DemoState, _env| data.add_window());

    let remove_window = button("RemoveWindow")
        .on_click(move |_ctx, data: &mut DemoState, _env| data.remove_window());

    let inc_master_count = button("IncreaseMasterCount")
        .on_click(move |_ctx, data: &mut DemoState, _env| data.increase_master_count());

    let dec_master_count = button("DecreaseMasterCount")
        .on_click(move |_ctx, data: &mut DemoState, _env| data.decrease_master_count());

    let flip_h = button(|data: &DemoState, _env: &_| {
        format!("FlipHorziontal: {}", data.flipped.is_flipped_horizontal())
    })
    .on_click(move |_ctx, data: &mut DemoState, _env| data.toggle_flipped_horizontal());

    let flip_v = button(|data: &DemoState, _env: &_| {
        format!("FlipVertical: {}", data.flipped.is_flipped_vertical())
    })
    .on_click(move |_ctx, data: &mut DemoState, _env| data.toggle_flipped_vertical());

    let rotation = button(|data: &DemoState, _env: &_| format!("Rotation: {:?}", data.rotation))
        .on_click(move |_ctx, data: &mut DemoState, _env| data.rotate());

    let reserve_space =
        button(|data: &DemoState, _env: &_| format!("Reserve Space: {:?}", data.reserve_space))
            .on_click(move |_ctx, data: &mut DemoState, _env| data.toggle_reserve_space());

    let reserve_column_space = RadioGroup::new(vec![
        ("None", ReserveOption::None),
        ("Reserve", ReserveOption::Reserve),
        ("ReserveAndCenter", ReserveOption::ReserveAndCenter),
    ])
    .lens(DemoState::reserve_column_space);

    let flex = Flex::column()
        .with_child(selector)
        .with_flex_child(inc_master, 1.0)
        .with_flex_child(dec_master, 1.0)
        .with_flex_child(inc_master_count, 1.0)
        .with_flex_child(dec_master_count, 1.0)
        .with_flex_child(add_window, 1.0)
        .with_flex_child(remove_window, 1.0)
        .with_flex_child(flip_h, 1.0)
        .with_flex_child(flip_v, 1.0)
        .with_flex_child(rotation, 1.0)
        .with_flex_child(reserve_space, 1.0)
        .with_flex_child(reserve_column_space, 1.0);

    flex.fix_width(240.0).background(PRIMARY)
}

fn layout_preview() -> impl Widget<DemoState> {
    Painter::new(|ctx, data: &DemoState, _| {
        let parent_size = ctx.size();
        let modifiers = LayoutModifiers::from(data);
        let mut options = LayoutOptions::from(data);
        options.container_size = leftwm_layouts::geometry::Rect {
            x: 0,
            y: 0,
            w: parent_size.width as u32,
            h: parent_size.height as u32,
        };

        let layout: LayoutEnum = data.layout.into();
        let calcs = leftwm_layouts::apply(&layout, data.window_count, &options, &modifiers);
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
