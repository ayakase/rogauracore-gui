#[derive(Clone, Copy)]
pub enum Speed {
    Slow,
    Medium,
    Fast,
}

impl Speed {
    pub const ALL: [Speed; 3] = [Speed::Slow, Speed::Medium, Speed::Fast];

    pub fn as_arg(self) -> &'static str {
        match self {
            Speed::Slow => "slow",
            Speed::Medium => "medium",
            Speed::Fast => "fast",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Speed::Slow => "Slow",
            Speed::Medium => "Medium",
            Speed::Fast => "Fast",
        }
    }
}

#[derive(Clone, Copy)]
pub enum Brightness {
    Off,
    Low,
    Medium,
    High,
}

impl Brightness {
    pub const ALL: [Brightness; 4] = [
        Brightness::Off,
        Brightness::Low,
        Brightness::Medium,
        Brightness::High,
    ];

    pub fn as_arg(self) -> &'static str {
        match self {
            Brightness::Off => "off",
            Brightness::Low => "low",
            Brightness::Medium => "medium",
            Brightness::High => "high",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Brightness::Off => "Off",
            Brightness::Low => "Low",
            Brightness::Medium => "Medium",
            Brightness::High => "High",
        }
    }
}

pub enum AuraCommand {
    SingleStatic {
        color: String,
    },
    SingleBreathing {
        color1: String,
        color2: Option<String>,
        speed: Option<Speed>,
    },
    SinglePulsing {
        color: String,
        speed: Speed,
    },
    SingleColorcycle {
        speed: Speed,
    },
    MultiStatic {
        colors: [String; 4],
    },
    MultiBreathing {
        colors: [String; 4],
        speed: Speed,
    },
    Rainbow {
        speed: Option<Speed>,
    },
    Brightness {
        brightness: Brightness,
    },
}

impl AuraCommand {
    pub fn args(&self) -> Vec<String> {
        match self {
            AuraCommand::SingleStatic { color } => {
                vec!["single_static".into(), color.clone()]
            }
            AuraCommand::SingleBreathing {
                color1,
                color2,
                speed,
            } => {
                let mut args = vec!["single_breathing".into(), color1.clone()];
                if let Some(color2) = color2 {
                    args.push(color2.clone());
                }
                if let Some(speed) = speed {
                    args.push(speed.as_arg().into());
                }
                args
            }
            AuraCommand::SinglePulsing { color, speed } => {
                vec![
                    "single_pulsing".into(),
                    color.clone(),
                    speed.as_arg().into(),
                ]
            }
            AuraCommand::SingleColorcycle { speed } => {
                vec!["single_colorcycle".into(), speed.as_arg().into()]
            }
            AuraCommand::MultiStatic { colors } => {
                let mut args = vec!["multi_static".into()];
                args.extend(colors.iter().cloned());
                args
            }
            AuraCommand::MultiBreathing { colors, speed } => {
                let mut args = vec!["multi_breathing".into()];
                args.extend(colors.iter().cloned());
                args.push(speed.as_arg().into());
                args
            }
            AuraCommand::Rainbow { speed } => {
                let mut args = vec!["rainbow".into()];
                if let Some(speed) = speed {
                    args.push(speed.as_arg().into());
                }
                args
            }
            AuraCommand::Brightness { brightness } => {
                vec!["brightness".into(), brightness.as_arg().into()]
            }
        }
    }

    pub fn display(&self) -> String {
        format!("rogauracore {}", self.args().join(" "))
    }
}
