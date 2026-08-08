use anyhow::Result;

#[derive(Debug)]
pub enum Instruction {
    SubFile {
        position: [f32; 3],
        matrix: [f32; 9],
        file: String,
    },

    Triangle {
        color: u32,
        points: [[f32; 3]; 3],
        inverted: bool,
    },

    Quad {
        color: u32,
        points: [[f32; 3]; 4],
        inverted: bool,
    },
}

#[derive(Clone, Copy)]
pub struct BfcState {
    pub clockwise: bool,
    pub invert_next: bool,
}

impl Default for BfcState {
    fn default() -> Self {
        Self {
            clockwise: false,
            invert_next: false,
        }
    }
}

pub struct Parser {
    pub bfc: BfcState,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            bfc: BfcState::default(),
        }
    }

    pub fn parse(&mut self, data: &str) -> Result<Vec<Instruction>> {
        let mut instructions = Vec::new();

        for line in data.lines() {
            if let Some(inst) = self.parse_line(line)? {
                instructions.push(inst);
            }
        }

        Ok(instructions)
    }

    fn parse_line(&mut self, line: &str) -> Result<Option<Instruction>> {
        let line = line.trim();

        if line.is_empty() {
            return Ok(None);
        }

        let mut parts = line.split_whitespace();

        let Some(kind) = parts.next() else {
            return Ok(None);
        };

        match kind {
            "0" => {
                self.parse_comment(parts.collect());

                Ok(None)
            }

            "1" => {
                let values: Vec<&str> = parts.collect();

                if values.len() < 14 {
                    return Ok(None);
                }

                let _color: u32 = values[0].parse()?;

                let position = [values[1].parse()?, values[2].parse()?, values[3].parse()?];

                let mut matrix = [0.0f32; 9];

                for i in 0..9 {
                    matrix[i] = values[4 + i].parse()?;
                }

                Ok(Some(Instruction::SubFile {
                    position,
                    matrix,
                    file: values[13..].join(" "),
                }))
            }

            "2" | "5" => Ok(None),

            "3" => {
                let values: Vec<f32> = parts.map(|v| v.parse()).collect::<Result<_, _>>()?;

                if values.len() != 10 {
                    return Ok(None);
                }

                let mut inverted = self.bfc.invert_next;

                self.bfc.invert_next = false;

                if self.bfc.clockwise {
                    inverted = !inverted;
                }

                Ok(Some(Instruction::Triangle {
                    color: values[0] as u32,

                    points: [
                        [values[1], values[2], values[3]],
                        [values[4], values[5], values[6]],
                        [values[7], values[8], values[9]],
                    ],

                    inverted,
                }))
            }

            "4" => {
                let values: Vec<f32> = parts.map(|v| v.parse()).collect::<Result<_, _>>()?;

                if values.len() != 13 {
                    return Ok(None);
                }

                let mut inverted = self.bfc.invert_next;

                self.bfc.invert_next = false;

                if self.bfc.clockwise {
                    inverted = !inverted;
                }

                Ok(Some(Instruction::Quad {
                    color: values[0] as u32,

                    points: [
                        [values[1], values[2], values[3]],
                        [values[4], values[5], values[6]],
                        [values[7], values[8], values[9]],
                        [values[10], values[11], values[12]],
                    ],

                    inverted,
                }))
            }

            _ => Ok(None),
        }
    }

    fn parse_comment(&mut self, args: Vec<&str>) {
        if args.first() != Some(&"BFC") {
            return;
        }

        for arg in &args[1..] {
            match *arg {
                "CW" => self.bfc.clockwise = true,

                "CCW" => self.bfc.clockwise = false,

                "INVERTNEXT" => self.bfc.invert_next = true,

                _ => {}
            }
        }
    }
}
