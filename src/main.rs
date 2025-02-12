use std::fmt::{
    Display,
    Formatter,
};

const BOILERPLATE_HEAD: &str = r#"<!DOCTYPE html>
<html>
  <head>
    <meta charset = "utf-8">
    <title>Static matrix rain</title>
    <script type = "text/javascript" src="https://aframe.io/releases/1.6.0/aframe.min.js"></script>
  </head>
  <body>
  <a-scene>
  <a-entity camera look-controls enabled = "false"></a-entity>
  <a-sky color = "black"></a-sky>"#;
const BOILERPLATE_TAIL: &str = r#"</a-scene>
  </body>
</html>"#;

const Y: i32 = 70;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    z: i32,
}
impl Point {
    pub fn new_start() -> Self {
        Self {
            x: fastrand::i32(-10..10),
            z: fastrand::i32(-10..10),
        }
    }
    pub fn new_end(start: Self) -> Self {
        Self {
            x: start.x.saturating_add(fastrand::i32(-10..10)),
            z: start.z.saturating_add(fastrand::i32(-10..10)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Rain {
    start: Point,
    end: Point,
    bit: bool,

    delay: u32,
    duration: u32,
    rotation: u16,
}
impl Rain {
    pub fn new(delay: u32) -> Self {
        let start = Point::new_start();
        let end = Point::new_end(start);

        Self {
            start,
            end,
            bit: fastrand::bool(),

            delay,
            duration: fastrand::u32(10..30),
            rotation: fastrand::u16(..360),
        }
    }
}
impl Display for Rain {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, r#"<a-text color = "lawngreen" value = "{bit}" rotation = "0 {rotation} 0" position = "{sx} {sy} {sz}" animation = "delay: {delay}; dur: {duration}; property: position; loop: true; to: {ex} {ey} {ez}; easing = "easeInSine"></a-text>"#,
            bit = match self.bit {
                true => 1,
                false => 0,
            },
            sx = self.start.x,
            sy = Y,
            sz = self.start.z,
            ex = self.end.x,
            ey = -Y,
            ez = self.end.z,
            rotation = self.rotation,
            delay = self.delay,
            duration = self.duration * 200,
        )
    }
}

fn main() {
    println!("{}", BOILERPLATE_HEAD);
    (0..700)
        .map(|i| Rain::new(i))
        .for_each(|r| println!("{r}"));
    println!("{}", BOILERPLATE_TAIL);
}
