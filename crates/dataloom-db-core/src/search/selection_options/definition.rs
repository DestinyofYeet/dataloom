pub enum SelectionOptionsValues {
    Min { column: String },
    Max { column: String },
}

pub struct SelectionOptions {
    pub(super) option: SelectionOptionsValues,
}
