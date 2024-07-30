use std::io;
use std::path::Path;
use std::error::Error;
use std::fs::File;
use plotters::prelude::*;
use csv::ReaderBuilder;
use rand::Rng;

fn main() {
    loop {
    println!("\n_________________________________\n|    select the type of chart   |\n|                               |\n| 1. Pie chart                  |\n| 2. Bar chart                  |\n| 3. Area chart                 |\n| 4. Scatter plot               |\n|                               |\n|   type 'qp' to exit program.  |\n|_______________________________|\n");
    let u_ans = get_input("Enter your choice(1, 2, 3, 4, qp):");

    if u_ans == "qp" {
        print!("closing program...");
        std::process::exit(0);
    }

    if !["1", "2", "3", "4"].contains(&u_ans.as_str()) {
        println!("\n**************************\nError: Invalid Choice.\nPlease try again.\n\n**************************");
        continue;
    }
    
    match u_ans.as_str() {
        "1" => {
            let file_in = get_file("Enter File name (don't need to type '.CSV'):");
            let file_out = check_file("Enter output file name:", "pie_output");
            if let Err(_err) = draw_pie_chart(&file_in, &file_out) {
                eprintln!("\n********** Error: {:?} **********", _err);
            } else {
            println!("\n********** Generating: {} **********",file_out.1);}

        }
        "2" => {
            let file_in = get_file("Enter File name (don't need to type '.CSV'):");
            let file_out = check_file("Enter output file name:", "bar_output");
            if let Err(_err) = draw_bar_chart(&file_in, &file_out) {
                eprintln!("\n********** Error: {:?} **********", _err);
            } else {
            println!("\n********** Generating: {} **********",file_out.1);}
                }
        
        "3" => {
            let file_in = get_file("Enter File name (don't need to type '.CSV'):");
            let file_out = check_file("Enter output file name:", "line_output"); // Change the output folder name if needed
            if let Err(_err) = draw_line_chart(&file_in, &file_out) {
                eprintln!("\n********** Error: {:?} **********", _err);
            } else {
            println!("\n********** Generating: {} **********",file_out.1);}
        }

        "4" => {
            let file_in = get_file("Enter File name (don't need to type'.CSV'):");
            let file_out = check_file("Enter output file name:","scatter_output");
            if let Err(_err) = draw_scatter_plot(&file_in, &file_out) {
                eprintln!("\n********** Error: {:?} **********", _err);
            } else {
            println!("\n********** Generating: {} **********",file_out.1);}
        }

        _ => {
            // This should not be reached because we've already validated user info
            println!("Invalid choice.");
        }
    }
}

}


fn get_input(info:&str) -> String {
    println!("{}",info);
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Error: user info.");
    user_input.trim().to_string()
}

fn get_file(info: &str) -> String {
    loop {
    let file_name = get_input(info);
    let file_path = format!("data/{}.csv",file_name);
    
    if file_name ==  "qp" {
        print!("closing program...");
        std::process::exit(0);
    }
    if Path::new(&file_path).exists() {
        return file_path;
        }
        else {
            println!("\n******************************************************\nThe specified file does not exist. Please try again.\n\n******************************************************\n");
        }
    }
}

fn check_file(info: &str, data_type: &str) -> (String,String){
    loop {
        let file_name = get_input(info);
        let file_path = format!("{}/{}.png",data_type,file_name);

        if file_name ==  "qp" {
            print!("closing program...");
            std::process::exit(0);
        }
        if Path::new(&file_path).exists() {
            println!("\n******************************************************\nThe specified file already exist. Please try again.\n\n******************************************************\n");
        }
        else {
            return (file_path,file_name);
        }
    }
}
// function for scatter
#[derive(Debug)]
struct DataPoint {
    x: f64,
    y: f64,
}
struct DataBar {
    category: String,
    value: f64,
}

struct DataPie {
    label: String,
    value: f64,
    color: RGBColor,
}

fn draw_scatter_plot(input_file1: &str, output_file: &(String, String)) -> Result<(), Box<dyn std::error::Error>> {
    let data1 = read_data_scatter(input_file1)?;
    let root_area = BitMapBackend::new(&output_file.0, (600, 400)).into_drawing_area();
    root_area.fill(&WHITE)?;
    let x_max_value = data1.iter().map(|entry| entry.x).fold(0.0, f64::max);
    let x_min_value = data1.iter().map(|entry| entry.x).fold(0.0, f64::min);
    let y_max_value = data1.iter().map(|entry| entry.y).fold(0.0, f64::max);
    let y_min_value = data1.iter().map(|entry| entry.y).fold(0.0, f64::min);


    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(&output_file.1, ("sans-serif", 40))
        .build_cartesian_2d(x_min_value - 10.0..x_max_value + 10.0,
        y_min_value - 10.0..y_max_value + 10.0,)?;

    ctx.configure_mesh().draw()?;
    ctx.draw_series(
        data1.iter().map(|point| Circle::new((point.x, point.y), 2, &BLACK)),
    )?;
    Ok(())
}

fn read_data_scatter(path: &str) -> Result<Vec<DataPoint>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(File::open(path)?);
    let mut data = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let x: f64 = record[0].parse()?;
        let y: f64 = record[1].parse()?;
        let data_point = DataPoint { x, y };
        data.push(data_point);
    }

    Ok(data)
} 

