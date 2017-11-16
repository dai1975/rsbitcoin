use super::opcode::*;
use super::Instruction;
use super::pushee::Pushee;

pub struct Parser;

#[derive(Debug)]
pub struct Parsed<'a> {
   pub offset:      usize,
   pub opcode:      u8,
   pub instruction: Instruction<'a>,
}
pub struct Iter<'a> {
   pub bytecode: &'a [u8],
   pub cursor:   usize,
}

impl Parser {
   pub fn iter<'x>(bytecode: &'x [u8]) -> Iter<'x> {
      Iter { bytecode: bytecode, cursor: 0 }
   }
   pub fn parse<'x>(bytecode: &'x [u8]) -> ::Result<Vec<Parsed<'x>>> {
      let mut v = Vec::new();
      for r in Parser::iter(bytecode) {
         if let Err(e) = r {
            return Err(e);
         }
         v.push(r.unwrap());
      }
      Ok(v)
   }
   pub fn parse_raw<'x>(bytecode: &'x [u8]) -> ::Result<Vec<Instruction<'x>>> {
      let mut v = Vec::new();
      for r in Parser::iter(bytecode) {
         if let Err(e) = r {
            return Err(e);
         }
         v.push(r.unwrap().instruction);
      }
      Ok(v)
   }
}

impl <'a> Iter<'a> {
   fn parse_pushdata(&self) -> ::Result<(usize,usize)> {
      let code = self.bytecode[self.cursor];
      let info = OPCODE_INFO[code as usize];
      let (offset, datalen) = match code {
         OP_PUSHDATA1 => {
            if self.bytecode.len() <= self.cursor+1 {
               parse_script_error!(format!("cannot get length of PUSHDATA1 at {}", self.cursor));
            }
            let v = self.bytecode[self.cursor + 1];
            (1, v as usize)
         },
         OP_PUSHDATA2 => {
            if self.bytecode.len() <= self.cursor+2 {
               parse_script_error!(format!("cannot get length of PUSHDATA2 at {}", self.cursor));
            }
            let v:u16 = (self.bytecode[self.cursor + 1] as u16)
               | (self.bytecode[self.cursor + 2] as u16) << 8;
            (2, v as usize)
         },
         OP_PUSHDATA4 => {
            if self.bytecode.len() <= self.cursor+4 {
               parse_script_error!(format!("cannot get length of PUSHDATA4 at {}", self.cursor));
            }
            let v:u32 = (self.bytecode[self.cursor+1] as u32)
               | (self.bytecode[self.cursor+2] as u32) << 8
               | (self.bytecode[self.cursor+3] as u32) << 16
               | (self.bytecode[self.cursor+4] as u32) << 24;
            (4, v as usize)
         },
         _ => {
            (0, code as usize)
         }
      };
      let from = self.cursor + 1 + offset;
      let to   = from + datalen;
      if 0 < datalen && self.bytecode.len() < to {
         parse_script_error!(format!("cannot get data[{}] of {} at {}+{}", datalen, info.name, self.cursor, 1+offset));
      }
      Ok((from, to))
   }
}

impl <'a> ::std::iter::Iterator for Iter<'a> {
   type Item = ::Result<Parsed<'a>>;

   fn next(&mut self) -> Option<::Result<Parsed<'a>>> {
      if self.bytecode.len() <= self.cursor {
         return None
      }
      let cursor0 = self.cursor;
      let code = self.bytecode[self.cursor];
      //let info = OPCODE_INFO[code as usize];
      //println!("    next. code[{}]={:x}={}...", cursor0, code, OPCODE_INFO[code as usize].name);
      let inst = match code {
         OP_PUSHDATAFIX_01 ... OP_PUSHDATA4 => {
            match self.parse_pushdata() {
               Err(e) => return Some(Err(e)),
               Ok((from, to)) => {
                  self.cursor = to;
                  Instruction::Push(Pushee::new_data(&self.bytecode[from..to]))
               },
            }
         },
         OP_0 => {
            self.cursor += 1;
            Instruction::Push(Pushee::new_value(0))
         },
         OP_1 ... OP_16 => {
            self.cursor += 1;
            Instruction::Push(Pushee::new_value(code-OP_1+1))
         },
         _ => {
            self.cursor += 1;
            Instruction::Op(code)
         }
      };
      Some(Ok(Parsed{offset:cursor0, opcode:code, instruction:inst}))
   }
}

