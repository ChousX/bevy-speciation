use crate::{
    body::Torso,
    head::Cranium,
    primitives::*,
    sockets_symmetry::{BilateralPair, BodySymmetry, SymmetricSocket},
    species::Species,
    validation_errors::OrganismValidationError,
};

#[derive(Clone, Debug)]
pub struct Organism {
    pub(crate) genome_seed: GenomeSeed,
    pub(crate) symmetry: BodySymmetry,
    pub(crate) head: Cranium,
    pub(crate) torso: Torso,
}

impl Organism {
    pub fn genome_seed(&self) -> GenomeSeed {
        self.genome_seed
    }
    pub fn symmetry(&self) -> BodySymmetry {
        self.symmetry
    }
    pub fn head(&self) -> &Cranium {
        &self.head
    }
    pub fn torso(&self) -> &Torso {
        &self.torso
    }
}

#[derive(Clone, Debug)]
pub struct UnvalidatedOrganism {
    pub genome_seed: GenomeSeed,
    pub symmetry: BodySymmetry,
    pub head: Cranium,
    pub torso: Torso,
}

impl Organism {
    /// Validate a manually constructed organism against a species
    pub fn validate(
        input: UnvalidatedOrganism,
        species: &Species,
    ) -> Result<Self, OrganismValidationError> {
        let vertebra_count = input.torso.spine.vertebrae.len();

        // Validate appendage attachment indices
        for attachment in &input.torso.spine.appendages {
            if attachment.vertebra_index.0 as usize >= vertebra_count {
                return Err(OrganismValidationError::VertebraIndexOutOfBounds {
                    index: attachment.vertebra_index,
                    vertebra_count,
                });
            }
        }

        // Validate feature attachment indices
        for attachment in &input.torso.spine.features {
            if attachment.vertebra_index.0 as usize >= vertebra_count {
                return Err(OrganismValidationError::VertebraIndexOutOfBounds {
                    index: attachment.vertebra_index,
                    vertebra_count,
                });
            }
        }

        // Validate required sockets are filled
        for socket_rule in &species.torso().spine.appendage_sockets {
            if socket_rule.required {
                // Check that at least one valid vertebra has an attachment
                let has_attachment = socket_rule.vertebra_indices.iter().any(|idx| {
                    input.torso.spine.appendages.iter().any(|a| {
                        a.vertebra_index == *idx
                            && matches!(&a.socket,
                                SymmetricSocket::Medial(s) |
                                SymmetricSocket::Lateral(BilateralPair { left: s, .. })
                                if s.attachment.is_some()
                            )
                    })
                });

                if !has_attachment {
                    return Err(OrganismValidationError::RequiredSocketEmpty {
                        vertebra_index: *socket_rule.vertebra_indices.first(),
                    });
                }
            }
        }

        Ok(Self {
            genome_seed: input.genome_seed,
            symmetry: input.symmetry,
            head: input.head,
            torso: input.torso,
        })
    }
}
