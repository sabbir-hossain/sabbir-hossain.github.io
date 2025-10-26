use winit::keyboard::KeyCode;

#[derive(Debug, PartialEq, Clone)]
pub enum InputOption {
  InsertInput,
  Start,
  ClearInput,
  BlockInput,
}

#[derive(Debug, Clone)]
pub struct KeyboardInput {
  pub input: String,
  pub current_key: InputOption,  
}

impl KeyboardInput {
    pub fn handle_input(&mut self, physical_key: KeyCode ) {
      match physical_key {
        KeyCode::Enter | KeyCode::NumpadEnter => {
          self.current_key = InputOption::Start;
        }
        KeyCode::F9 => {
          self.current_key = InputOption::ClearInput;
          self.input.clear();
        }

        KeyCode::Digit0 => self.input.push('0'),
        KeyCode::Digit1 => self.input.push('1'),
        KeyCode::Digit2 => self.input.push('2'),
        KeyCode::Digit3 => self.input.push('3'),
        KeyCode::Digit4 => self.input.push('4'),
        KeyCode::Digit5 => self.input.push('5'),
        KeyCode::Digit6 => self.input.push('6'),
        KeyCode::Digit7 => self.input.push('7'),
        KeyCode::Digit8 => self.input.push('8'),
        KeyCode::Digit9 => self.input.push('9'),
        KeyCode::Period => self.input.push('.'),

        KeyCode::Numpad0 => self.input.push('0'),
        KeyCode::Numpad1 => self.input.push('1'),
        KeyCode::Numpad2 => self.input.push('2'),
        KeyCode::Numpad3 => self.input.push('3'),
        KeyCode::Numpad4 => self.input.push('4'),
        KeyCode::Numpad5 => self.input.push('5'),
        KeyCode::Numpad6 => self.input.push('6'),
        KeyCode::Numpad7 => self.input.push('7'),
        KeyCode::Numpad8 => self.input.push('8'),
        KeyCode::Numpad9 => self.input.push('9'),
        KeyCode::NumpadDecimal => self.input.push('.'),

        KeyCode::Backspace => {
            self.input.pop();
            () // Explicit unit return
        }

        _ => {
            log::info!("Invalid key pressed: {:?}", physical_key);
        }
        
      }
    }

    pub fn as_f32(&self) -> Option<f32> {
        self.input.parse::<f32>().ok()
    }
}