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

const APP_ID: &str = "org.air.moppler";

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id(APP_ID)
        .flags(ApplicationFlags::HANDLES_OPEN | ApplicationFlags::HANDLES_COMMAND_LINE)
        .build();

    app.connect_command_line(move |app, cmdline| {
        build_ui(&app, cmdline.arguments());
        0
    });
    app.run_with_args(&env::args().collect::<Vec<_>>());
    // Run the application
    app.run();
}

fn build_ui(app: &Application, arguments: Vec<OsString>) {
    let filename = arguments
        .get(1)
        .and_then(|arg| arg.clone().into_string().ok())
        .unwrap();

    let open_file_button = gtk4::Button::from_icon_name("document-open");

    let page_indicator = Label::builder().label("Counting").build();

    let drawing_area = DrawingArea::builder()
        .width_request(100)
        .height_request(100)
        .hexpand(true)
        .vexpand(true)
        .build();

    let app_wrapper = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .build();

    let bottom_bar = gtk4::Box::builder().hexpand_set(true).build();

    bottom_bar.append(&page_indicator);

    let header_bar = gtk4::HeaderBar::builder().build();

    header_bar.pack_start(&open_file_button);
    app_wrapper.append(&drawing_area);
    app_wrapper.append(&bottom_bar);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Pidif")
        .child(&app_wrapper)
        .build();
    window.set_titlebar(Some(&header_bar));

    open_file_button.connect_clicked(clone!(@weak window => move |_button| {
        let filechooser = gtk4::FileChooserDialog::builder()
            .title("Choose a PDF...")
            .action(gtk4::FileChooserAction::Open)
            .modal(true)
            .build();
        filechooser.add_button("_Cancel", gtk4::ResponseType::Cancel);
        filechooser.add_button("_Open", gtk4::ResponseType::Accept);
        filechooser.set_transient_for(Some(&window));
        filechooser.connect_response(|d, response| {
            if response == gtk4::ResponseType::Accept {
                println!("yes!!! {}", &d.file().unwrap().path().unwrap().to_string_lossy() );
               // video.set_file(Some(&d.file().unwrap()));
            }
            d.destroy();
        });
        filechooser.show()
    }));

    let doc = PopplerDocument::new_from_file(filename, "").unwrap();

    let num_pages = doc.get_n_pages();
    let num_pages_ref = Rc::new(RefCell::new(num_pages));

    let current_page = Rc::new(RefCell::new(1));
    let current_page_copy_another = current_page.clone();
    let current_page_view = current_page.clone();

    let drawing_areawidth = drawing_area.content_height();
    let drawing_areaheight = drawing_area.content_width();
    let surface =
        cairo::ImageSurface::create(cairo::Format::Rgb24, drawing_areawidth, drawing_areaheight)
            .unwrap();
    let ctx = Context::new(&surface).unwrap();

    let update_page_status = clone!(@strong num_pages_ref, @strong page_indicator, @strong drawing_area => move || {
        let page_status: String = format!("{} of {}", *current_page_copy_another.borrow_mut(), num_pages);
        // println!("trying to update indi to {}", page_status);
        let page_status_s: &str = &page_status[..];
        page_indicator.set_label(page_status_s);
        drawing_area.queue_draw();
        true
    });

    update_page_status();
    // println!("Document has {} page(s)", num_pages);

    let click = gtk4::GestureClick::new();
    click.set_button(0);
    click.connect_pressed(
        glib::clone!(@weak drawing_area, @strong num_pages, @strong update_page_status => move |_count, _, x, y| {
            // println!("clicked! {} {}", x, y);
            // println!("window {}", drawing_area.width());
            let center = drawing_area.width() / 2;
            if y < (drawing_area.height() / 5) as f64 {
            // println!("toggle fullscreen");
                if header_bar.is_visible() {
                    header_bar.hide();
                    bottom_bar.hide();
                } else {
                    header_bar.show();
                    bottom_bar.show();
                }
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
        // let sur = surface.borrow();
        // Set the label to "Hello World!" after the button has been clicked on
        // let cr = cairo::Pattern::
        let current_page_number = &current_page_view.borrow_mut();
        // println!("current page is {} ", current_page_number);
        context.set_source_rgba(1.0, 1.0, 1.0, 1.0);
        context.paint().unwrap();
        context.fill().expect("uh oh");
        // context.identity_matrix();
        // context.show_page();
        // context.save();
        context.paint().unwrap();

        //        for page_num in 0..num_pages {
        //            println!("{}", next_button.label().unwrap());
        //  if page_num as i32 == **current_page_number - 1 {
        let page = doc.get_page(**current_page_number - 1).unwrap();
        let (w, h) = page.get_size();
        // println!("page has size {}, {}", w, h);
        // println!("sufrface has size {}, {}", area.width(), area.height(),);
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
            Err(v) => println!("error: {v:?}"),
            Ok(_v) => println!("Yay"),
        }

        // println!("Text: {:?}", page.get_text().unwrap_or(""));

        // ctx.restore().unwrap();
        ctx.show_page().unwrap();
        //          }
        // }
    });

    // FIXME: move iterator to poppler
    // g_object_unref (page);
    //surface.write_to_png("file.png");
    // surface.finish();

    // Present window
    window.present();
}
