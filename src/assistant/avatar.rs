// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Animated Talking Head Avatar for the Assistant

use std::sync::Arc;
use parking_lot::RwLock;
use super::AvatarStyle;

/// Avatar animation states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AvatarAnimation {
    Idle,
    Speaking,
    Listening,
    Thinking,
    Wave,
    Nod,
    Shake,
    Blink,
    Custom(u32),
}

/// Face expression for the avatar
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Expression {
    Neutral,
    Happy,
    Concerned,
    Thinking,
    Alert,
    Confident,
}

/// Avatar head position and orientation
#[derive(Debug, Clone, Copy)]
pub struct HeadPose {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub scale: f32,
}

impl Default for HeadPose {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            scale: 1.0,
        }
    }
}

/// Mouth shape for lip-sync
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouthShape {
    Closed,
    SlightlyOpen,
    Open,
    Wide,
    OShape,
    EShape,
    AShape,
}

/// Eye state
#[derive(Debug, Clone, Copy)]
pub struct EyeState {
    pub left_open: f32,
    pub right_open: f32,
    pub look_x: f32,
    pub look_y: f32,
}

impl Default for EyeState {
    fn default() -> Self {
        Self {
            left_open: 1.0,
            right_open: 1.0,
            look_x: 0.0,
            look_y: 0.0,
        }
    }
}

/// Avatar visual state
#[derive(Debug, Clone)]
pub struct AvatarState {
    pub animation: AvatarAnimation,
    pub expression: Expression,
    pub head_pose: HeadPose,
    pub mouth_shape: MouthShape,
    pub eyes: EyeState,
    pub frame: u32,
}

impl Default for AvatarState {
    fn default() -> Self {
        Self {
            animation: AvatarAnimation::Idle,
            expression: Expression::Neutral,
            head_pose: HeadPose::default(),
            mouth_shape: MouthShape::Closed,
            eyes: EyeState::default(),
            frame: 0,
        }
    }
}

/// Animated talking head avatar
pub struct Avatar {
    pub style: AvatarStyle,
    pub state: Arc<RwLock<AvatarState>>,
    pub color_scheme: AvatarColors,
}

#[derive(Debug, Clone)]
pub struct AvatarColors {
    pub skin: String,
    pub hair: String,
    pub eyes: String,
    pub outline: String,
    pub glow: String,
    pub background: String,
}

impl Default for AvatarColors {
    fn default() -> Self {
        Self {
            skin: "#1a1a2e".to_string(),
            hair: "#0f3460".to_string(),
            eyes: "#e94560".to_string(),
            outline: "#e94560".to_string(),
            glow: "#00ff88".to_string(),
            background: "#0d0d1a".to_string(),
        }
    }
}

impl Avatar {
    pub fn new(style: AvatarStyle) -> Self {
        let color_scheme = match style {
            AvatarStyle::CyberPunk => AvatarColors {
                skin: "#1a1a2e".to_string(),
                hair: "#e94560".to_string(),
                eyes: "#00ff88".to_string(),
                outline: "#e94560".to_string(),
                glow: "#00ff88".to_string(),
                background: "#0d0d1a".to_string(),
            },
            AvatarStyle::Professional => AvatarColors {
                skin: "#2d3436".to_string(),
                hair: "#636e72".to_string(),
                eyes: "#0984e3".to_string(),
                outline: "#0984e3".to_string(),
                glow: "#74b9ff".to_string(),
                background: "#1e272e".to_string(),
            },
            AvatarStyle::Hacker => AvatarColors {
                skin: "#0a0a0a".to_string(),
                hair: "#00ff00".to_string(),
                eyes: "#00ff00".to_string(),
                outline: "#00ff00".to_string(),
                glow: "#00ff00".to_string(),
                background: "#000000".to_string(),
            },
            _ => AvatarColors::default(),
        };

        Self {
            style,
            state: Arc::new(RwLock::new(AvatarState::default())),
            color_scheme,
        }
    }

    pub fn animate(&self, animation: AvatarAnimation) {
        let mut state = self.state.write();
        state.animation = animation;
        state.frame = 0;
        
        // Set appropriate expression for animation
        state.expression = match animation {
            AvatarAnimation::Speaking => Expression::Confident,
            AvatarAnimation::Listening => Expression::Neutral,
            AvatarAnimation::Thinking => Expression::Thinking,
            AvatarAnimation::Wave => Expression::Happy,
            _ => Expression::Neutral,
        };
    }

    pub fn set_expression(&self, expression: Expression) {
        self.state.write().expression = expression;
    }

    pub fn set_mouth(&self, shape: MouthShape) {
        self.state.write().mouth_shape = shape;
    }

    pub fn blink(&self) {
        let mut state = self.state.write();
        state.eyes.left_open = 0.0;
        state.eyes.right_open = 0.0;
    }

    pub fn look_at(&self, x: f32, y: f32) {
        let mut state = self.state.write();
        state.eyes.look_x = x.clamp(-1.0, 1.0);
        state.eyes.look_y = y.clamp(-1.0, 1.0);
    }

