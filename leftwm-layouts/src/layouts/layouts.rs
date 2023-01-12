use serde::{Deserialize, Serialize};

use crate::{
    geometry::{ReserveColumnSpace, Rotation, SplitAxis},
    LayoutDefinition,
};

use super::new_layout_definition::{AlternateStack, Columns, Main, Stack};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layouts {
    pub layouts: Vec<LayoutDefinition>,
}

impl Eq for Layouts {}

impl Layouts {
    pub fn get(&self, name: &str) -> Option<&LayoutDefinition> {
        self.layouts.iter().find(|&l| l.name.as_str() == name)
    }

    pub fn get_mut<'a>(&'a mut self, name: &str) -> Option<&'a mut LayoutDefinition> {
        self.layouts.iter_mut().find(|l| l.name.as_str() == name)
    }

    pub fn names(&self) -> Vec<String> {
        self.layouts.iter().map(|x| x.name.clone()).collect()
    }

    pub fn len(&self) -> usize {
        self.layouts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.layouts.is_empty()
    }

    pub fn get_index(&self, name: &str) -> Option<usize> {
        self.layouts.iter().position(|l| l.name.as_str() == name)
    }

    #[allow(dead_code)]
    fn append_or_overwrite(&mut self, layout: LayoutDefinition) {
        match self.layouts.iter().position(|x| x.name == layout.name) {
            None => self.layouts.insert(0, layout.clone()),
            Some(i) => {
                self.layouts[i] = layout;
            }
        }
    }

    // pub fn from_config_with_defaults(config: &str) -> Self {
    //     let mut layouts: Layouts = Layouts::default();
    //     let custom_layouts: Vec<LayoutDefinition> = ron::from_str(config).unwrap();
    //     for custom_layout in custom_layouts {
    //         layouts.append_or_overwrite(custom_layout);
    //     }
    //     layouts
    // }

    // pub fn from_config(config: &str) -> Self {
    //     let layouts: Vec<LayoutDefinition> = ron::from_str(config).unwrap();
    //     Self { layouts }
    // }
}

impl Default for Layouts {
    fn default() -> Self {
        // single column layouts
        let even_horizontal = LayoutDefinition {
            name: String::from("EvenHorizontal"),
            main: None,
            stack: Stack {
                split: Some(SplitAxis::Vertical),
                ..Default::default()
            },
            ..Default::default()
        };
        let even_vertical = LayoutDefinition {
            name: String::from("EvenVertical"),
            main: None,
            stack: Stack::default(),
            ..Default::default()
        };
        let monocle = LayoutDefinition {
            name: String::from("Monocle"),
            main: None,
            stack: Stack {
                split: None,
                ..Default::default()
            },
            ..Default::default()
        };
        let grid = LayoutDefinition {
            name: String::from("Grid"),
            main: None,
            stack: Stack {
                split: Some(SplitAxis::Grid),
                ..Default::default()
            },
            ..Default::default()
        };

        // two column layouts
        let main_and_vert_stack = LayoutDefinition {
            name: String::from("MainAndVertStack"),
            main: Some(Main::default()),
            stack: Stack::default(),
            ..Default::default()
        };
        let main_and_horizontal_stack = LayoutDefinition {
            name: String::from("MainAndHorizontalStack"),
            main: Some(Main::default()),
            stack: Stack {
                split: Some(SplitAxis::Vertical),
                ..Default::default()
            },
            ..Default::default()
        };
        let right_main_and_vert_stack = LayoutDefinition {
            name: String::from("RightMainAndVertStack"),
            main: Some(Main::default()),
            stack: Stack::default(),
            root: Columns {
                rotate: Rotation::South,
                ..Default::default()
            },
            ..Default::default()
        };
        let fibonacci = LayoutDefinition {
            name: String::from("Fibonacci"),
            main: Some(Main::default()),
            stack: Stack {
                split: Some(SplitAxis::Fibonacci),
                ..Default::default()
            },
            ..Default::default()
        };
        let dwindle = LayoutDefinition {
            name: String::from("Dwindle"),
            main: Some(Main::default()),
            stack: Stack {
                split: Some(SplitAxis::Dwindle),
                ..Default::default()
            },
            ..Default::default()
        };
        let main_and_deck = LayoutDefinition {
            name: String::from("MainAndDeck"),
            main: Some(Main {
                split: None,
                ..Default::default()
            }),
            stack: Stack {
                split: None,
                ..Default::default()
            },
            ..Default::default()
        };
        let center_main = LayoutDefinition {
            name: String::from("CenterMain"),
            main: Some(Main::default()),
            stack: Stack::default(),
            alternate_stack: Some(AlternateStack::default()),
            ..Default::default()
        };
        let center_main_balanced = LayoutDefinition {
            name: String::from("CenterMainBalanced"),
            main: Some(Main::default()),
            stack: Stack {
                split: Some(SplitAxis::Dwindle),
                ..Default::default()
            },
            alternate_stack: Some(AlternateStack::default()),
            ..Default::default()
        };
        let center_main_fluid = LayoutDefinition {
            name: String::from("CenterMainFluid"),
            main: Some(Main::default()),
            stack: Stack {
                split: None,
                ..Default::default()
            },
            alternate_stack: Some(AlternateStack::default()),
            reserve: ReserveColumnSpace::Reserve,
            ..Default::default()
        };

        let layouts = vec![
            even_horizontal,
            even_vertical,
            monocle,
            grid,
            main_and_vert_stack,
            main_and_horizontal_stack,
            right_main_and_vert_stack,
            fibonacci,
            dwindle,
            main_and_deck,
            center_main,
            center_main_balanced,
            center_main_fluid,
        ];

        Self { layouts }
    }
}
