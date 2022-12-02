use crate::{
    geometry::surface::SurfaceGeometry,
    objects::{Objects, Surface},
    partial2::PartialObject,
    services::Service,
};

/// A partial [`Surface`]
#[derive(Clone, Default)]
pub struct PartialSurface {
    /// The surface's geometry
    pub geometry: Option<SurfaceGeometry>,
}

impl PartialObject for PartialSurface {
    type Full = Surface;

    fn build(self, _: &mut Service<Objects>) -> Self::Full {
        let geometry = self
            .geometry
            .expect("Can't build `Surface` without geometry");

        Surface::new(geometry)
    }
}
