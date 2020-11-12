use std::cell::RefCell;
use std::rc::Rc;use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader};
mod fdw;

const CUBE_VERT_NUM: usize = 12;
static CUBE_VERT: [f32;CUBE_VERT_NUM*9] = [
    -0.5,-0.5, 0.5,  0.5,-0.5, 0.5,  -0.5, 0.5, 0.5,  0.5, 0.5, 0.5, -0.5, 0.5, 0.5,  0.5,-0.5, 0.5,
    -0.5,-0.5,-0.5, -0.5, 0.5,-0.5,   0.5,-0.5,-0.5,  0.5, 0.5,-0.5,  0.5,-0.5,-0.5, -0.5, 0.5,-0.5,
    -0.5, 0.5, 0.5,  0.5, 0.5, 0.5,  -0.5, 0.5,-0.5,  0.5, 0.5, 0.5,  0.5, 0.5,-0.5, -0.5, 0.5,-0.5,
    -0.5,-0.5, 0.5,  -0.5,-0.5,-0.5,  0.5,-0.5, 0.5,  0.5,-0.5, 0.5, -0.5,-0.5,-0.5,  0.5,-0.5,-0.5,
     0.5, 0.5, 0.5,  0.5,-0.5, 0.5,   0.5, 0.5,-0.5,  0.5, 0.5,-0.5,  0.5,-0.5, 0.5,  0.5,-0.5,-0.5,
    -0.5,-0.5, 0.5, -0.5, 0.5, 0.5,  -0.5, 0.5,-0.5, -0.5,-0.5, 0.5, -0.5, 0.5,-0.5, -0.5,-0.5,-0.5,
];

static CUBE_NOR: [f32;CUBE_VERT_NUM*9] = [
    0.0,0.0, 1.0, 0.0,0.0, 1.0, 0.0,0.0, 1.0, 0.0,0.0, 1.0, 0.0,0.0, 1.0, 0.0,0.0, 1.0,
    0.0,0.0,-1.0, 0.0,0.0,-1.0, 0.0,0.0,-1.0, 0.0,0.0,-1.0, 0.0,0.0,-1.0, 0.0,0.0,-1.0,
    0.0, 1.0,0.0, 0.0, 1.0,0.0, 0.0, 1.0,0.0, 0.0, 1.0,0.0, 0.0, 1.0,0.0, 0.0, 1.0,0.0,
    0.0,-1.0,0.0, 0.0,-1.0,0.0, 0.0,-1.0,0.0, 0.0,-1.0,0.0, 0.0,-1.0,0.0, 0.0,-1.0,0.0,
     1.0,0.0,0.0,  1.0,0.0,0.0,  1.0,0.0,0.0,  1.0,0.0,0.0,  1.0,0.0,0.0,  1.0,0.0,0.0, 
    -1.0,0.0,0.0, -1.0,0.0,0.0, -1.0,0.0,0.0, -1.0,0.0,0.0, -1.0,0.0,0.0, -1.0,0.0,0.0, 
];

static CUBE_COL: [f32;CUBE_VERT_NUM*12] = [
    1.0,0.5,0.5,1.0, 1.0,0.5,0.5,1.0, 1.0,0.5,0.5,1.0,  1.0,0.5,0.5,1.0, 1.0,0.5,0.5,1.0, 1.0,0.5,0.5,1.0,
    1.0,0.5,0.5,1.0, 1.0,0.5,0.5,1.0, 1.0,0.5,0.5,1.0,  1.0,0.5,0.5,1.0, 1.0,0.5,0.5,1.0, 1.0,0.5,0.5,1.0,
    0.5,0.5,1.0,1.0, 0.5,0.5,1.0,1.0, 0.5,0.5,1.0,1.0,  0.5,0.5,1.0,1.0, 0.5,0.5,1.0,1.0, 0.5,0.5,1.0,1.0,
    0.5,0.5,1.0,1.0, 0.5,0.5,1.0,1.0, 0.5,0.5,1.0,1.0,  0.5,0.5,1.0,1.0, 0.5,0.5,1.0,1.0, 0.5,0.5,1.0,1.0,
    0.5,1.0,0.5,1.0, 0.5,1.0,0.5,1.0, 0.5,1.0,0.5,1.0,  0.5,1.0,0.5,1.0, 0.5,1.0,0.5,1.0, 0.5,1.0,0.5,1.0,
    0.5,1.0,0.5,1.0, 0.5,1.0,0.5,1.0, 0.5,1.0,0.5,1.0,  0.5,1.0,0.5,1.0, 0.5,1.0,0.5,1.0, 0.5,1.0,0.5,1.0,
];

