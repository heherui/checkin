mod widget
{
    use gtk4::{gdk, glib, traits::{StyleContextExt, WidgetExt}};

    use crate::ui::component::table_cell::data::TableCellData;

    mod imp
    {
        use std::cell::RefCell;

        use gtk4::{glib::{self, subclass::{object::{ObjectImpl, ObjectImplExt}, types::{ObjectSubclass, ObjectSubclassExt}}}, subclass::{box_::BoxImpl, widget::WidgetImpl}, traits::{BoxExt, WidgetExt}};

        #[derive(Default)]
        pub struct TableCell
        {
            pub container: RefCell<Option<gtk4::Box>>,
            pub label: RefCell<Option<gtk4::Label>>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for TableCell
        {
            type Type = super::TableCell;
            type ParentType = gtk4::Box;
            
            const NAME: &'static str = "TableCell";
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

                *self.container.borrow_mut() = Some(container);
                *self.label.borrow_mut() = Some(label);
            }
        }
        impl WidgetImpl for TableCell {}
        impl BoxImpl for TableCell {}
    }

    glib::wrapper!
    {
        pub struct TableCell(ObjectSubclass<imp::TableCell>)
            @extends gtk4::Widget, gtk4::Box;
    }

    
    impl TableCell
    {
        pub fn new()-> Self
        {
            return glib::Object::new::<Self>();
        }

        pub fn set_data(&self, data:TableCellData)-> Result<(),String>
        {
            use gtk4::glib::subclass::types::ObjectSubclassExt;

            let imp = imp::TableCell::from_obj(self);
            let container_ref = imp.container.borrow();
            let container = container_ref
                .as_ref()
                .ok_or_else(|| "TableCell container not initialized".to_string())?;
            let label_ref = imp.label.borrow();
            let label = label_ref
                .as_ref()
                .ok_or_else(|| "TableCell label not initialized".to_string())?;

            let text = match &data
            {
                TableCellData::Normal { person } => person.name.clone(),
                TableCellData::Block { label, .. } => label.clone().unwrap_or_default(),
            };

            let cell_bg = match &data
            {
                TableCellData::Block { cell_background_color, .. } => Some(cell_background_color.as_str()),
                _ => None,
            };

            let label_color = match &data
            {
                TableCellData::Block { label_display_color, .. } => Some(label_display_color.as_str()),
                _ => None,
            };

            label.set_text(&text);

            if let Some(color) = cell_bg
            {
                if let Ok(rgba) = gdk::RGBA::parse(color)
                {
                    let css = format!("* {{ background-color: {}; }}", rgba.to_string());
                    let provider = gtk4::CssProvider::new();
                    provider.load_from_data(&css);
                    container
                        .style_context()
                        .add_provider(&provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
                }
            }

            if let Some(color) = label_color
            {
                if let Ok(rgba) = gdk::RGBA::parse(color)
                {
                    let css = format!("* {{ color: {}; }}", rgba.to_string());
                    let provider = gtk4::CssProvider::new();
                    provider.load_from_data(&css);
                    label
                        .style_context()
                        .add_provider(&provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
                }
            }

            Ok(())
        }
    }
}

mod data 
{
    use crate::core::Person;

    #[derive(Debug, Clone)]
    pub enum TableCellData
    {
        Normal 
        {
            person:Person
        },

        /// A cell cannot be checked, for view only.
        Block 
        {
            cell_background_color:String,
            label_display_color:String,
            label:Option<String>,
        },
    }
}
