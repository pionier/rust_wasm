

//=====================================================================use std::ops::Add;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Neg;

fn is_near_zero(a:f32) ->bool {
    ( -std::f32::EPSILON < a )&&( a < std::f32::EPSILON )
}

fn is_near_eq(a:f32, b:f32) ->bool {
    let dif = if a < b { b - a }else{ a - b };
    is_near_zero(dif)
}

#[allow(unused)]
fn get_sign( a:f32 ) -> i32 {
    if      a < -std::f32::EPSILON { -1 }
    else if a >  std::f32::EPSILON {  1 }
    else{                        0 }
}

#[allow(unused)]
fn get_lerp_rate( h:f32, low:f32, high:f32 ) -> f32 {
    if( is_near_eq(low, high) ){
        return 0.5;
    }
    (h-low)/(high-low)
}

//= Color<T> ============================================================
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Color<T>{
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T
}

impl<T: Copy> Color<T> {

    pub fn set_gray(&mut self, c: T){
        self.r = c;
        self.g = c;
        self.b = c;
        self.a = c;
    }
}

fn lerp3_color_u8( rate:f32, c0:&Color<u8>, c1:&Color<u8> ) -> Color<u8> {
    let r = c0.r as f32 + rate*(c1.r as f32 - c0.r as f32);
    let g = c0.g as f32 + rate*(c1.g as f32 - c0.g as f32);
    let b = c0.b as f32 + rate*(c1.b as f32 - c0.b as f32);
    let a = c0.a as f32 + rate*(c1.a as f32 - c0.a as f32);
    let dst = Color::<u8>{ r: r as u8, g: g as u8, b: b as u8, a: a as u8 };
    dst
}

//= Vec3D ============================================================
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Vec3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3D {

    fn calc_length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn normalize(&mut self){
        let leng = self.calc_length();
        if !is_near_zero(leng) {
            self.x /= leng;
            self.y /= leng;
            self.z /= leng;
        }
    }

    fn is_near_zero(&self) -> bool {
        is_near_zero(self.x)&&is_near_zero(self.y)&&is_near_zero(self.z)
    }

    fn zero(&mut self){
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }
}

fn lerp3( rate:f32, v0:&Vec3D, v1:&Vec3D ) -> Vec3D {
    let dst = Vec3D{
        x: v0.x + rate*(v1.x - v0.x),
        y: v0.y + rate*(v1.y - v0.y),
        z: v0.z + rate*(v1.z - v0.z)
    };
    dst
}

//= Vec4D ============================================================
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Vec4D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub h: f32
}

impl Add for &Vec4D {
    type Output = Vec4D;
    fn add(self, rhs: &Vec4D) -> Self::Output {
        Vec4D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            h: self.h + rhs.h
        }
    }
}

impl Sub for &Vec4D {
    type Output = Vec4D;
    fn sub(self, rhs: &Vec4D) -> Self::Output {
        Vec4D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            h: self.h - rhs.h
        }
    }
}

impl Neg for Vec4D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec4D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            h: -self.h
        }
    }
}

impl Vec4D {

    fn mul_scalor(&self, scale: f32) -> Vec4D {
        Vec4D{ x:self.x*scale, y:self.y*scale, z:self.z*scale, h:self.h*scale }
    }

    fn calc_length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.h.powi(2)).sqrt()
    }

    fn is_zero(&self) -> bool {
        is_near_zero(self.calc_length())
    }

    fn normalize(&mut self){
        let leng = self.calc_length();
        if !is_near_zero(leng) {
            self.x /= leng;
            self.y /= leng;
            self.z /= leng;
            self.h /= leng;
        }
    }

    fn shrink(&self) -> Vec3D {
        let dst = Vec3D{ x:self.x, y:self.y, z:self.z };
        dst
    }

    #[allow(dead_code)]
    fn is_near_zero(&self) -> bool {
        is_near_zero(self.x)&&is_near_zero(self.y)&&is_near_zero(self.z)&&is_near_zero(self.h)
    }

    #[allow(dead_code)]
    fn zero(&mut self){
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
        self.h = 0.0;
    }
}

//= Vertex3D ============================================================
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Vertex3D{
    vertex: Vec3D,
    normal: Vec3D,
    color:  Color<u8>
}

//= Vertex3DTex ============================================================
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Vertex3DTex{
    pub vertex: Vec3D,
    pub normal: Vec3D,
    pub texture: Vec3D,
    pub color:  Color<u8>
}

//= Vertex4D ============================================================
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Vertex4D{
    pub vertex: Vec4D,
    pub normal: Vec4D,
    pub color:  Color<u8>
}

impl Vertex4D {

