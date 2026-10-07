# Description
PDF/UA-2 (ISO 14289-2:2024) requires PDF 2.0.

**This mode is incomplete.** Unlike the other files in this folder, this is not yet a
clause-by-clause account of the standard. It lists what krilla does for PDF/UA-2 today and
what is known to be missing.

See `README.md` for the meaning of each color.

# What krilla does

- krilla only allows PDF 2.0 for this mode. 🟢
- krilla writes `pdfuaid:part` (2) and `pdfuaid:rev` (2024) (clause 5). 🟢
- krilla always sets `DisplayDocTitle` to true for this mode (clause 8.11.2). 🟢
- krilla forces the user to provide a document title. 🟢
- krilla writes the root `Document` structure element in the PDF 2.0 namespace (clause 8.2.5.2). 🟢
- krilla writes its standard structure types in the PDF 1.7 or PDF 2.0 namespace and role
  maps its own types (`Datetime`, `Terms`) to PDF 2.0 types (clause 8.2.4). 🟢
- krilla writes `FENote` for the `Note` tag, since PDF/UA-2 does not allow `Note` (clause
  8.2.5.14). 🟢
- krilla writes a `Contents` entry on every widget annotation of a field with an alternative
  name, and requires that name (clause 8.10.2.3). The name has to be set before the widget
  is added to the page. If the enclosing `Form` element has an `Alt`, it has to be the same
  text (clause 8.9.4.2), which is up to the user. 🟢
- krilla writes the `refs` attribute of a tag as its `Ref` entry, and checks that every id
  in it belongs to a tag. 🟢
- krilla requires a `TOCI` to have `refs`, on itself or on one of its descendants (clause
  8.2.5.8). 🟢
- krilla requires a `Note` to have `refs`, on itself or on one of its descendants (clause
  8.2.5.14). That the content citing the note refers back to it is up to the user. 🟣
- krilla writes a destination that leads to a tag (`XyzDestination::with_tag`) as a
  structure destination: in the `Dest` of link annotations and outline entries, as the
  target of a named destination, and as the `SD` of a go-to action, next to a `D` that
  leads to the page. It checks that the tag exists (clause 8.8). 🟢
- krilla rejects link annotations, go-to actions and outline entries whose destination does
  not lead to a tag (clause 8.8). 🟢
- krilla applies every check it applies for PDF/UA-1 (see `PDF_UA1.md`): codepoint mappings,
  alternative text, heading titles, tagging, font licenses, embedded file descriptions,
  embedded PDFs and the document outline. 🟢

# Known to be missing

- A go-to action with a named structure destination gets the same name as `D` and `SD`,
  so its `D` leads to a structure destination as well. Whether that is allowed for `D` has
  not been checked against ISO 32000-2. 🟠
- A named destination that is registered with the document but never linked to is not
  checked. 🟠
- krilla does not write the `NoteType` attribute of `FENote`, which then has its default
  value `None`. 🔴
- A list whose items have `Lbl` elements needs a `ListNumbering` other than `None` (clause
  8.2.5.25). This is up to the user and not checked. krilla does not have the PDF 2.0
  values `Ordered`, `Unordered` and `Description`. 🟣
- The `Document` element must not contain content items directly (ISO 32005, table 5). This
  is up to the user and not checked. 🟣
- Mathematical expressions can be given as MathML. krilla has no way to attach it. 🔴
- Whether a document outline is required as in PDF/UA-1 has not been checked against the
  standard; krilla requires one for now, which is the stricter choice. 🟠

# Checked with

veraPDF 1.30.3, profile "PDF/UA-2 + Tagged PDF": the `validate_pdf_ua2_example` test document
(a title, a paragraph and a link to a URI) conforms, and so do
`validate_pdf_ua2_form_and_footnote` (a footnote and its citation, and a push button) and
`validate_pdf_ua2_toc` (a table of contents item and its heading) and
`validate_pdf_ua2_structure_destinations` (a link annotation, a go-to action, a named
destination and an outline entry that lead to a heading). So do a plain paragraph
and a list with `Decimal` numbering. A list with labels and `ListNumbering::None` fails
8.2.5.25. Before krilla rejected them, a link or outline entry with an XYZ destination
failed 8.8 and a `TOCI` without `Ref` failed 8.2.5.8. veraPDF does not check that a
footnote's citation has a `Ref`; it only checks that the `Ref` entries that exist match up.
