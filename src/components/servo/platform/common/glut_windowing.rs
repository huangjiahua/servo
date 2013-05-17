/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! A windowing implementation using GLUT.
///
/// GLUT is a very old and bare-bones toolkit. However, it has good cross-platform support, at
/// least on desktops. It is designed for testing Servo without the need of a UI.

use windowing::{ApplicationMethods, CompositeCallback, LoadUrlCallback, ResizeCallback};
use windowing::{WindowMethods};

use alert::{Alert, AlertMethods};
use geom::size::Size2D;
use glut::glut::{DOUBLE, WindowHeight, WindowWidth};
use glut::glut;

/// A structure responsible for setting up and tearing down the entire windowing system.
pub struct Application;

impl ApplicationMethods for Application {
    pub fn new() -> Application {
        glut::init();
        glut::init_display_mode(DOUBLE);
        Application
    }
}

/// The type of a window.
pub struct Window {
    glut_window: glut::Window,

    composite_callback: Option<CompositeCallback>,
    resize_callback: Option<ResizeCallback>,
    load_url_callback: Option<LoadUrlCallback>,
}

impl WindowMethods<Application> for Window {
    /// Creates a new window.
    pub fn new(_: &Application) -> @mut Window {
        // Create the GLUT window.
        let glut_window = glut::create_window(~"Servo");
        glut::reshape_window(glut_window, 800, 600);

        // Create our window object.
        let window = @mut Window {
            glut_window: glut_window,

            composite_callback: None,
            resize_callback: None,
            load_url_callback: None,
        };

        // Register event handlers.
        do glut::reshape_func(window.glut_window) |width, height| {
            match window.resize_callback {
                None => {}
                Some(callback) => callback(width as uint, height as uint),
            }
        }
        do glut::display_func {
            // FIXME(pcwalton): This will not work with multiple windows.
            match window.composite_callback {
                None => {}
                Some(callback) => callback(),
            }
        }
        do glut::keyboard_func |key, _, _| {
            window.handle_key(key)
        }

        window
    }

    /// Returns the size of the window.
    pub fn size(&self) -> Size2D<f32> {
        Size2D(glut::get(WindowWidth) as f32, glut::get(WindowHeight) as f32)
    }

    /// Presents the window to the screen (perhaps by page flipping).
    pub fn present(&mut self) {
        glut::swap_buffers();
        glut::post_redisplay();
    }

    /// Registers a callback to run when a composite event occurs.
    pub fn set_composite_callback(&mut self, new_composite_callback: CompositeCallback) {
        self.composite_callback = Some(new_composite_callback)
    }

    /// Registers a callback to run when a resize event occurs.
    pub fn set_resize_callback(&mut self, new_resize_callback: ResizeCallback) {
        self.resize_callback = Some(new_resize_callback)
    }

    /// Registers a callback to be run when a new URL is to be loaded.
    pub fn set_load_url_callback(&mut self, new_load_url_callback: LoadUrlCallback) {
        self.load_url_callback = Some(new_load_url_callback)
    }

    /// Spins the event loop.
    pub fn check_loop(@mut self) {
        glut::check_loop()
    }
}

impl Window {
    /// Helper function to handle keyboard events.
    fn handle_key(&self, key: u8) {
        debug!("got key: %d", key as int);
        if key == 12 {  // ^L
            self.load_url()
        }
    }

    /// Helper function to pop up an alert box prompting the user to load a URL.
    fn load_url(&self) {
        match self.load_url_callback {
            None => error!("no URL callback registered, doing nothing"),
            Some(callback) => {
                let mut alert: Alert = AlertMethods::new("Navigate to:");
                alert.add_prompt();
                alert.run();
                let value = alert.prompt_value();
                if "" == value {    // To avoid crashing on Linux.
                    callback("http://purple.com/")
                } else {
                    callback(value)
                }
            }
        }
    }
}

