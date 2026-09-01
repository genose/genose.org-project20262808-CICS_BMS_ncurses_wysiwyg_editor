//! Property system for BMS fields - mirrors Lua OBJECTS-DEFINITIONS structure
//! Each property has initial and edited states for memory efficiency and undo/redo support

use serde::{Serialize, Deserialize};
use std::fmt;

/// A property that tracks initial and edited states
/// Mirrors Lua: {initial = X, edited = Y}
/// 
/// This allows:
/// - Memory efficiency: only edited values consume extra space
/// - Undo/redo support: can revert to initial value
/// - Default values: initial provides sensible defaults
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Property<T: Clone> {
    pub initial: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited: Option<T>,
}

impl<T: Clone> Property<T> {
    /// Create a new property with initial value
    pub fn new(initial: T) -> Self {
        Self { initial, edited: None }
    }

    /// Get current value (edited if set, otherwise initial)
    pub fn get(&self) -> T {
        self.edited.clone().unwrap_or_else(|| self.initial.clone())
    }

    /// Set edited value
    pub fn set(&mut self, value: T) {
        self.edited = Some(value);
    }

    /// Reset to initial value
    pub fn reset(&mut self) {
        self.edited = None;
    }

    /// Check if property has been edited
    pub fn is_edited(&self) -> bool {
        self.edited.is_some()
    }

    /// Get reference to edited value if it exists
    pub fn get_edited(&self) -> Option<&T> {
        self.edited.as_ref()
    }

    /// Get reference to initial value
    pub fn get_initial(&self) -> &T {
        &self.initial
    }
}

impl<T: Clone + fmt::Debug> fmt::Display for Property<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_edited() {
            write!(f, "Property(edited={:?}, initial={:?})", self.edited, self.initial)
        } else {
            write!(f, "Property(initial={:?})", self.initial)
        }
    }
}

/// A property with value constraints (min/max)
/// Ensures values stay within bounds
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstrainedProperty<T: Clone + PartialOrd> {
    pub initial: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited: Option<T>,
    pub min: T,
    pub max: T,
}

impl<T: Clone + PartialOrd + std::fmt::Debug> ConstrainedProperty<T> {
    /// Create a new constrained property with initial value and bounds
    pub fn new(initial: T, min: T, max: T) -> Self {
        // Clamp initial value to bounds
        let initial = if initial < min { min.clone() } else if initial > max { max.clone() } else { initial };
        Self { initial, edited: None, min, max }
    }

    /// Get current value (edited if set, otherwise initial)
    pub fn get(&self) -> T {
        self.edited.clone().unwrap_or_else(|| self.initial.clone())
    }

    /// Set edited value with validation
    pub fn set(&mut self, value: T) -> Result<(), String> {
        if value < self.min || value > self.max {
            Err(format!(
                "Value {:?} must be between {:?} and {:?}",
                value, self.min, self.max
            ))
        } else {
            self.edited = Some(value);
            Ok(())
        }
    }

    /// Set without validation (for internal use)
    pub fn set_unchecked(&mut self, value: T) {
        self.edited = Some(value);
    }

    /// Reset to initial value
    pub fn reset(&mut self) {
        self.edited = None;
    }

    /// Check if property has been edited
    pub fn is_edited(&self) -> bool {
        self.edited.is_some()
    }

    /// Get reference to edited value if it exists
    pub fn get_edited(&self) -> Option<&T> {
        self.edited.as_ref()
    }

    /// Get reference to initial value
    pub fn get_initial(&self) -> &T {
        &self.initial
    }

    /// Get min value reference
    pub fn min(&self) -> &T {
        &self.min
    }

    /// Get max value reference
    pub fn max(&self) -> &T {
        &self.max
    }
}

/// Enum property with available options
/// Ensures only valid enum values can be set
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumProperty<T: Clone + Eq + std::hash::Hash> {
    pub initial: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited: Option<T>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub available: Vec<T>,
}

impl<T: Clone + Eq + std::hash::Hash + std::fmt::Debug> EnumProperty<T> {
    /// Create a new enum property with initial value and available options
    pub fn new(initial: T, available: Vec<T>) -> Self {
        Self { initial, edited: None, available }
    }

    /// Get current value (edited if set, otherwise initial)
    pub fn get(&self) -> T {
        self.edited.clone().unwrap_or_else(|| self.initial.clone())
    }

    /// Set edited value with validation
    pub fn set(&mut self, value: T) -> Result<(), String> {
        if self.available.contains(&value) {
            self.edited = Some(value);
            Ok(())
        } else {
            Err(format!(
                "Value {:?} must be one of: {:?}",
                value, self.available
            ))
        }
    }

    /// Set without validation (for internal use)
    pub fn set_unchecked(&mut self, value: T) {
        self.edited = Some(value);
    }

    /// Reset to initial value
    pub fn reset(&mut self) {
        self.edited = None;
    }

