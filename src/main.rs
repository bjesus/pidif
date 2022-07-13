use cairo::Context;
use gio::ApplicationFlags;
use glib::clone;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, DrawingArea, Label};
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

    // Connect to "activate" signal of `app`
    // app.connect_activate(|app| build_ui(app, None));

    // app.connect_open(move |app, files, _| build_ui(app, Some(&files[0])));
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
    // Create a button with label and margins
    let button = Button::builder()
        .label("load file")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let page_indicator = Label::builder().label("Counting").build();
    let back_button = Button::builder().label("Back").build();
    let next_button = Button::builder().label("Next").build();

    let pagew = DrawingArea::builder()
        .width_request(100)
        .height_request(100)
        .hexpand(true)
        .vexpand(true)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    let mb = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let buttom_bar = gtk::Box::builder()
        .hexpand_set(true)
        // .orientation(gtk::Orientation::Vertical)
        .build();

    buttom_bar.append(&back_button);
    buttom_bar.append(&page_indicator);
    buttom_bar.append(&next_button);

    mb.append(&button);
    mb.append(&pagew);
    mb.append(&buttom_bar);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Moppler")
        .child(&mb)
        .build();
    let doc = PopplerDocument::new_from_file(filename, "").unwrap();

    let num_pages = doc.get_n_pages();

    let num_pages_ref = Rc::new(RefCell::new(num_pages));

    let current_page = Rc::new(RefCell::new(1));
    let current_page_copy = current_page.clone();
    let current_page_copy_another = current_page.clone();
    let current_page_view = current_page.clone();

    let pagewwidth = pagew.content_height();
    let pagewheight = pagew.content_width();
    println!(
        "setting pagew width {} and height {}",
        pagewwidth, pagewheight,
    );
    let surface =
        cairo::ImageSurface::create(cairo::Format::Rgb24, pagewwidth, pagewheight).unwrap();
    let ctx = Context::new(&surface).unwrap();

    let update_page_status = clone!(@strong num_pages_ref, @strong page_indicator, @strong pagew => move || {
           let page_status: String = format!("{} of {}", *current_page_copy_another.borrow_mut(), num_pages);
           println!("trying to update indi to {}", page_status);
           let page_status_s: &str = &page_status[..];
           page_indicator.set_label(page_status_s);
    // ctx.paint();
    //     ctx.fill();
    //     surface.flush();
    //     // surface.finish();
        pagew.queue_draw();
        true
       });

    next_button.connect_clicked(clone!(@strong update_page_status => move |_| {
        *current_page.borrow_mut() += 1;
        update_page_status();
    }));

    back_button.connect_clicked(clone!(@strong update_page_status => move |_| {
        println!("change page down from {}", *current_page_copy.borrow_mut());
        *current_page_copy.borrow_mut() -= 1;
        update_page_status();
    }));
    update_page_status();
    println!("Document has {} page(s)", num_pages);

    // set_page_status();

    // let surface = cairo::PdfSurfac::new(420.0, 595.0, "output.pdf").unwrap();
    pagew.set_draw_func(move |_area, context, _a, _b| {
        // let sur = surface.borrow();
        // Set the label to "Hello World!" after the button has been clicked on
        // let cr = cairo::Pattern::
        let current_page_number = &current_page_view.borrow_mut();
        println!("current page is {} ", current_page_number);
        println!("drawn");
        context.set_source_rgba(1.0, 1.0, 1.0, 1.0);
        context.paint().unwrap();
        context.fill().expect("uh oh");
        // context.identity_matrix();
        // context.show_page();
        // context.save();
        context.paint().unwrap();

        for page_num in 0..num_pages {
            println!("{}", next_button.label().unwrap());
            if page_num as i32 == **current_page_number - 1 {
                let page = doc.get_page(page_num).unwrap();
                let (w, h) = page.get_size();
                println!("page {} has size {}, {}", page_num, w, h);
                println!("sufrface has size {}, {}", pagewwidth, pagewheight);
                // surface.set_size(w, h).unwrap();
                context.save().unwrap();
                context.scale(2.0, 2.0);
                page.render(&context);

                let r = ctx.paint();
                match r {
                    Err(v) => println!("error: {v:?}"),
                    Ok(_v) => println!("Yay"),
                }

                // println!("Text: {:?}", page.get_text().unwrap_or(""));

                // ctx.restore().unwrap();
                ctx.show_page().unwrap();
            }
        }
    });

    // FIXME: move iterator to poppler
    // g_object_unref (page);
    //surface.write_to_png("file.png");
    // surface.finish();

    // Present window
    window.present();
}
