use std::cell::RefCell;
use std::rc::Rc;use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader};
pub mod fdw;
pub mod shapes;


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
    0.5,1.0,1.0,1.0, 0.5,1.0,1.0,1.0, 0.5,1.0,1.0,1.0,  0.5,1.0,1.0,1.0, 0.5,1.0,1.0,1.0, 0.5,1.0,1.0,1.0,
    0.5,0.5,1.0,1.0, 0.5,0.5,1.0,1.0, 0.5,0.5,1.0,1.0,  0.5,0.5,1.0,1.0, 0.5,0.5,1.0,1.0, 0.5,0.5,1.0,1.0,
    1.0,1.0,0.5,1.0, 1.0,1.0,0.5,1.0, 1.0,1.0,0.5,1.0,  1.0,1.0,0.5,1.0, 1.0,1.0,0.5,1.0, 1.0,1.0,0.5,1.0,
    0.5,1.0,0.5,1.0, 0.5,1.0,0.5,1.0, 0.5,1.0,0.5,1.0,  0.5,1.0,0.5,1.0, 0.5,1.0,0.5,1.0, 0.5,1.0,0.5,1.0,
    1.0,0.5,1.0,1.0, 1.0,0.5,1.0,1.0, 1.0,0.5,1.0,1.0,  1.0,0.5,1.0,1.0, 1.0,0.5,1.0,1.0, 1.0,0.5,1.0,1.0,
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
    let document = document();
    let body = body0();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas.get_context("webgl2")?
                        .unwrap()
                        .dyn_into::<WebGl2RenderingContext>()?;

    // Generate Shader programs
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
            vec3 nor3 = normalize(vNormal.xyz);
            float rate = clamp( dot(nor3, vec3(0.0,1.0,0.0)), 0.0, 1.0 )*0.3 + 0.7;
            oFragColor = vec4(vColor.r*rate, vColor.g*rate, vColor.b*rate, vColor.a);
        }
        "#,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;

    let vert_shader2 = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r#"#version 300 es
        layout(location = 0) in vec4 position;
        layout(location = 1) in vec4 normal;
        layout(location = 2) in vec4 color;
        uniform   mat4 mMatrix2;
        out   vec4 vNormal;
        out   vec4 vColor;
        void main(){
            gl_Position = mMatrix2 * position;
            vNormal = normal;
            vColor = color;
        }
        "#,
    )?;
    let frag_shader2 = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r#"#version 300 es
        precision mediump float;
        in	vec4 vNormal;
        in	vec4 vColor;
        out vec4 oFragColor;
        void main() {
            vec4 newCol = vColor;
            newCol.r = clamp( vColor.r+0.5, 0.5, 1.0 );
            oFragColor = newCol;
        }
        "#,
    )?;
    let program2 = link_program(&context, &vert_shader2, &frag_shader2)?;

    let tsrct = shapes::generate_tesseract(1.0);

    let col0 = fdw::Color::<u8>{ r:0,g:0,b:0,a:255};
    let col1 = fdw::Color::<u8>{ r:255,g:255,b:255,a:255};
    let tile_num = 16;
    let tile_triangle = tile_num*tile_num*6;
    let tiled_floor = shapes::generate_tiled_floor(1.0, tile_num, col0, col1);
    let vtx_vec = tiled_floor.0;
    let nor_vec = tiled_floor.1;
    let mut col_vec: Vec<f32> = Vec::new();
    for v in tiled_floor.2{
        let col = v as f32/255.0;
        col_vec.push(col);
    }

    // Objects ------------------------------------------------------------------------
    const CUBE4_SIZE :usize = 128;
    let mut vtx_buf: Vec<f32> = Vec::with_capacity(CUBE4_SIZE);
    let mut nor_buf: Vec<f32> = Vec::with_capacity(CUBE4_SIZE);
    let mut col_buf: Vec<f32> = Vec::with_capacity(CUBE4_SIZE);
    
    // Cube ------------------------------------------------------------------------
    vtx_buf.extend_from_slice(&CUBE_VERT);
    nor_buf.extend_from_slice(&CUBE_NOR);
    col_buf.extend_from_slice(&CUBE_COL);
    
    let v0: js_sys::Float32Array;
    let v1: js_sys::Float32Array;
    let v2: js_sys::Float32Array;
    let v_buf = create_f32_buffer(&context, &CUBE_VERT)?;
    let n_buf = create_f32_buffer(&context, &CUBE_NOR)?;
    let c_buf = create_f32_buffer(&context, &CUBE_COL)?;

    unsafe{
        v0 = js_sys::Float32Array::view(&CUBE_VERT);
        v1 = js_sys::Float32Array::view(&CUBE_NOR);
        v2 = js_sys::Float32Array::view(&CUBE_COL);
    }

    let shader1 = ShaderSet3{
        culling: false,
        gl_prog: program,
        vtx_buf: [vtx_buf, nor_buf, col_buf],
        gl_buf: [ v_buf, n_buf, c_buf ],
        vertices: [ v0, v1, v2 ],
        atr_idx: [ 0,1,2 ],
        atr_size: [3,3,4],
        tri_num: (CUBE_VERT_NUM as i32)*3
    };

    // Floor ------------------------------------------------------------------------
    let v_buf2 = create_f32_buffer(&context, &vtx_vec)?;
    let n_buf2 = create_f32_buffer(&context, &nor_vec)?;
    let c_buf2 = create_f32_buffer(&context, &col_vec)?;

    let v00: js_sys::Float32Array;
    let v01: js_sys::Float32Array;
    let v02: js_sys::Float32Array;
    unsafe{
        v00 = js_sys::Float32Array::view(&vtx_vec);
        v01 = js_sys::Float32Array::view(&nor_vec);
        v02 = js_sys::Float32Array::view(&col_vec);
    }
    let shader2 = ShaderSet3{
        culling: true,
        gl_prog: program2,
        vtx_buf: [vtx_vec,nor_vec,col_vec],
        gl_buf: [ v_buf2, n_buf2, c_buf2 ],
        vertices: [ v00, v01, v02 ],
        atr_idx: [ 0,1,2 ],
        atr_size: [3,3,4],
        tri_num: tile_triangle
    };

    // clear BG ---------------------------------------------------------------------
    context.clear_color(0.8, 0.8, 1.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    
    // Dwpth Test, Culling
    context.enable(WebGl2RenderingContext::CULL_FACE);
    context.enable(WebGl2RenderingContext::DEPTH_TEST);
    context.depth_func(WebGl2RenderingContext::LEQUAL);
    let canvas_rate = canvas.width() as f32/canvas.height() as f32;

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");
    body.append_child(&val)?;

    let closure_captured = Rc::new(RefCell::new(None));
    let closure_cloned = Rc::clone(&closure_captured);
    let main_loop_rc = Rc::new(RefCell::new(MainLoop::new(context, canvas_rate, tsrct, shader1, shader2)));

    {   // setup requestAnimationFrame Loop
        let app_for_closure = Rc::clone(&main_loop_rc);
        closure_cloned.replace(Some(Closure::wrap(Box::new(move |time: f64|{
            app_for_closure.borrow_mut().on_animation_frame(time);
            request_animation_frame(closure_captured.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut(f64)>)));
        request_animation_frame(closure_cloned.borrow().as_ref().unwrap());
    }

    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>){
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register 'requestAnimationFrame' OK");
}

pub struct MainLoop {
    pub context: WebGl2RenderingContext,
    pub canvas_rate: f32,
    pub tsrct: Vec<fdw::TriPylam>,
    pub is_changed: bool,
    pub shader1: ShaderSet3,
    pub shader2: ShaderSet3,
    pub old_sh: [f32;8]
}

impl MainLoop {
    pub fn new(context: WebGl2RenderingContext, canvas_rate: f32, tsrct: Vec<fdw::TriPylam>, shader1: ShaderSet3, shader2: ShaderSet3) -> MainLoop {
        MainLoop{
            context: context,
            canvas_rate: canvas_rate,
            tsrct: tsrct,
            is_changed: false,
            shader1: shader1,
            shader2: shader2,
            old_sh: [1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0]
        }
    }

    pub fn on_animation_frame(&mut self, time: f64){
        
        // 変化がなければ何もしない
        if !self.check_change() {
            return;
        }
        // 新しい条件を記録
        unsafe{
            for idx in 0..8 {
                self.old_sh[idx] = SH_0.value[idx];
            }
        }

        // 画面クリア
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        // 視点の設定
        let mut view_pos = fdw::Views::create(2.0, 10.0);
        unsafe{
            view_pos.rotate( SH_0.value[1]*3.0 );
        }
        let vp_mat = view_pos.gen_view_proj(self.canvas_rate);
        
        // オブジェクトの座標(平行移動)
        let pos_vec;
        unsafe{
            pos_vec = fdw::Vec4D{ x:0.0, y:0.0, z:0.0, h:SH_0.value[0]*2.0 };
        }

        // オブジェクトの回転
        let r_xy;
        let r_yz;
        let r_zx;
        let r_zh;
        let r_hx;
        let r_yh;
        unsafe{
            r_xy = SH_0.value[2]*3.0;
            r_yz = SH_0.value[3]*3.0;
            r_zx = SH_0.value[4]*3.0;
            r_yh = SH_0.value[5]*3.0;
            r_zh = SH_0.value[6]*3.0;
            r_hx = SH_0.value[7]*3.0;
        }
        // 回転行列の設定
        let mxy = fdw::Mat4D::rotate(r_xy,0);
        let myz = fdw::Mat4D::rotate(r_yz,1);
        let mzx = fdw::Mat4D::rotate(r_zx,2);
        let myh = fdw::Mat4D::rotate(r_yh,5);
        let mzh = fdw::Mat4D::rotate(r_zh,3);
        let mhx = fdw::Mat4D::rotate(r_hx,4);
        let rot_mat = mxy.mul_r(&myz).mul_r(&mzx).mul_r(&mhx).mul_r(&myh).mul_r(&mzh);

        // 4D affine変換：データをバッファに詰める
        self.shader1.make_buf(&self.context, &self.tsrct, &rot_mat, &pos_vec);
        
        // 描画：視点変換
        draw_scene(&self.context, &self.shader1, &vp_mat, "mMatrix");

        // Floor
        draw_scene(&self.context, &self.shader2, &vp_mat, "mMatrix2");
    }

    // 変化があれば２回 true を返す
    fn check_change(&mut self) -> bool {
        for (idx,old) in self.old_sh.iter().enumerate() {
            unsafe{
                if *old != SH_0.value[idx] {
                    self.is_changed = true;
                    return true;
                }
            }
        }
        if self.is_changed {
            self.is_changed = false;
            return true;
        }
        false
    }
}

#[allow(unused_assignments)]
fn draw_scene( context: &WebGl2RenderingContext, shader: &ShaderSet3, mvp_mat: &fdw::Mat4D, unif_name: &str){

    if shader.culling {
        context.enable(WebGl2RenderingContext::CULL_FACE);    
    }else{
        context.disable(WebGl2RenderingContext::CULL_FACE);    
    }
    shader.set_prog(context);
    let loc = context.get_uniform_location(&shader.gl_prog, unif_name).ok_or("failed to get uniform location").unwrap();
    let loc = Some(&loc);
    context.uniform_matrix4fv_with_f32_array(loc,false, &mvp_mat.a);
    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, shader.tri_num);
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
)-> Result<web_sys::WebGlBuffer, JsValue>{
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
    Ok(v_buf)
}

// color ------------------------------------------------------------------------
#[allow(unused)]
fn create_u8_buffer(
    context: &WebGl2RenderingContext,
    colors: &[u8]
)-> Result<web_sys::WebGlBuffer, JsValue>{
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
    
    Ok(c_buf)
}

//= SlideHolder ============================================================
struct SlideHolder{
    value: [f32;8],
}

//= ShaderSet ============================================================
const SS3_LEN: usize = 3;
pub struct ShaderSet3{
    culling: bool,
    gl_prog: WebGlProgram,
    vtx_buf: [Vec<f32>;SS3_LEN],
    gl_buf: [web_sys::WebGlBuffer;SS3_LEN],
    vertices: [js_sys::Float32Array;SS3_LEN],
    atr_idx: [u32;SS3_LEN],
    atr_size: [i32;SS3_LEN],
    tri_num: i32
}

impl ShaderSet3{
    fn set_prog(&self, context: &WebGl2RenderingContext){
        context.use_program(Some(&self.gl_prog));
        for idx in 0..3 {
            context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.gl_buf[idx]));

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &self.vertices[idx],
                WebGl2RenderingContext::STATIC_DRAW,
            );

            context.enable_vertex_attrib_array(self.atr_idx[idx]);        
            context.vertex_attrib_pointer_with_i32(
                self.atr_idx[idx],
                self.atr_size[idx],
                WebGl2RenderingContext::FLOAT, false, 0, 0
            );
        }
    }

    fn make_buf(&mut self, context: &WebGl2RenderingContext, tsrct: &Vec<fdw::TriPylam>, rot_mat: &fdw::Mat4D, pos_vec: &fdw::Vec4D){
        
        self.gl_buf[0] = create_f32_buffer(&context, &self.vtx_buf[0]).unwrap();
        self.gl_buf[1] = create_f32_buffer(&context, &self.vtx_buf[1]).unwrap();
        self.gl_buf[2] = create_f32_buffer(&context, &self.vtx_buf[2]).unwrap();

        unsafe{
            self.vertices[0] = js_sys::Float32Array::view(&self.vtx_buf[0]);
            self.vertices[1] = js_sys::Float32Array::view(&self.vtx_buf[1]);
            self.vertices[2] = js_sys::Float32Array::view(&self.vtx_buf[2]);
        }

        let h_pos = 0.0;
        &self.vtx_buf[0].clear();
        &self.vtx_buf[1].clear();
        &self.vtx_buf[2].clear();
        for plm in tsrct{
            let new_plm = plm.affine_transform( &rot_mat, &pos_vec );
            new_plm.make_arrays(h_pos, &mut self.vtx_buf);
        }
        unsafe{
            self.vertices[0] = js_sys::Float32Array::view(&self.vtx_buf[0]);
            self.vertices[1] = js_sys::Float32Array::view(&self.vtx_buf[1]);
            self.vertices[2] = js_sys::Float32Array::view(&self.vtx_buf[2]);
        }

        self.tri_num = (self.vtx_buf[0].len()*3) as i32;
    }
}
