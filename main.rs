use colored::*;
use image::{/*DynamicImage, GenericImageView, ImageBuffer,*/ Rgba};
use plotters::prelude::*;
use rand::Rng;
use std::time::Instant;
const OUT_FILE_NAME: &str = "histogram.png";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let img = image::open("rocks/rock6.JPG").expect("Failed to open image");
    let mut img = img.grayscale().to_rgba8();
    let (width, height) = img.dimensions();
    let mut npixel: Rgba<u8> = Rgba([0, 0, 0, 0]);
    let mut rng = rand::thread_rng();
    let mut adjblocks: Vec<(u32, u32)> = vec![];
    let mut mastercatalog: Vec<((u8, u8, u8), (u32, u32), (u32, u32), u32, f64)> = vec![]; //Color COMcoords geocoords pixelcount catalogsize
    let mut catalogcolor: Vec<(u8, u8, u8)> = vec![];
    let mut catalogcoords: Vec<(u32, u32)> = vec![];
    let conversionfactor: f64 = 45.0;
    const WHITE: Rgba<u8> = Rgba([255u8, 255u8, 255u8, 255u8]);
    const BLACK: Rgba<u8> = Rgba([0u8, 0u8, 0u8, 255u8]);

    println!("Dimensions: {width} {height}");
    println!(
        "{} {:?}",
        "Converting to Black and White:".red(),
        start.elapsed()
    );
    for i in 0..width {
        //Convert to black and white with threshold = 150
        for j in 0..height {
            let pixel = img.get_pixel(i, j);

            //Exclude Small Objects
            //Count Adjacent Whiets

            if pixel[0] > 150 {
                npixel = WHITE;
            } else {
                npixel = BLACK;
            }
            img.put_pixel(i, j, Rgba([npixel[0], npixel[1], npixel[2], 255]));
        }
    }

    println!("{} {:?}", "Catalogging rocks: ".red(), start.elapsed());
    for j in 1..height - 1 {
        //Find bodies
        for i in 1..width - 1 {
            let pixel = *img.get_pixel(i, j);
            if pixel == WHITE {
                //Scan for new white block
                npixel[0] = rng.gen_range(1..254);
                npixel[1] = rng.gen_range(1..254);
                npixel[2] = rng.gen_range(1..254);
                catalogcolor.push((npixel[0], npixel[1], npixel[2]));
                catalogcoords.push((0, 0));
                adjblocks.push((i, j));
                img.put_pixel(i, j, Rgba([npixel[0], npixel[1], npixel[2], 255]));
                let mut counter = 0;
                let mut boundries = [i, i, j, j];
                while adjblocks.len() > 0 {
                    counter += 1;
                    let l = adjblocks[0].0;
                    let m = adjblocks[0].1;

                    //Finds the left right top bottom extreams to find center
                    if l < boundries[0] {
                        boundries[0] = l;
                    }
                    if l > boundries[1] {
                        boundries[1] = l;
                    }
                    if m < boundries[2] {
                        boundries[2] = m;
                    }
                    if m > boundries[3] {
                        boundries[3] = m;
                    }

                    img.put_pixel(l, m, Rgba([npixel[0], npixel[1], npixel[2], 255]));
                    //Sums the cordinates of every pixel in a group to later be divided to find average position
                    catalogcoords[catalogcolor.len() - 1].0 += l;
                    catalogcoords[catalogcolor.len() - 1].1 += m;

                    if m > 0
                        && *img.get_pixel(l, m - 1) == WHITE
                        && adjblocks.iter().filter(|&n| *n == (l, m - 1)).count() == 0
                    {
                        adjblocks.push((l, m - 1));
                    }
                    if l > 0
                        && *img.get_pixel(l - 1, m) == WHITE
                        && adjblocks.iter().filter(|&n| *n == (l - 1, m)).count() == 0
                    {
                        adjblocks.push((l - 1, m));
                    }
                    if l < width - 1
                        && *img.get_pixel(l + 1, m) == WHITE
                        && adjblocks.iter().filter(|&n| *n == (l + 1, m)).count() == 0
                    {
                        adjblocks.push((l + 1, m));
                    }
                    if m < height - 1
                        && *img.get_pixel(l, m + 1) == WHITE
                        && adjblocks.iter().filter(|&n| *n == (l, m + 1)).count() == 0
                    {
                        adjblocks.push((l, m + 1));
                    }
                    adjblocks.remove(0);
                }

                //Entry into the catalog

                mastercatalog.push((
                    (npixel[0], npixel[1], npixel[2]),
                    (0, 0),
                    (
                        (boundries[0] + boundries[1]) / 2,
                        (boundries[2] + boundries[3]) / 2,
                    ),
                    counter,
                    0.0,
                ));
                let numberwtfisrusteven =
                    ((boundries[1] - mastercatalog[catalogcolor.len() - 1].2 .0).pow(2)
                        + (boundries[3] - mastercatalog[catalogcolor.len() - 1].2 .1).pow(2))
                        as f64;
                let mut nonsence = mastercatalog.len() - 1;
                mastercatalog[nonsence].4 = numberwtfisrusteven.sqrt();

                catalogcoords[catalogcolor.len() - 1].0 /= counter;
                catalogcoords[catalogcolor.len() - 1].1 /= counter;
                //catalogpixelcount.push(counter);

                img.put_pixel(
                    catalogcoords[catalogcolor.len() - 1].0,
                    catalogcoords[catalogcolor.len() - 1].1,
                    Rgba([255, 0, 0, 255]),
                );
                img.put_pixel(
                    mastercatalog[catalogcolor.len() - 1].2 .0,
                    mastercatalog[catalogcolor.len() - 1].2 .1,
                    Rgba([255, 255, 0, 255]),
                );
            }
        }
    }
    img.save("Continuity.png").expect("Failed to save image");

    println!("Drawing Circles {:?}", start.elapsed());

    for i in 1..catalogcolor.len() - 1 {
        for j in (0..360).map(|x| (x as f64)) {
            let coss: f64 = (j / 60.0).cos() as f64;
            let sinn: f64 = (j / 60.0).sin() as f64;

            if mastercatalog[i].2 .0 as f64 - mastercatalog[i].4 >= 1.0
                && mastercatalog[i].2 .0 as f64 + mastercatalog[i].4 < width as f64
            {
                if mastercatalog[i].2 .1 as f64 - mastercatalog[i].4 >= 1.0
                    && mastercatalog[i].2 .1 as f64 + mastercatalog[i].4 < height as f64
                {
                    img.put_pixel(
                        (mastercatalog[i].2 .0 as i32 + (mastercatalog[i].4 * coss) as i32) as u32,
                        (mastercatalog[i].2 .1 as i32 + (mastercatalog[i].4 * sinn) as i32) as u32,
                        Rgba([255, 0, 0, 255]),
                    );
                }
            }
        }
    }

    img.save("Output.png").expect("Failed to save image");

    //Estimating gradations
    /*
    let mut gradation: Vec<u32> = vec![];
    //println!("{:?}",);
    for _i in 1..(mastercatalog.4
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max) as i32)
    {
        gradation.push(0);
    }
    for i in 1..catalogcolor.len() - 2 {
        //gradation[catalogsize[i] as usize] += 1;
    }
    println!("Gradation {:?}", gradation);
    */
    /*
      let root = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();

      root.fill(&WHITE)?;

      let mut chart = ChartBuilder::on(&root)
          .x_label_area_size(35)
          .y_label_area_size(40)
          .margin(5)
          .caption("Histogram Test", ("sans-serif", 50.0))
          .build_cartesian_2d((0u32..100u32).into_segmented(), 0u32..100u32)?;
      chart
          .configure_mesh()
          .disable_x_mesh()
          .y_desc("Count")
          .x_desc("Bucket")
          .axis_desc_style(("sans-serif", 15))
          .draw()?;
      let data = [
          0u32, 1, 1, 1, 4, 2, 5, 7, 8, 6, 4, 2, 1, 8, 3, 3, 3, 4, 4, 3, 3, 3,
      ];
      chart.draw_series(
          Histogram::vertical(&chart)
              .data(gradation.iter().map(|x: &u32| (*x, 1))),
      )?;
      root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
      println!("Result has been saved to {}", OUT_FILE_NAME);

    */

    //For testing purposes
    let mut catalogindex: usize = rng.gen_range(1..catalogcolor.len());
    catalogindex = 100;
    println!(
        "Random object: Color {:?} COM Center is {:?} GEO Center is {:?} Pixel count: {:?} Radius: {:?}",
        mastercatalog[catalogindex].0, catalogcoords[catalogindex], mastercatalog[catalogindex].2, mastercatalog[catalogindex].3, mastercatalog[catalogindex].4
    );

    println!("Object Count: {}", catalogcolor.len());
    println!("{} {:?}", "Done: ".red(), start.elapsed());

    Ok(())
}

//fn entry_point() {
//    main().unwrap()
//}