    fn shrink(&self) -> Vertex3D {
        Vertex3D{
            vertex: Vec3D{ x:self.vertex.x, y:self.vertex.y, z:self.vertex.z },
            normal: Vec3D{ x:self.normal.x, y:self.normal.y, z:self.normal.z },
            color: self.color
        }
    }

    fn affine_transform( &self, mtx: &Mat4D, pos: &Vec4D ) -> Vertex4D {
        Vertex4D{
            vertex: &mtx.mul_vec(&self.vertex) + pos,
            normal: mtx.mul_vec(&self.normal),
            color: self.color
        }
    }

    fn scale( &self, scale: f32 ) -> Vertex4D {
        Vertex4D{
            vertex: self.vertex.mul_scalor(scale),
            normal: self.normal,
            color: self.color
        }
    }
}

//= Vertex4DTex ============================================================
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Vertex4DTex{
    vertex: Vec4D,
    normal: Vec4D,
    texture: Vec4D,
    color:  Color<u8>
}

#[allow(dead_code)]
impl Vertex4DTex {

    fn shrink(&self) -> Vertex3DTex {
        Vertex3DTex{
            vertex: Vec3D{ x:self.vertex.x, y:self.vertex.y, z:self.vertex.z },
            normal: Vec3D{ x:self.normal.x, y:self.normal.y, z:self.normal.z },
            texture: Vec3D{ x:self.texture.x, y:self.texture.y, z:self.texture.z },
            color: self.color
        }
    }
}

//= shrink_lerp ============================================================
fn shrink_lerp( hpos:f32, vtx0:&Vertex4D, vtx1:&Vertex4D) -> Vertex3D {
    let rate = get_lerp_rate(hpos, vtx0.vertex.h, vtx1.vertex.h);
    Vertex3D{
        vertex: lerp3( rate, &vtx0.vertex.shrink(), &vtx1.vertex.shrink() ),
        normal: lerp3( rate, &vtx0.normal.shrink(), &vtx1.normal.shrink() ),
        color: lerp3_color_u8( rate, &vtx0.color, &vtx1.color )
    }
}

#[allow(dead_code)]
//= shrink_lerp_tex ============================================================
fn shrink_lerp_tex( hpos:f32, vtx0:&Vertex4DTex, vtx1:&Vertex4DTex) -> Vertex3DTex {
    let rate = get_lerp_rate(hpos, vtx0.vertex.h, vtx1.vertex.h);
    Vertex3DTex{
        vertex:  lerp3( rate, &vtx0.vertex.shrink(),  &vtx1.vertex.shrink() ),
        normal:  lerp3( rate, &vtx0.normal.shrink(),  &vtx1.normal.shrink() ),
        texture: lerp3( rate, &vtx0.texture.shrink(), &vtx1.texture.shrink() ),
        color:   lerp3_color_u8( rate, &vtx0.color, &vtx1.color )
    }
}

//= Triangle ============================================================
#[derive(Debug, PartialEq, Default)]
pub struct Triangle{
    vertex: [Vertex3D;3],
}

impl Triangle {

    fn push_buffer( &self, v_array: &mut Vec<f32>, n_array: &mut Vec<f32>, c_array: &mut Vec<f32>){
        for v in self.vertex.iter() {
            v_array.push(v.vertex.x);
            v_array.push(v.vertex.y);
            v_array.push(v.vertex.z);
            n_array.push(v.normal.x);
            n_array.push(v.normal.y);
            n_array.push(v.normal.z);
            c_array.push(v.color.r as f32/255.0);
            c_array.push(v.color.g as f32/255.0);
            c_array.push(v.color.b as f32/255.0);
            c_array.push(v.color.a as f32/255.0);            
        }
    }
}

//= Mat4D ============================================================
const AXIS_XY: i32 = 0;
const AXIS_YZ: i32 = 1;
const AXIS_ZX: i32 = 2;
const AXIS_YH: i32 = 3;
const AXIS_ZH: i32 = 4;
const AXIS_XH: i32 = 5;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Mat4D{
    pub a: [f32;16],
}

impl Mat4D {
    const LEN_F32: usize = 16;

    pub fn mul_vec(&self, pos: &Vec4D) -> Vec4D {
        let dst = Vec4D {
            x: self.a[0]*pos.x+self.a[4]*pos.y+self.a[ 8]*pos.z+self.a[12]*pos.h,
            y: self.a[1]*pos.x+self.a[5]*pos.y+self.a[ 9]*pos.z+self.a[13]*pos.h,
            z: self.a[2]*pos.x+self.a[6]*pos.y+self.a[10]*pos.z+self.a[14]*pos.h,
            h: self.a[3]*pos.x+self.a[7]*pos.y+self.a[11]*pos.z+self.a[15]*pos.h
        };
        dst
    }
    pub fn mul_r(&self, mtx: &Mat4D) -> Mat4D {
        self*mtx
    }

