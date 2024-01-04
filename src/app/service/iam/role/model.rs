use serde::{Serialize, Deserialize};

/// A representation of a user role in the system.
///
/// This struct is used to define and handle properties associated with a user role.
///
/// # Fields
/// - `id`: An integer representing the unique identifier of the role.
/// - `name`: A string holding the name of the role.
///
/// # Examples
///
/// Creating a new `Role`:
///
/// ```
/// let admin_role = Role::new(1, "Administrator");
/// ```
///
/// Using a builder to create a `Role`:
///
/// ```
/// let role = Role::builder()
///     .id(2)
///     .name("Editor")
///     .build();
/// ```
///
/// # Methods
///
/// - `new`: Constructs a new `Role`.
/// - `builder`: Provides a `RoleBuilder` for building a `Role` instance.
#[derive(Serialize, Deserialize, Clone)]
pub struct Role {
    pub id: i32,
    pub name: String,
}

impl Role {
    pub fn new(id: i32, role_name: &str) -> Self {
        Self {
            id,
            name: String::from(role_name),
        }
    }

    pub fn builder() -> RoleBuilder {
        RoleBuilder::new()
    }
}
/// Builder for constructing a `Role` instance.
///
/// Allows for customizable creation of `Role` objects. Each field can be set independently before finalizing the construction.
///
/// # Fields
/// - `id`: An integer representing the role's unique identifier.
/// - `name`: A string specifying the name of the role.
///
/// # Methods
///
/// - `default`: Creates a new `RoleBuilder` with default values for `id` and `name`.
/// - `id` (deprecated): Sets the `id` of the role. This method is deprecated and may be removed in future versions.
/// - `name`: Sets the `name` of the role.
/// - `build`: Finalizes and builds a `Role` instance based on the set values.
///
/// The `build` method clones the `name` field to create a new `Role`, ensuring the builder can be reused or modified later without affecting the created `Role`.
pub struct RoleBuilder {
    pub id: i32,
    pub name: String,
}

impl Default for RoleBuilder {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
        }
    }
}

impl RoleBuilder {
    fn new() -> Self {
        Self {
            id: 0,
            name: String::default(),
        }
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = String::from(name);
        self
    }

    pub fn build(&self) -> Role {
        Role {
            id: self.id,
            name: self.name.clone(),
        }
    }
}