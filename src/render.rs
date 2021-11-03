use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct RenderState {
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
}

#[derive(Debug)]
pub enum RenderError {
    FailedAdapterCreate,
    FailedDeviceRequest(wgpu::RequestDeviceError),
}

impl Display for RenderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match &self {
            RenderError::FailedAdapterCreate => write!(f, "Failed to create adapter"),
            RenderError::FailedDeviceRequest(error) => {
                write!(f, "Device request failed: {}", error)
            }
        }
    }
}

impl std::error::Error for RenderError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            RenderError::FailedAdapterCreate => None,
            RenderError::FailedDeviceRequest(ref error) => Some(error),
        }
    }
}

impl std::convert::From<wgpu::RequestDeviceError> for RenderError {
    fn from(e: wgpu::RequestDeviceError) -> Self {
        RenderError::FailedDeviceRequest(e)
    }
}

impl RenderState {
    pub async fn new(window: &winit::window::Window) -> Result<Self, RenderError> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter: wgpu::Adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(RenderError::FailedAdapterCreate)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await?;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        Ok(Self {
            surface,
            adapter,
            device,
            queue,
            config,
            size,
        })
    }
}