    pub fn identity() -> Mat4D {
        let mut mtx = Mat4D{ a: [0.0;Mat4D::LEN_F32] };
        mtx.a[ 0] = 1.0;
        mtx.a[ 5] = 1.0;
        mtx.a[10] = 1.0;
        mtx.a[15] = 1.0;
        mtx
    }
    
    pub fn transpose(&self) -> Mat4D {
        let ret = Mat4D{ a: [
                self.a[0], self.a[4], self.a[ 8], self.a[12], 
                self.a[1], self.a[5], self.a[ 9], self.a[13], 
                self.a[2], self.a[6], self.a[10], self.a[14], 
                self.a[3], self.a[7], self.a[11], self.a[15]
            ]
        };
        ret
    }

    pub fn rotate(angle: f32, plane: i32 ) -> Mat4D {
        let mut mtx = Mat4D{ a: [0.0;Mat4D::LEN_F32] };
        let cos_x = angle.cos();
        let sin_x = angle.sin();
        match plane {
            AXIS_XY => {  // XY 0
                mtx.a[0] = cos_x;
                mtx.a[5] = cos_x;
                mtx.a[1] = sin_x;
                mtx.a[4] = -sin_x;
                mtx.a[10] = 1.0;
                mtx.a[15] = 1.0;
                },
            AXIS_YZ => {  // YZ 1
                mtx.a[0] = 1.0;
                mtx.a[ 5] = cos_x;
                mtx.a[10] = cos_x;
                mtx.a[ 6] = sin_x;
                mtx.a[ 9] = -sin_x;
                mtx.a[15] = 1.0;
            },
            AXIS_ZX => {  // ZX 2
                mtx.a[ 0] = cos_x;
                mtx.a[10] = cos_x;
                mtx.a[8] = -sin_x;
                mtx.a[2] = sin_x;
                mtx.a[5] = 1.0;
                mtx.a[15] = 1.0;
                },
            AXIS_YH => {  // YH 3
                mtx.a[0] = 1.0;
                mtx.a[ 5] = cos_x;
                mtx.a[15] = cos_x;
                mtx.a[ 7] = sin_x;
                mtx.a[13] = -sin_x;
                mtx.a[10] = 1.0;
                },
            AXIS_ZH => {  // ZH 4
                mtx.a[0] = 1.0;
                mtx.a[5] = 1.0;
                mtx.a[10] = cos_x;
                mtx.a[15] = cos_x;
                mtx.a[11] = sin_x;
                mtx.a[14] = -sin_x;
                },
            AXIS_XH => {  // HX 5
                mtx.a[ 0] = cos_x;
                mtx.a[15] = cos_x;
                mtx.a[ 3] = -sin_x;
                mtx.a[12] = sin_x;
                mtx.a[5] = 1.0;
                mtx.a[10] = 1.0;
                },
            _ => {  // Identity
                mtx.a[ 0] = 1.0;
                mtx.a[ 5] = 1.0;
                mtx.a[10] = 1.0;
                mtx.a[15] = 1.0;
                },
            }
        mtx
    }

    pub fn look_at( eye: &Vec3D, place: &Vec3D, upper: &Vec3D ) -> Mat4D {
        let mut vz = Vec3D{ x:eye.x-place.x, y:eye.y-place.y, z:eye.z-place.z };
        if  vz.is_near_zero(){
            return Mat4D::identity();
        }
        vz.normalize();

        let mut vx = Vec3D{
            x:upper.y*vz.z - upper.z*vz.y,
            y:upper.z*vz.x - upper.x*vz.z,
            z:upper.x*vz.y - upper.y*vz.x
        };
        if  vx.is_near_zero() {
            vx.zero();
        }else{
            vx.normalize();
        }

        let mut vy = Vec3D{
            x:vz.y*vx.z - vz.z*vx.y,
            y:vz.z*vx.x - vz.x*vx.z,
            z:vz.x*vx.y - vz.y*vx.x,
        };
        if  vy.is_near_zero(){
            vy.zero();
        }else{
            vy.normalize();
        }
        let ret = Mat4D{
            a: [ vx.x, vy.x, vz.x, 0.0,
                 vx.y, vy.y, vz.y, 0.0,
                 vx.z, vz.y, vz.z, 0.0,
                 -(vx.x*eye.x + vx.y*eye.y + vx.z*eye.z),
                 -(vy.x*eye.x + vy.y*eye.y + vy.z*eye.z),
                 -(vz.x*eye.x + vz.y*eye.y + vz.z*eye.z),  1.0
            ]
        };
        ret
    }

