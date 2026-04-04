pub mod widget
{
    use gtk4::{glib, traits::{BoxExt, GridExt, WidgetExt}};

    use crate::ui::component::table::data::TableData;
    use crate::ui::component::table_cell::widget::TableCell;

    mod imp
    {
        use std::cell::RefCell;

        use gtk4::{glib::{self, subclass::{object::{ObjectImpl, ObjectImplExt}, types::{ObjectSubclass, ObjectSubclassExt}}}, subclass::{box_::BoxImpl, widget::WidgetImpl}, traits::{BoxExt, GridExt, WidgetExt}};

        #[derive(Default)]
        pub struct Table
        {
            pub grid: RefCell<Option<gtk4::Grid>>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for Table
        {
            type Type = super::Table;
            type ParentType = gtk4::Box;

            const NAME: &'static str = "CheckinTable";
        }

        impl ObjectImpl for Table
        {
            fn constructed(&self)
            {
                self.parent_constructed();

                let obj = self.obj();
                obj.set_hexpand(true);
                obj.set_vexpand(true);
                obj.set_halign(gtk4::Align::Fill);
                obj.set_valign(gtk4::Align::Fill);

                let grid = gtk4::Grid::new();
                grid.set_row_spacing(1);
                grid.set_column_spacing(1);
                grid.set_row_homogeneous(true);
                grid.set_column_homogeneous(true);
                grid.set_hexpand(true);
                grid.set_vexpand(true);
                grid.set_halign(gtk4::Align::Fill);
                grid.set_valign(gtk4::Align::Fill);

                obj.append(&grid);

                *self.grid.borrow_mut() = Some(grid);
            }
        }
        impl WidgetImpl for Table {}
        impl BoxImpl for Table {}
    }

    glib::wrapper!
    {
        pub struct Table(ObjectSubclass<imp::Table>)
            @extends gtk4::Widget, gtk4::Box;
    }

    impl Table
    {
        pub fn new()-> Self
        {
            return glib::Object::new::<Self>();
        }

        pub fn set_data(&self, data:TableData)-> Result<(),String>
        {
            use gtk4::glib::subclass::types::ObjectSubclassExt;

            let imp = imp::Table::from_obj(self);
            let grid_ref = imp.grid.borrow();
            let grid = grid_ref
                .as_ref()
                .ok_or_else(|| "Table grid not initialized".to_string())?;

            while let Some(child) = grid.first_child()
            {
                grid.remove(&child);
            }

            for (y, row) in data.rows().iter().enumerate()
            {
                for (x, cell_data) in row.iter().enumerate()
                {
                    let cell = TableCell::new();
                    cell.set_data(cell_data.clone())?;
                    grid.attach(&cell, x as i32, y as i32, 1, 1);
                }
            }

            Ok(())
        }
    }
}

pub mod data
{
    use crate::ui::component::table_cell::data::TableCellData;

    #[derive(Debug)]
    pub struct TableData
    {
        rows:Vec<Vec<TableCellData>>
    }

    impl TableData
    {
        pub fn new(rows:Vec<Vec<TableCellData>>)-> Self
        {
            return Self { rows };
        }

        pub fn rows(&self)-> &Vec<Vec<TableCellData>>
        {
            return &self.rows;
        }
    }
}
