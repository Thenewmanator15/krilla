//! Destinations in a PDF document.
//!
//! In some cases, you might want to refer to other locations within the same document, for
//! example when defining the outline, or when link to a different section in the document
//! from a link. To achieve this, you can use destinations, which are associated with a page
//! and a specific location on that page.

use std::hash::{Hash, Hasher};
use std::sync::Arc;

use pdf_writer::{Obj, Ref, Str};
use tiny_skia_path::Transform;

use crate::chunk_container::ChunkContainer;
use crate::configure::ValidationError;
use crate::error::{KrillaError, KrillaResult};
use crate::geom::Point;
use crate::interchange::tagging::TagId;
use crate::serialize::{PageInfo, SerializeContext};
use crate::surface::Location;

/// The type of destination.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Destination {
    /// An XYZ destination.
    Xyz(XyzDestination),
    /// A named destination.
    Named(NamedDestination),
}

impl Destination {
    pub(crate) fn serialize(
        &self,
        sc: &mut SerializeContext,
        buffer: Obj,
        location: Option<Location>,
    ) -> KrillaResult<()> {
        if self.tag().is_none() {
            sc.register_validation_error(ValidationError::NonStructureDestination(location));
        }

        self.write(sc, buffer)
    }

    /// The tag that the destination leads to, if any.
    pub(crate) fn tag(&self) -> Option<&TagId> {
        match self {
            Destination::Xyz(xyz) => xyz.tag(),
            Destination::Named(named) => named.xyz_dest.tag(),
        }
    }

    /// Whether the destination is written as a structure destination.
    pub(crate) fn is_structure_destination(&self, sc: &SerializeContext) -> bool {
        self.tag().is_some() && sc.writes_structure_destinations()
    }

    /// The same destination, but leading to the page rather than to a tag. A named
    /// destination stays as it is, since its name can only stand for one destination.
    pub(crate) fn to_page_destination(&self) -> Self {
        match self {
            Destination::Xyz(xyz) => Destination::Xyz(xyz.without_tag()),
            Destination::Named(named) => Destination::Named(named.clone()),
        }
    }

    /// Write the destination without checking it against the validators.
    pub(crate) fn write(&self, sc: &mut SerializeContext, buffer: Obj) -> KrillaResult<()> {
        match self {
            Destination::Xyz(xyz) => {
                let ref_ = sc.register_xyz_destination(xyz.clone());
                buffer.primitive(ref_);

                Ok(())
            }
            Destination::Named(named) => named.serialize(sc, buffer),
        }
    }
}

/// A destination associated with a name.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct NamedDestination {
    pub(crate) name: Arc<String>,
    pub(crate) xyz_dest: Arc<XyzDestination>,
}

impl From<NamedDestination> for Destination {
    fn from(val: NamedDestination) -> Self {
        Destination::Named(val)
    }
}

impl NamedDestination {
    /// Create a new named destination.
    ///
    /// When used as part of a link annotation, the destination will be automatically registered
    /// with the [`Document`](crate::Document).
    ///
    /// That said, you can also manually register a destination without linking to it by calling
    /// [`Document::register_named_destination`](crate::Document::register_named_destination).
    pub fn new(name: String, xyz_dest: XyzDestination) -> Self {
        Self {
            name: Arc::new(name),
            xyz_dest: Arc::new(xyz_dest),
        }
    }

    pub(crate) fn serialize(
        &self,
        sc: &mut SerializeContext,
        destination: Obj,
    ) -> KrillaResult<()> {
        sc.register_named_destination(self.clone())
            .ok_or_else(|| KrillaError::DuplicateNamedDestination(Arc::clone(&self.name)))?;
        destination.primitive(Str(self.name.as_bytes()));

        Ok(())
    }
}

#[derive(Debug)]
struct XyzDestRepr {
    page_index: usize,
    point: Point,
    tag: Option<TagId>,
}

impl Hash for XyzDestRepr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.page_index.hash(state);
        self.point.x.to_bits().hash(state);
        self.point.y.to_bits().hash(state);
        self.tag.hash(state);
    }
}

impl PartialEq for XyzDestRepr {
    fn eq(&self, other: &Self) -> bool {
        self.page_index == other.page_index
            && self.point.x == other.point.x
            && self.point.y == other.point.y
            && self.tag == other.tag
    }
}

impl Eq for XyzDestRepr {}

/// A destination pointing to a specific location at a specific page.
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct XyzDestination(Arc<XyzDestRepr>);

impl From<XyzDestination> for Destination {
    fn from(val: XyzDestination) -> Self {
        Destination::Xyz(val)
    }
}

impl XyzDestination {
    /// Create a new XYZ destination. `page_index` should be the index (i.e. number) of the
    /// target page, and point indicates the specific location on that page that should be
    /// targeted. If the `page_index` is out of range, export will panic.
    pub fn new(page_index: usize, point: Point) -> Self {
        Self(Arc::new(XyzDestRepr {
            page_index,
            point,
            tag: None,
        }))
    }

