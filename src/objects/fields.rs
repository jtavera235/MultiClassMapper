use crate::models::{Access, FieldType};

#[derive(Clone, Debug)]
pub struct Field {
    pub field_type: FieldType,
    pub name: String,
    pub access: Access,
}

impl Field {
    pub fn new(name: String, field_type: FieldType, access: Access) -> Field {
        Field {
            field_type,
            name,
            access,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_field_type(&self) -> FieldType {
        self.field_type.clone()
    }

    pub fn get_access(&self) -> Access {
        self.access.clone()
    }
}
