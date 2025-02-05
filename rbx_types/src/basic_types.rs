#[allow(unused)]
use crate::utils::lerp;

/// Represents any Roblox enum value.
///
/// Roblox enums are not strongly typed, so the meaning of a value depends on
/// where they're assigned.
///
/// A list of all enums and their values are available [on the Roblox Developer
/// Hub](https://developer.roblox.com/en-us/api-reference/enum).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct Enum {
    value: u32,
}

impl Enum {
    pub fn from_u32(value: u32) -> Self {
        Self { value }
    }

    pub fn to_u32(self) -> u32 {
        self.value
    }
}

/// The standard 2D vector type used in Roblox.
///
/// ## See Also
/// * [`Vector2int16`][struct.Vector2int16.html]
/// * [Vector2 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Vector2)
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[cfg(feature = "mlua")]
impl mlua::UserData for Vector2 {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("X", |_lua, this| Ok(this.x));
        fields.add_field_method_get("Y", |_lua, this| Ok(this.y));

        fields.add_field_method_get("Magnitude", |_lua, this| {
            Ok(f32::sqrt(this.x.powi(2) + this.y.powi(2)))
        });
    }
}

/// A version of [`Vector2`][Vector2] whose coordinates are signed 16-bit
/// integers.
///
/// ## See Also
/// * [`Vector2`][Vector2], which is used for most values.
/// * [Vector2int16 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Vector2int16)
///
/// [Vector2]: struct.Vector2.html
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector2int16 {
    pub x: i16,
    pub y: i16,
}

impl Vector2int16 {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

#[cfg(feature = "mlua")]
impl mlua::UserData for Vector2int16 {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("X", |_lua, this| Ok(this.x));
        fields.add_field_method_get("Y", |_lua, this| Ok(this.y));

        fields.add_field_method_get("Magnitude", |_lua, this| {
            Ok(f32::sqrt((this.x.pow(2) + this.y.pow(2)).into()))
        });
    }
}

/// The standard 3D vector type used in Roblox.
///
/// ## See Also
/// * [`Vector3int16`][struct.Vector3int16.html]
/// * [Vector3 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Vector3)
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[cfg(feature = "mlua")]
impl mlua::UserData for Vector3 {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("X", |_lua, this| Ok(this.x));
        fields.add_field_method_get("Y", |_lua, this| Ok(this.y));
        fields.add_field_method_get("Z", |_lua, this| Ok(this.z));

        fields.add_field_method_get("Magnitude", |_lua, this| {
            Ok(f32::sqrt(this.x.powi(2) + this.y.powi(2) + this.z.powi(2)))
        });
    }
}

fn approx_unit_or_zero(value: f32) -> Option<i32> {
    if value.abs() <= std::f32::EPSILON {
        Some(0)
    } else if value.abs() - 1.0 <= std::f32::EPSILON {
        Some(1.0f32.copysign(value) as i32)
    } else {
        None
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// If the vector is a positive or negative basis vector, returns
    /// its corresponding ID. Otherwise, returns None.
    /// The mapping goes like this:
    /// (1.0, 0.0, 0.0) -> 0
    /// (0.0, 1.0, 0.0) -> 1
    /// (0.0, 0.0, 1.0) -> 2
    /// (-1.0, 0.0, 0.0) -> 3
    /// (0.0, -1.0, 0.0) -> 4
    /// (0.0, 0.0, -1.0) -> 5
    // We accidentally did not follow this convention, but that's okay, it's not
    // a huge deal and not something we can change now.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_normal_id(&self) -> Option<u8> {
        fn get_normal_id(position: u8, value: i32) -> Option<u8> {
            match value {
                1 => Some(position),
                -1 => Some(position + 3),
                _ => None,
            }
        }

        let x = approx_unit_or_zero(self.x);
        let y = approx_unit_or_zero(self.y);
        let z = approx_unit_or_zero(self.z);

        match (x, y, z) {
            (Some(x), Some(0), Some(0)) => get_normal_id(0, x),
            (Some(0), Some(y), Some(0)) => get_normal_id(1, y),
            (Some(0), Some(0), Some(z)) => get_normal_id(2, z),
            _ => None,
        }
    }
}

