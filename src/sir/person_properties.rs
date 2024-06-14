pub enum DiseaseStatus {
    S,
    I,
    R,
    D,
}
eosim::define_person_property_from_enum!(DiseaseStatus, DiseaseStatus::S);

pub enum AgeGroup {
    Child,
    Adult,
}

eosim::define_person_property_from_enum!(AgeGroup, AgeGroup::Child);
