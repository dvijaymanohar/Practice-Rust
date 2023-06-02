extern crate opencv;

use opencv::prelude::*;
use opencv::videoio;

fn main() -> opencv::Result<()> {
    // Initialize the OpenCV environment
    let _ = opencv::core::init_opencv();

    // Open the default camera (index 0)
    let mut camera = videoio::VideoCapture::new_default(0)?;

    // Check if the camera is opened successfully
    if !camera.is_opened()? {
        panic!("Failed to open camera");
    }

    // Create a window to display the camera feed
    highgui::named_window("Camera", 1)?;

    loop {
        let mut frame = Mat::default();

        // Read a frame from the camera
        camera.read(&mut frame)?;

        // Check if the frame is empty
        if frame.size()?.width > 0 {
            highgui::imshow("Camera", &mut frame)?;
        }

        // Wait for a key press (10ms delay)
        if highgui::wait_key(10)? > 0 {
            break;
        }
    }

    // Release the camera and destroy the window
    camera.release()?;
    highgui::destroy_all_windows()?;

    Ok(())
}