/// A version of [`Vector3`][Vector3] whose coordinates are signed 16-bit
/// integers. `Vector3int16` is often used when working with Terrain.
///
/// ## See Also
/// * [`Vector3`][Vector3], which is used for most values.
/// * [Vector3int16 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Vector3int16)
///
/// [Vector3]: struct.Vector3.html
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector3int16 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl Vector3int16 {
    pub fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }
}

#[cfg(feature = "mlua")]
impl mlua::UserData for Vector3int16 {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("X", |_lua, this| Ok(this.x));
        fields.add_field_method_get("Y", |_lua, this| Ok(this.y));
        fields.add_field_method_get("Z", |_lua, this| Ok(this.z));

        fields.add_field_method_get("Magnitude", |_lua, this| {
            Ok(f32::sqrt(
                (this.x.pow(2) + this.y.pow(2) + this.z.pow(2)).into(),
            ))
        });
    }
}

/// Represents a position and orientation in 3D space.
///
/// ## See Also
/// * [CFrame on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/CFrame)
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct CFrame {
    pub position: Vector3,
    pub orientation: Matrix3,
}

impl CFrame {
    pub fn new(position: Vector3, orientation: Matrix3) -> Self {
        Self {
            position,
            orientation,
        }
    }
}

/// Used to represent the `orientation` field of `CFrame` and not a standalone
/// type in Roblox.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Matrix3 {
    pub x: Vector3,
    pub y: Vector3,
    pub z: Vector3,
}

impl Matrix3 {
    pub fn new(x: Vector3, y: Vector3, z: Vector3) -> Self {
        Self { x, y, z }
    }

    pub fn identity() -> Self {
        Self {
            x: Vector3::new(1.0, 0.0, 0.0),
            y: Vector3::new(0.0, 1.0, 0.0),
            z: Vector3::new(0.0, 0.0, 1.0),
        }
    }

    pub fn transpose(&self) -> Self {
        Self {
            x: Vector3::new(self.x.x, self.y.x, self.z.x),
            y: Vector3::new(self.x.y, self.y.y, self.z.y),
            z: Vector3::new(self.x.z, self.y.z, self.z.z),
        }
    }
}

/// Represents any color, including HDR colors.
///
/// ## See Also
/// * [`Color3uint8`](struct.Color3uint8.html), which is used instead of
///   `Color3` on some types and does not represent HDR colors.
/// * [Color3 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Color3)
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Color3 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color3 {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

#[cfg(feature = "mlua")]
impl mlua::UserData for Color3 {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("R", |_lua, this| Ok(this.r));
        fields.add_field_method_get("G", |_lua, this| Ok(this.g));
        fields.add_field_method_get("B", |_lua, this| Ok(this.b));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("Lerp", |_lua, this, (color, alpha): (Color3, f32)| {
            let r = lerp!(this.r, color.r, alpha);
            let g = lerp!(this.g, color.g, alpha);
            let b = lerp!(this.b, color.b, alpha);
            Ok(Self::new(r, g, b))
        });
        methods.add_method("ToHex", |_lua, this, _: ()| {
            let r = (this.r * 255.) as u8;
            let g = (this.g * 255.) as u8;
            let b = (this.b * 255.) as u8;
            Ok(hex::encode([r, g, b]))
        });
        methods.add_method("ToHSV", |_lua, this, _: ()| {
            let max = this.r.max(this.g.max(this.b));
            let min = this.r.max(this.g.max(this.b));

            let Color3 { r, g, b } = *this;

            let mut hue = 0.;
            let c = max - min;
            let k = 1. / (6. * c);

            if c != 0. {
                hue = match max {
                    v if v == r => ((g - b) * k) % 1.,
                    v if v == g => ((b - r) * k) + (1. / 3.),
                    _ => ((r - g) * k) + (2. / 3.),
                };
            }

            let sat = if max == 0. { 0. } else { c / max };
            Ok((hue, sat, max))
        });
    }
}

