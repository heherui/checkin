use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, ComboBoxText, Entry, Label, Orientation, Window};

use crate::core::{CellKind, Subject};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellEditDraft {
    pub kind: CellKind,
    pub name: Option<String>,
}

impl CellEditDraft {
    pub fn from_subject(subject: Option<&Subject>) -> Self {
        match subject {
            Some(Subject::Some(name)) => Self {
                kind: CellKind::Active,
                name: Some(name.clone()),
            },
            Some(Subject::Block(name)) => Self {
                kind: CellKind::Blocked,
                name: Some(name.clone()),
            },
            Some(Subject::Transparent) => Self {
                kind: CellKind::Transparent,
                name: None,
            },
            None => Self {
                kind: CellKind::Active,
                name: None,
            },
        }
    }

    pub fn into_subject(self) -> Option<Subject> {
        match self.kind {
            CellKind::Active => {
                let name = self.name.unwrap_or_default().trim().to_owned();
                if name.is_empty() {
                    None
                } else {
                    Some(Subject::Some(name))
                }
            }
            CellKind::Blocked => Some(Subject::Block(
                self.name.unwrap_or_default().trim().to_owned(),
            )),
            CellKind::Transparent => Some(Subject::Transparent),
        }
    }
}

pub struct CellEditDialog;

impl CellEditDialog {
    pub fn present<F>(cell: &GtkBox, initial: CellEditDraft, on_save: F)
    where
        F: Fn(CellEditDraft) + 'static,
    {
        let window = Self::build(cell);
        let content = GtkBox::new(Orientation::Vertical, 8);
        content.set_margin_top(14);
        content.set_margin_bottom(14);
        content.set_margin_start(14);
        content.set_margin_end(14);

        let type_label = Label::new(Some("类型"));
        type_label.set_xalign(0.0);
        let kind_combo = ComboBoxText::new();
        kind_combo.append(Some("active"), "Active");
        kind_combo.append(Some("blocked"), "Blocked");
        kind_combo.append(Some("transparent"), "Transparent");
        kind_combo.set_active_id(Some(Self::kind_id(initial.kind)));

        let name_label = Label::new(Some("名称"));
        name_label.set_xalign(0.0);
        let name_entry = Entry::new();
        name_entry.set_placeholder_text(Some("可为空"));
        name_entry.set_text(initial.name.as_deref().unwrap_or(""));
        Self::sync_name_editor_state(
            initial.kind,
            &name_label,
            &name_entry,
            initial.name.as_deref().unwrap_or(""),
        );

        {
            let name_label = name_label.clone();
            let name_entry = name_entry.clone();
            kind_combo.connect_changed(move |combo| {
                let kind = combo
                    .active_id()
                    .as_deref()
                    .and_then(Self::kind_from_id)
                    .unwrap_or(CellKind::Active);
                Self::sync_name_editor_state(kind, &name_label, &name_entry, "");
            });
        }

        let actions = GtkBox::new(Orientation::Horizontal, 8);
        let cancel_button = Button::with_label("取消");
        let save_button = Button::with_label("保存");
        actions.append(&cancel_button);
        actions.append(&save_button);

        {
            let window = window.clone();
            cancel_button.connect_clicked(move |_| {
                window.close();
            });
        }

        {
            let window = window.clone();
            let kind_combo_for_save = kind_combo.clone();
            let name_entry_for_save = name_entry.clone();
            save_button.connect_clicked(move |_| {
                let kind = kind_combo_for_save
                    .active_id()
                    .as_deref()
                    .and_then(Self::kind_from_id)
                    .unwrap_or(initial.kind);
                let draft = CellEditDraft {
                    kind,
                    name: if kind == CellKind::Transparent {
                        None
                    } else {
                        Some(name_entry_for_save.text().to_string())
                    },
                };
                on_save(draft);
                window.close();
            });
        }

        content.append(&type_label);
        content.append(&kind_combo);
        content.append(&name_label);
        content.append(&name_entry);
        content.append(&actions);
        window.set_child(Some(&content));
        window.present();
    }

    fn build(cell: &GtkBox) -> Window {
        let window = Window::builder()
            .modal(true)
            .title("编辑单元格")
            .default_width(320)
            .default_height(180)
            .build();

        if let Some(parent) = cell.root().and_then(|root| root.downcast::<Window>().ok()) {
            window.set_transient_for(Some(&parent));
        }

        window
    }

    fn kind_id(kind: CellKind) -> &'static str {
        match kind {
            CellKind::Active => "active",
            CellKind::Blocked => "blocked",
            CellKind::Transparent => "transparent",
        }
    }

    fn kind_from_id(id: &str) -> Option<CellKind> {
        match id {
            "active" => Some(CellKind::Active),
            "blocked" => Some(CellKind::Blocked),
            "transparent" => Some(CellKind::Transparent),
            _ => None,
        }
    }

    fn sync_name_editor_state(kind: CellKind, label: &Label, entry: &Entry, fallback_text: &str) {
        let editable = kind != CellKind::Transparent;
        label.set_visible(editable);
        entry.set_visible(editable);
        entry.set_sensitive(editable);
        if editable {
            if entry.text().is_empty() {
                entry.set_text(fallback_text);
            }
        } else {
            entry.set_text("");
        }
    }
}
