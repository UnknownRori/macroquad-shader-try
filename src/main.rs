use macroquad::prelude::*;

const FRAGMENT_SHADER: &str = include_str!("./fragment.glsl");

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
    // let texture = load_texture("./fairy0001.png").await.unwrap();
    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                ("iResolution".to_string(), UniformType::Float2),
                ("iTime".to_string(), UniformType::Float1),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    loop {
        clear_background(WHITE);

        gl_use_material(&material);
        material.set_uniform("iResolution", vec2(800f32, 600f32).to_array());
        material.set_uniform("iTime", get_time() as f32);
        draw_texture_ex(
            &render_target.texture,
            // &texture,
            0f32,
            0f32,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(800f32, 600f32)),
                ..Default::default()
            },
        );
        gl_use_default_material();

        next_frame().await
    }
}