    pub fn frustrum(left:f32, right:f32, bottom:f32, top:f32, near:f32, far:f32) -> Mat4D {
        let rl = right-left;
        let tb = top-bottom;
        let fnr = far-near;
        if is_near_zero(rl)||is_near_zero(tb)||is_near_zero(fnr){
            return Mat4D::identity();
        }
        let near2 = near*2.0;
        let ret = Mat4D{
            a: [
                near2/rl, 0.0,0.0,0.0,
                0.0,near2/tb,0.0,0.0,
                (right+left)/rl,(top+bottom)/tb,-(far+near)/fnr,-1.0,
                0.0,0.0,-(far*near2)/fnr,0.0
            ]
        };
        ret
    }

    pub fn perspective(fovy:f32, aspect:f32, near:f32, far:f32) -> Mat4D {
        let top = near*((fovy*std::f32::consts::PI/360.0).tan());
        let right = top*aspect;
        Mat4D::frustrum(-right,right,-top,top,near,far)
    }
}

impl Mul for &Mat4D {
    type Output = Mat4D;

    fn mul(self, rhs: Self) -> Mat4D {
        let mtx = Mat4D{ a: [

                self.a[ 0]*rhs.a[ 0]+self.a[ 1]*rhs.a[ 4]+self.a[ 2]*rhs.a[ 8]+self.a[ 3]*rhs.a[12],
                self.a[ 0]*rhs.a[ 1]+self.a[ 1]*rhs.a[ 5]+self.a[ 2]*rhs.a[ 9]+self.a[ 3]*rhs.a[13],
                self.a[ 0]*rhs.a[ 2]+self.a[ 1]*rhs.a[ 6]+self.a[ 2]*rhs.a[10]+self.a[ 3]*rhs.a[14],
                self.a[ 0]*rhs.a[ 3]+self.a[ 1]*rhs.a[ 7]+self.a[ 2]*rhs.a[11]+self.a[ 3]*rhs.a[15],
                self.a[ 4]*rhs.a[ 0]+self.a[ 5]*rhs.a[ 4]+self.a[ 6]*rhs.a[ 8]+self.a[ 7]*rhs.a[12],
                self.a[ 4]*rhs.a[ 1]+self.a[ 5]*rhs.a[ 5]+self.a[ 6]*rhs.a[ 9]+self.a[ 7]*rhs.a[13],
                self.a[ 4]*rhs.a[ 2]+self.a[ 5]*rhs.a[ 6]+self.a[ 6]*rhs.a[10]+self.a[ 7]*rhs.a[14],
                self.a[ 4]*rhs.a[ 3]+self.a[ 5]*rhs.a[ 7]+self.a[ 6]*rhs.a[11]+self.a[ 7]*rhs.a[15],
                self.a[ 8]*rhs.a[ 0]+self.a[ 9]*rhs.a[ 4]+self.a[10]*rhs.a[ 8]+self.a[11]*rhs.a[12],
                self.a[ 8]*rhs.a[ 1]+self.a[ 9]*rhs.a[ 5]+self.a[10]*rhs.a[ 9]+self.a[11]*rhs.a[13],
                self.a[ 8]*rhs.a[ 2]+self.a[ 9]*rhs.a[ 6]+self.a[10]*rhs.a[10]+self.a[11]*rhs.a[14],
                self.a[ 8]*rhs.a[ 3]+self.a[ 9]*rhs.a[ 7]+self.a[10]*rhs.a[11]+self.a[11]*rhs.a[15],
                self.a[12]*rhs.a[ 0]+self.a[13]*rhs.a[ 4]+self.a[14]*rhs.a[ 8]+self.a[15]*rhs.a[12],
                self.a[12]*rhs.a[ 1]+self.a[13]*rhs.a[ 5]+self.a[14]*rhs.a[ 9]+self.a[15]*rhs.a[13],
                self.a[12]*rhs.a[ 2]+self.a[13]*rhs.a[ 6]+self.a[14]*rhs.a[10]+self.a[15]*rhs.a[14],
                self.a[12]*rhs.a[ 3]+self.a[13]*rhs.a[ 7]+self.a[14]*rhs.a[11]+self.a[15]*rhs.a[15],
            ]
        };
        mtx
    }
}

//= Views ============================================================
pub struct Views{
    pub eye: Vec3D,
    pub look_at: Vec3D,
    pub eye_base: Vec3D,
}

