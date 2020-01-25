use crate::event::MouseEvent;
use crate::renderer::WGPURenderer;
use winit::event;
use winit::event::{DeviceEvent, WindowEvent};

pub struct WindowState {
  size: (f32, f32),
  physical_size: (f32, f32),
  hidpi_factor: f32,
  mouse_position: (f32, f32),
}

impl WindowState {
  pub fn update_size(&mut self, size: &winit::dpi::LogicalSize) {
    self.size.0 = size.width as f32;
    self.size.1 = size.height as f32;
    let physical = size.to_physical(self.hidpi_factor as f64);
    self.physical_size.0 = physical.width as f32;
    self.physical_size.1 = physical.height as f32;
  }

  pub fn mouse_move_to(&mut self, position: &winit::dpi::LogicalPosition) {
    self.mouse_position.0 = position.x as f32;
    self.mouse_position.1 = position.y as f32;
  }
}

pub struct Window<AppState> {
  pub window_state: WindowState,
  click_listeners: Vec<Box<dyn FnMut(MouseEvent, &mut AppState)>>,
  resize_listeners: Vec<Box<dyn FnMut(&mut AppState, &mut WGPURenderer)>>,
}

impl<AppState> Window<AppState> {
  pub fn listener<T: FnMut(MouseEvent, &mut AppState) + 'static>(&mut self, func: T) {
    self.click_listeners.push(Box::new(func));
  }

  pub fn add_resize_listener<T: FnMut(&mut AppState, &mut WGPURenderer) + 'static>(
    &mut self,
    func: T,
  ) {
    self.resize_listeners.push(Box::new(func));
  }
  pub fn emit_resize(&mut self, state: &mut AppState, renderer: &mut WGPURenderer) {
    for listener in self.resize_listeners.iter_mut() {
      listener(state, renderer)
    }
  }

  pub fn new(size: (f32, f32), hidpi_factor: f32) -> Self {
    Window {
      window_state: WindowState {
        size,
        physical_size: (size.0 * hidpi_factor, size.1 * hidpi_factor),
        hidpi_factor,
        mouse_position: (0.0, 0.0),
      },
      click_listeners: Vec::new(),
      resize_listeners: Vec::new(),
    }
  }

  pub fn event(
    &mut self,
    event: winit::event::Event<()>,
    state: &mut AppState,
    renderer: &mut WGPURenderer,
  ) {
    match event {
      event::Event::WindowEvent { event, .. } => match event {
        WindowEvent::Resized(size) => {
          self.window_state.update_size(&size);
          self.emit_resize(state, renderer);
          log::info!("Resizing to {:?}", size);
        }
        WindowEvent::MouseInput { button, state, .. } => {
          println!("mouse click");
          // for listener in self.click_listeners.iter_mut() {
          //   listener(MouseEvent { x: 1.0, y: 1.0 }, &mut self.app_state)
          // }
        }
        WindowEvent::CursorMoved { position, .. } => {
          self.window_state.mouse_move_to(&position);
          println!("mouse move");
        }
        _ => (),
      },
      DeviceEvent => {}
    }
  }
}