fn draw_bar_chart(input_file1: &str, output_file: &(String, String)) -> Result<(), Box<dyn std::error::Error>> {
    let data1 = read_data_bar(input_file1)?;

    let root_area = BitMapBackend::new(&output_file.0, (800, 700)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let max_value = data1.iter().map(|entry| entry.value).fold(0.0, f64::max);

    let mut chart = ChartBuilder::on(&root_area)
        .caption(&output_file.1, ("sans-serif", 40).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(
            0.0..data1.len() as f64 + 0.5,
            0.0..max_value,
        )?;

    chart.configure_mesh().draw()?;

    let colors = [BLUE, RED, GREEN, YELLOW];

    for (index, entry) in data1.iter().enumerate() {
        let color = colors[index % colors.len()];
    
        chart.draw_series(std::iter::once(Rectangle::new(
            [(index as f64, 0.0), (index as f64 + 1.0, entry.value)],
            color.filled(),
        )))?;
    
        chart.draw_series(std::iter::once(Text::new(
            entry.category.to_string(),
            (index as f64 + 1.0 / 2.0, -1.0),
            ("sans-serif", 20).into_font().color(&BLACK),
        )))?;
    }
    
    Ok(())
    
}

fn read_data_bar(path: &str) -> Result<Vec<DataBar>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(File::open(path)?);
    let mut data = Vec::new();

    for result in rdr.records() {
        let record = result?;
        if record.len() >= 2 {
            let category = record[0].to_string();
            let value: f64 = record[1].parse()?;
            let data_point = DataBar {
                category,
                value,
            };
            data.push(data_point);
        }
    }

    Ok(data)
}

impl DataPie {
    fn new(label: String, value: f64, color: RGBColor) -> Self {
        Self { label, value, color }
    }
}

fn draw_pie_chart(input_file: &str, output_file: &(String, String)) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(&output_file.0, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let center = (400, 300);
    let radius = 200.0;
    let mut data = read_data_pie(input_file)?;
    let total: f64 = data.iter().map(|d| d.value).sum();
    let adjustment_factor = 100.0 / total;
    for datum in &mut data {
        datum.value *= adjustment_factor;
    }
    let mut start_angle = 0.0;

    for pie_data in &data {
        let value = pie_data.value;
        let end_angle = start_angle + value / 100.0 * 2.0 * std::f64::consts::PI;

        let points: Vec<_> = std::iter::once(center)
            .chain((0..=100).map(|p| {
                let angle = start_angle + (end_angle - start_angle) * p as f64 / 100.0;
                (
                    (center.0 as f64 + radius * angle.cos()).round() as i32,
                    (center.1 as f64 + radius * angle.sin()).round() as i32,
                )
            }))
            .collect();

        let polygon = Polygon::new(points, pie_data.color.mix(0.5));
        root.draw(&polygon)?;
        
        let label_angle = (start_angle + end_angle) / 2.0;
        let label_distance = radius + 20.0; 
        let label_x = (center.0 as f64 + label_distance * label_angle.cos()).round() as i32;
        let label_y = (center.1 as f64 + label_distance * label_angle.sin()).round() as i32;

        let label_style = TextStyle::from(("sans-serif", 25).into_font()).color(&BLACK);
        root.draw_text(&pie_data.label, &label_style, (label_x, label_y))?;

        let percentage_x = (label_x as f64 + 30.0).round() as i32;  
        let percentage_y = (label_y as f64).round() as i32;

        root.draw_text(&format!("{:.1}%", value), &label_style, (percentage_x, percentage_y))?;

        start_angle = end_angle;
        let title_style = TextStyle::from(("sans-serif", 40).into_font()).color(&BLACK);
        let title_x = 350; 
        let title_y = 30; 
        root.draw_text(&output_file.1, &title_style, (title_x, title_y))?;
    }

    Ok(())
}


fn read_data_pie(path: &str) -> Result<Vec<DataPie>, Box<dyn Error>> {
    let mut data = Vec::new();
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    for (line, result) in rdr.records().enumerate() {
        let record = result?;
        let label = record[0].to_string();

        let value: f64 = record.get(1)
            .ok_or(format!("Error:Missing value on line {}", line + 2))?
            .trim()
            .parse()
            .map_err(|e| format!("Error: on line {}: {}", line + 2, e))?;

            let mut rng = rand::thread_rng();

        let color = match record.len() {
                2..=4 => RGBColor(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255)), // Random color if not specified
                5 => RGBColor(
                    record[2].trim().parse().map_err(|e| format!("Error: on line {}: {}", line + 2, e))?,
                    record[3].trim().parse().map_err(|e| format!("Error: on line {}: {}", line + 2, e))?,
                    record[4].trim().parse().map_err(|e| format!("Error: on line {}: {}", line + 2, e))?,
                ),
                _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unsupported number of columns"))),
            };
        let pie_data = DataPie::new(label, value, color);
        data.push(pie_data);
    }

    Ok(data)
}

fn draw_line_chart(input_file: &str, output_file: &(String, String)) -> Result<(), Box<dyn Error>> {
    let data = read_data_scatter(input_file)?;

    let x_max = data.iter().map(|point| point.x).fold(f64::NEG_INFINITY, f64::max);
    let y_max = data.iter().map(|point| point.y).fold(f64::NEG_INFINITY, f64::max);
    let x_min = data.iter().map(|point| point.x).fold(f64::INFINITY, f64::min);
    let y_min = data.iter().map(|point| point.y).fold(f64::INFINITY, f64::min);

    let root_area = BitMapBackend::new(&output_file.0, (800, 600)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(&output_file.1, ("sans-serif", 40))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    ctx.configure_mesh().draw()?;

    let points: Vec<(f64, f64)> = data.iter().map(|point| (point.x, point.y)).collect();

    // Draw the line chart
    ctx.draw_series(AreaSeries::new(points.iter().map(|(x, y)| (*x, *y)),  0.0 , &BLUE.mix(0.2))
        .border_style(&BLACK),
    )?;

    

    Ok(())
}