impl Views{
    pub fn create(eye_height: f32, sight_len: f32) -> Views{
        let view_pos =  Views{
            eye:        Vec3D{ x:0.0, y:eye_height, z:sight_len },
            look_at:    Vec3D{ x:0.0, y:0.0, z:0.0 },
            eye_base:   Vec3D{ x:0.0, y:eye_height, z:sight_len }
        };
        view_pos
    }

    pub fn look_at( &self, upper: &Vec3D ) -> Mat4D {
        let mut vz = Vec3D{ x:self.eye.x-self.look_at.x, y:self.eye.y-self.look_at.y, z:self.eye.z-self.look_at.z };
        if  vz.is_near_zero() {
            return Mat4D::identity();
        }
        vz.normalize();

        let mut vx = Vec3D{
            x:upper.y*vz.z - upper.z*vz.y,
            y:upper.z*vz.x - upper.z*vz.z,
            z:upper.x*vz.y - upper.y*vz.x
        };
        if  vx.is_near_zero() {
            vx.zero();
        }else{
            vx.normalize();
        }

        let mut vy = Vec3D{
            x:vz.y*vx.z - vz.z*vx.y,
            y:vz.z*vx.x - vz.x*vx.z,
            z:vz.x*vx.y - vz.y*vx.x,
        };
        if  vy.is_near_zero() {
            vy.zero();
        }else{
            vy.normalize();
        }
        let ret = Mat4D{
            a: [
                vx.x, vy.x, vz.x, 0.0,
                vx.y, vy.y, vz.y, 0.0,
                vx.z, vy.z, vz.z, 0.0,
                -(vx.x*self.eye.x + vx.y*self.eye.y + vx.z*self.eye.z),
                -(vy.x*self.eye.x + vy.y*self.eye.y + vy.z*self.eye.z),
                -(vz.x*self.eye.x + vz.y*self.eye.y + vz.z*self.eye.z), 1.0
            ]
        };
        ret
    }

    pub fn rotate( &mut self, angle: f32 ){
        let sin_x = angle.sin();
        let cos_x = angle.cos();
        let eye_x = self.eye_base.x * cos_x - self.eye_base.z * sin_x;
        let eye_z = self.eye_base.x * sin_x + self.eye_base.z * cos_x;
        self.eye.x = self.look_at.x+eye_x;
        self.eye.z = self.look_at.z+eye_z;
    }

    // Generate View Matrix * Perspective Matrix
    pub fn gen_view_proj( &self, cnv_rate: f32 ) -> Mat4D {
        let v_mat = self.look_at(&Vec3D{ x:0.0, y:1.0, z:0.0});
        let proj_mat = Mat4D::perspective(45.0, cnv_rate, 0.1, 100.0);
        &v_mat*&proj_mat
    }
}

//= Light ============================================================
pub struct Light{
    pub position: Vec3D,
    pub up: Vec3D,
    pub ambient: Vec4D
}



//= TriPylam4D ============================================================

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct TriPylam{
    pub vertex: [Vertex4D;4],
    pub normal: Vec4D
}

pub enum DivPattern{
    Zero4,  // 4点とも3D空間に含まれる：Triangleを4つ作成
    Zero3,  // 3点が３D空間に含まれる
    Zero2,  // 2点が３D空間に含まれる
    Zero1,  // 1点が３D空間に含まれる
    Zero0A,  // どの点も３D空間に含まれない：Triangleを2つ作成
    Zero0B,  // どの点も３D空間に含まれない
    No      // ３D空間と交差しない
}

impl TriPylam{

    pub fn new( v0: &Vertex4D, v1: &Vertex4D, v2: &Vertex4D, v3: &Vertex4D, nor: &Vec4D ) -> TriPylam {
        let plm = TriPylam{
            vertex: [*v0,*v1,*v2,*v3],
            normal: *nor
        };
        plm
    }
    pub fn new_with_center( v0: &Vertex4D, v1: &Vertex4D, v2: &Vertex4D, v3: &Vertex4D, center: &Vec4D ) -> Option<TriPylam> {
        let nor = calc_normal4d( &v0.vertex, &v1.vertex, &v2.vertex, &v3.vertex, center )?;
        let plm = TriPylam::new(v0, v1, v2, v3, &nor);
        Some(plm)
    }

    pub fn affine_transform( &self, mtx: &Mat4D, pos: &Vec4D ) -> TriPylam {
        TriPylam{
            vertex: [
                self.vertex[0].affine_transform(mtx,pos),
                self.vertex[1].affine_transform(mtx,pos),
                self.vertex[2].affine_transform(mtx,pos),
                self.vertex[3].affine_transform(mtx,pos)
            ],
            normal: mtx.mul_vec(&self.normal)
        }
    }