/*
  next を分割したいのだが。
  たとえば
    fn next0(&self) -> Instruction<'a>
  というサブ関数に分離し、next(&mut self) から
    let r = self.next0()
  と呼ぶ形になる。
  しかし next 中の self は trait で指定された通り(&mut self)なのでライフタイムを指定できない。
  next0 の返すライフタイム('a) と不整合が起きる。
*/


#[test]
fn test_decode() {
   use ::utils::h2b;
   let bytecode = h2b(concat!("48", "3045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba01",
                       "41", "04c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7ab")).unwrap();
   // 0x48=72, 0x41=65, 0x48+0x41=137
   
   use super::{Parser, Instruction as I};
   use super::pushee::Pushee;
   let mut parsed = Parser::iter(bytecode.as_slice());

   {
      let n = parsed.next();
      assert_matches!(n, Some(Ok(_)));
      let parsed = n.unwrap().unwrap();
      assert_eq!(parsed.offset, 0);
      assert_eq!(parsed.opcode, OP_PUSHDATAFIX_48);
      assert_matches!(parsed.instruction, I::Push(Pushee::Data(_)));
      if let I::Push(Pushee::Data(data)) = parsed.instruction {
         assert_eq!(data.len(), 0x48);
         assert_eq!(data, &bytecode[1..(1+0x48)]);
         assert_eq!(data[0..4], [0x30, 0x45, 0x02, 0x21]);
      }
   }
   {
      let n = parsed.next();
      assert_matches!(n, Some(Ok(_)));
      let parsed = n.unwrap().unwrap();
      assert_eq!(parsed.offset, 0x49);
      assert_eq!(parsed.opcode, OP_PUSHDATAFIX_41);
      assert_matches!(parsed.instruction, I::Push(Pushee::Data(_)));
      if let I::Push(Pushee::Data(data)) = parsed.instruction {
         assert_eq!(data.len(), 0x41);
         assert_eq!(data, &bytecode[0x4a..(0x4a+0x41)]);
         assert_eq!(data[0..4], [0x04, 0xc5, 0x4f, 0x8e]);
      }
   }
}

#[test]
fn test_decode_failed() {
   use ::utils::h2b;
   let bytecode = h2b(concat!("48", "3045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba01",
                       "4c", "FF", "")).unwrap();
   // 0x48=72, 0x41=65, 0x48+0x41=137
   
   use super::{Parser, Instruction as I};
   let mut parsed = Parser::iter(bytecode.as_slice());

   {
      let n = parsed.next();
      assert_matches!(n, Some(Ok(_)));
      let parsed = n.unwrap().unwrap();
      assert_eq!(parsed.offset, 0);
      assert_eq!(parsed.opcode, OP_PUSHDATAFIX_48);
      assert_matches!(parsed.instruction, I::Push(Pushee::Data(_)));
      if let I::Push(Pushee::Data(data)) = parsed.instruction {
         assert_eq!(data.len(), 0x48);
         assert_eq!(data, &bytecode[1..(1+0x48)]);
         assert_eq!(data[0..4], [0x30, 0x45, 0x02, 0x21]);
      }
   }
   {
      let n = parsed.next();
      assert_matches!(n, Some(Err(_)));
   }
}

#[test]
fn test_parse() {
   use ::utils::{h2b, FmtVec};
   let bytecode = h2b(concat!("48", "3045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba01",
                              "41", "04c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7ab")).unwrap();

   let instructions = Parser::parse_raw(bytecode.as_slice()).unwrap();
   assert_eq!("[72] [65]", format!("{}", FmtVec(instructions)));
}
   
