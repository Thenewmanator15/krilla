# Description
PDF/UA-2 (ISO 14289-2:2024) requires PDF 2.0.

**This mode is incomplete.** Unlike the other files in this folder, this is not yet a
clause-by-clause account of the standard. It lists what krilla does for PDF/UA-2 today and
what is known to be missing. A document that needs one of the missing features is exported
without an error, but does not conform.

See `README.md` for the meaning of each color.

# What krilla does

- krilla only allows PDF 2.0 for this mode. 🟢
- krilla writes `pdfuaid:part` (2) and `pdfuaid:rev` (2024) (clause 5). 🟢
- krilla always sets `DisplayDocTitle` to true for this mode (clause 8.11.2). 🟢
- krilla forces the user to provide a document title. 🟢
- krilla writes the root `Document` structure element in the PDF 2.0 namespace (clause 8.2.5.2). 🟢
- krilla writes its standard structure types in the PDF 1.7 or PDF 2.0 namespace and role
  maps its own types (`Datetime`, `Terms`) to PDF 2.0 types (clause 8.2.4). 🟢
- krilla applies every check it applies for PDF/UA-1 (see `PDF_UA1.md`): codepoint mappings,
  alternative text, heading titles, tagging, font licenses, embedded file descriptions,
  embedded PDFs and the document outline. 🟢

# Known to be missing

- Destinations in the same document have to be structure destinations (clause 8.8). krilla
  writes XYZ and named destinations for link annotations and outline entries. 🔴
- Each `TOCI` has to identify its target with a `Ref` entry (clause 8.2.5.8). krilla does
  not write `Ref`. 🔴
- Footnotes and endnotes have to be `FENote` elements, linked to their references in both
  directions with `Ref`. krilla writes the PDF 1.7 `Note` type. 🔴
- A widget annotation without a label needs a `Contents` entry (clause 8.10.2.3). krilla
  writes the field's alternative name, but no `Contents`. 🔴
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
(a title, a paragraph and a link to a URI) conforms. So do a plain paragraph and a list with
`Decimal` numbering. A link or outline entry with an XYZ destination fails 8.8, a `TOCI`
fails 8.2.5.8, a `Note` fails 8.2.5.14 and a list with labels and `ListNumbering::None` fails
8.2.5.25.