    pub fn scale(&self, scale: f32) -> TriPylam {
        TriPylam{
            vertex: [
                self.vertex[0].scale(scale),
                self.vertex[1].scale(scale),
                self.vertex[2].scale(scale),
                self.vertex[3].scale(scale)
            ],
            normal: self.normal
        }
    }

    fn check_div_pattern(&self, hpos: f32) -> (DivPattern, &Vertex4D, &Vertex4D, &Vertex4D, &Vertex4D) {
        let vtx0: &Vertex4D = &self.vertex[0];
        let vtx1: &Vertex4D = &self.vertex[1];
        let vtx2: &Vertex4D = &self.vertex[2];
        let vtx3: &Vertex4D = &self.vertex[3];

        let zero_pattern = (
            get_sign(vtx0.vertex.h-hpos),
            get_sign(vtx1.vertex.h-hpos),
            get_sign(vtx2.vertex.h-hpos),
            get_sign(vtx3.vertex.h-hpos)
        );
        let zero_num: i32   = if zero_pattern.0 == 0 {1}else{0}
                            + if zero_pattern.1 == 0 {1}else{0}
                            + if zero_pattern.2 == 0 {1}else{0}
                            + if zero_pattern.3 == 0 {1}else{0};
        let pat_sum = (zero_pattern.0 + zero_pattern.1 +zero_pattern.2 +zero_pattern.3).abs();

        match (zero_num, pat_sum) {
            (4,_) => (DivPattern::Zero4, vtx0, vtx1, vtx2, vtx3),
            (3,_) => match zero_pattern {
                    (1,0,0,0)|(-1,0,0,0) => (DivPattern::Zero3, vtx1, vtx2, vtx3, vtx0),
                    (0,1,0,0)|(0,-1,0,0) => (DivPattern::Zero3, vtx0, vtx2, vtx3, vtx1),
                    (0,0,1,0)|(0,0,-1,0) => (DivPattern::Zero3, vtx0, vtx1, vtx3, vtx2),
                    (0,0,0,1)|(0,0,0,-1) => (DivPattern::Zero3, vtx0, vtx1, vtx2, vtx3),
                    _ => (DivPattern::No, vtx0, vtx1, vtx2, vtx3)
                },
            (2,0) => match zero_pattern {
                    (1,-1,0,0)|(-1,1,0,0) => (DivPattern::Zero2, vtx2, vtx3, vtx0, vtx1),
                    (1,0,-1,0)|(-1,0,1,0) => (DivPattern::Zero2, vtx1, vtx3, vtx0, vtx2),
                    (1,0,0,-1)|(-1,0,0,1) => (DivPattern::Zero2, vtx1, vtx2, vtx0, vtx3),
                    (0,1,-1,0)|(0,-1,1,0) => (DivPattern::Zero2, vtx0, vtx3, vtx1, vtx2),
                    (0,1,0,-1)|(0,-1,0,1) => (DivPattern::Zero2, vtx0, vtx2, vtx1, vtx3),
                    (0,0,1,-1)|(0,0,-1,1) => (DivPattern::Zero2, vtx0, vtx1, vtx2, vtx3),
                    _ => (DivPattern::No, vtx0, vtx1, vtx2, vtx3)
                },
            (1,1) => match zero_pattern {
                (0,-1,1,1)|(0,1,-1,-1) => (DivPattern::Zero1, vtx0, vtx1, vtx2, vtx3),
                (0,1,-1,1)|(0,-1,1,-1) => (DivPattern::Zero1, vtx0, vtx2, vtx1, vtx3),
                (0,1,1,-1)|(0,-1,-1,1) => (DivPattern::Zero1, vtx0, vtx3, vtx1, vtx2),
                (-1,0,1,1)|(1,0,-1,-1) => (DivPattern::Zero1, vtx1, vtx0, vtx2, vtx3),
                (1,0,-1,1)|(-1,0,1,-1) => (DivPattern::Zero1, vtx1, vtx2, vtx0, vtx3),
                (1,0,1,-1)|(-1,0,-1,1) => (DivPattern::Zero1, vtx1, vtx3, vtx0, vtx2),
                (-1,1,0,1)|(1,-1,0,-1) => (DivPattern::Zero1, vtx2, vtx0, vtx1, vtx3),
                (1,-1,0,1)|(-1,1,0,-1) => (DivPattern::Zero1, vtx2, vtx1, vtx0, vtx3),
                (1,1,0,-1)|(-1,-1,0,1) => (DivPattern::Zero1, vtx2, vtx3, vtx0, vtx1),
                _ => (DivPattern::No, vtx0, vtx1, vtx2, vtx3)
                },
            (0,0) => match zero_pattern {
                (1,1,-1,-1)|(-1,-1,1,1) => (DivPattern::Zero0A, vtx0, vtx1, vtx2, vtx3),
                (1,-1,1,-1)|(-1,1,-1,1) => (DivPattern::Zero0A, vtx0, vtx2, vtx1, vtx3),
                (1,-1,-1,1)|(-1,1,1,-1) => (DivPattern::Zero0A, vtx0, vtx3, vtx1, vtx2),
                _ => (DivPattern::No, vtx0, vtx1, vtx2, vtx3)
            }
            (0,2) => match zero_pattern {
                (-1,1,1,1)|(1,-1,-1,-1) => (DivPattern::Zero0B, vtx0, vtx1, vtx2, vtx3),
                (1,-1,1,1)|(-1,1,-1,-1) => (DivPattern::Zero0B, vtx1, vtx0, vtx2, vtx3),
                (1,1,-1,1)|(-1,-1,1,-1) => (DivPattern::Zero0B, vtx2, vtx0, vtx1, vtx3),
                (1,1,1,-1)|(-1,-1,-1,1) => (DivPattern::Zero0B, vtx3, vtx0, vtx1, vtx2),
                _ => (DivPattern::No, vtx0, vtx1, vtx2, vtx3)
            }
            _ => (DivPattern::No, vtx0, vtx1, vtx2, vtx3),
        }
    }

