use cairo::Context;
use gio::ApplicationFlags;
use glib::clone;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, DrawingArea, Label};
use poppler::PopplerDocument;
use std::cell::RefCell;
use std::env;
use std::ffi::OsString;
use std::rc::Rc;

const APP_ID: &str = "com.yoavmoshe.pidif";

fn main() {
    let app = Application::builder()
        .application_id(APP_ID)
        .flags(ApplicationFlags::HANDLES_OPEN | ApplicationFlags::HANDLES_COMMAND_LINE)
        .build();

    app.connect_command_line(move |app, cmdline| {
        build_ui(&app, cmdline.arguments());
        0
    });
    app.run_with_args(&env::args().collect::<Vec<_>>());
    app.run();
}

fn build_ui(app: &Application, arguments: Vec<OsString>) {
    let filename = arguments
        .get(1)
        .and_then(|arg| arg.clone().into_string().ok());

    let open_file_button = gtk4::Button::from_icon_name("document-open");

    let app_wrapper = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .build();

    let bottom_bar = gtk4::Box::builder().hexpand_set(true).build();
    let header_bar = gtk4::HeaderBar::builder().build();

    header_bar.pack_start(&open_file_button);
    app_wrapper.append(&bottom_bar);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Pidif")
        .child(&app_wrapper)
        .build();
    window.set_titlebar(Some(&header_bar));

    let toggle_fullscreen = clone!(@weak header_bar, @weak bottom_bar => move || {
        if header_bar.is_visible() {
            header_bar.hide();
            bottom_bar.hide();
        } else {
            header_bar.show();
            bottom_bar.show();
        }
    });

    let load_doc = move |filename: String| {
        let drawing_area = DrawingArea::builder()
            .width_request(100)
            .height_request(100)
            .hexpand(true)
            .vexpand(true)
            .build();
        let first_child = app_wrapper.first_child().unwrap();
        let last_child = app_wrapper.last_child().unwrap();
        if &first_child != &last_child {
            app_wrapper.remove(&first_child);
        }

        app_wrapper.prepend(&drawing_area);

        let page_indicator = Label::builder().label("Counting").build();
        let old_indicator = bottom_bar.last_child();
        if old_indicator.is_some() {
            bottom_bar.remove(&old_indicator.unwrap());
        }
        bottom_bar.append(&page_indicator);

        let doc = PopplerDocument::new_from_file(filename, "").unwrap();

        let num_pages = doc.get_n_pages();
        let num_pages_ref = Rc::new(RefCell::new(num_pages));

        let current_page = Rc::new(RefCell::new(1));
        let current_page_copy_another = current_page.clone();
        let current_page_view = current_page.clone();

        let surface = cairo::ImageSurface::create(cairo::Format::Rgb24, 0, 0).unwrap();
        let ctx = Context::new(&surface).unwrap();

        let update_page_status = clone!(@strong num_pages_ref, @weak page_indicator, @strong drawing_area => move || {
            let page_status: String = format!("{} of {}", *current_page_copy_another.borrow_mut(), num_pages);
            let page_status_s: &str = &page_status[..];
            page_indicator.set_label(page_status_s);
            drawing_area.queue_draw();

        });

        update_page_status();

        let click = gtk4::GestureClick::new();
        click.set_button(0);
        click.connect_pressed(
                 glib::clone!(@weak drawing_area, @strong toggle_fullscreen, @strong num_pages, @strong update_page_status => move |_count, _, x, y| {
                     let center = drawing_area.width() / 2;
                     if y < (drawing_area.height() / 5) as f64 {
                     toggle_fullscreen();
                     } else if x > center as f64 &&  *current_page.borrow_mut() < num_pages {
                        *current_page.borrow_mut() += 1;
                     } else if x < center as f64 && *current_page.borrow_mut() > 1 {
                        *current_page.borrow_mut()  -= 1;
                     }
                     update_page_status();

                 }),
             );

        drawing_area.add_controller(&click);

        drawing_area.set_draw_func(move |area, context, _a, _b| {
            let current_page_number = &current_page_view.borrow_mut();
            context.set_source_rgba(1.0, 1.0, 1.0, 1.0);
            context.paint().unwrap();
            context.fill().expect("uh oh");
            context.paint().unwrap();

            let page = doc.get_page(**current_page_number - 1).unwrap();
            let (w, h) = page.get_size();
            let width_diff = area.width() as f64 / w;
            let height_diff = area.height() as f64 / h;
            context.save().unwrap();
            if width_diff > height_diff {
                context.scale(height_diff, height_diff);
            } else {
                context.scale(width_diff, width_diff);
            }
            page.render(&context);

            let r = ctx.paint();
            match r {
                Err(v) => println!("Error painting PDF: {v:?}"),
                Ok(_v) => println!(""),
            }

            ctx.show_page().unwrap();
        });
    };

    if filename.is_some() {
        load_doc(filename.unwrap());
    }

    open_file_button.connect_clicked(clone!(@weak window, @strong load_doc => move |_button| {
        let filechooser = gtk4::FileChooserDialog::builder()
            .title("Choose a PDF...")
            .action(gtk4::FileChooserAction::Open)
            .modal(true)
            .build();
        filechooser.add_button("_Cancel", gtk4::ResponseType::Cancel);
        filechooser.add_button("_Open", gtk4::ResponseType::Accept);
        filechooser.set_transient_for(Some(&window));
        filechooser.connect_response(clone!(@strong load_doc => move |d, response| {
            if response == gtk4::ResponseType::Accept {
                let path = d.file().unwrap().path().unwrap().into_os_string().into_string().unwrap();
                load_doc(path);
            }
            d.destroy();
        }));
        filechooser.show()
    }));
    window.present();
}
