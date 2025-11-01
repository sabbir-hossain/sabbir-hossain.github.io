use std::rc::Rc;
use std::vec;
use log;
  
use crate::option::{ViewConfig, ViewObject};
use crate::utils::color::{BACKGROUND_COLOR, get_random_color};
use crate::utils::{camera, constant};
use crate::utils::vertex::Vertex;

use super::a_capital::A;
use super::b_capital::B;
use super::c_capital::C;
use super::d_capital::D;
use super::e_capital::E;
use super::f_capital::F;
use super::g_capital::G;
use super::h_capital::H;
use super::i_capital::I;
use super::j_capital::J;
use super::k_capital::K;
use super::l_capital::L;
use super::m_capital::M;
use super::n_capital::N;
use super::o_capital::O;
use super::p_capital::P;
use super::q_capital::Q;
use super::r_capital::R;
use super::s_capital::S;
use super::t_capital::T;
use super::u_capital::U;
use super::v_capital::V;
use super::w_capital::W;
use super::x_capital::X;
use super::y_capital::Y;
use super::z_capital::Z;

// use super::zero::Zero;
use super::one::One;
use super::two::Two;
use super::three::Three;
use super::four::Four;
use super::five::Five;
use super::six::Six;
use super::seven::Seven;
use super::eight::Eight;
use super::nine::Nine;
use super::dot::Dot;
use super::divide::Divide;
use super::minus::Minus;
use super::plus::Plus;
use super::equal::Equal;
use super::multiply::Multiply;
use super::first_bracket_left::FirstBracketLeft;
use super::first_bracket_right::FirstBracketRight;

use super::comma::Comma;
use super::single_quote::SingleQuote;
// use super::exclamation::Exclamation;
// use super::question::Question;
// use super::less::Less;
// use super::greater::Greater;


#[derive(Debug, Clone)]
pub struct Text {
  vertices: Vec<Vertex>,
  indices: Vec<u16>,
  indices_len: u32,
 
  pub content: String,
  pub color: [f32; 3],
  pub position: (f32, f32),
  pub line_position: (f32, f32),
  pub init_position: (f32, f32),
  pub previous_length: u16,
  pub unit_x: f32,
  pub unit_y: f32,
  
  // pub font_x: f32,
  pub font_size: u16,
  pub font_y: f32,
  pub font_wide_h: f32,
  pub font_wide_v: f32,

  pub font_space: f32,
  is_pow_enable: bool,
  pub is_vertical: bool,
  
  pub a: A,
  pub b: B,
  pub c: C,
  pub d: D,
  pub e: E,
  pub f: F,
  pub g: G,
  pub h: H,
  pub i: I,
  pub j: J,
  pub k: K,
  pub l: L,
  pub m: M,
  pub n: N,
  pub o: O,
  pub p: P,
  pub q: Q,
  pub r: R,
  pub s: S,
  pub t: T,
  pub u: U,
  pub v: V,
  pub w: W,
  pub x: X,
  pub y: Y,
  pub z: Z,
  
  pub zero: O,
  pub one: One,
  pub two: Two,
  pub three: Three,
  pub four: Four,
  pub five: Five,
  pub six: Six,
  pub seven: Seven,
  pub eight: Eight,
  pub nine: Nine,
  pub dot: Dot,
  pub divide: Divide,
  pub minus: Minus,
  pub plus: Plus,
  pub equal: Equal,
  pub multiply: Multiply,
  pub first_bracket_left: FirstBracketLeft,
  pub first_bracket_right: FirstBracketRight,
  pub comma: Comma,
  pub single_quote: SingleQuote,
  // pub exclamation: Exclamation,
  // pub question: Question,
  // pub less: Less,
  // pub greater: Greater,
}

impl Text {
  