impl From<Color3uint8> for Color3 {
    fn from(value: Color3uint8) -> Self {
        Self {
            r: value.r as f32 / 255.0,
            g: value.g as f32 / 255.0,
            b: value.b as f32 / 255.0,
        }
    }
}

/// Represents non-HDR colors, i.e. those whose individual color channels do not
/// exceed 1. This type is used for serializing properties like
/// [`BasePart.Color`][BasePart.Color], but is not exposed as a distinct type to
/// Lua code.
///
/// ## See Also
/// * [`Color3`](struct.Color3.html), which is more common and can represent HDR
///   colors.
///
/// [BasePart.Color]: https://developer.roblox.com/en-us/api-reference/property/BasePart/Color
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color3uint8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color3uint8 {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl From<Color3> for Color3uint8 {
    fn from(value: Color3) -> Self {
        Self {
            r: ((value.r.max(0.0).min(1.0)) * 255.0).round() as u8,
            g: ((value.g.max(0.0).min(1.0)) * 255.0).round() as u8,
            b: ((value.b.max(0.0).min(1.0)) * 255.0).round() as u8,
        }
    }
}

/// Represents a ray in 3D space. Direction does not have to be a unit vector,
/// and is used by APIs like [`Workspace:FindPartOnRay`][FindPartOnRay] to set a
/// max distance.
///
/// ## See Also
/// * [Ray on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Ray)
///
/// [FindPartOnRay]: https://developer.roblox.com/en-us/api-reference/function/WorldRoot/FindPartOnRay
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self { origin, direction }
    }
}

/// Represents a bounding box in 3D space.
///
/// ## See Also
/// * [`Region3int16`](struct.Region3int16.html)
/// * [Region3 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Region3)
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Region3 {
    pub min: Vector3,
    pub max: Vector3,
}

impl Region3 {
    pub fn new(min: Vector3, max: Vector3) -> Self {
        Self { min, max }
    }
}

/// A version of [`Region3`][Region3] that uses signed 16-bit integers instead
/// of floats. `Region3int16` is generally used in Terrain APIs.
///
/// ## See Also
/// * [`Region`][Region3]
/// * [Region3int16 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Region3int16)
///
/// [Region3]: struct.Region3.html
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Region3int16 {
    pub min: Vector3int16,
    pub max: Vector3int16,
}

impl Region3int16 {
    pub fn new(min: Vector3int16, max: Vector3int16) -> Self {
        Self { min, max }
    }
}

/// Represents a bounding rectangle in 2D space.
///
/// ## See Also
/// * [Rect on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Rect)
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub min: Vector2,
    pub max: Vector2,
}

impl Rect {
    pub fn new(min: Vector2, max: Vector2) -> Self {
        Self { min, max }
    }
}

/// Standard unit for measuring UI given as `scale`, a fraction of the
/// container's size and `offset`, display-indepdendent pixels.
///
/// ## See Also
/// * [UDim on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/UDim)
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct UDim {
    pub scale: f32,
    pub offset: i32,
}

impl UDim {
    pub fn new(scale: f32, offset: i32) -> Self {
        Self { scale, offset }
    }
}

/// Standard 2D unit for measuring UI given as `scale`, a fraction of the
/// container's size and `offset`, display-indepdendent pixels.
///
/// ## See Also
/// * [UDim2 on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/UDim2)
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct UDim2 {
    pub x: UDim,
    pub y: UDim,
}

impl UDim2 {
    pub fn new(x: UDim, y: UDim) -> Self {
        Self { x, y }
    }
}

/// A range between two numbers.
///
/// ## See Also
/// * [NumberRange on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/NumberRange)
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct NumberRange {
    pub min: f32,
    pub max: f32,
}

impl NumberRange {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }
}

/// A series of colors that can be tweened through.
///
/// ## See Also
/// * [ColorSequence on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/ColorSequence)
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct ColorSequence {
    pub keypoints: Vec<ColorSequenceKeypoint>,
}