    fn create_triangle( self, hpos:f32, pat: (DivPattern,&Vertex4D,&Vertex4D,&Vertex4D,&Vertex4D) ) -> Triangle {
        let mut trg: Triangle = Default::default();
        match pat.0 {
            DivPattern::Zero3 => {
                trg.vertex[0] = pat.1.shrink();
                trg.vertex[1] = pat.2.shrink();
                trg.vertex[2] = pat.3.shrink();
            },
            DivPattern::Zero2 => {
                trg.vertex[0] = pat.1.shrink();
                trg.vertex[1] = pat.2.shrink();
                trg.vertex[2] = shrink_lerp(hpos, pat.3, pat.4 );
            },
            DivPattern::Zero1 => {
                trg.vertex[0] = pat.1.shrink();
                trg.vertex[1] = shrink_lerp(hpos, pat.2, pat.3 );
                trg.vertex[2] = shrink_lerp(hpos, pat.2, pat.4 );
            },
            DivPattern::Zero0B => {
                trg.vertex[0] = shrink_lerp(hpos, pat.1, pat.2 );
                trg.vertex[1] = shrink_lerp(hpos, pat.1, pat.3 );
                trg.vertex[2] = shrink_lerp(hpos, pat.1, pat.4 );
            },
            _ => ()
        }
        trg
    }
    // DivPattern::Zero0A 専用
    fn create_2triangles( self, hpos:f32, vtx0: &Vertex4D, vtx1: &Vertex4D, vtx2: &Vertex4D, vtx3: &Vertex4D ) -> (Triangle, Triangle) {
        let v0 = shrink_lerp(hpos, vtx0, vtx2);
        let v1 = shrink_lerp(hpos, vtx0, vtx3);
        let v2 = shrink_lerp(hpos, vtx1, vtx2);
        let v3 = shrink_lerp(hpos, vtx1, vtx3);
        let trg0 = Triangle{ vertex: [ v0, v1, v2 ] };
        let trg1 = Triangle{ vertex: [ v1, v2, v3 ] };
        (trg0, trg1)
    }

    // DivPattern::Zero4 専用
    fn create_4triangles( self, vtx0: &Vertex4D, vtx1: &Vertex4D, vtx2: &Vertex4D, vtx3: &Vertex4D ) -> (Triangle, Triangle, Triangle, Triangle) {
        let trg0 = Triangle{ vertex: [ vtx0.shrink(), vtx1.shrink(), vtx2.shrink() ] };
        let trg1 = Triangle{ vertex: [ vtx1.shrink(), vtx2.shrink(), vtx3.shrink() ] };
        let trg2 = Triangle{ vertex: [ vtx2.shrink(), vtx3.shrink(), vtx0.shrink() ] };
        let trg3 = Triangle{ vertex: [ vtx3.shrink(), vtx0.shrink(), vtx1.shrink() ] };
        (trg0, trg1, trg2, trg3)
    }

