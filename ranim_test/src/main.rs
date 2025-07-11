// mod hierachical;
// use std::collections::HashMap;

// use layout::{adt::dag::NodeHandle, core::base::Orientation, gv::{record::record_builder, GraphBuilder}, std_shapes::shapes::{Element, ShapeKind}, topo::layout::VisualGraph};

// fn main() {
//     let mut dir = Orientation::TopToBottom;
    
//     // Set the graph orientation based on the 'rankdir' property.
//     // dir = Orientation::LeftToRight;

//     let mut vg = VisualGraph::new(dir);

//     // Keeps track of the newly created nodes and indexes them by name.
//     let mut node_map: HashMap<String, NodeHandle> = HashMap::new();


//     let node_order = vec!["1","2"];
    
//     // Create and register all of the nodes.
//     for node_name in node_order.iter() {
//         let node_prop = self.nodes.get(node_name).unwrap();

//         PropertyList
//         let shape = 
        
//         node_map.insert(node_name.to_string(), handle);
//     }

//     // Create and register all of the edges.
//     for edge_prop in &self.edges {
//         let shape = Self::get_arrow_from_attributes(
//             &edge_prop.props,
//             edge_prop.is_directed,
//             edge_prop.from_port.clone(),
//             edge_prop.to_port.clone(),
//         );
//         let from = node_map.get(&edge_prop.from).unwrap();
//         let to = node_map.get(&edge_prop.to).unwrap();
//         vg.add_edge(shape, *from, *to);
//     }

use dagre_rust::{GraphConfig, GraphEdge, GraphNode};
use derive_more::{Deref, DerefMut};
use graphlib_rust::Graph;
use rand::seq::SliceRandom;
use rand::SeedableRng;
//     vg
    
//     println!("Hello, world!");
// }
use ranim::glam::{DVec3, dvec2};
// use rand::{SeedableRng, seq::SliceRandom};
use ranim::{
    animation::transform::TransformAnimSchedule, color::palettes::manim, components::Anchor,
    items::vitem::Rectangle, prelude::*, timeline::TimeMark, utils::rate_functions::linear,
};
#[repr(C)]
struct Foo {
    a: i32,
    b: Bar, // Bar 没有 #[repr(C)]
}

struct Bar {
    v: Vec<String>
}

#[scene]
struct BubbleSortScene(pub usize);

impl TimelineConstructor for BubbleSortScene {
    fn construct(self, timeline: &RanimTimeline, _camera: &mut Rabject<CameraFrame>) {
        let num = self.0;

        let frame_size = dvec2(8.0 * 16.0 / 9.0, 8.0);
        let padded_frame_size = frame_size * 0.9;

        let anim_step_duration = 15.0 / num.pow(2) as f64;

        let width_unit = padded_frame_size.x / num as f64;
        let height_unit = padded_frame_size.y / num as f64;

        let mut rng = rand_chacha::ChaChaRng::seed_from_u64(114514);
        let mut heights = (1..=num)
            .map(|x| x as f64 * height_unit)
            .collect::<Vec<f64>>();
        heights.shuffle(&mut rng);

        let padded_frame_bl = dvec2(padded_frame_size.x / -2.0, padded_frame_size.y / -2.0);
        let mut rects = heights
            .iter()
            .enumerate()
            .map(|(i, &height)| {
                let mut rect = Rectangle(width_unit, height).build();
                let target_bc_coord = padded_frame_bl.extend(0.0)
                    + DVec3::X * (width_unit * i as f64 + width_unit / 2.0);
                rect.scale(DVec3::splat(0.8))
                    .put_anchor_on(Anchor::edge(0, -1, 0), target_bc_coord)
                    .set_color(manim::WHITE)
                    .set_stroke_width(0.0)
                    .set_fill_opacity(0.5);
                timeline.insert(rect)
            })
            .collect::<Vec<_>>();

        let shift_right = DVec3::X * width_unit;
        for i in (1..num).rev() {
            for j in 0..i {
                timeline.play(
                    rects[j]
                        .transform(|data| {
                            data.set_color(manim::BLUE_C).set_fill_opacity(0.5);
                        })
                        .with_duration(anim_step_duration)
                        .with_rate_func(linear)
                        .apply(),
                );
                timeline.play(
                    rects[j + 1]
                        .transform(|data| {
                            data.set_color(manim::BLUE_C).set_fill_opacity(0.5);
                        })
                        .with_duration(anim_step_duration)
                        .with_rate_func(linear)
                        .apply(),
                );
                timeline.sync();

                if heights[j] > heights[j + 1] {
                    timeline.play(
                        rects[j]
                            .transform(|data| {
                                data.shift(shift_right)
                                    .set_color(manim::BLUE_C)
                                    .set_fill_opacity(0.5);
                            })
                            .with_duration(anim_step_duration)
                            .with_rate_func(linear)
                            .apply(),
                    );
                    timeline.play(
                        rects[j + 1]
                            .transform(|data| {
                                data.shift(-shift_right)
                                    .set_color(manim::BLUE_C)
                                    .set_fill_opacity(0.5);
                            })
                            .with_duration(anim_step_duration)
                            .with_rate_func(linear)
                            .apply(),
                    );
                    timeline.sync();
                    heights.swap(j, j + 1);
                    rects.swap(j, j + 1);
                }
                timeline.play(
                    rects[j]
                        .transform(|data| {
                            data.set_color(manim::WHITE).set_fill_opacity(0.5);
                        })
                        .with_duration(anim_step_duration)
                        .with_rate_func(linear)
                        .apply(),
                );
                timeline.play(
                    rects[j + 1]
                        .transform(|data| {
                            data.set_color(manim::WHITE).set_fill_opacity(0.5);
                        })
                        .with_duration(anim_step_duration)
                        .with_rate_func(linear)
                        .apply(),
                );
                timeline.sync();
            }
        }

        timeline.insert_time_mark(
            timeline.duration_secs() / 2.0,
            TimeMark::Capture(format!("preview-{num}.png")),
        );
    }
}

fn main() {
    // run_scene_app(BubbleSortScene(100));
    // #[cfg(not(feature = "app"))]
    // {
    //     render_scene(
    //         BubbleSortScene(10),
    //         &AppOptions {
    //             output_filename: "output-10.mp4",
    //             ..Default::default()
    //         },
    //     );
    //     render_scene(
    //         BubbleSortScene(100),
    //         &AppOptions {
    //             output_filename: "output-100.mp4",
    //             ..Default::default()
    //         },
    //     );
    // }
    let mut g: Graph<_, GraphNode, GraphEdge>= graphlib_rust::Graph::new(None);
    let mut node1 = GraphNode::default();
    node1.width = 100.0;
    node1.height = 50.0;
    let mut node2 = GraphNode::default();
    node2.width = 100.0;
    node2.height = 50.0;
    let n1 = "1".to_string();
    let n2 = "2".to_string();
    g.set_edge(&n1, &n2, Some(GraphEdge::default()), None).unwrap()
        .set_node(n1, Some(node1))
        .set_node(n2, Some(node2));
    dagre_rust::layout::layout(&mut g);
    println!("{:?}",g.node(&"1".to_string()));
    println!("{:?}",g.node(&"2".to_string()));
}

#[derive(Deref,DerefMut)]
struct G(Graph<GraphConfig,GraphNode,GraphEdge>);


#[derive]
struct A;
// impl Default for G {
//     fn default() -> Self {
        
//     }
// }

