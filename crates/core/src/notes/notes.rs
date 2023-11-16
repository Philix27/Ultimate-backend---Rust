use crate::notes::components_form::ComponentsForm;
use std::time::Duration;

struct NotesComponents {
    index: u32,
    form: ComponentsForm,
    content: String
}
struct Notes {
    title: String,
    comps: Vec<NotesComponents>,
    created_at: Duration,
    updated_at: Duration,
}
