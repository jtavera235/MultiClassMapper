use crate::models::FieldType;

#[derive(Clone, Debug)]
pub struct Field {
    pub field_type: FieldType,
    pub name: String,
}

impl Field {
    pub fn new(name: String, field_type: FieldType) -> Field {
        Field {
            field_type,
            name,
        }
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn get_field_type(&self) -> FieldType {
        return self.field_type.clone();
    }
}
