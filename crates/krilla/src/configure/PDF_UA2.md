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
  structure destination (ISO 32000-2, 12.3.2.3): in the `Dest` of link annotations and
  outline entries, and as the `SD` of a go-to action, next to a `D` that leads to the page
  (ISO 32000-2, table 202). A named destination that leads to a tag is a dictionary with
  the page destination as `D` and the structure destination as `SD` (ISO 32000-2,
  12.3.2.4). It checks that the tag exists (clause 8.8). 🟢
- krilla writes the `mathml` attribute of a `Formula` as a file associated with the tag,
  with the relationship `Supplement` (clause 8.2.5.29.1; ISO 32000-2, 14.13). Equal MathML
  is written once. An associated file is an embedded file, so PDF/A-4 does not allow it and
  PDF/A-4f is needed for a document that has to be both. 🟢
- krilla writes the `note_type` attribute of a `Note` as the `NoteType` of the `FENote`
  (clause 8.2.5.14.2), and the `aria_role` attribute of any tag as the `role` of the
  `ARIA-1.1` attribute owner (clause 8.2.6.4). 🟢
- krilla has the `Artifact` tag for an artifact that only means something next to real
  content, such as a line number (clause 8.3.2), and writes its kind as the `Type` and
  `Subtype` of the `Artifact` attribute owner. Using it where it applies is up to the
  user. 🟣
- krilla does not require an alternative description on a `Formula` that has MathML.
  Clause 8.2.5.29.2 asks for one only on a formula that is not mathematical. 🟢
- krilla rejects link annotations, go-to actions and outline entries whose destination does
  not lead to a tag (clause 8.8). 🟢
- krilla applies the checks it applies for PDF/UA-1 (see `PDF_UA1.md`): codepoint mappings,
  alternative text, heading titles, tagging, font licenses, embedded file descriptions and
  embedded PDFs. It does not require a document outline, which clause 8.12.2 only
  recommends for longer documents. 🟢

# Known to be missing

- A named destination that is registered with the document but never linked to is not
  checked. 🟠
- krilla does not require MathML on a `Formula`, because it cannot tell a mathematical
  expression from another kind of formula, which does not need it (clause 8.2.5.29.2).
  veraPDF does not check this either, so it is up to the user. 🟣
- krilla cannot write MathML as structure elements, only as an associated file. 🔴
- A link to a target in the same document should be a `Reference` rather than a `Link`
  (clause 8.2.5.20). This is up to the user. 🟣
- Leaders in a table of contents have to be artifacts (clause 8.2.5.8). This is up to the
  user. 🟣
- The section that holds a bibliography needs the role `doc-bibliography` (clause
  8.2.5.31), which the `aria_role` attribute writes. Using it is up to the user. 🟣
- A list whose items have `Lbl` elements needs a `ListNumbering` other than `None` (clause
  8.2.5.25). This is up to the user and not checked. The PDF 2.0 values `Ordered`,
  `Unordered` and `Description` are there for lists that none of the others fit, such as
  a list of terms. 🟣
- A caption has to be a child of the element that encloses what it captions (clause
  8.2.5.27). This is up to the user. The PDF 2.0 `Aside` tag can enclose a figure and its
  caption where no other element does. 🟣
- The `Document` element must not contain content items directly (ISO 32005, table 5). This
  is up to the user and not checked. 🟣

# Read against

ISO 14289-2:2024 clauses 5, 8.2.4, 8.2.5.2, 8.2.5.8, 8.2.5.12, 8.2.5.14, 8.2.5.20, 8.2.5.25,
8.2.5.27 to 8.2.5.29, 8.8, 8.9.3.3, 8.10.2.3 and 8.12.2, and ISO 32000-2:2020 12.3.2.3,
12.3.2.4 and tables 202, 355, 372 and 382. The rest of the standard has not been gone
through for this file.

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