    /// Check if property has been edited
    pub fn is_edited(&self) -> bool {
        self.edited.is_some()
    }

    /// Check if a value is available
    pub fn is_available(&self, value: &T) -> bool {
        self.available.contains(value)
    }

    /// Get all available options
    pub fn available(&self) -> &Vec<T> {
        &self.available
    }
}

/// Flag property (boolean) with additional state
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FlagProperty {
    pub initial: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited: Option<bool>,
}

impl FlagProperty {
    pub fn new(initial: bool) -> Self {
        Self { initial, edited: None }
    }

    pub fn get(&self) -> bool {
        self.edited.unwrap_or(self.initial)
    }

    pub fn set(&mut self, value: bool) {
        self.edited = Some(value);
    }

    pub fn toggle(&mut self) {
        self.edited = Some(!self.get());
    }

    pub fn reset(&mut self) {
        self.edited = None;
    }

    pub fn is_edited(&self) -> bool {
        self.edited.is_some()
    }
}

impl Default for FlagProperty {
    fn default() -> Self {
        Self::new(false)
    }
}

/// Nested property for complex structures
/// Allows accessing nested properties with dot notation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NestedProperty<T: Clone> {
    pub initial: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited: Option<T>,
}

impl<T: Clone> NestedProperty<T> {
    pub fn new(initial: T) -> Self {
        Self { initial, edited: None }
    }

    pub fn get(&self) -> T {
        self.edited.clone().unwrap_or_else(|| self.initial.clone())
    }

    pub fn set(&mut self, value: T) {
        self.edited = Some(value);
    }

    pub fn reset(&mut self) {
        self.edited = None;
    }

    pub fn is_edited(&self) -> bool {
        self.edited.is_some()
    }
}

/// Trait for property-like behavior
pub trait PropertyBehavior<T> {
    fn get(&self) -> T;
    fn set(&mut self, value: T);
    fn reset(&mut self);
    fn is_edited(&self) -> bool;
}

impl<T: Clone> PropertyBehavior<T> for Property<T> {
    fn get(&self) -> T { Property::get(self) }
    fn set(&mut self, value: T) { Property::set(self, value); }
    fn reset(&mut self) { Property::reset(self); }
    fn is_edited(&self) -> bool { Property::is_edited(self) }
}

impl<T: Clone + PartialOrd + std::fmt::Debug> PropertyBehavior<T> for ConstrainedProperty<T> {
    fn get(&self) -> T { ConstrainedProperty::get(self) }
    fn set(&mut self, value: T) { let _ = ConstrainedProperty::set(self, value); }
    fn reset(&mut self) { ConstrainedProperty::reset(self); }
    fn is_edited(&self) -> bool { ConstrainedProperty::is_edited(self) }
}

impl<T: Clone + Eq + std::hash::Hash + std::fmt::Debug> PropertyBehavior<T> for EnumProperty<T> {
    fn get(&self) -> T { EnumProperty::get(self) }
    fn set(&mut self, value: T) { let _ = EnumProperty::set(self, value); }
    fn reset(&mut self) { EnumProperty::reset(self); }
    fn is_edited(&self) -> bool { EnumProperty::is_edited(self) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_basic() {
        let mut prop = Property::new(10);
        assert_eq!(prop.get(), 10);
        assert!(!prop.is_edited());
        
        prop.set(20);
        assert_eq!(prop.get(), 20);
        assert!(prop.is_edited());
        
        prop.reset();
        assert_eq!(prop.get(), 10);
        assert!(!prop.is_edited());
    }

    #[test]
    fn test_constrained_property() {
        let mut prop = ConstrainedProperty::new(5, 1, 10);
        assert_eq!(prop.get(), 5);
        
        assert!(prop.set(3).is_ok());
        assert_eq!(prop.get(), 3);
        
        assert!(prop.set(0).is_err());  // Below min
        assert!(prop.set(11).is_err()); // Above max
        
        assert!(prop.set(5).is_ok());
        assert_eq!(prop.get(), 5);
    }

    #[test]
    fn test_enum_property() {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        enum TestEnum { A, B, C }
        
        let mut prop = EnumProperty::new(TestEnum::A, vec![TestEnum::A, TestEnum::B, TestEnum::C]);
        assert!(matches!(prop.get(), TestEnum::A));
        
        assert!(prop.set(TestEnum::B).is_ok());
        assert!(matches!(prop.get(), TestEnum::B));
        
        // This would fail to compile because TestEnum::D doesn't exist
        // but we can test with a string enum
        let mut prop2 = EnumProperty::new("a", vec!["a", "b"]);
        assert!(prop2.set("b").is_ok());
        assert!(prop2.set("c").is_err());
    }

    #[test]
    fn test_flag_property() {
        let mut prop = FlagProperty::new(false);
        assert!(!prop.get());
        
        prop.set(true);
        assert!(prop.get());
        
        prop.toggle();
        assert!(!prop.get());
        
        prop.reset();
        assert!(!prop.get());
    }
}