  // , font_size: u32, color: [f32; 3], position: (f32, f32)
  pub fn new(
    content: String, 
    view_config: Rc<ViewConfig>, 
    position: (f32, f32),
    font_size: Option<u16>, 
    color: Option<[f32; 3]>, 
    is_vertical: bool,
  ) -> Self {

    let unit_x: f32 = view_config.unit_x;
    let unit_y: f32 = view_config.unit_y;

    // let font_size: u16 = 288;
    // let font_size: u16 = 128;
    // let font_size: u16 = 96;
    // let font_size: u16 = 48;
    // let font_size: u16 = 24;
    // let font_size: u16 = 18;
    let font_ratio: f32 = constant::TOTAL / view_config.total_height as f32; // This is a placeholder value, adjust as needed
    let font_wide_h: f32 = 0.18;
    let font_wide_v: f32 = 0.11;
    let font_space: f32 = 0.2;

    let font_size = font_size.unwrap_or(16);
    let color = color.unwrap_or(get_random_color());

    let a = A::new( view_config.clone() );
    let b = B::new( view_config.clone() );
    let c = C::new( view_config.clone() );
    let d = D::new( view_config.clone() );
    let e = E::new( view_config.clone() );
    let f = F::new( view_config.clone() );
    let g = G::new( view_config.clone() );
    let h = H::new( view_config.clone() );
    let i = I::new( view_config.clone() );
    let j = J::new( view_config.clone() );
    let k = K::new( view_config.clone() );
    let l = L::new( view_config.clone() );
    let m = M::new( view_config.clone() );
    let n = N::new( view_config.clone() );
    let o = O::new( view_config.clone() );
    let p = P::new( view_config.clone() );
    let q = Q::new( view_config.clone() );
    let r = R::new( view_config.clone() );
    let s = S::new( view_config.clone() );
    let t = T::new( view_config.clone() );
    let u = U::new( view_config.clone() );
    let v = V::new( view_config.clone() );
    let w = W::new( view_config.clone() );
    let x = X::new( view_config.clone() );
    let y = Y::new( view_config.clone() );
    let z = Z::new( view_config.clone() );
    let zero = O::new( view_config.clone() );
    let one = One::new( view_config.clone() );
    let two = Two::new( view_config.clone() );
    let three = Three::new( view_config.clone() );
    let four = Four::new( view_config.clone() );
    let five = Five::new( view_config.clone() );
    let six = Six::new( view_config.clone() );
    let seven = Seven::new( view_config.clone() );
    let eight = Eight::new( view_config.clone() );
    let nine = Nine::new( view_config.clone() );
    let dot = Dot::new( view_config.clone() );
    let divide = Divide::new( view_config.clone() );
    let minus = Minus::new( view_config.clone() );
    let plus = Plus::new( view_config.clone() );
    let equal = Equal::new( view_config.clone() );
    let multiply = Multiply::new( view_config.clone() );
    let first_bracket_left = FirstBracketLeft::new(view_config.clone());
    let first_bracket_right = FirstBracketRight::new(view_config.clone());
    let comma = Comma::new( view_config.clone() );
    let single_quote = SingleQuote::new( view_config.clone() );
    // let exclamation = Exclamation::new( view_config.clone() );
    // let question = Question::new( view_config.clone() );
    // let less = Less::new( view_config.clone() );
    // let greater = Greater::new( view_config.clone() );

    Text {
      // output: vec![],
      vertices: vec![],
      indices: vec![],
      indices_len: 0,
      is_vertical,
      content,
      color,
      position,
      line_position: position,
      init_position: position,
      previous_length: 0,
      
      font_size,
      font_y: font_ratio * font_size as f32,
      font_wide_h,
      font_wide_v,
      font_space,
      is_pow_enable: false,

      unit_x,
      unit_y,

      a,
      b,
      c,
      d,
      e,
      f,
      g,
      h,
      i,
      j,
      k,
      l,
      m,
      n,
      o,
      p,
      q,
      r,
      s,
      t,
      u,
      v,
      w,
      x,
      y,
      z,
      
      zero,
      one,
      two,
      three,
      four,
      five,
      six,
      seven,
      eight,
      nine,
      dot,
      divide,
      minus,
      plus,
      equal,
      multiply,
      first_bracket_left,
      first_bracket_right,

      comma,
      single_quote,
      // exclamation,
      // question,
      // less,
      // greater,
    }
  }

