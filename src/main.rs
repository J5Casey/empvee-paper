use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Orientation};

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("com.github.empvee_paper")
        .build();

    // Connect to "activate" signal of the application
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Empvee Paper")
        .default_width(600)
        .default_height(400)
        .build();


    //Button Box
    let button_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .build();

        
    //Start Button
    let start_button = Button::with_label("Start Wallpaper");
    start_button.connect_clicked(|_| {
        println!("Start button clicked!");
        // We'll add the mpvpaper start code here later
    });

    //Stop Button
    let stop_button = Button::with_label("Stop Wallpaper");
    stop_button.connect_clicked(|_| {
        println!("Stop button clicked!");
        // We'll add the mpvpaper stop code here later
    });


    //Append Buttons
    button_box.append(&start_button);
    button_box.append(&stop_button);


    // Add the button box to the window
    window.set_child(Some(&button_box));

    // Show the window
    window.present();
}
