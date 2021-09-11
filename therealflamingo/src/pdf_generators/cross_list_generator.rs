use std::borrow::Borrow;
use std::fs::File;
use std::io::BufWriter;
use printpdf::*;

struct NameAmount {
    name: String,
    amount: f64,
    draw_squares: bool,
}


pub fn generate_cross_list(names: String) -> Vec<u8> {
    let (page_height, page_width) = (420.0, 297.0);
    let box_height: f64 = 15.0;
    let line_width: f64 = 4.0;
    let small_line_width: f64 = 1.0;
    let names_pr_page :i64 = ((page_height - 2.0 * box_height) / box_height) as i64;
    let name_width = page_width / 90.0 * 20.0;
    let name_font_size = 16.0;

    let (doc, page1, layer1) = PdfDocument::new(
        "Krydsliste",
        Mm(page_width),
        Mm(page_height),
        "Side 1",
    );
    let mut current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc
        .add_external_font(File::open("fonts/times.ttf").unwrap())
        .unwrap();

    draw_header(current_layer.borrow(), font.borrow(), box_height, line_width, small_line_width, page_width, page_height);

    let mut name_vector: Vec<&str> = names.split('\n').collect();
    let missing_names = names_pr_page as usize - (name_vector.len() % names_pr_page as usize);
    println!("{},{},{}", missing_names, name_vector.len(), names_pr_page);
    if missing_names != 0 {
        for _ in 0..missing_names {
            name_vector.push("");
        }
    }
    let mut index = 0;
    for str in name_vector.clone() {
        current_layer.set_outline_thickness(small_line_width);
        let name_splitter_height = page_height - (3 + (index % names_pr_page)) as f64 * box_height;
        let p1 = Point::new(Mm(name_width), Mm(name_splitter_height + box_height / 2.0));
        let p2 = Point::new(Mm(page_width), Mm(name_splitter_height + box_height / 2.0));
        current_layer.add_shape(draw_line(p1, p2));

        current_layer.set_outline_thickness(line_width);
        let p3 = Point::new(Mm(0.0), Mm(name_splitter_height));
        let p4 = Point::new(Mm(page_width), Mm(name_splitter_height));
        current_layer.add_shape(draw_line(p3,p4));

        current_layer.begin_text_section();
        current_layer.set_font(&font, name_font_size);
        current_layer.set_text_cursor(Mm(1.0),Mm(name_splitter_height + 2.0));
        current_layer.set_text_rendering_mode(TextRenderingMode::Fill);
        let mut s = str.to_string();
        if s.len() > 24 {
            s = format!("{}...", &s.clone()[..24]);
        }
        current_layer.write_text(s, &font);
        current_layer.end_text_section();

        index += 1;
        if index % names_pr_page == 0 && index as usize != name_vector.len() {
            let (page_index, page_layer_index) = doc.add_page(Mm(page_width), Mm(page_height), format!("Side {}", index / names_pr_page + 1));
            current_layer = doc.get_page(page_index).get_layer(page_layer_index);
            draw_header(current_layer.borrow(), font.borrow(), box_height, line_width, small_line_width, page_width, page_height);
        }
    }

    let mut v = Vec::<u8>::new();
    match doc.save(&mut BufWriter::new(&mut v)) {
        Err(_) => return Vec::<u8>::new(),
        _ => ()
    }
    v
}

fn draw_header(layer: &PdfLayerReference, font: &IndirectFontRef, box_height: f64, line_width: f64, small_line_width: f64, page_width: f64, page_height: f64) {
    let font_size: f64 = 60.0;
    let header_items: Vec<NameAmount> = vec![
        NameAmount {
            name: String::from("Navn:"),
            amount: 20.0,
            draw_squares: false,
        }, NameAmount {
            name: String::from("ØL"),
            amount: 30.0,
            draw_squares: true,
        }, NameAmount {
            name: String::from("Vand"),
            amount: 20.0,
            draw_squares: true,
        }, NameAmount {
            name: String::from("GD"),
            amount: 20.0,
            draw_squares: true,
        },
    ];

    layer.set_outline_thickness(line_width);

    let header_horizontal_line: f64 = page_height - box_height * 2.0;
    let squares_total: f64 = header_items.iter().map(|x| x.amount).sum();
    let square_size: f64 = page_width / squares_total;
    let mut current_line_offset: f64 = 0.0;

    let horizontal_line = draw_line(
        Point::new(Mm(0.0), Mm(header_horizontal_line)),
        Point::new(Mm(page_width), Mm(header_horizontal_line)),
    );
    layer.add_shape(horizontal_line);

    for item in header_items.iter() {
        layer.begin_text_section();
        layer.set_font(font, font_size);
        layer.set_text_cursor(Mm(current_line_offset + 5.0), Mm(header_horizontal_line + 5.0));
        layer.set_text_rendering_mode(TextRenderingMode::Fill);
        layer.write_text(item.name.as_str(), font);
        layer.end_text_section();

        if item.draw_squares {
            layer.set_outline_thickness(small_line_width);
            for i in 0..item.amount as i64 {
                let x_off_set = current_line_offset + i as f64 * square_size;
                let line = draw_line(
                    Point::new(Mm(x_off_set), Mm(header_horizontal_line)),
                    Point::new(Mm(x_off_set), Mm(0.0)),
                );

                layer.add_shape(line)
            }
        }

        current_line_offset += item.amount * square_size;
        layer.set_outline_thickness(line_width);
        let line: Line = draw_line(
            Point::new(Mm(current_line_offset), Mm(page_height)),
            Point::new(Mm(current_line_offset), Mm(0.0)),
        );

        layer.add_shape(line);
    }
}

fn draw_line(p1: Point, p2: Point) -> Line {
    // Quadratic shape. The "false" determines if the next (following)
    // point is a bezier handle (for curves)
    // If you want holes, simply reorder the winding of the points to be
    // counterclockwise instead of clockwise.
    let points = vec![(p1, false), (p2, false)];

    // Is the shape stroked? Is the shape closed? Is the shape filled?
    let line = Line {
        points,
        is_closed: true,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };

    line
}