    pub fn update(&self, delta_time: f32) {
        let mut state = self.state.write();
        state.frame += 1;
        
        // Auto-blink every ~3 seconds
        if state.frame % 180 == 0 {
            state.eyes.left_open = 0.0;
            state.eyes.right_open = 0.0;
        } else if state.frame % 180 == 6 {
            state.eyes.left_open = 1.0;
            state.eyes.right_open = 1.0;
        }

        // Subtle idle movement
        if state.animation == AvatarAnimation::Idle {
            let time = state.frame as f32 * delta_time;
            state.head_pose.y = (time * 0.5).sin() * 2.0;
            state.head_pose.rotation = (time * 0.3).sin() * 1.0;
        }

        // Speaking mouth animation
        if state.animation == AvatarAnimation::Speaking {
            let shapes = [
                MouthShape::Closed,
                MouthShape::SlightlyOpen,
                MouthShape::Open,
                MouthShape::AShape,
                MouthShape::OShape,
                MouthShape::EShape,
            ];
            let idx = (state.frame / 4) as usize % shapes.len();
            state.mouth_shape = shapes[idx];
        }
    }

    pub fn render_svg(&self) -> String {
        let state = self.state.read();
        let colors = &self.color_scheme;
        
        format!(r##"
<svg viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <filter id="glow">
      <feGaussianBlur stdDeviation="3" result="coloredBlur"/>
      <feMerge>
        <feMergeNode in="coloredBlur"/>
        <feMergeNode in="SourceGraphic"/>
      </feMerge>
    </filter>
    <linearGradient id="faceGrad" x1="0%" y1="0%" x2="0%" y2="100%">
      <stop offset="0%" style="stop-color:{};stop-opacity:1" />
      <stop offset="100%" style="stop-color:#0a0a15;stop-opacity:1" />
    </linearGradient>
  </defs>
  
  <!-- Background -->
  <rect width="200" height="200" fill="{}"/>
  
  <!-- Head outline with glow -->
  <g transform="translate({}, {}) rotate({})">
    <!-- Face shape -->
    <ellipse cx="100" cy="100" rx="60" ry="70" 
             fill="url(#faceGrad)" 
             stroke="{}" stroke-width="2" filter="url(#glow)"/>
    
    <!-- Hair/Top -->
    <path d="M50 80 Q100 30 150 80" 
          fill="none" stroke="{}" stroke-width="3" filter="url(#glow)"/>
    
    <!-- Eyes -->
    <g>
      <!-- Left eye -->
      <ellipse cx="75" cy="90" rx="12" ry="{}" fill="{}" filter="url(#glow)"/>
      <circle cx="{}" cy="90" r="4" fill="#fff"/>
      
      <!-- Right eye -->
      <ellipse cx="125" cy="90" rx="12" ry="{}" fill="{}" filter="url(#glow)"/>
      <circle cx="{}" cy="90" r="4" fill="#fff"/>
    </g>
    
    <!-- Nose (subtle) -->
    <path d="M100 95 L100 110" stroke="{}" stroke-width="1" opacity="0.5"/>
    
    <!-- Mouth -->
    {}
    
    <!-- Tech lines -->
    <path d="M40 70 L30 50" stroke="{}" stroke-width="1" opacity="0.6"/>
    <path d="M160 70 L170 50" stroke="{}" stroke-width="1" opacity="0.6"/>
    <circle cx="30" cy="48" r="3" fill="{}" filter="url(#glow)"/>
    <circle cx="170" cy="48" r="3" fill="{}" filter="url(#glow)"/>
  </g>
  
  <!-- Status indicator -->
  <circle cx="180" cy="20" r="8" fill="{}" filter="url(#glow)"/>
</svg>
"##,
            colors.skin,
            colors.background,
            state.head_pose.x,
            state.head_pose.y,
            state.head_pose.rotation,
            colors.outline,
            colors.hair,
            state.eyes.left_open * 8.0,
            colors.eyes,
            75.0 + state.eyes.look_x * 5.0,
            state.eyes.right_open * 8.0,
            colors.eyes,
            125.0 + state.eyes.look_x * 5.0,
            colors.outline,
            self.render_mouth(&state),
            colors.glow,
            colors.glow,
            colors.glow,
            colors.glow,
            match state.animation {
                AvatarAnimation::Listening => "#00ff88",
                AvatarAnimation::Speaking => "#e94560",
                AvatarAnimation::Thinking => "#ffd700",
                _ => "#666666",
            }
        )
    }

    fn render_mouth(&self, state: &AvatarState) -> String {
        let colors = &self.color_scheme;
        match state.mouth_shape {
            MouthShape::Closed => format!(
                r#"<path d="M85 125 Q100 130 115 125" stroke="{}" stroke-width="2" fill="none"/>"#,
                colors.outline
            ),
            MouthShape::SlightlyOpen => format!(
                r#"<ellipse cx="100" cy="125" rx="10" ry="3" fill="{}" stroke="{}" stroke-width="1"/>"#,
                colors.background, colors.outline
            ),
            MouthShape::Open => format!(
                r#"<ellipse cx="100" cy="125" rx="12" ry="6" fill="{}" stroke="{}" stroke-width="1"/>"#,
                colors.background, colors.outline
            ),
            MouthShape::Wide => format!(
                r#"<ellipse cx="100" cy="125" rx="15" ry="8" fill="{}" stroke="{}" stroke-width="1"/>"#,
                colors.background, colors.outline
            ),
            MouthShape::OShape => format!(
                r#"<circle cx="100" cy="125" r="8" fill="{}" stroke="{}" stroke-width="1"/>"#,
                colors.background, colors.outline
            ),
            MouthShape::EShape => format!(
                r#"<ellipse cx="100" cy="125" rx="14" ry="4" fill="{}" stroke="{}" stroke-width="1"/>"#,
                colors.background, colors.outline
            ),
            MouthShape::AShape => format!(
                r#"<ellipse cx="100" cy="128" rx="10" ry="10" fill="{}" stroke="{}" stroke-width="1"/>"#,
                colors.background, colors.outline
            ),
        }
    }
}