  pub fn draw(&mut self) -> Vec<ViewObject> {
    // Drawing logic for the text
    // log::info!("Drawing text: [{}]", self.content);

    // let s = "BCDGPQ";
    // let s = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456^7 7/8 9.";
    // log::info!("line position 1: ({}, {})", self.line_position.0, self.line_position.1);
    // let mut previous_x: f32 = self.position.0;
    // let mut previous_length: u16 = 0;
    let mut is_line_end: f32 = 0.0;
    let allow_eol_list = [' ', '.', ',', ';', ':', '!', '?'];

    let chars: Vec<char> = self.content.chars().collect();
    let len = chars.len();

    // for c in self.content.clone().chars() {
    for (i, c) in chars.iter().enumerate() {
    // while let Some(c) = char_iter.clone().next() {
      let text_obj = self.get_coordinates(*c);
      // log::info!("Character: [{}] 
      //     pos: [{}, {}] l: {}, 
      //     t: ({}, {})", c, self.position.0, self.position.1, self.previous_length, text_obj.max_x, text_obj.max_y);

      if self.is_vertical && self.is_pow_enable == false {
        self.position.0 =  self.line_position.0;
        self.position.1 -= text_obj.max_y* 1.2;
        self.line_position.1 = self.position.1;
      } else {
        self.position.0 += text_obj.max_x;
        // self.position.0 += (text_obj.max_x + 0.2 * self.unit_x);
        if self.position.0 + text_obj.max_x * 1.25 > 0.9 {
          is_line_end = if i + 1 < len && !allow_eol_list.contains(&chars[i + 1]) { text_obj.max_y* 2.0 } else { 0.0 };

        }
      }

      self.previous_length += text_obj.vertex_len as u16;

      self.vertices.extend(text_obj.vertices.as_ref().clone());
      self.indices.extend(text_obj.indices.as_ref().clone());
      self.indices_len += text_obj.indices_len;

      if is_line_end != 0.0 {
        // log::info!("Line break at character: [{} -> {}]", c, chars[i + 1]);
        // is_line_end = false;
        let text_obj2 = self.get_coordinates('-');
        self.previous_length += text_obj2.vertex_len as u16;
          
        self.vertices.extend(text_obj2.vertices.as_ref().clone());
        self.indices.extend(text_obj2.indices.as_ref().clone());
        self.indices_len += text_obj2.indices_len;
      
        self.position.0 = self.line_position.0;
        self.position.1 -= is_line_end;
        self.line_position.1 = self.position.1;
        is_line_end = 0.0;
      }
    }
        
    // log::info!("line position 2: ({}, {})", self.line_position.0, self.line_position.1);


    let model_matrix = camera::build_transformations(
      constant::TRANSLATION, 
      constant::ROTATION,
      constant::SCALING
    );
    // let texture_matrix = cgmath::Matrix4::from_scale(1.0).into();

    // log::info!("Vertices: {:?}", self.vertices);
    // log::info!("Indices: {:?}", self.indices);
    // log::info!("Indices length: {:?}", self.indices_len);
    let texture_offset = cgmath::Vector2::new(0.0, 0.0);

    vec![
      ViewObject {
        texture_offset,
        model_matrix,
        vertices: Rc::new(self.vertices.clone()), 
        indices: Rc::new(self.indices.clone()),
        indices_len: self.indices_len,
        label: format!("text : [{}]", self.content),
        image_data: None,
        tx_rotation: 0.0,
      }
    ]
  }

