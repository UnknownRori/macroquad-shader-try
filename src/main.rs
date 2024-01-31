use macroquad::{
    miniquad::{BlendFactor, BlendState, Equation},
    prelude::*,
};

const FRAGMENT_SHADER: &str = include_str!("./fragment.glsl");
const REFLECTION_FRAGMENT_SHADER: &str = include_str!("./reflection_fragment.glsl");
const REFLECTION_VERTEX_SHADER: &str = include_str!("./reflection_vertex.glsl");

const VERTEX_SHADER: &str = include_str!("./vertex.glsl");

#[allow(clippy::cast_possible_truncation)]
pub fn window_conf() -> Conf {
    Conf {
        window_title: "Shader".to_owned(),
        fullscreen: false,
        window_height: 600,
        window_width: 800,
        window_resizable: false,
        icon: None, // TODO : Update this later
        high_dpi: true,
        ..Default::default()
    }
}

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