static mut SH_0: SlideHolder = SlideHolder{
    value: [ 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0 ],
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body0() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas.get_context("webgl2")?
                        .unwrap()
                        .dyn_into::<WebGl2RenderingContext>()?;

    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r#"#version 300 es
        layout(location = 0) in vec4 position;
        layout(location = 1) in vec4 normal;
        layout(location = 2) in vec4 color;
        uniform   mat4 mMatrix;
        out   vec4 vNormal;
        out   vec4 vColor;
        void main(){
            gl_Position = mMatrix * position;
            vNormal = normal;
            vColor = color;
        }
        "#,
    )?;
    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r#"#version 300 es
        precision mediump float;
        in	vec4 vNormal;
        in	vec4 vColor;
        out vec4 oFragColor;
        void main() {
            mediump float rate = clamp( dot(vNormal, vec4(0.0,1.0,0.0,0.0)), 0.5, 1.0 );
            oFragColor = vColor*rate;
        }
        "#,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));
 
    let col0 = fdw::Color::<u8>{ r:0,g:0,b:0,a:255};
    let col1 = fdw::Color::<u8>{ r:255,g:255,b:255,a:255};
    let mut tiled_floor = fdw::generate_tiled_floor(1.0, 2, col0, col1);
    // -> (Vec<f32>,Vec<f32>,Vec<u8>) {

    let mut vtx_vec: Vec<f32> = Vec::new();
    vtx_vec.extend_from_slice(&CUBE_VERT);
    vtx_vec.append(&mut tiled_floor.0);

    let mut nor_vec: Vec<f32> = Vec::new();
    nor_vec.extend_from_slice(&CUBE_NOR);
    nor_vec.append(&mut tiled_floor.1);

    let mut col_vec: Vec<f32> = Vec::new();
    col_vec.extend_from_slice(&CUBE_COL);
    for v in tiled_floor.2{
        let col = v as f32/255.0;
        col_vec.push(col);
    } 

    // vertex ------------------------------------------------------------------------
    create_f32_buffer(&context, &vtx_vec)?;
//    create_f32_buffer(&context, &CUBE_VERT)?;
    context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(0);
    
    // normal ------------------------------------------------------------------------
    create_f32_buffer(&context, &nor_vec)?;
