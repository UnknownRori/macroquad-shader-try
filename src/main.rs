use macroquad::{
    miniquad::{BlendFactor, BlendState, Equation},
    prelude::*,
};

const FRAGMENT_SHADER: &str = include_str!("./fragment.glsl");
const REFLECTION_FRAGMENT_SHADER: &str = include_str!("./reflection_fragment.glsl");
const REFLECTION_VERTEX_SHADER: &str = include_str!("./reflection_vertex.glsl");

const VERTEX_SHADER: &str = include_str!("./vertex.glsl");

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Shader".to_owned(),
        fullscreen: false,
        window_height: 600,
        window_width: 800,
        window_resizable: true,
        icon: None, // TODO : Update this later
        high_dpi: true,
        ..Default::default()
    }
}

// #[macroquad::main(window_conf)]
// Normalize for drawing
// async fn main() {
//     let texture = load_texture("./unknownrori.png").await.unwrap();
//     loop {
//         clear_background(WHITE);
//
//         let screen = vec2(screen_width(), screen_height());
//
//         let normalize_size = vec2(200f32 / screen.x, 200f32 / screen.y);
//         let normalize_aspect = normalize_size.x / normalize_size.y;
//         let size = vec2(0.1, 0.1 / normalize_aspect);
//         draw_rectangle(
//             (vec2(0.5, 0.5) * screen).x,
//             (vec2(0.5, 0.5) * screen).y,
//             (size * screen).x,
//             (size * screen).y,
//             RED,
//         );
//
//         let normalized_texture_size =
//             vec2(texture.size().x / screen.x, texture.size().y / screen.y);
//         let normalized_aspect = normalized_texture_size.x / normalized_texture_size.y;
//         let size = vec2(0.1, 0.1 / normalized_aspect);
//         draw_texture_ex(
//             &texture,
//             (vec2(0.1, 0.1) * screen).x,
//             (vec2(0.1, 0.1) * screen).y,
//             WHITE,
//             DrawTextureParams {
//                 dest_size: Some(size * screen),
//                 ..Default::default()
//             },
//         );
//
//         next_frame().await
//     }
// }

#[macroquad::main(window_conf)]
async fn main() {
    let render_target = render_target(800, 600);
    // let texture = load_texture("./unknownrori.png").await.unwrap();
    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
            // vertex: REFLECTION_VERTEX_SHADER,
            // fragment: REFLECTION_FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                ("iResolution".to_string(), UniformType::Float2),
                ("iTime".to_string(), UniformType::Float1),
            ],
            // pipeline_params: PipelineParams {
            //     color_blend: Some(BlendState::new(
            //         Equation::Add,
            //         BlendFactor::Value(miniquad::BlendValue::SourceAlpha),
            //         BlendFactor::OneMinusValue(miniquad::BlendValue::SourceAlpha),
            //     )),
            //     alpha_blend: Some(BlendState::new(
            //         Equation::Add,
            //         BlendFactor::Zero,
            //         BlendFactor::One,
            //     )),
            //     ..Default::default()
            // },
            ..Default::default()
        },
    )
    .unwrap();

    loop {
        clear_background(WHITE);

        // draw_texture_ex(
        //     // &render_target.texture,
        //     &texture,
        //     200f32,
        //     100f32,
        //     // 0f32,
        //     // 0f32,
        //     WHITE,
        //     DrawTextureParams {
        //         // dest_size: Some(vec2(800f32, 600f32)),
        //         dest_size: Some(texture.size().normalize() * 200.0),
        //         ..Default::default()
        //     },
        // );

        gl_use_material(&material);
        material.set_uniform("iResolution", vec2(800f32, 600f32).to_array());
        material.set_uniform("iTime", get_time() as f32);

        draw_texture_ex(
            &render_target.texture,
            // &texture,
            // 200f32,
            // 250f32,
            0f32,
            0f32,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(800f32, 600f32)),
                // dest_size: Some(texture.size().normalize() * 200.0),
                ..Default::default()
            },
        );
        gl_use_default_material();

        next_frame().await
    }
}
