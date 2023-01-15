use crate::{
    geometry::{Reserve, Rotation, Split},
    LayoutDefinition,
};

use super::layout_definition::{Columns, Main, SecondStack, Stack};

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

pub fn even_horizontal() -> LayoutDefinition {
    LayoutDefinition {
        name: EVEN_HORIZONTAL.to_string(),
        main: None,
        stack: Stack {
            split: Some(Split::Vertical),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn even_vertical() -> LayoutDefinition {
    LayoutDefinition {
        name: EVEN_VERTICAL.to_string(),
        main: None,
        stack: Stack::default(),
        ..Default::default()
    }
}

pub fn monocle() -> LayoutDefinition {
    LayoutDefinition {
        name: MONOCLE.to_string(),
        main: None,
        stack: Stack {
            split: None,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn grid() -> LayoutDefinition {
    LayoutDefinition {
        name: GRID.to_string(),
        main: None,
        stack: Stack {
            split: Some(Split::Grid),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn main_and_vert_stack() -> LayoutDefinition {
    LayoutDefinition {
        name: MAIN_AND_VERT_STACK.to_string(),
        main: Some(Main::default()),
        stack: Stack::default(),
        ..Default::default()
    }
}

pub fn main_and_horizontal_stack() -> LayoutDefinition {
    LayoutDefinition {
        name: MAIN_AND_HORIZONTAL_STACK.to_string(),
        main: Some(Main::default()),
        stack: Stack {
            split: Some(Split::Vertical),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn right_main_and_vert_stack() -> LayoutDefinition {
    LayoutDefinition {
        name: RIGHT_MAIN_AND_VERT_STACK.to_string(),
        main: Some(Main::default()),
        stack: Stack::default(),
        columns: Columns {
            rotate: Rotation::South,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn fibonacci() -> LayoutDefinition {
    LayoutDefinition {
        name: FIBONACCI.to_string(),
        main: Some(Main::default()),
        stack: Stack {
            split: Some(Split::Fibonacci),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn dwindle() -> LayoutDefinition {
    LayoutDefinition {
        name: DWINDLE.to_string(),
        main: Some(Main::default()),
        stack: Stack {
            split: Some(Split::Dwindle),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn main_and_deck() -> LayoutDefinition {
    LayoutDefinition {
        name: MAIN_AND_DECK.to_string(),
        main: Some(Main {
            split: None,
            ..Default::default()
        }),
        stack: Stack {
            split: None,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn center_main() -> LayoutDefinition {
    LayoutDefinition {
        name: CENTER_MAIN.to_string(),
        main: Some(Main::default()),
        stack: Stack::default(),
        second_stack: Some(SecondStack::default()),
        ..Default::default()
    }
}

pub fn center_main_balanced() -> LayoutDefinition {
    LayoutDefinition {
        name: CENTER_MAIN_BALANCED.to_string(),
        main: Some(Main::default()),
        stack: Stack {
            split: Some(Split::Dwindle),
            ..Default::default()
        },
        second_stack: Some(SecondStack::default()),
        ..Default::default()
    }
}

pub fn center_main_fluid() -> LayoutDefinition {
    LayoutDefinition {
        name: CENTER_MAIN_FLUID.to_string(),
        main: Some(Main::default()),
        stack: Stack {
            split: None,
            ..Default::default()
        },
        second_stack: Some(SecondStack::default()),
        reserve: Reserve::Reserve,
        ..Default::default()
    }
}
