use std::fmt::Debug;
use std::sync::Arc;
use zoon::futures_signals::signal_vec::VecDiff;
use zoon::signal::Mutable;
use zoon::*;
use crate::form_subcomponents::form_text;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "serde")]
pub struct ChecklistItem {
    pub name: String,
    is_checked: Mutable<bool>,
}

impl ChecklistItem {
    pub fn from(x: String) -> ChecklistItem {
        ChecklistItem {
            name: x,
            is_checked: Mutable::new(false),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "serde")]
pub struct Selector {
    pub available_options: MutableVec<Arc<ChecklistItem>>,
    filter: Mutable<String>,
}

impl<'a> Selector {
    pub fn new(option_vec: Vec<String>) -> Selector {
        Selector {
            available_options: MutableVec::new_with_values(
                option_vec
                    .clone()
                    .into_iter()
                    .map(|x| Arc::new(ChecklistItem::from(x)))
                    .collect(),
            ),
            filter: Mutable::new(String::new()),
        }
    }

    pub fn checked_items_signal(
        &'static self,
        vec: impl SignalVec<Item=String>,
    ) -> &'static Selector {
        zoon::println!("checked signal");
        let x = Task::start_droppable({
            let lock = self.available_options.lock_mut();
            let y = vec.for_each(move |checked: VecDiff<String>| {
                match checked {
                    VecDiff::Replace { values } => values.iter().for_each(|item| {
                        lock.iter().for_each(|checklist_item| {
                            if checklist_item.name.eq(item) {
                                checklist_item.is_checked.set_neq(true)
                            }
                        })
                    }),
                    VecDiff::InsertAt { .. } => {}
                    VecDiff::UpdateAt { .. } => {}
                    VecDiff::RemoveAt { .. } => {}
                    VecDiff::Move { .. } => {}
                    VecDiff::Push { .. } => {}
                    VecDiff::Pop { .. } => {}
                    VecDiff::Clear { .. } => {}
                }
                async {}
            });
            async {}
        });
        self
    }

    fn filter_input_box(&'static self) -> impl Element {
        TextInput::new()
            .label_hidden("")
            .placeholder(Placeholder::new("Type to filter"))
            .on_change(move |filter| self.filter.set_neq(filter))
    }

    pub fn to_component(&'static self, title: &str) -> impl Element {
        Column::new()
            .item(form_text(title))
            .item(self.filter_input_box())
            .item(self.checklist_panel())
    }

    pub fn selected_items(&self) -> Vec<String> {
        let lock = self.available_options.lock_mut();
        lock.iter()
            .filter(|x| (*x).is_checked.get())
            .map(|item| (*item).name.clone())
            .collect()
    }

    fn checklist_panel(
        &'static self,
    ) -> impl Element {
        Column::new().items_signal_vec({
            self.available_options
                // items
                .signal_vec_cloned()
                .filter_signal_cloned(|item| {
                    let item_clone = item.clone();
                    self.filter
                        .signal_cloned()
                        .map(move |filter| item_clone.name.to_lowercase().contains(&filter))
                })
                .map(|y| self.checklist_item(y))
        })
    }

    fn checklist_item(&'static self, item: Arc<ChecklistItem>) -> impl Element {
        static ACTIVE_ICON: &str = "data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23ededed%22%20stroke-width%3D%223%22/%3E%3C/svg%3E";
        static COMPLETED_ICON: &str = "data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23bddad5%22%20stroke-width%3D%223%22/%3E%3Cpath%20fill%3D%22%235dc2af%22%20d%3D%22M72%2025L42%2071%2027%2056l-4%204%2020%2020%2034-52z%22/%3E%3C/svg%3E";
        let item_name = String::from(&item.name);
        Row::new()
            .item(
                Checkbox::new()
                    .label_hidden("")
                    // .checked_signal(self.available_options.signal_vec_cloned())
                    .checked_signal(item.is_checked.signal())
                    .on_change(move |checked| {
                        item.is_checked.set_neq(checked);
                    })
                    .icon(|checked| {
                        El::new()
                            .s(Width::new(40))
                            .s(Height::new(40))
                            .s(Background::new().url_signal(
                                checked.signal().map_bool(|| COMPLETED_ICON, || ACTIVE_ICON),
                            ))
                    }),
            )
            .item(Paragraph::new().content(item_name))
    }
}
