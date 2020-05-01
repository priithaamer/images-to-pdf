use printpdf::*;
use std::fs::File;
use std::io::{BufWriter, Cursor};

const DPI: f64 = 72.0;

fn main() {
    let mut filenames: Vec<String> = vec![];

    for filename in std::env::args().skip(1) {
        filenames.push(filename);
    }

    let doc = create_images_pdf(&filenames);
    save_pdf_file(doc, &String::from("document.pdf"));
}

fn create_images_pdf(filenames: &Vec<String>) -> PdfDocumentReference {
    let doc = PdfDocument::empty("Document");

    for filename in filenames {
        let image = read_image_from_file(&filename);

        let (page, layer) = doc.add_page(
            image.image.width.into_pt(DPI).into(),
            image.image.height.into_pt(DPI).into(),
            "Layer 1",
        );
        let current_layer = doc.get_page(page).get_layer(layer);
        image.add_to_layer(current_layer, None, None, None, None, None, Some(DPI));
    }

    doc
}

fn read_image_from_file(filename: &String) -> Image {
    let image_bytes = std::fs::read(&std::path::Path::new(&filename)).unwrap();
    let mut reader = Cursor::new(image_bytes);

    let decoder = image::jpeg::JpegDecoder::new(&mut reader).unwrap();
    Image::try_from(decoder).unwrap()
}

fn save_pdf_file(pdf: PdfDocumentReference, filename: &String) {
    pdf.save(&mut BufWriter::new(File::create(filename).unwrap()))
        .unwrap();
}