/// A single color and point in time of a [`ColorSequence`][ColorSequence]
///
/// ## See Also
/// * [ColorSequenceKeypoint on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/ColorSequenceKeypoint)
///
/// [ColorSequence]: struct.ColorSequence.html
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct ColorSequenceKeypoint {
    pub time: f32,
    pub color: Color3,
}

impl ColorSequenceKeypoint {
    pub fn new(time: f32, color: Color3) -> Self {
        Self { time, color }
    }
}

/// A sequence of numbers on a timeline. Each point contains a timestamp, a
/// value, and a range that allows for randomized values.
///
/// ## See Also
/// * [NumberSequence on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/NumberSequence)
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct NumberSequence {
    pub keypoints: Vec<NumberSequenceKeypoint>,
}

/// A single value, envelope, and point in time of a [`NumberSequence`][NumberSequence]
///
/// ## See Also
/// * [`NumberSequence`][NumberSequence]
/// * [NumberSequenceKeypoint on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/NumberSequenceKeypoint)
///
/// [NumberSequence]: struct.NumberSequence.html
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct NumberSequenceKeypoint {
    pub time: f32,
    pub value: f32,
    pub envelope: f32,
}

impl NumberSequenceKeypoint {
    pub fn new(time: f32, value: f32, envelope: f32) -> Self {
        Self {
            time,
            value,
            envelope,
        }
    }
}

#[cfg(feature = "serde")]
serde_tuple! {
    Vector2(x: f32, y: f32),
    Vector2int16(x: i16, y: i16),
    Vector3(x: f32, y: f32, z: f32),
    Vector3int16(x: i16, y: i16, z: i16),

    Color3(r: f32, g: f32, b: f32),
    Color3uint8(r: u8, g: u8, b: u8),

    UDim(scale: f32, offset: i32),
    UDim2(x: UDim, y: UDim),

    NumberRange(min: f32, max: f32),

    Rect(min: Vector2, max: Vector2),
    Region3(min: Vector3, max: Vector3),
    Region3int16(min: Vector3int16, max: Vector3int16),

    Matrix3(x: Vector3, y: Vector3, z: Vector3),
}

#[cfg(all(test, feature = "serde"))]
mod serde_test {
    use super::*;

    use std::fmt::Debug;

    use serde::{de::DeserializeOwned, Serialize};

    fn test_ser<T: Debug + PartialEq + Serialize + DeserializeOwned>(value: T, output: &str) {
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, output);

        let deserialized: T = serde_json::from_str(output).unwrap();
        assert_eq!(deserialized, value);
    }

    #[test]
    fn vec2_json() {
        test_ser(Vector2 { x: 2.0, y: 3.5 }, "[2.0,3.5]");
    }

    #[test]
    fn udim_json() {
        test_ser(
            UDim {
                scale: 1.0,
                offset: 175,
            },
            "[1.0,175]",
        );
    }

    #[test]
    fn udim2_json() {
        test_ser(
            UDim2 {
                x: UDim {
                    scale: 0.0,
                    offset: 30,
                },
                y: UDim {
                    scale: 1.0,
                    offset: 60,
                },
            },
            "[[0.0,30],[1.0,60]]",
        );
    }

    #[test]
    fn region3_json() {
        test_ser(
            Region3 {
                min: Vector3::new(-1.0, -2.0, -3.0),
                max: Vector3::new(4.0, 5.0, 6.0),
            },
            "[[-1.0,-2.0,-3.0],[4.0,5.0,6.0]]",
        );
    }

    #[test]
    fn matrix3_json() {
        test_ser(
            Matrix3 {
                x: Vector3::new(1.0, 2.0, 3.0),
                y: Vector3::new(4.0, 5.0, 6.0),
                z: Vector3::new(7.0, 8.0, 9.0),
            },
            "[[1.0,2.0,3.0],[4.0,5.0,6.0],[7.0,8.0,9.0]]",
        );
    }
}
