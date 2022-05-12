use std::cmp;
use std::collections::HashMap;

use druid::piet::{Text, TextLayout, TextLayoutBuilder};
use druid::widget::{Button, Container, Flex, LabelText, List, Painter, RadioGroup};
use druid::{
    AppLauncher, Color, Data, Lens, LocalizedString, Point, Rect, RenderContext, Widget, WidgetExt,
    WindowDesc,
};
use leftwm_layouts::geometry::{Flipped, ReserveColumnSpace, Rotation, Size};
use leftwm_layouts::Layouts;

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
        let layouts = leftwm_layouts::default_layouts().to_owned();
        let names = layouts.layout_names();
        let name = names.get(0).unwrap();
        Self {
            layouts,
            current_layout: name.to_owned(),
            window_count: 3,
        }
    }
}

impl DemoState {
    fn current(&self) -> &leftwm_layouts::LayoutDefinition {
        self.layouts
            .layouts
            .get(self.current_layout.as_str())
            .unwrap()
    }

    fn current_mut(&mut self) -> &mut leftwm_layouts::LayoutDefinition {
        self.layouts
            .layouts
            .get_mut(self.current_layout.as_str())
            .unwrap()
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
        self.current_mut().increase_main_size()
    }

    fn decrease_main_width(&mut self) {
        self.current_mut().decrease_main_size()
    }

    fn increase_main_count(&mut self) {
        self.current_mut().main_window_count += 1
    }

    fn decrease_main_count(&mut self) {
        self.current_mut().main_window_count =
            self.current_mut().main_window_count.saturating_sub(1)
    }

    fn toggle_flipped_horizontal(&mut self) {
        self.current_mut().flipped = self.current_mut().flipped.toggle_horizontal()
    }

    fn toggle_flipped_vertical(&mut self) {
        self.current_mut().flipped = self.current_mut().flipped.toggle_vertical()
    }

    /*fn toggle_reserve_space(&mut self) {
        self.reserve_space = !self.reserve_space
    }*/

    fn rotate(&mut self) {
        self.current_mut().rotation = self.current_mut().rotation.clockwise();
    }
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
    let names = leftwm_layouts::default_layouts().layout_names();

    let mut col = Flex::column();
    for key in names {
        let button = Button::new(key.to_owned())
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

    let flip_h = button(|data: &DemoState, _env: &_| {
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
    .on_click(move |_ctx, data: &mut DemoState, _env| data.toggle_flipped_vertical());

    let rotation =
        button(|data: &DemoState, _env: &_| format!("Rotation: {:?}", data.current().rotation))
            .on_click(move |_ctx, data: &mut DemoState, _env| data.rotate());

    /*let reserve_space =
    button(|data: &DemoState, _env: &_| format!("Reserve Space: {:?}", data.curr().reserve_column_space))
        .on_click(move |_ctx, data: &mut DemoState, _env| data.toggle_reserve_space());*/

    /*let reserve_column_space = RadioGroup::new(vec![
        ("None", ReserveOption::None),
        ("Reserve", ReserveOption::Reserve),
        ("ReserveAndCenter", ReserveOption::ReserveAndCenter),
    ]).on_click(move |_ctx, data: &mut DemoState, _env| {
        data.current_mut().reserve_column_space = ReserveOption::None.into()
    });*/

    let reserve_none = button(|data: &DemoState, _env: &_| {
        format!(
            "ReserveColumnSpace: {:?}",
            data.current().reserve_column_space
        )
    })
    .on_click(move |_ctx, data: &mut DemoState, _env| {
        data.current_mut().reserve_column_space = ReserveColumnSpace::None
    });

    let reserve_yes = button(|data: &DemoState, _env: &_| {
        format!(
            "ReserveColumnSpace: {:?}",
            data.current().reserve_column_space
        )
    })
    .on_click(move |_ctx, data: &mut DemoState, _env| {
        data.current_mut().reserve_column_space = ReserveColumnSpace::Reserve
    });

    let reserve_and_center = button(|data: &DemoState, _env: &_| {
        format!(
            "ReserveColumnSpace: {:?}",
            data.current().reserve_column_space
        )
    })
    .on_click(move |_ctx, data: &mut DemoState, _env| {
        data.current_mut().reserve_column_space = ReserveColumnSpace::ReserveAndCenter
    });

    let flex = Flex::column()
        .with_child(col)
        .with_flex_child(inc_main, 1.0)
        .with_flex_child(dec_main, 1.0)
        .with_flex_child(inc_main_count, 1.0)
        .with_flex_child(dec_main_count, 1.0)
        .with_flex_child(add_window, 1.0)
        .with_flex_child(remove_window, 1.0)
        .with_flex_child(flip_h, 1.0)
        .with_flex_child(flip_v, 1.0)
        .with_flex_child(rotation, 1.0)
        .with_flex_child(reserve_none, 1.0)
        .with_flex_child(reserve_yes, 1.0)
        .with_flex_child(reserve_and_center, 1.0);
    //.with_flex_child(reserve_space, 1.0)
    //.with_flex_child(reserve_column_space, 1.0);

    flex.fix_width(240.0).background(PRIMARY)
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
        let calcs = leftwm_layouts::apply(layout, data.window_count, container);
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