//    create_f32_buffer(&context, &CUBE_NOR)?;

    context.vertex_attrib_pointer_with_i32(1, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(1);
    
    // color ------------------------------------------------------------------------
    create_f32_buffer(&context, &col_vec)?;
//    create_f32_buffer(&context, &CUBE_COL)?;
    context.vertex_attrib_pointer_with_i32(2, 4, WebGl2RenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(2);

    // clear BG ---------------------------------------------------------------------
    context.clear_color(0.8, 0.8, 1.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    
    // Views ---------------------------------------------------------------------
    let mut view_pos = fdw::Views{
        eye: fdw::Vec3D{ x:0.0, y:2.0, z:6.0 },
        look_at:      fdw::Vec3D{ x:0.0, y:0.0, z:-2.0 },
        eye_base: fdw::Vec3D{ x:0.0, y:2.0, z:6.0 },
    };

    // light ---------------------------------------------------------------------
    let light0 = fdw::Light{
        position:   fdw::Vec3D{ x:0.0, y:20.0, z:0.0 },
        up:         fdw::Vec3D{ x:0.0, y:0.0, z:0.0 },
        ambient:    fdw::Vec4D{ x:0.3, y:0.3, z:0.3, h:1.0 }
    };

    // Dwpth Test, Culling
//    context.enable(WebGl2RenderingContext::CULL_FACE);
    context.enable(WebGl2RenderingContext::DEPTH_TEST);
    context.depth_func(WebGl2RenderingContext::LEQUAL);

    console_log!("Light!");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");
    body.append_child(&val)?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
/*
        if i > 30000 {
            //body0().set_text_content(Some("All done!"));
            val.set_text_content(Some("All done!"));

            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            let _ = f.borrow_mut().take();
            return;
        }
*/
        draw_scene(&context, &program, &mut view_pos);

        // Set the body's text content to how many times this
        // requestAnimationFrame callback has fired.
        i += 1;
        let text = format!("requestAnimationFrame has been called {} times.", i);
        val.set_text_content(Some(&text));

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}


fn draw_scene( context: &WebGl2RenderingContext, program: &WebGlProgram, view: &mut fdw::Views ){
    let doc = document();
    let canvas = doc.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    let mut rx = 0.0;
    let mut ry = 0.0;
    let mut rz = 0.0;
    unsafe{
        view.look_at.x = SH_0.value[2]*2.0;
        view.look_at.y = SH_0.value[3]*2.0;
        view.look_at.z = -SH_0.value[4]*2.0;
        rx = SH_0.value[5];
        ry = SH_0.value[6];
        rz = SH_0.value[7];
    }
    let v_mat = view.look_at(&fdw::Vec3D{ x:0.0, y:1.0, z:0.0});
    let cnv_rate = canvas.width() as f32/canvas.height() as f32;
    let proj_mat = fdw::Mat4D::perspective(45.0, cnv_rate, 0.1, 100.0);
    let m_mat = &v_mat*&proj_mat;
    let m_mat2 = m_mat;

    let mdl_mat = fdw::Mat4D::identity();
    let mx = fdw::Mat4D::rotate(rx,0);
    let my = fdw::Mat4D::rotate(ry,1);
    let mz = fdw::Mat4D::rotate(rz,2);
    let m_mat = &m_mat*&mdl_mat;
    let m_mat = &mx*&m_mat;
    let m_mat = &my*&m_mat; 
    let m_mat = &mz*&m_mat; 

    let loc = context.get_uniform_location(&program, "mMatrix").ok_or("failed to get uniform location").unwrap();
    let loc = Some(&loc);
    context.uniform_matrix4fv_with_f32_array(loc,false,&m_mat.a);
    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, (CUBE_VERT_NUM as i32)*3);

    context.uniform_matrix4fv_with_f32_array(loc,false,&m_mat2.a);
    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, (CUBE_VERT_NUM as i32)*3, 24);

    unsafe{
        view.rotate(-SH_0.value[1]*3.0);
    }
}

#[wasm_bindgen]
pub fn change_val(num : i32, no: usize){
    unsafe{
        SH_0.value[no] = ((num-300) as f32)/100.0;
    }
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

// vertex ------------------------------------------------------------------------
fn create_f32_buffer(
    context: &WebGl2RenderingContext,
    vertices: &[f32]
)-> Result<(), JsValue>{
    let v_buf = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&v_buf));

    unsafe {
        let vert_array = js_sys::Float32Array::view(vertices);
        
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
    Ok(())
}

// color ------------------------------------------------------------------------
#[allow(unused)]
fn create_u8_buffer(
    context: &WebGl2RenderingContext,
    colors: &[u8]
)-> Result<(), JsValue>{
    let c_buf = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&c_buf));
    
    unsafe {
        let col_array = js_sys::Uint8Array::view(&colors);
        
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &col_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
    
    Ok(())
}

//= SlideHolder ============================================================
struct SlideHolder{
    value: [f32;8],
}