  pub fn erase_background(&mut self, color: Option<[f32; 3]>) -> Vec<ViewObject> {
    /*
    log::info!("
      text : {}
      0: ({}, {})
      1: ({}, {})
      2: ({}, {})
      3: ({}, {})
      font_y: {}
      position: {:?}
      init_position: {:?}
      line_position: {:?}
      unit_y: {}
      ",
      self.content,
      self.init_position.0, self.line_position.1,
      self.init_position.0, self.position.1 + self.font_y,
      self.position.0, self.line_position.1,
      self.position.0, self.position.1 + self.font_y,
      self.font_y,
      self.position,
      self.init_position,
      self.line_position,
      self.unit_y
    );
     */

    let bg_color = color.unwrap_or(BACKGROUND_COLOR);
    
    let vertices = vec![
      Vertex { // 0
        position: [self.init_position.0, self.line_position.1,  0.0], 
        color: bg_color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 1
        position: [self.init_position.0, self.position.1 + self.font_y,  0.0], 
        color: bg_color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 2
        position: [self.position.0, self.line_position.1,  0.0], 
        color: bg_color, tex_coords: [0.0, 0.0], mode: 0,
      },
      Vertex { // 3
        position: [self.position.0, self.position.1 + self.font_y,  0.0], 
        color: bg_color, tex_coords: [0.0, 0.0], mode: 0,
      },
    ];

    let indices = vec![
      0, 2, 1,
      1, 2, 3
    ];
    
    let indices_len = indices.len() as u32;
    
    let model_matrix = camera::build_transformations(
      constant::TRANSLATION, 
      constant::ROTATION,
      constant::SCALING
    );
    // let texture_matrix = cgmath::Matrix4::from_scale(1.0).into();

    // log::info!("Vertices: {:?}", self.vertices);
    // log::info!("Indices: {:?}", self.indices);
    // log::info!("Indices length: {:?}", self.indices_len);
    let texture_offset = cgmath::Vector2::new(0.0, 0.0);

    vec![
      ViewObject {
        texture_offset,
        model_matrix,
        vertices: Rc::new(vertices), 
        indices: Rc::new(indices),
        indices_len,
        label: format!("Erase bg for : [{}]", self.content),
        image_data: None,
        tx_rotation: 0.0,
      }
    ]
  }

  fn get_coordinates(&mut self, c: char) -> TextObject {
    const RATIO: f32 = 0.55;
    let allow_power_list = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '+'];

    if self.is_pow_enable == true && !allow_power_list.contains(&c)  {
      self.position.1 = self.line_position.1;
      self.font_y = if self.is_pow_enable { self.font_y * 2.0 } else { self.font_y };
      self.is_pow_enable = false;
    }

