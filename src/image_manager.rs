use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use image;
use image::{DynamicImage, FilterType};

use pix::canvas::Canvas;



/// A manager that manages all images to print.
pub struct ImageManager {
    images: Vec<DynamicImage>,
    index: isize,
}

impl ImageManager {
    /// Intantiate the image manager.
    pub fn from(images: Vec<DynamicImage>) -> ImageManager {
        ImageManager {
            images,
            index: 0,
        }
    }

    /// Instantiate the image manager, and load the images from the given paths.
    pub fn load(paths: Vec<&str>, size: &(u32, u32)) -> ImageManager {
        // Show a status message
        println!("Load and process {} image(s)...", paths.len());

        // Load the images from the paths
        let image_manager = ImageManager::from(
            paths.iter()
                .map(|path| load_image(path, &size))
                .collect()
        );

        // We succeeded
        println!("All images have been loaded successfully");

        image_manager
    }

    /// Tick the image 
    pub fn tick(&mut self, canvas: &mut Canvas) {
        // Get the image index bound
        let bound = self.images.len();

        // Get the image to use
        let image = &mut self.images[
            self.index as usize % bound
        ];

        // Update the image on the canvas
        canvas.update_image(image);

        // Increase the index
        self.index += 1;
    }

    /// Start working in the image manager.
    ///
    /// This will start walking through all image frames,
    /// and pushes each frame to all painters,
    /// with the specified frames per second.
    pub fn work(&mut self, canvas: &mut Canvas, fps: u32) {
        loop {
            // Tick to use the next image
            self.tick(canvas);

            // Sleep until we need to show the next image
            sleep(Duration::from_millis(
                (1000f32 / (fps as f32)) as u64
            ));
        }
    }
}



/// Load the image at the given path, and size it correctly
fn load_image(path: &str, size: &(u32, u32)) -> DynamicImage {
    // Create a path instance
    let path = Path::new(&path);

    // Check whether the path exists
    if !path.is_file() {
        panic!("The given path does not exist or is not a file");
    }

    // Load the image
    let image = image::open(&path).unwrap();

    // Resize the image to fit the screen
    image.resize_exact(
        size.0,
        size.1,
        FilterType::Gaussian,
    )
}
