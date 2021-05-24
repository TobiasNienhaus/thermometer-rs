use std::os::raw::c_uint;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

pub enum CamError {
    RasCam(rascam::CameraError),
    Io(std::io::Error)
}

impl From<rascam::CameraError> for CamError {
    fn from(e: rascam::CameraError) -> Self {
        CamError::RasCam(e)
    }
}

impl From<std::io::Error> for CamError {
    fn from(e: std::io::Error) -> Self {
        CamError::Io(e)
    }
}

pub enum Encoding {
    // TODO more types
    Gif,
    Jpeg,
    Png
}

impl Encoding {
    fn to_c_uint(&self) -> c_uint {
        match self {
            Encoding::Gif => rascam::MMAL_ENCODING_GIF,
            Encoding::Jpeg => rascam::MMAL_ENCODING_JPEG,
            Encoding::Png => rascam::MMAL_ENCODING_PNG,
        }
    }
}

pub struct Camera {
    cam_info: rascam::CameraInfo, // TODO replace with serious cam
}

impl Camera {
    pub fn take_and_save(&self, path: &PathBuf, filename: &str) -> Result<(), CamError> {
        let mut file = path.clone();
        file.push(filename);
        file.set_extension("jpg"); // TODO get from requested

        let mut cam = rascam::SimpleCamera::new(self.cam_info.clone())?;

        cam.activate()?;

        // TODO some other non random const value
        std::thread::sleep(std::time::Duration::from_millis(2000));

        let b = cam.take_one()?;
        File::create(&file)?.write_all(&b)?;
        Ok(())
    }
}

pub fn init() -> Result<Vec<Camera>, rascam::CameraError> {
    let info = rascam::info()?;
    info.cameras
        .into_iter()
        .map(|cam| Camera { cam_info: cam })
        .collect()
}