    match c {
      'A' | 'a' => {
          self.a.get_coordinates(self)
      },
      'B' | 'b' => {
          self.b.get_coordinates(self)
      },
      'C' | 'c' => {
        self.c.get_coordinates(self)
      },
      'D' | 'd' => {
          self.d.get_coordinates(self)
      },
      'E' | 'e' => {
          self.e.get_coordinates(self)
      },
      'F' | 'f' => {
          self.f.get_coordinates(self)
      },       
      'G' | 'g' => {
          self.g.get_coordinates(self)
      },
      'H' | 'h' => {
          self.h.get_coordinates(self)
      },
      'I' | 'i' => {
          self.i.get_coordinates(self)
      },
      'J' | 'j' => {
          self.j.get_coordinates(self)
      },
      'K' | 'k' => {
          self.k.get_coordinates(self)
      },
      'L' | 'l' => {
          self.l.get_coordinates(self)
      },
      'M' | 'm' => {
          self.m.get_coordinates(self)
      },
      'N' | 'n' => {
          self.n.get_coordinates(self)
      },
      'O' | 'o' => {
          self.o.get_coordinates(self)
      },
      'P' | 'p' => {
          self.p.get_coordinates(self)
      },
      'Q' | 'q' => {
          self.q.get_coordinates(self)
      },
      'R' | 'r' => {
          self.r.get_coordinates(self)
      },
      'S' | 's' => {
          self.s.get_coordinates(self)
      },
      'T' | 't' => {
          self.t.get_coordinates(self)
      },
      'U' | 'u' => {
          self.u.get_coordinates(self)
      },
      'V' | 'v' => {
          self.v.get_coordinates(self)
      },
      'W' | 'w' => {
          self.w.get_coordinates(self)
      },
      'X' | 'x' => {
          self.x.get_coordinates(self)
      },
      'Y' | 'y' => {
          self.y.get_coordinates(self)
      },
      'Z' | 'z' => {
          self.z.get_coordinates(self)
      },
      '0' => {
          self.o.get_coordinates(self)
      },
      '1' => {
          self.one.get_coordinates(self)
      },
      '2' => {
          self.two.get_coordinates(self)
      },
      '3' => {
          self.three.get_coordinates(self)
      },
      '4' => {
          self.four.get_coordinates(self)
      },
      '5' => {
          self.five.get_coordinates(self)
      },
      '6' => {
          self.six.get_coordinates(self)
      },
      '7' => {
          self.seven.get_coordinates(self)
      },
      '8' => {
          self.eight.get_coordinates(self)
      },
      '9' => {
          self.nine.get_coordinates(self)
      },
      '.' => {
          self.dot.get_coordinates(self)
      },
      ' ' => {
        if self.is_vertical {
          TextObject {
            vertices: Rc::new(vec![]),
            indices: Rc::new(vec![]),
            indices_len: 0,
            max_x: self.font_y * RATIO * self.unit_x,
            max_y: self.font_y * RATIO * self.unit_y,
            vertex_len: 0,
          }
        } else {
          TextObject {
            vertices: Rc::new(vec![]),
            indices: Rc::new(vec![]),
            indices_len: 0,
            max_x: self.font_y * RATIO * self.unit_x,
            max_y: self.font_y * RATIO * self.unit_y,
            vertex_len: 0,
          }
        }
      },
      '^' => {
        self.position.1 = self.position.1 + self.font_y * 0.8;
        self.font_y = self.font_y * 0.5;
        self.is_pow_enable = true;
        TextObject {
          vertices: Rc::new(vec![]),
          indices: Rc::new(vec![]), 
          indices_len: 0,
          max_x: self.font_y * 0.2 * self.unit_x,
          max_y: self.font_y * RATIO * self.unit_y,
          vertex_len: 0,
        }
      },
      '/' => {
          self.divide.get_coordinates(self)
      },
      '-' => {
          self.minus.get_coordinates(self)
      },
      '+' => {
          self.plus.get_coordinates(self)
      },
      '=' => {
          self.equal.get_coordinates(self)
      },
      '*' => {
          self.multiply.get_coordinates(self)
      },
      '(' => {
        self.first_bracket_left.get_coordinates(self)
      },
      ')' => {
        self.first_bracket_right.get_coordinates(self)
      },
      ',' => {
          self.comma.get_coordinates(self)
      },
      '\'' | '"' => {
          self.single_quote.get_coordinates(self)
      },      
      // '!' => {
      //     self.exclamation.get_coordinates(self)
      // },
      // '?' => {
      //     self.question.get_coordinates(self)
      // },
      // '<' => {
      //     self.less.get_coordinates(self)
      // },
      // '>' => {
      //     self.greater.get_coordinates(self)
      // },
      _ => {
        log::info!("Unknown character: {}", c);
        TextObject {
            vertices: Rc::new(vec![]),
            indices: Rc::new(vec![]),
            indices_len: 0,
            max_x: 0.0,
            max_y: 0.0,
            vertex_len: 0,
        }
      }
    }
  }
  
}


#[derive(Debug, Clone)]
pub struct TextObject {
	pub vertices: Rc<Vec<Vertex>>, 
	pub indices: Rc<Vec<u16>>,
  pub indices_len: u32,
  pub max_x: f32,
  pub max_y: f32,
  pub vertex_len: u32,
}