    pub fn make_arrays( self, h_pos: f32, vtx_array: &mut [Vec<f32>;3]){
        let mut v_array = Vec::<f32>::new();
        let mut n_array = Vec::<f32>::new();
        let mut c_array = Vec::<f32>::new();
        let pattern = self.check_div_pattern( h_pos );
        match pattern.0 {
            DivPattern::Zero0A => {
                let tri_pack = self.create_2triangles(h_pos, pattern.1 ,pattern.2, pattern.3, pattern.4);
                tri_pack.0.push_buffer(&mut v_array, &mut n_array, &mut c_array);
                tri_pack.1.push_buffer(&mut v_array, &mut n_array, &mut c_array);
            },
            DivPattern::Zero4 => {
                let tri_pack = self.create_4triangles(pattern.1 ,pattern.2, pattern.3, pattern.4);
                tri_pack.0.push_buffer(&mut v_array, &mut n_array, &mut c_array);
                tri_pack.1.push_buffer(&mut v_array, &mut n_array, &mut c_array);
                tri_pack.2.push_buffer(&mut v_array, &mut n_array, &mut c_array);
                tri_pack.3.push_buffer(&mut v_array, &mut n_array, &mut c_array);
            },
            DivPattern::No => {
                // なにもしない
            }
            _ => {
                let tri_pack = self.create_triangle(h_pos, pattern);                
                tri_pack.push_buffer(&mut v_array, &mut n_array, &mut c_array);
            }
        }
        vtx_array[0].append(&mut v_array);
        vtx_array[1].append(&mut n_array);
        vtx_array[2].append(&mut c_array);
    }
}

//= TriPylam4DTex ============================================================

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct TriPylamTex{
    pub vertex:  [Vec4D;4],
    pub texture: [Vec3D;4],
    pub normal: Vec4D
}

impl TriPylamTex{

    #[allow(dead_code)]
    fn new( v0: Vec4D, v1: Vec4D, v2: Vec4D, v3: Vec4D, t0: Vec3D, t1: Vec3D, t2: Vec3D, t3: Vec3D, nor: Vec4D ) -> TriPylamTex {
        let plm = TriPylamTex{
            vertex: [v0,v1,v2,v3],
            texture: [t0, t1, t2, t3],
            normal: nor
        };
        plm
    }
}

//---------------------------------------------------------------------
// inProd4D
// 四次元ベクトルの内積を計算
pub fn inner_product4d( vec0: &Vec4D, vec1: &Vec4D ) -> f32 {
	 vec0.x*vec1.x + vec0.y*vec1.y + vec0.z*vec1.z + vec0.h*vec1.h
}

//---------------------------------------------------------------------
// shimada_product
// 島田積（４次元外積）
pub fn shimada_product ( v0: &Vec4D, v1: &Vec4D, v2: &Vec4D ) -> Vec4D {
	let vec = Vec4D{
		x: -(v0.y*v1.z*v2.h+v0.z*v1.h*v2.y+v0.h*v1.y*v2.z-v0.y*v1.h*v2.z-v0.z*v1.y*v2.h-v0.h*v1.z*v2.y),
		y:  (v0.x*v1.z*v2.h+v0.z*v1.h*v2.x+v0.h*v1.x*v2.z-v0.x*v1.h*v2.z-v0.z*v1.x*v2.h-v0.h*v1.z*v2.x),
		z: -(v0.x*v1.y*v2.h+v0.y*v1.h*v2.x+v0.h*v1.x*v2.y-v0.x*v1.h*v2.y-v0.y*v1.x*v2.h-v0.h*v1.y*v2.x),
		h:  (v0.x*v1.y*v2.z+v0.y*v1.z*v2.x+v0.z*v1.x*v2.y-v0.x*v1.z*v2.y-v0.y*v1.x*v2.z-v0.z*v1.y*v2.x)
    };
	vec
}

//---------------------------------------------------------------------
// calcNormal4D
// 四面体の法線を計算
//	vertex:   頂点、( x, y, z, h ) x 4個
//	center:	  四面体の属する図形の重心
fn calc_normal4d ( vtx0: &Vec4D, vtx1: &Vec4D, vtx2: &Vec4D, vtx3: &Vec4D, center: &Vec4D ) -> Option<Vec4D> {
	let v0 = vtx1-vtx0;
	let	v1 = vtx2-vtx0;
	let v2 = vtx3-vtx0;

	// 島田積によって法線を求める
	let mut nor = shimada_product(&v0,&v1,&v2);
    
    // 後処理：正負の方向を定める
    let mut dir = vtx0 - center;
    // 長さ0でない法線基準ベクトルを得る
    if dir.is_zero() {
        dir = vtx1 - center;
        if dir.is_zero() {
            dir = vtx2 - center;
            if dir.is_zero() {
                dir = vtx3 - center;
                if dir.is_zero() {
                    return None;
                }
            }
        }
    }

	let sign = inner_product4d( &nor, &dir );
    if sign < -std::f32::EPSILON {
        // 方向反転
		nor = -nor;
	}else
	if is_near_zero(sign) {
        return None;
    }
    nor.normalize();
    Some(nor)
}
