use gtk4::prelude::*;
use gtk4::{Box as GtkBox, ComboBoxText, Dialog, Entry, Label, Orientation, ResponseType, Window};

use crate::core::{CellKind, Subject};

/// Editable cell data used by the edit dialog.
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

/// Modal dialog used by edit mode for changing type and name.
pub struct CellEditDialog;

impl CellEditDialog {
    pub fn present<F>(cell: &GtkBox, initial: CellEditDraft, on_save: F)
    where
        F: Fn(CellEditDraft) + 'static,
    {
        let dialog = Self::build(cell, &initial);
        let content = dialog.content_area();
        let form = GtkBox::new(Orientation::Vertical, 8);

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

        form.append(&type_label);
        form.append(&kind_combo);
        form.append(&name_label);
        form.append(&name_entry);
        content.append(&form);

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

        dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
                let kind = kind_combo
                    .active_id()
                    .as_deref()
                    .and_then(Self::kind_from_id)
                    .unwrap_or(initial.kind);
                let draft = CellEditDraft {
                    kind,
                    name: if kind == CellKind::Transparent {
                        None
                    } else {
                        Some(name_entry.text().to_string())
                    },
                };
                on_save(draft);
            }

            dialog.close();
        });

        dialog.present();
    }

    fn build(cell: &GtkBox, _initial: &CellEditDraft) -> Dialog {
        let dialog = Dialog::builder()
            .modal(true)
            .title("编辑单元格")
            .default_width(320)
            .build();

        if let Some(window) = cell.root().and_then(|root| root.downcast::<Window>().ok()) {
            dialog.set_transient_for(Some(&window));
        }

        dialog.add_button("取消", ResponseType::Cancel);
        dialog.add_button("保存", ResponseType::Accept);
        dialog
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
