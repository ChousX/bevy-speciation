use crate::{
    anatomical_features::{AnatomicalFeature, MandibleStructure, SensoryOrgan},
    appendage::{AppendageClass, Terminus},
    body::Torso,
    head::Cranium,
    organism::Organism,
    primitives::*,
    sockets_symmetry::BodySymmetry,
    surface::*,
    tissue_muscle::MuscleIntensity,
    validation_errors::{GenerationError, SpeciesValidationError},
};
use bevy::prelude::{Dir3, LinearRgba};
use rand::prelude::*;

#[derive(Clone, Debug)]
pub struct IntegumentGenes {
    pub base_color: ValueRange<LinearRgba>,
    pub allowed_patterns: NonEmpty<SurfacePattern>,
    pub roughness: ValueRange<Roughness>,
    pub metallic: ValueRange<Metallic>,
}

#[derive(Clone, Debug)]
pub struct TissueEnvelopeGenes {
    pub profile: CrossSectionProfile,
    pub radius_range: ValueRange<Curve>,
    pub bulge_count: InclusiveRange<Count>,
    pub bulge_intensity: ValueRange<MuscleIntensity>,
}

#[derive(Clone, Debug)]
pub struct BoneGenes {
    pub length: ValueRange<Length>,
    pub tissue: TissueEnvelopeGenes,
}

#[derive(Clone, Debug)]
pub struct LimbGenes {
    pub segment_count: InclusiveRange<Count>,
    pub segment: BoneGenes,
    pub allowed_termini: NonEmpty<Terminus>,
    pub branching_probability: Normalized,
}

#[derive(Clone, Debug)]
pub struct AppendageGenes {
    pub class: AppendageClass,
    pub limb: LimbGenes,
    pub patagium_probability: Normalized,
    pub integument: IntegumentGenes,
}

#[derive(Clone, Debug)]
pub struct SocketPlacement {
    pub position: LocalPosition,
    pub normal: Dir3,
}

#[derive(Clone, Debug)]
pub enum SymmetricPlacement {
    Medial(SocketPlacement),
    Lateral { offset: LocalPosition, normal: Dir3 },
}

#[derive(Clone, Debug)]
pub struct AppendageSocketRule {
    pub vertebra_indices: NonEmpty<VertebraIndex>,
    pub placement: SymmetricPlacement,
    pub allowed: NonEmpty<AppendageGenes>,
    pub required: bool,
}

#[derive(Clone, Debug)]
pub struct FeatureSocketRule {
    pub vertebra_indices: NonEmpty<VertebraIndex>,
    pub placement: SymmetricPlacement,
    pub allowed: NonEmpty<AnatomicalFeature>,
    pub required: bool,
}

#[derive(Clone, Debug)]
pub struct SensorySocketRule {
    pub placement: SymmetricPlacement,
    pub allowed: NonEmpty<SensoryOrgan>,
    pub required: bool,
}

#[derive(Clone, Debug)]
pub struct CraniumGenes {
    pub bone: BoneGenes,
    pub sensory_sockets: Vec<SensorySocketRule>,
    pub mandible: Option<NonEmpty<MandibleStructure>>,
    pub feature_sockets: Vec<FeatureSocketRule>,
    pub integument: IntegumentGenes,
}

#[derive(Clone, Debug)]
pub struct SpineGenes {
    pub vertebra_count: InclusiveRange<Count>,
    pub vertebra: BoneGenes,
    pub appendage_sockets: Vec<AppendageSocketRule>,
    pub feature_sockets: Vec<FeatureSocketRule>,
}

#[derive(Clone, Debug)]
pub struct TorsoGenes {
    pub spine: SpineGenes,
    pub base_tissue: TissueEnvelopeGenes,
    pub integument: IntegumentGenes,
}

#[derive(Clone, Debug)]
pub struct UnvalidatedSpecies {
    pub name: String,
    pub symmetry: BodySymmetry,
    pub head: CraniumGenes,
    pub torso: TorsoGenes,
}

#[derive(Clone, Debug)]
pub struct Species {
    name: String,
    symmetry: BodySymmetry,
    head: CraniumGenes,
    torso: TorsoGenes,
}

impl Species {
    /// Validate and construct a Species from unvalidated input
    pub fn new(input: UnvalidatedSpecies) -> Result<Self, SpeciesValidationError> {
        let max_vertebrae = *input.torso.spine.vertebra_count.end();

        // Validate appendage socket indices
        for socket_rule in &input.torso.spine.appendage_sockets {
            for idx in socket_rule.vertebra_indices.iter() {
                if idx.0 >= max_vertebrae.value() {
                    return Err(SpeciesValidationError::SocketIndexOutOfBounds {
                        socket_index: *idx,
                        max_vertebrae,
                    });
                }
            }
        }

        // Validate feature socket indices
        for socket_rule in &input.torso.spine.feature_sockets {
            for idx in socket_rule.vertebra_indices.iter() {
                if idx.0 >= max_vertebrae.value() {
                    return Err(SpeciesValidationError::SocketIndexOutOfBounds {
                        socket_index: *idx,
                        max_vertebrae,
                    });
                }
            }
        }

        Ok(Self {
            name: input.name,
            symmetry: input.symmetry,
            head: input.head,
            torso: input.torso,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn symmetry(&self) -> BodySymmetry {
        self.symmetry
    }
    pub fn head(&self) -> &CraniumGenes {
        &self.head
    }
    pub fn torso(&self) -> &TorsoGenes {
        &self.torso
    }

    pub fn min_vertebrae(&self) -> Count {
        *self.torso.spine.vertebra_count.start()
    }

    pub fn max_vertebrae(&self) -> Count {
        *self.torso.spine.vertebra_count.end()
    }

    /// Generate a valid Organism from this Species using the given seed
    pub fn generate(&self, seed: GenomeSeed) -> Result<Organism, GenerationError> {
        let mut rng = Self::rng_from_seed(seed);

        let vertebra_count = self.sample_vertebra_count(&mut rng);
        let head = self.generate_cranium(&mut rng)?;
        let torso = self.generate_torso(&mut rng, vertebra_count)?;

        Ok(Organism {
            genome_seed: seed,
            symmetry: self.symmetry,
            head,
            torso,
        })
    }

    fn rng_from_seed(_seed: GenomeSeed) -> impl Rng {
        // Placeholder: use seed to create deterministic RNG
        todo!("Implement seeded RNG")
    }

    fn sample_vertebra_count(&self, _rng: &mut impl Rng) -> Count {
        todo!("Sample from vertebra_count range")
    }

    fn generate_cranium(&self, _rng: &mut impl Rng) -> Result<Cranium, GenerationError> {
        todo!("Generate cranium from CraniumGenes")
    }

    fn generate_torso(
        &self,
        _rng: &mut impl Rng,
        _vertebra_count: Count,
    ) -> Result<Torso, GenerationError> {
        todo!("Generate torso from TorsoGenes")
    }
}
