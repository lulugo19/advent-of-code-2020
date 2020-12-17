extern crate advent_of_code_2020;
extern crate plotters;

use advent_of_code_2020::day17::{conway_nd, input_generator};
use plotters::coord::Shift;
use plotters::prelude::*;
use plotters::style::text_anchor::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let root_drawing_area =
		BitMapBackend::new("visualization/day17_plot.png", (600, 800)).into_drawing_area();
	root_drawing_area.fill(&WHITE)?;
	// And we can split the drawing area into 3x3 grid
	let (top_area, bottom_area) = root_drawing_area.split_vertically(50);

	let text_pos = Pos::new(HPos::Center, VPos::Center);
	top_area.draw(&Text::new(
		"AOC Day17",
		(300, 20),
		TextStyle::from(("sans-serif", 30).into_font()).pos(text_pos),
	))?;

	top_area.draw(&Text::new(
		"active cube count in various dimensions",
		(300, 45),
		TextStyle::from(("sans-serif", 18).into_font()).pos(text_pos),
	))?;

	let child_drawing_areas = bottom_area.split_evenly((3, 2));
	// Then we fill the drawing area with different color
	for (area, idx) in child_drawing_areas.into_iter().zip(0..) {
		build_chart(&area, idx + 2)?;
	}
	Ok(())
}

fn build_chart<'a>(
	area: &'a DrawingArea<BitMapBackend, Shift>,
	n: usize,
) -> Result<(), Box<dyn std::error::Error>> {
	const INPUT: &'static str = ".#.#.#..\n\
	..#....#\n\
	#####..#\n\
	#####..#\n\
	#####..#\n\
	###..#.#\n\
	#..##.##\n\
	#.#.####";

	let cycles = if n < 7 { 6 } else { 4 };
	println!("{} dimensions", n);
	println!("-------------");
	let active_count = conway_nd(n, &input_generator(INPUT, n), cycles);
	println!();
	let mut max = *active_count.iter().max().unwrap();
	max += (max as f64 * 0.05) as usize;
	let mut chart = ChartBuilder::on(area)
		// Set the caption of the chart
		.caption(format!("n = {}", n), ("sans-serif", 15).into_font())
		// Set the size of the label region
		.x_label_area_size(37)
		.y_label_area_size(55)
		.margin(10)
		// Finally attach a coordinate on the drawing area and make a chart context
		.build_cartesian_2d((0..cycles).into_segmented(), 0..max)?;

	// Then we can draw a mesh
	chart
		.configure_mesh()
		// We can customize the maximum number of labels allowed for each axis
		.x_labels(7)
		.y_labels(10)
		.y_label_formatter(&|y| {
			if *y > 10_000 {
				format!("{} k", *y / 1000)
			} else if *y > 1000 {
				format!("{:.1} k", *y as f64 / 1000.0)
			} else {
				y.to_string()
			}
		})
		.x_desc("cycle")
		.axis_desc_style(("sans-serif", 15).into_font().style(FontStyle::Bold))
		.y_desc("active count")
		// We can also change the format of the label text
		.draw()?;

	// And we can draw something in the drawing area
	chart.draw_series(
		Histogram::vertical(&chart)
			.style(RED.mix(0.5).filled())
			.data(active_count.iter().enumerate().map(|(idx, x)| (idx, *x))),
	)?;
	Ok(())
}
