use tui::widgets::ListState;

pub trait NewTrait: Clone + Default {}
impl<T> NewTrait for T where T: Clone + Default {}

pub struct StatefulList<T: NewTrait> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T: NewTrait> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn selected_object(&self) -> T {
        let object = if let Some(index) = self.state.selected() {
            self.items[index].clone()
        } else {
            T::default()
        };

        return object;
    }
}

pub struct App {
    pub items: StatefulList<crate::api::hnitem::HNItem>,
}

impl App {
    pub fn new(hnitems: Vec<crate::api::hnitem::HNItem>) -> App {
        App {
            items: StatefulList::with_items(hnitems),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new(vec![])
    }
}
