use crate::rinecraft::Rinecraft;
use rendiation::WindowEventSession;
use crate::rinecraft::RinecraftState;
use rendiation_math::*;
use rendiation_render_entity::*;

impl Rinecraft{
  pub fn use_orbit_controller(&mut self){
    self.window_session.add_mouse_motion_listener(|state, _| {
      if state.window_state.is_left_mouse_down {
        state.orbit_controller.rotate(Vec2::new(
          -state.window_state.mouse_motion.0,
          -state.window_state.mouse_motion.1,
        ))
      }
      if state.window_state.is_right_mouse_down {
        state.orbit_controller.pan(Vec2::new(
          -state.window_state.mouse_motion.0,
          -state.window_state.mouse_motion.1,
        ))
      }
    });
    self.window_session.add_mouse_wheel_listener(|state, _| {
      let delta = state.window_state.mouse_wheel_delta.1;
      state.orbit_controller.zoom(1.0 - delta * 0.1);
    });
  }
}

pub fn init_orbit_controller(window_session: &mut WindowEventSession<RinecraftState>) {
  window_session.add_mouse_motion_listener(|state, _| {
    if state.window_state.is_left_mouse_down {
      state.orbit_controller.rotate(Vec2::new(
        -state.window_state.mouse_motion.0,
        -state.window_state.mouse_motion.1,
      ))
    }
    if state.window_state.is_right_mouse_down {
      state.orbit_controller.pan(Vec2::new(
        -state.window_state.mouse_motion.0,
        -state.window_state.mouse_motion.1,
      ))
    }
  });
  window_session.add_mouse_wheel_listener(|state, _| {
    let delta = state.window_state.mouse_wheel_delta.1;
    state.orbit_controller.zoom(1.0 - delta * 0.1);
  });
}

pub fn init_world(window_session: &mut WindowEventSession<RinecraftState>) {
  window_session.add_mouse_down_listener(|state, _| {
    
  });
}
