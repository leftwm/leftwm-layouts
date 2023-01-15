use crate::{
    geometry::{Reserve, Rotation, Split},
    Layout,
};

use super::layout::{Columns, Main, SecondStack, Stack};

const EVEN_HORIZONTAL: &str = "EvenHorizontal";
const EVEN_VERTICAL: &str = "EvenVertical";
const MONOCLE: &str = "Monocle";
const GRID: &str = "Grid";

const MAIN_AND_VERT_STACK: &str = "MainAndVertStack";
const MAIN_AND_HORIZONTAL_STACK: &str = "MainAndHorizontalStack";
const RIGHT_MAIN_AND_VERT_STACK: &str = "RightMainAndVertStack";
const FIBONACCI: &str = "Fibonacci";
const DWINDLE: &str = "Dwindle";
const MAIN_AND_DECK: &str = "MainAndDeck";

const CENTER_MAIN: &str = "CenterMain";
const CENTER_MAIN_BALANCED: &str = "CenterMainBalanced";
const CENTER_MAIN_FLUID: &str = "CenterMainFluid";

pub fn even_horizontal() -> Layout {
    Layout {
        name: EVEN_HORIZONTAL.to_string(),
        columns: Columns {
            main: None,
            stack: Stack {
                split: Some(Split::Vertical),
                ..Default::default()
            },
            ..Columns::default()
        },
        ..Default::default()
    }
}

pub fn even_vertical() -> Layout {
    Layout {
        name: EVEN_VERTICAL.to_string(),
        columns: Columns {
            main: None,
            stack: Stack::default(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn monocle() -> Layout {
    Layout {
        name: MONOCLE.to_string(),
        columns: Columns {
            main: None,
            stack: Stack {
                split: None,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn grid() -> Layout {
    Layout {
        name: GRID.to_string(),
        columns: Columns {
            main: None,
            stack: Stack {
                split: Some(Split::Grid),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn main_and_vert_stack() -> Layout {
    Layout {
        name: MAIN_AND_VERT_STACK.to_string(),
        columns: Columns {
            main: Some(Main::default()),
            stack: Stack::default(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn main_and_horizontal_stack() -> Layout {
    Layout {
        name: MAIN_AND_HORIZONTAL_STACK.to_string(),
        columns: Columns {
            main: Some(Main::default()),
            stack: Stack {
                split: Some(Split::Vertical),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn right_main_and_vert_stack() -> Layout {
    Layout {
        name: RIGHT_MAIN_AND_VERT_STACK.to_string(),
        columns: Columns {
            main: Some(Main::default()),
            stack: Stack::default(),
            rotate: Rotation::South,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn fibonacci() -> Layout {
    Layout {
        name: FIBONACCI.to_string(),
        columns: Columns {
            main: Some(Main::default()),
            stack: Stack {
                split: Some(Split::Fibonacci),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn dwindle() -> Layout {
    Layout {
        name: DWINDLE.to_string(),
        columns: Columns {
            main: Some(Main::default()),
            stack: Stack {
                split: Some(Split::Dwindle),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn main_and_deck() -> Layout {
    Layout {
        name: MAIN_AND_DECK.to_string(),
        columns: Columns {
            main: Some(Main {
                split: None,
                ..Default::default()
            }),
            stack: Stack {
                split: None,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn center_main() -> Layout {
    Layout {
        name: CENTER_MAIN.to_string(),
        columns: Columns {
            main: Some(Main::default()),
            stack: Stack::default(),
            second_stack: Some(SecondStack::default()),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn center_main_balanced() -> Layout {
    Layout {
        name: CENTER_MAIN_BALANCED.to_string(),
        columns: Columns {
            main: Some(Main::default()),
            stack: Stack {
                split: Some(Split::Dwindle),
                ..Default::default()
            },
            second_stack: Some(SecondStack::default()),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn center_main_fluid() -> Layout {
    Layout {
        name: CENTER_MAIN_FLUID.to_string(),
        columns: Columns {
            main: Some(Main::default()),
            stack: Stack {
                split: None,
                ..Default::default()
            },
            second_stack: Some(SecondStack::default()),
            ..Default::default()
        },
        reserve: Reserve::Reserve,
        ..Default::default()
    }
}
