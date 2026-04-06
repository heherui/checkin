use gtk4::{
    gdk,
    glib::{self, subclass::types::ObjectSubclassExt},
};

use crate::ui::component::table_cell::data::{TableCellColors, TableCellData};

mod imp
{
    use std::cell::RefCell;

    use gtk4::{
        glib::{
            self,
            subclass::{
                object::{ObjectImpl, ObjectImplExt},
                types::{ObjectSubclass, ObjectSubclassExt},
            },
        },
        subclass::{box_::BoxImpl, widget::WidgetImpl},
        traits::{BoxExt, StyleContextExt, WidgetExt},
    };

    #[derive(Default)]
    pub struct TableCell
    {
        pub container: RefCell<Option<gtk4::Box>>,
        pub label: RefCell<Option<gtk4::Label>>,
        pub container_css: RefCell<Option<gtk4::CssProvider>>,
        pub label_css: RefCell<Option<gtk4::CssProvider>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TableCell
    {
        type Type = super::TableCell;
        type ParentType = gtk4::Box;

        const NAME: &'static str = "CheckinTableCell";
    }

    impl ObjectImpl for TableCell
    {
        fn constructed(&self)
        {
            self.parent_constructed();

            let obj = self.obj();
            obj.set_hexpand(true);
            obj.set_vexpand(true);
            obj.set_halign(gtk4::Align::Fill);
            obj.set_valign(gtk4::Align::Fill);

            let container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
            container.set_hexpand(true);
            container.set_vexpand(true);
            container.set_halign(gtk4::Align::Fill);
            container.set_valign(gtk4::Align::Fill);

            let label = gtk4::Label::new(None);
            label.set_hexpand(true);
            label.set_vexpand(true);
            label.set_halign(gtk4::Align::Center);
            label.set_valign(gtk4::Align::Center);

            container.append(&label);
            obj.append(&container);

            let container_css = gtk4::CssProvider::new();
            container
                .style_context()
                .add_provider(&container_css, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
            let label_css = gtk4::CssProvider::new();
            label
                .style_context()
                .add_provider(&label_css, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

            *self.container.borrow_mut() = Some(container);
            *self.label.borrow_mut() = Some(label);
            *self.container_css.borrow_mut() = Some(container_css);
            *self.label_css.borrow_mut() = Some(label_css);
        }
    }
    impl WidgetImpl for TableCell {}
    impl BoxImpl for TableCell {}
}

glib::wrapper! {
    pub struct TableCell(ObjectSubclass<imp::TableCell>)
        @extends gtk4::Widget, gtk4::Box;
}

impl TableCell
{
    pub fn new() -> Self
    {
        return glib::Object::new::<Self>();
    }

    pub fn from(data: TableCellData) -> Result<Self, String>
    {
        let cell = Self::new();
        if let Err(e) = cell.set_data(data)
        {
            return Err(format!("fail to set data for cell: \n\t{e}"));
        };
        return Ok(cell);
    }

    pub fn set_data(&self, data: TableCellData) -> Result<(), String>
    {
        let label = match data.label
        {
            Some(text) => text,
            None => String::new(),
        };
        self.set_label(&label)?;

        self.set_colors(data.colors)?;

        return Ok(());
    }

    pub fn set_label(&self, text: &String) -> Result<(), String>
    {
        use gtk4::glib::subclass::types::ObjectSubclassExt;

        let imp = imp::TableCell::from_obj(self);
        let label_ref = imp.label.borrow();
        let label = label_ref
            .as_ref()
            .ok_or_else(|| "TableCell label not initialized".to_string())?;

        label.set_text(&text);

        return Ok(());
    }

    pub fn set_colors(&self, colors: TableCellColors) -> Result<(), String>
    {
        let imp = imp::TableCell::from_obj(self);

        let container_css_ref = imp.container_css.borrow();
        let container_css = container_css_ref
            .as_ref()
            .ok_or_else(|| "TableCell container css not initialized".to_string())?;
        let label_css_ref = imp.label_css.borrow();
        let label_css = label_css_ref
            .as_ref()
            .ok_or_else(|| "TableCell label css not initialized".to_string())?;

        let border_color = match gdk::RGBA::parse(&colors.border_color)
        {
            Ok(rgba) => rgba.to_string(),
            Err(e) =>
            {
                eprintln!("fail to parse border color for TableCell {self:?}: {e:?}");
                String::from("rgba(0,0,0,0)")
            }
        };

        let is_transparent = colors.background_color == "rgba(0,0,0,0)"
            && colors.border_color == "rgba(0,0,0,0)"
            && colors.label_color == "rgba(0,0,0,0)";

        let background_css_text = match gdk::RGBA::parse(&colors.background_color)
        {
            Ok(rgba) =>
            {
                if is_transparent
                {
                    "* { background-color: transparent; border: 2px solid transparent; padding: 0; }".to_string()
                }
                else
                {
                    format!(
                        "* {{ background-color: {}; border: 2px solid {}; padding: 8px 10px; }}",
                        rgba.to_string(),
                        border_color
                    )
                }
            }
            Err(e) =>
            {
                eprintln!("fail to parse background color for TableCell {self:?}: {e:?}");
                "* { background-color: transparent; border: 2px solid transparent; padding: 0; }"
                    .to_string()
            }
        };

        let label_css_text = match gdk::RGBA::parse(&colors.label_color)
        {
            Ok(rgba) => format!(
                "* {{ color: {}; font-size: 13px; font-weight: 620; }}",
                rgba.to_string()
            ),
            Err(e) =>
            {
                eprintln!("fail to parse label color for TabelCell {self:?}: {e:?}");
                "* { color: inherit; font-size: 13px; font-weight: 620; }".to_string()
            }
        };

        container_css.load_from_data(&background_css_text);
        label_css.load_from_data(&label_css_text);

        return Ok(());
    }
}