    /// Let the destination lead to the tag with the given id, which makes it a structure
    /// destination.
    ///
    /// A structure destination names a tag (a structure element) as its target rather
    /// than a page. The page and point are still used for the position to scroll to.
    /// PDF/UA-2 requires every destination in the same document to be one.
    ///
    /// Structure destinations are only written for PDF 2.0 and when the document has a
    /// tag tree. Otherwise, the tag is ignored. The tag tree has to contain a tag with
    /// this id.
    pub fn with_tag(self, tag: TagId) -> Self {
        Self(Arc::new(XyzDestRepr {
            page_index: self.0.page_index,
            point: self.0.point,
            tag: Some(tag),
        }))
    }

    /// The tag that the destination leads to, if any.
    pub(crate) fn tag(&self) -> Option<&TagId> {
        self.0.tag.as_ref()
    }

    /// The same destination, but leading to the page rather than to a tag.
    pub(crate) fn without_tag(&self) -> Self {
        Self::new(self.0.page_index, self.0.point)
    }

    pub(crate) fn serialize(
        &self,
        sc: &mut SerializeContext,
        chunk_container: &mut ChunkContainer,
        root_ref: Ref,
    ) {
        let chunk = &mut chunk_container.non_stream.destinations;
        let destination = chunk.destination(root_ref);

        let page_info = sc.page_infos().get(self.0.page_index).unwrap_or_else(|| {
            panic!(
                "attempted to link to page {}, but document only has {} pages",
                self.0.page_index + 1,
                sc.page_infos().len()
            )
        });

        let (ref_, surface_size) = match page_info {
            PageInfo::Krilla {
                ref_, surface_size, ..
            } => (ref_, surface_size),
            PageInfo::Pdf { ref_, size, .. } => (ref_, size),
        };

        let page_ref = *ref_;
        let page_size = surface_size.height();

        let mut mapped_point = self.0.point.to_tsp();
        // Convert to PDF coordinates
        let invert_transform = Transform::from_row(1.0, 0.0, 0.0, -1.0, 0.0, page_size);
        invert_transform.map_point(&mut mapped_point);

        // A structure destination has a structure element in place of the page.
        let target = self
            .tag()
            .and_then(|id| sc.structure_destination_target(id))
            .unwrap_or(page_ref);

        destination
            .page(target)
            .xyz(mapped_point.x, mapped_point.y, None);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::annotation::{LinkAnnotation, Target};
    use crate::error::KrillaError;
    use crate::geom::{Point, Rect};
    use crate::Document;

    use super::{NamedDestination, XyzDestination};

    #[test]
    fn named_duplicate_rejected() {
        let mut document = Document::new();
        assert_eq!(
            document.register_named_destination(NamedDestination::new(
                "same".to_string(),
                XyzDestination::new(0, Point::from_xy(0.0, 0.0)),
            )),
            Some(())
        );
        assert_eq!(
            document.register_named_destination(NamedDestination::new(
                "same".to_string(),
                XyzDestination::new(0, Point::from_xy(100.0, 100.0)),
            )),
            None
        );
    }

    #[test]
    fn named_duplicate_same_location_allowed() {
        let mut document = Document::new();
        assert_eq!(
            document.register_named_destination(NamedDestination::new(
                "same".to_string(),
                XyzDestination::new(0, Point::from_xy(0.0, 0.0)),
            )),
            Some(())
        );
        assert_eq!(
            document.register_named_destination(NamedDestination::new(
                "same".to_string(),
                XyzDestination::new(0, Point::from_xy(0.0, 0.0)),
            )),
            Some(())
        );

        assert!(document.finish().is_ok());
    }

    #[test]
    fn named_duplicate_annotation_rejected() {
        let mut document = Document::new();
        let mut page = document.start_page();

        page.add_annotation(
            LinkAnnotation::new(
                Rect::from_xywh(0.0, 0.0, 100.0, 100.0).unwrap(),
                Target::Destination(
                    NamedDestination::new(
                        "same".to_string(),
                        XyzDestination::new(0, Point::from_xy(0.0, 0.0)),
                    )
                    .into(),
                ),
            )
            .into(),
        );
        page.add_annotation(
            LinkAnnotation::new(
                Rect::from_xywh(0.0, 100.0, 100.0, 100.0).unwrap(),
                Target::Destination(
                    NamedDestination::new(
                        "same".to_string(),
                        XyzDestination::new(0, Point::from_xy(100.0, 100.0)),
                    )
                    .into(),
                ),
            )
            .into(),
        );
        drop(page);

        assert_eq!(
            document.finish(),
            Err(KrillaError::DuplicateNamedDestination(Arc::new(
                "same".to_string()
            )))
        );
    }

    #[test]
    fn named_duplicate_manual_then_annotation_rejected() {
        let mut document = Document::new();
        assert_eq!(
            document.register_named_destination(NamedDestination::new(
                "same".to_string(),
                XyzDestination::new(0, Point::from_xy(0.0, 0.0)),
            )),
            Some(())
        );

        let mut page = document.start_page();
        page.add_annotation(
            LinkAnnotation::new(
                Rect::from_xywh(0.0, 0.0, 100.0, 100.0).unwrap(),
                Target::Destination(
                    NamedDestination::new(
                        "same".to_string(),
                        XyzDestination::new(0, Point::from_xy(100.0, 100.0)),
                    )
                    .into(),
                ),
            )
            .into(),
        );
        drop(page);

        assert_eq!(
            document.finish(),
            Err(KrillaError::DuplicateNamedDestination(Arc::new(
                "same".to_string()
            )))
        );
    }
}
