
//= データ定義 ====================================================================
/* 正八胞体 : 現時点ではまだ四角錘１つ */
const CUBE4_VTX_NUM: usize = 5;
static CUBE4_VTX: [Vec4D; CUBE4_VTX_NUM] = [
    Vec4D{ x:1.0, y:-1.0, z:1.0, h:0.0 },   // 最初の四角錘(Zero4)
    Vec4D{ x:1.0, y:1.0, z:1.0, h:0.0 },
    Vec4D{ x:-1.0, y:1.0, z:1.0, h:0.0 },
    Vec4D{ x:1.0, y:1.0, z:-1.0, h:0.0 },
    Vec4D{ x:0.0, y:0.0, z:0.0, h:-1.0 },
];

const CUBE4_VTX_INDEX_NUM: usize = 5;
static CUBE4_VTX_INDEX: [usize; CUBE4_VTX_INDEX_NUM] = [
    0,1,2,3,4
];

const CUBE4_NOR_NUM: usize = 5;
static CUBE4_NOR: [Vec4D; CUBE4_NOR_NUM] = [
    Vec4D{ x:0.0, y:0.1, z:0.2, h:0.3 },
    Vec4D{ x:3.0, y:0.1, z:0.2, h:0.3 },
    Vec4D{ x:0.0, y:3.1, z:0.2, h:0.3 },
    Vec4D{ x:0.0, y:0.1, z:3.2, h:0.3 },
    Vec4D{ x:0.0, y:0.1, z:0.2, h:3.3 },
];

const CUBE4_NOR_INDEX_NUM: usize = 5;
static CUBE4_NOR_INDEX: [usize; CUBE4_NOR_INDEX_NUM] = [
    0,1,2,3,4
];

const CUBE4_COL_NUM: usize = 5;
static CUBE4_COL: [Color<u8>; CUBE4_COL_NUM] = [
    Color{ r:0x80, g:0x80, b:0x80, a:0x80 },
    Color{ r:0x80, g:0x00, b:0x00, a:0x01 },
    Color{ r:0xff, g:0xff, b:0xff, a:0xff },
    Color{ r:0x0f, g:0x0f, b:0x0f, a:0x0f },
    Color{ r:0xf0, g:0xf0, b:0xf0, a:0xf0 },
];

const CUBE4_COL_INDEX_NUM: usize = 5;
static CUBE4_COL_INDEX: [usize; CUBE4_COL_INDEX_NUM] = [
    0,1,2,3,4
];

//=====================================================================use std::ops::Add;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Neg;
use std::mem;

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

impl Color<u8>{
    const LEN_F32: usize = 1;

    fn expand_f32(&self) -> (f32,f32,f32,f32) {
        ((self.r as f32)/255.0, (self.g as f32)/255.0, (self.b as f32)/255.0, (self.a as f32)/255.0 )
    }

    fn pack_f32(&self) -> f32 {
        let raw_bytes = [self.r,self.g,self.b,self.a];
        let packet = u32::from_ne_bytes(raw_bytes);
        unsafe{
            std::mem::transmute::<u32, f32>(packet)
        }
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
    const LEN_F32: usize = 3;
    const LEN_U8: usize = Vec3D::LEN_F32*mem::size_of::<f32>();

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
    const LEN_F32: usize = 4;
    const LEN_U8: usize = Vec4D::LEN_F32*mem::size_of::<f32>();

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

    fn is_near_zero(&self) -> bool {
        is_near_zero(self.x)&&is_near_zero(self.y)&&is_near_zero(self.z)&&is_near_zero(self.h)
    }

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

impl Vertex3D {
    const LEN_F32: usize = Vec3D::LEN_F32*2 + Color::<u8>::LEN_F32;
}

//= Vertex3DTex ============================================================
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Vertex3DTex{
    pub vertex: Vec3D,
    pub normal: Vec3D,
    pub texture: Vec3D,
    pub color:  Color<u8>
}

impl Vertex3DTex {
    const LEN_F32: usize = Vec3D::LEN_F32*3 + Color::<u8>::LEN_F32;
}


//= Vertex4D ============================================================
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Vertex4D{
    pub vertex: Vec4D,
    pub normal: Vec4D,
    pub color:  Color<u8>
}

impl Vertex4D {
    const LEN_F32: usize = Vec4D::LEN_F32*2 + Color::<u8>::LEN_F32;

    fn shrink(&self) -> Vertex3D {
        Vertex3D{
            vertex: Vec3D{ x:self.vertex.x, y:self.vertex.y, z:self.vertex.z },
            normal: Vec3D{ x:self.normal.x, y:self.normal.y, z:self.normal.z },
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

impl Vertex4DTex {
    const LEN_F32: usize = Vec4D::LEN_F32*3 + Color::<u8>::LEN_F32;

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
    const LEN_F32: usize = Vertex3D::LEN_F32 + Color::<u8>::LEN_F32;

    fn decompose(&self, buf: &mut Vec<f32>){
        for idx in 0..3{
            buf.push(self.vertex[idx].vertex.x);
            buf.push(self.vertex[idx].vertex.y);
            buf.push(self.vertex[idx].vertex.z);
            buf.push(self.vertex[idx].normal.x);
            buf.push(self.vertex[idx].normal.y);
            buf.push(self.vertex[idx].normal.z);
            buf.push( self.vertex[idx].color.pack_f32() );
/*
            let color_pack = self.vertex[idx].color.expand_f32();
            buf.push(color_pack.0);
            buf.push(color_pack.1);
            buf.push(color_pack.2);
            buf.push(color_pack.3);
*/
/*
            println!("DecV{}: {},{},{}",idx,self.vertex[idx].vertex.x,self.vertex[idx].vertex.y,self.vertex[idx].vertex.z);
            println!("   N{}: {},{},{}",idx,self.vertex[idx].normal.x,self.vertex[idx].normal.y,self.vertex[idx].normal.z);
            println!("   C{}: {},{},{},{}",idx, color_pack.0,color_pack.1,color_pack.2,color_pack.3);
*/
        }
    }
}

//= TriangleTex ============================================================
#[derive(Debug, PartialEq, Default)]
pub struct TriangleTex{
    vertex: [Vertex3DTex;3],
}

impl TriangleTex {
//    const LEN_F32: i32 = 13;

    fn decompose(&self, buf: &mut Vec<f32>){
        for idx in 0..3{
            buf.push(self.vertex[idx].vertex.x);
            buf.push(self.vertex[idx].vertex.y);
            buf.push(self.vertex[idx].vertex.z);
            buf.push(self.vertex[idx].normal.x);
            buf.push(self.vertex[idx].normal.y);
            buf.push(self.vertex[idx].normal.z);
            buf.push(self.vertex[idx].texture.x);
            buf.push(self.vertex[idx].texture.y);
            buf.push(self.vertex[idx].texture.z);
            let color_pack = self.vertex[idx].color.expand_f32();
            buf.push(color_pack.0);
            buf.push(color_pack.1);
            buf.push(color_pack.2);
            buf.push(color_pack.3);
/*
            println!("DecV{}: {},{},{}",idx,self.vertex[idx].vertex.x,self.vertex[idx].vertex.y,self.vertex[idx].vertex.z);
            println!("   N{}: {},{},{}",idx,self.vertex[idx].normal.x,self.vertex[idx].normal.y,self.vertex[idx].normal.z);
            println!("   T{}: {},{},{}",idx,self.vertex[idx].texture.x,self.vertex[idx].texture.y,self.vertex[idx].texture.z);
            println!("   C{}: {},{},{},{}",idx, color_pack.0,color_pack.1,color_pack.2,color_pack.3);
*/
        }
    }
}

//= Mat4D ============================================================
const AXIS_XY: i32 = 0;
const AXIS_YZ: i32 = 1;
const AXIS_ZX: i32 = 2;
const AXIS_ZH: i32 = 3;
const AXIS_XH: i32 = 4;
const AXIS_YH: i32 = 5;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Mat4D{
    pub a: [f32;16],
}

impl Mat4D {
    const LEN_F32: usize = 16;
    const LEN_U8: usize = Mat4D::LEN_F32*mem::size_of::<f32>();

    pub fn mul_vec(&self, pos: &Vec4D) -> Vec4D {
        let dst = Vec4D {
            x: self.a[0]*pos.x+self.a[4]*pos.y+self.a[ 8]*pos.z+self.a[12]*pos.h,
            y: self.a[1]*pos.x+self.a[5]*pos.y+self.a[ 9]*pos.z+self.a[13]*pos.h,
            z: self.a[2]*pos.x+self.a[6]*pos.y+self.a[10]*pos.z+self.a[14]*pos.h,
            h: self.a[3]*pos.x+self.a[7]*pos.y+self.a[11]*pos.z+self.a[15]*pos.h
        };
        dst
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
            AXIS_XY => {  // XY
                mtx.a[0] = cos_x;
                mtx.a[5] = cos_x;
                mtx.a[1] = sin_x;
                mtx.a[4] = -sin_x;
                mtx.a[10] = 1.0;
                mtx.a[15] = 1.0;
                },
            AXIS_YZ => {  // YZ
                mtx.a[0] = 1.0;
                mtx.a[ 5] = cos_x;
                mtx.a[10] = cos_x;
                mtx.a[ 6] = sin_x;
                mtx.a[ 9] = -sin_x;
                mtx.a[15] = 1.0;
            },
            AXIS_ZX => {  // ZX
                mtx.a[ 0] = cos_x;
                mtx.a[10] = cos_x;
                mtx.a[8] = sin_x;
                mtx.a[2] = -sin_x;
                mtx.a[5] = 1.0;
                mtx.a[15] = 1.0;
                },
            AXIS_YH => {  // YH
                mtx.a[0] = 1.0;
                mtx.a[ 5] = cos_x;
                mtx.a[15] = cos_x;
                mtx.a[ 7] = -sin_x;
                mtx.a[13] = sin_x;
                mtx.a[10] = 1.0;
                },
            AXIS_ZH => {  // ZH
                mtx.a[0] = 1.0;
                mtx.a[5] = 1.0;
                mtx.a[10] = cos_x;
                mtx.a[15] = cos_x;
                mtx.a[11] = sin_x;
                mtx.a[14] = -sin_x;
                },
            AXIS_XH => {  // HX
                mtx.a[ 0] = cos_x;
                mtx.a[15] = cos_x;
                mtx.a[ 3] = sin_x;
                mtx.a[12] = -sin_x;
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
/*
            a: [ vx.x, vx.y, vx.z, -(vx.x*eye.x + vx.y*eye.y + vx.z*eye.z),
                 vy.x, vy.y, vy.z, -(vy.x*eye.x + vy.y*eye.y + vy.z*eye.z),
                 vz.x, vz.y, vz.z, -(vz.x*eye.x + vz.y*eye.y + vz.z*eye.z),
                 0.0,  0.0,  0.0,  1.0
            ]
*/
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
/*
                vx.x, vx.y, vx.z, -(vx.x*self.eye.x + vx.y*self.eye.y + vx.z*self.eye.z),
                vy.x, vy.y, vy.z, -(vy.x*self.eye.x + vy.y*self.eye.y + vy.z*self.eye.z),
                vz.x, vz.y, vz.z, -(vz.x*self.eye.x + vz.y*self.eye.y + vz.z*self.eye.z),
                0.0,  0.0,  0.0,  1.0
*/
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
//        let eye_x = (-self.look_at.x + self.eye.x) * cos_x - (-self.look_at.z + self.eye.z) * sin_x;
        let eye_x = self.eye_base.x * cos_x - self.eye_base.z * sin_x;
//        let eye_z = (-self.look_at.x + self.eye.x) * sin_x + (-self.look_at.z + self.eye.z) * cos_x;
        let eye_z = self.eye_base.x * sin_x + self.eye_base.z * cos_x;
        self.eye.x = self.look_at.x+eye_x;
        self.eye.z = self.look_at.z+eye_z;
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

enum DivPattern{
    Zero4,  // 4点とも3D空間に含まれる：Triangleを4つ作成
    Zero3,  // 3点が３D空間に含まれる
    Zero2,  // 2点が３D空間に含まれる
    Zero1,  // 1点が３D空間に含まれる
    Zero0A,  // どの点も３D空間に含まれない：Triangleを2つ作成
    Zero0B,  // どの点も３D空間に含まれない
    No      // ３D空間と交差しない
}

impl TriPylam{
    const VERTEX_NUM: usize = 4;

    fn new( v0: &Vertex4D, v1: &Vertex4D, v2: &Vertex4D, v3: &Vertex4D, nor: &Vec4D ) -> TriPylam {
        let plm = TriPylam{
            vertex: [*v0,*v1,*v2,*v3],
            normal: *nor
        };
        plm
    }
    fn new_with_center( v0: &Vertex4D, v1: &Vertex4D, v2: &Vertex4D, v3: &Vertex4D, center: &Vec4D ) -> Option<TriPylam> {
        let nor = calc_normal4d( &v0.vertex, &v1.vertex, &v2.vertex, &v3.vertex, center )?;
        let plm = TriPylam::new(v0, v1, v2, v3, &nor);
        Some(plm)
    }
    fn check_div_pattern(&self, hpos: f32) -> (DivPattern, &Vertex4D, &Vertex4D, &Vertex4D, &Vertex4D) {
        let mut pattern = AXIS_XY;
        let mut vtx0: &Vertex4D = &self.vertex[0];
        let mut vtx1: &Vertex4D = &self.vertex[1];
        let mut vtx2: &Vertex4D = &self.vertex[2];
        let mut vtx3: &Vertex4D = &self.vertex[3];

        let zero_pattern = (
            get_sign(vtx0.vertex.h-hpos),
            get_sign(vtx1.vertex.h-hpos),
            get_sign(vtx2.vertex.h-hpos),
            get_sign(vtx3.vertex.h-hpos)
        );
        let zero_num: i32   = if( zero_pattern.0 == 0 ){1}else{0}
                            + if( zero_pattern.1 == 0 ){1}else{0}
                            + if( zero_pattern.2 == 0 ){1}else{0}
                            + if( zero_pattern.3 == 0 ){1}else{0};
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
}

#[test]
fn test_tripylam(){
    let v0 = Vec4D{ x:1.0, y:0.0, z:0.0, h:0.0 };
    let v1 = Vec4D{ x:0.0, y:1.0, z:0.0, h:0.0 };
    let v2 = Vec4D{ x:0.0, y:0.0, z:1.0, h:0.0 };
    let v3 = Vec4D{ x:0.0, y:0.0, z:0.0, h:1.0 };
    let center0 = Vec4D{ x:0.0, y:0.0, z:0.0, h:0.0 };
    let center1 = Vec4D{ x:1.0, y:1.0, z:1.0, h:1.0 };
    let col = Color{ r:255, g:255, b:255, a:255 };

    let vtx0 = Vertex4D{ vertex: v0, normal: v0, color: col };
    let vtx1 = Vertex4D{ vertex: v1, normal: v1, color: col };
    let vtx2 = Vertex4D{ vertex: v2, normal: v2, color: col };
    let vtx3 = Vertex4D{ vertex: v3, normal: v3, color: col };

    let plm0 = TriPylam::new_with_center(&vtx0, &vtx1, &vtx2, &vtx3, &center0);
    let plm1 = TriPylam::new_with_center(&vtx0, &vtx1, &vtx2, &vtx3, &center1);

    let nor0 = plm0.unwrap().normal;
    let nor1 = plm1.unwrap().normal;
    
//    println!("Nor0: {},{},{},{}",nor0.x, nor0.y, nor0.z, nor0.h);
//    println!("Nor1: {},{},{},{}",nor1.x, nor1.y, nor1.z, nor1.h);

    let nor2 = &nor0+&nor1;
    assert!( nor2.is_zero());
}

//= TriPylam4DTex ============================================================

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct TriPylamTex{
    pub vertex:  [Vec4D;4],
    pub texture: [Vec3D;4],
    pub normal: Vec4D
}

impl TriPylamTex{
    const VERTEX_NUM: usize = 4;

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
    if( dir.is_zero() ){
        dir = vtx1 - center;
        if( dir.is_zero() ){
            dir = vtx2 - center;
            if( dir.is_zero() ){
                dir = vtx3 - center;
                if( dir.is_zero() ){
                    return None;
                }
            }
        }
    }

	let sign = inner_product4d( &nor, &dir );
    if( sign < -std::f32::EPSILON ){
        // 方向反転
		nor = -nor;
	}else
	if is_near_zero(sign) {
        return None;
    }
    nor.normalize();
    Some(nor)
}

#[test]
fn test_calc_normal4d(){
    let v0 = Vec4D{ x:0.0, y:0.0, z:0.0, h:0.0 };
    let v1 = Vec4D{ x:0.0, y:0.0, z:0.0, h:0.0 };
    let v2 = Vec4D{ x:0.0, y:0.0, z:0.0, h:0.0 };
    let v3 = Vec4D{ x:0.0, y:0.0, z:0.0, h:0.0 };
    let center0 = Vec4D{ x:0.0, y:0.0, z:0.0, h:0.0 };

    let nor = calc_normal4d(&v0, &v1, &v2, &v3, &center0);
    assert_eq!( nor, None );
    
    let mut vt = Vec4D{ x:0.0, y:0.0, z:0.0, h:1.0 };
    vt.normalize();
    assert!( is_near_zero(vt.x), "Nor0.X: {}", vt.x );
    assert!( is_near_zero(vt.y), "Nor0.Y: {}", vt.y );
    assert!( is_near_zero(vt.z), "Nor0.Z: {}", vt.z );
    assert!( is_near_eq(vt.h, 1.0), "Nor0.H: {}", vt.h );

    let v0 = Vec4D{ x:0.0, y:0.0, z:0.0, h:0.0 };
    let v1 = Vec4D{ x:1.0, y:0.0, z:0.0, h:0.0 };
    let v2 = Vec4D{ x:0.0, y:1.0, z:0.0, h:0.0 };
    let v3 = Vec4D{ x:0.0, y:0.0, z:1.0, h:0.0 };

    let center0 = Vec4D{ x:0.0, y:0.0, z:0.0, h:-1.0 };

    let nor0 = calc_normal4d(&v0, &v1, &v2, &v3, &center0).unwrap();
//    println!("Nor0: {},{},{},{}",nor0.x, nor0.y, nor0.z, nor0.h);

    assert!( is_near_zero(nor0.x), "Nor0.X: {}", nor0.x );
    assert!( is_near_zero(nor0.y), "Nor0.Y: {}", nor0.y );
    assert!( is_near_zero(nor0.z), "Nor0.Z: {}", nor0.z );
    assert!( is_near_eq(nor0.h, 1.0), "Nor0.H: {}", nor0.h );
}


//------------------------------------------------------------------
// affine4D
// ４次元アフィン変換
//	src:		// 頂点、( x, y, z, h )x数
//	rotate:		// ローカル座標系での回転
//	offs:		// ローカス座標系での位置オフセット
//	scale:		// 各方向のスケール

fn affine4d( src: &Vec<Vec4D>, rotate: &[f32;6], rot_plane: &[i32;6], offs: &Vec4D, scale: &Vec4D ) -> Vec<Vec4D> { 

    // 各Matrixの生成
    let mut scale_mtx = Mat4D::identity();
    scale_mtx.a[ 0] = scale.x;
    scale_mtx.a[ 5] = scale.y;
    scale_mtx.a[10] = scale.z;
    scale_mtx.a[15] = scale.h;
    let rotate_array: [Mat4D;6] = [
        Mat4D::rotate(rotate[0],rot_plane[0]),
        Mat4D::rotate(rotate[1],rot_plane[1]),
        Mat4D::rotate(rotate[2],rot_plane[2]),
        Mat4D::rotate(rotate[3],rot_plane[3]),
        Mat4D::rotate(rotate[4],rot_plane[4]),
        Mat4D::rotate(rotate[5],rot_plane[5])
    ];

    // 各Matrixの合成
    let mut rotate_mtx = rotate_array[0];
    rotate_mtx = &rotate_mtx*&rotate_array[1];
    rotate_mtx = &rotate_mtx*&rotate_array[2];
    rotate_mtx = &rotate_mtx*&rotate_array[3];
    rotate_mtx = &rotate_mtx*&rotate_array[4];
    rotate_mtx = &rotate_mtx*&rotate_array[5];
    let affine_mtx = &rotate_mtx*&scale_mtx;

    // 各頂点のaffine変換
    let mut dst: Vec<Vec4D> = Vec::new();
    for v in src {
        let tmp = &affine_mtx.mul_vec(v) + offs;
        dst.push(tmp);
    }
    dst
}

//=====================================================================
//JavaScriptから呼ばれる関数

#[no_mangle]
pub fn lerp1( v0: f32, v1: f32, rate: f32 ) -> f32 {
    v0 + rate*( v1 - v0)
}

// バッファを作成
#[no_mangle]
fn make_heap_buf( cap: usize ) -> usize {
    let mut vec_f32: Vec<f32> = Vec::with_capacity(cap);
    vec_f32.as_mut_ptr() as usize
}

// 頂点列をアフィン変換後、三角ポリゴン列を生成
#[no_mangle]
pub fn assemble_triangle_3d(
    // v:vertex, n:normal, c:color, ?i:index
    // ptr:buffer pointer, len:buffer length
    // mtx4: Matrix4D+Vec4(Offset)
    v_ptr: usize, v_len: usize, // 現在の予定では、頂点は5つ組となる。最後の一つは親のセンターとなる。
    n_ptr: usize, n_len: usize, // 現在の予定では、法線は5つ組となる。最後の一つは親の法線となる。
    c_ptr: usize, c_len: usize,
    vi_ptr: usize, ni_ptr: usize, ci_ptr: usize, idx_len: usize,
    poly_type: i32, // 生成するポリゴンの法線等のタイプ。 0: 親の法線、1: 親のセンター
    mtx4: usize,
    hpos: f32,
    rcv_buf: usize, buf_len: usize  // 受け取りバッファ
) -> usize {

    // ぬるぽチェック
    if((v_ptr==0)||(n_ptr==0)||(c_ptr==0)||(vi_ptr==0)||(ni_ptr==0)||(ci_ptr==0)||(mtx4==0)){
//        return 0;
    }

   // affine変換行列を作成
    let mut rot4 = Mat4D::identity();
    let mut offs: Vec4D = Default::default();
    unsafe{
        let mtx_ptr = mtx4 as *mut f32;
        let mtx_slice: &[f32] = std::slice::from_raw_parts(mtx_ptr, Mat4D::LEN_U8+Vec4D::LEN_U8);
        println!("Mtx: ");
        for x in 0..Mat4D::LEN_F32{
            rot4.a[x] = mtx_slice[x];
               print!("{}", rot4.a[x]);
        }
        offs.x = mtx_slice[Mat4D::LEN_F32];
        offs.y = mtx_slice[Mat4D::LEN_F32+1];
        offs.z = mtx_slice[Mat4D::LEN_F32+2];
        offs.h = mtx_slice[Mat4D::LEN_F32+3];
        println!("\r\nOffs: {},{},{},{}",offs.x,offs.y,offs.z,offs.h);
    }
    // インデックスを読込み
    let mut v_idx: Vec<u16> = Vec::with_capacity(idx_len);
    let mut n_idx: Vec<u16> = Vec::with_capacity(idx_len);
    let mut c_idx: Vec<u16> = Vec::with_capacity(idx_len);
    unsafe{
        let v_idx_ptr = vi_ptr as *const u16;
        let vi_slice: &[u16] = std::slice::from_raw_parts(v_idx_ptr, idx_len);
        let n_idx_ptr = ni_ptr as *const u16;
        let ni_slice: &[u16] = std::slice::from_raw_parts(n_idx_ptr, idx_len);
        let c_idx_ptr = ci_ptr as *const u16;
        let ci_slice: &[u16] = std::slice::from_raw_parts(c_idx_ptr, idx_len);
        for x in 0..idx_len {
            &v_idx.push(vi_slice[x]);
            &n_idx.push(ni_slice[x]);
            &c_idx.push(ci_slice[x]);
        }
    }
    // 頂点情報を読込み
    let mut v_buf: Vec<f32> = Vec::with_capacity(v_len);
    let mut n_buf: Vec<f32> = Vec::with_capacity(n_len);
    let mut c_buf: Vec<u32> = Vec::with_capacity(c_len);
    unsafe{
        let vbf_ptr = v_ptr as *const f32;
        let vbf_slice: &[f32] = std::slice::from_raw_parts(vbf_ptr, v_len);
        let nbf_ptr = n_ptr as *const f32;
        let nbf_slice: &[f32] = std::slice::from_raw_parts(nbf_ptr, n_len);
        let cbf_ptr = c_ptr as *const u32;
        let cbf_slice: &[u32] = std::slice::from_raw_parts(cbf_ptr, c_len);
        for x in 0..v_len {
            &v_buf.push(vbf_slice[x]);
        }
        for x in 0..n_len {
            &n_buf.push(nbf_slice[x]);
        }
        for x in 0..c_len {
            &c_buf.push(cbf_slice[x]);
        }
    }

    // 出力用バッファを用意
    let mut tri_buf: Vec<f32> = Vec::with_capacity(idx_len);

    // 4面体を形成 → 3角形を形成 → バッファに放り込む
    let mut idx = 0;
    let mut c_cnt = 0;
    const INDEX_UNIT: usize = 5;
    let mut item_count = 0;
    while idx < idx_len {
        //Vertexを4つ作り、アフィン変換を行い、しかる後四面体を作る
//        let mut vtx = [Vec4D{ x:0.0, y:0.0, z:0.0, h:0.0 };4];
//        let mut nor = [Vec4D{ x:0.0, y:0.0, z:0.0, h:0.0 };4];
//        let mut col = [Vec4D{ x:0.0, y:0.0, z:0.0, h:0.0 };4];
        let mut ver: [Vertex4D;TriPylam::VERTEX_NUM] = [ Default::default();TriPylam::VERTEX_NUM];
        print!("\r\n");
        for ii in 0..TriPylam::VERTEX_NUM{
            let idx_base = idx+ii;
            let v_base = v_idx[idx_base] as usize*mem::size_of::<i32>();
            let v = Vec4D{ x:v_buf[v_base], y:v_buf[v_base+1], z:v_buf[v_base+2], h:v_buf[v_base+3] };
            ver[ii].vertex = &rot4.mul_vec(&v) + &offs;
            
            let n_base = n_idx[idx_base] as usize*mem::size_of::<i32>();
            let n = Vec4D{ x:n_buf[n_base], y:n_buf[n_base+1], z:n_buf[n_base+2], h:n_buf[n_base+3] };
            ver[ii].normal = rot4.mul_vec(&n);

            let c_base = c_idx[c_cnt+ii] as usize;
            let c = c_buf[c_base];
            ver[ii].color.r = ((&c>>24)&0xff) as u8;
            ver[ii].color.g = ((&c>>16)&0xff) as u8;
            ver[ii].color.b = ((&c>> 8)&0xff) as u8;
            ver[ii].color.a = ((&c    )&0xff) as u8;
        }
        let mut plm: TriPylam = Default::default();
        match poly_type {
            0 => {  // normalをそのまま利用
                //let parent_normal = Vec4D{ x:0.0, y:0.0, z:0.0, h:0.0 };    // 親を持ってこないといけない
                let v_base = v_idx[idx+4] as usize;
                let v = Vec4D{ x:v_buf[v_base], y:v_buf[v_base+1], z:v_buf[v_base+2], h:v_buf[v_base+3] };
                let nor = &rot4.mul_vec(&v);
                plm = TriPylam::new( &ver[0], &ver[1], &ver[2], &ver[3], &nor );
            },
            1 => {  // centerからnormalを生成
                //let parent_normal = Vec4D{ x:0.0, y:0.0, z:0.0, h:0.0 };    // センターを持ってこないといけない
                let v_base = v_idx[idx+4] as usize;
                let v = Vec4D{ x:v_buf[v_base], y:v_buf[v_base+1], z:v_buf[v_base+2], h:v_buf[v_base+3] };
                let center = &rot4.mul_vec(&v) + &offs;
                plm = TriPylam::new_with_center( &ver[0], &ver[1], &ver[2], &ver[3], &center ).unwrap_or_default();
            },
            _ => {  // その他
                // plm = Default::default();
            }
        }
        let check_pattern = plm.check_div_pattern(hpos);
        let mut triangle: Triangle = Default::default();
        match check_pattern.0 {
            DivPattern::Zero0A => {
                let tri_pack = plm.create_2triangles( hpos, check_pattern.1, check_pattern.2, check_pattern.3, check_pattern.4 );
                tri_pack.0.decompose(&mut tri_buf);
                tri_pack.1.decompose(&mut tri_buf);
                item_count += 2;
                println!("CheckPat: Zero0A");
            },
            DivPattern::Zero4 => {
                let tri_pack = plm.create_4triangles( check_pattern.1, check_pattern.2, check_pattern.3, check_pattern.4 );
                tri_pack.0.decompose(&mut tri_buf);
                tri_pack.1.decompose(&mut tri_buf);
                tri_pack.2.decompose(&mut tri_buf);
                tri_pack.3.decompose(&mut tri_buf);
/*
                println!("Tri0: {},{},{}", tri_pack.0.vertex[0].vertex.x, tri_pack.0.vertex[0].vertex.y, tri_pack.0.vertex[0].vertex.z,);
                println!("    : {},{},{}", tri_pack.0.vertex[1].vertex.x, tri_pack.0.vertex[1].vertex.y, tri_pack.0.vertex[1].vertex.z,);
                println!("    : {},{},{}", tri_pack.0.vertex[2].vertex.x, tri_pack.0.vertex[2].vertex.y, tri_pack.0.vertex[2].vertex.z,);
                println!("Tri1: {},{},{}", tri_pack.1.vertex[0].vertex.x, tri_pack.1.vertex[0].vertex.y, tri_pack.1.vertex[0].vertex.z,);
                println!("    : {},{},{}", tri_pack.1.vertex[1].vertex.x, tri_pack.1.vertex[1].vertex.y, tri_pack.1.vertex[1].vertex.z,);
                println!("    : {},{},{}", tri_pack.1.vertex[2].vertex.x, tri_pack.1.vertex[2].vertex.y, tri_pack.1.vertex[2].vertex.z,);
                println!("Tri2: {},{},{}", tri_pack.2.vertex[0].vertex.x, tri_pack.2.vertex[0].vertex.y, tri_pack.2.vertex[0].vertex.z,);
                println!("    : {},{},{}", tri_pack.2.vertex[1].vertex.x, tri_pack.2.vertex[1].vertex.y, tri_pack.2.vertex[1].vertex.z,);
                println!("    : {},{},{}", tri_pack.2.vertex[2].vertex.x, tri_pack.2.vertex[2].vertex.y, tri_pack.2.vertex[2].vertex.z,);
                println!("Tri3: {},{},{}", tri_pack.3.vertex[0].vertex.x, tri_pack.3.vertex[0].vertex.y, tri_pack.3.vertex[0].vertex.z,);
                println!("    : {},{},{}", tri_pack.3.vertex[1].vertex.x, tri_pack.3.vertex[1].vertex.y, tri_pack.3.vertex[1].vertex.z,);
                println!("    : {},{},{}", tri_pack.3.vertex[2].vertex.x, tri_pack.3.vertex[2].vertex.y, tri_pack.3.vertex[2].vertex.z,);
*/
                item_count += 4;
                println!("CheckPat: Zero04");
            },
            _ => {
                triangle = plm.create_triangle(hpos, check_pattern);
                triangle.decompose(&mut tri_buf);
                item_count += 1;
                println!("CheckPat: Solo");
            }
        }

        idx += INDEX_UNIT;
        c_cnt += (INDEX_UNIT-1);
    }
    
    unsafe{
        let rcv_ptr = rcv_buf as *mut f32;
        let rcv_slice: &mut [f32] = std::slice::from_raw_parts_mut(rcv_ptr, buf_len);
        let loop_size = if tri_buf.len() < buf_len { tri_buf.len() }else{ buf_len };
        println!("LoopSize: {}, TriBuf: {}, BufLen: {}", loop_size, tri_buf.len(), buf_len);
        for x in 0..loop_size{
            rcv_slice[x] = tri_buf[x];
        }


    }

    tri_buf.len()
}

#[test]
fn test_look_at(){
    let eye_position = Vec3D{ x:0.0, y:2.0, z:6.0 };
    let look_at = Vec3D{ x:0.0, y:0.0, z:-4.0 };

    let v_mat = Mat4D::look_at(&eye_position, &look_at, &Vec3D{ x:0.0, y:1.0, z:0.0 });
    println!("Mat:");
    println!("{},{},{},{},",v_mat.a[ 0],v_mat.a[ 1],v_mat.a[ 2],v_mat.a[ 3]);
    println!("{},{},{},{},",v_mat.a[ 4],v_mat.a[ 5],v_mat.a[ 6],v_mat.a[ 7]);
    println!("{},{},{},{},",v_mat.a[ 8],v_mat.a[ 9],v_mat.a[10],v_mat.a[11]);
    println!("{},{},{},{},",v_mat.a[12],v_mat.a[13],v_mat.a[14],v_mat.a[15]);
}

#[test]
fn test_assemble_triangle_3d(){
    // 仮変数設定
    let mut v_buf: Vec<f32> = vec![
        1.0,-1.0,1.0,0.0,   // 最初の四角錘(Zero4)
        1.0, 1.0,1.0,0.0,
        -1.0,1.0,1.0,0.0,
        1.0,1.0,-1.0,0.0,
        0.0,0.0,0.0,-1.0,
        2.0,-2.0,2.0, 2.0,   // 第二の四角錘(Zero0B)
        2.0, 2.0,2.0, 2.0,
        -2.0,2.0,2.0, 2.0,
        0.0, 0.0,2.0,-2.0,
        0.0, 0.0,0.0, 2.0,
    ];
    let v_len = v_buf.len();
    let v_ptr = v_buf.as_mut_ptr() as usize;
    let mut vi_buf: Vec<u16> = vec![ 0,1,2,3,4, 5,6,7,8,9 ];
    let vi_ptr = vi_buf.as_mut_ptr() as usize;
    // インデックス長を設定
    let idx_len = vi_buf.len();

    let mut n_buf: Vec<f32> = vec![
        0.0,0.1,0.2,0.3,
        3.0,0.1,0.2,0.3,
        0.0,3.1,0.2,0.3,
        0.0,0.1,3.2,0.3,
        0.0,0.1,0.2,3.3
    ];
    let n_len = n_buf.len();
    let n_ptr = n_buf.as_mut_ptr() as usize;
    let mut ni_buf: Vec<u16> = vec![ 0,1,2,3,4, 0,1,2,3,4 ];
    let ni_ptr = ni_buf.as_mut_ptr() as usize;

    let mut c_buf: Vec<u32> = vec![ 0x80808080, 0x8000001, 0xffffffff, 0x0f0f0f0f, 0xf0f0f0f0 ];
    let c_len = c_buf.len();
    let c_ptr = c_buf.as_mut_ptr() as usize;
    let mut ci_buf: Vec<u16> = vec![ 4,3,2,1,0, 0,1,2,3,4 ];
    let ci_ptr = ci_buf.as_mut_ptr() as usize;

    let poly_type = 0;  // 生成するポリゴンの法線等のタイプ。 0: 親の法線、1: 親のセンター

    let mut mtx_buf: Vec<f32> = vec![
        1.0,0.0,0.0,0.0,
        0.0,1.0,0.0,0.0,
        0.0,0.0,1.0,0.0,
        0.0,0.0,0.0,1.0,
        0.0,0.0,0.0,0.0     // offs
    ];
    let mtx4 = mtx_buf.as_mut_ptr() as usize;

    let hpos = 0.0;

    const TRIANGLE_SIZE: usize = 21;
    let mut tri_buf: Vec<f32> = Vec::with_capacity(idx_len*TRIANGLE_SIZE);
    let tri_len = idx_len*TRIANGLE_SIZE;
    let tri_ptr = tri_buf.as_mut_ptr() as usize;
    let vec_num = assemble_triangle_3d(
        // v:vertex, n:normal, c:color, ?i:index
        // ptr:buffer pointer, len:buffer length
        // mtx4: Matrix4D+Vec4(Offset)
        v_ptr, v_len, // 現在の予定では、頂点は5つ組となる。最後の一つは親のセンターとなる。
        n_ptr, n_len, // 現在の予定では、法線は5つ組となる。最後の一つは親の法線となる。
        c_ptr, c_len,
        vi_ptr, ni_ptr, ci_ptr, idx_len,
        poly_type,
        mtx4,
        hpos,
        tri_ptr, tri_len
    );
/**/
    unsafe{
        let tri_buf = tri_ptr as *const f32;
        let tri_slice: &[f32] = std::slice::from_raw_parts(tri_buf, idx_len*TRIANGLE_SIZE);
        let mut idx = 0;
        println!("Item Count: {}", vec_num);
        for y in 0..6{
            println!("Vec{}:", y);
            for x in 0..3{
                print!("{}, ",tri_slice[idx]);
                idx += 1;
            }
            println!("\r\nNor{}:", y);
            for x in 3..6{
                print!("{}, ",tri_slice[idx]);
                idx += 1;
            }
            println!("\r\nCol{}: {:x}", y, std::mem::transmute::<f32, u32>(tri_slice[idx]) );
            idx += 1;

            println!("\r\nIdx: {}",idx);
        }
    }
/**/
}

// 頂点列をアフィン変換後、三角ポリゴン列を生成
pub fn draw_triangles(
    // v:vertex, n:normal, c:color, ?i:index
    // ptr:buffer pointer, len:buffer length
    // mtx4: Matrix4D+Vec4(Offset)
    vtx: &[Vec4D], v_len: usize, // 現在の予定では、頂点は5つ組となる。最後の一つは親のセンターとなる。
    nor: &[Vec4D], n_len: usize, // 現在の予定では、法線は5つ組となる。最後の一つは親の法線となる。
    col: &[Color<u8>], c_len: usize,
    v_idx: &[usize], n_idx: &[usize], c_idx: &[usize], idx_len: usize,
    poly_type: i32, // 生成するポリゴンの法線等のタイプ。 0: 親の法線、1: 親のセンター
    rot4: Mat4D,    // 関数外で作成
    offs: Vec4D,    // 関数外で作成
    hpos: f32,
    rcv_buf: usize, buf_len: usize  // 受け取りバッファ
) -> usize {

    // 出力用バッファを用意
    let mut tri_buf: Vec<f32> = Vec::with_capacity(idx_len);

    // 4面体を形成 → 3角形を形成 → バッファに放り込む
    let mut idx = 0;
    let mut c_cnt = 0;
    const INDEX_UNIT: usize = 5;
    let mut item_count = 0;
    while idx < idx_len {
        //Vertex4つをアフィン変換し、しかる後四面体を作る
        let mut ver: [Vertex4D;TriPylam::VERTEX_NUM] = [ Default::default();TriPylam::VERTEX_NUM];
        print!("\r\n");
        for ii in 0..TriPylam::VERTEX_NUM{
            let idx_base = idx+ii;

            let v_base = v_idx[idx_base];
            ver[ii].vertex = &rot4.mul_vec(&vtx[v_base]) + &offs;
            
            let n_base = n_idx[idx_base];
            ver[ii].normal = rot4.mul_vec(&nor[n_base]);

            let c_base = c_idx[c_cnt+ii] as usize;
            ver[ii].color  = col[c_base];
        }
        let mut plm: TriPylam = Default::default();
        match poly_type {
            0 => {  // normalをそのまま利用
                //let parent_normal = Vec4D{ x:0.0, y:0.0, z:0.0, h:0.0 };    // 親を持ってこないといけない
                let v_base = v_idx[idx+4] as usize;
                let normal = &rot4.mul_vec(&vtx[v_base]);
                plm = TriPylam::new( &ver[0], &ver[1], &ver[2], &ver[3], &normal );
            },
            1 => {  // centerからnormalを生成
                //let parent_normal = Vec4D{ x:0.0, y:0.0, z:0.0, h:0.0 };    // センターを持ってこないといけない
                let v_base = v_idx[idx+4] as usize;
                let center = &rot4.mul_vec(&vtx[v_base]) + &offs;
                plm = TriPylam::new_with_center( &ver[0], &ver[1], &ver[2], &ver[3], &center ).unwrap_or_default();
            },
            _ => {  // その他
                // plm = Default::default();
            }
        }
        let check_pattern = plm.check_div_pattern(hpos);
        let mut triangle: Triangle = Default::default();
        match check_pattern.0 {
            DivPattern::Zero0A => {
                let tri_pack = plm.create_2triangles( hpos, check_pattern.1, check_pattern.2, check_pattern.3, check_pattern.4 );
                tri_pack.0.decompose(&mut tri_buf);
                tri_pack.1.decompose(&mut tri_buf);
                item_count += 2;
                println!("CheckPat: Zero0A");
            },
            DivPattern::Zero4 => {
                let tri_pack = plm.create_4triangles( check_pattern.1, check_pattern.2, check_pattern.3, check_pattern.4 );
                tri_pack.0.decompose(&mut tri_buf);
                tri_pack.1.decompose(&mut tri_buf);
                tri_pack.2.decompose(&mut tri_buf);
                tri_pack.3.decompose(&mut tri_buf);
/*
                println!("Tri0: {},{},{}", tri_pack.0.vertex[0].vertex.x, tri_pack.0.vertex[0].vertex.y, tri_pack.0.vertex[0].vertex.z,);
                println!("    : {},{},{}", tri_pack.0.vertex[1].vertex.x, tri_pack.0.vertex[1].vertex.y, tri_pack.0.vertex[1].vertex.z,);
                println!("    : {},{},{}", tri_pack.0.vertex[2].vertex.x, tri_pack.0.vertex[2].vertex.y, tri_pack.0.vertex[2].vertex.z,);
                println!("Tri1: {},{},{}", tri_pack.1.vertex[0].vertex.x, tri_pack.1.vertex[0].vertex.y, tri_pack.1.vertex[0].vertex.z,);
                println!("    : {},{},{}", tri_pack.1.vertex[1].vertex.x, tri_pack.1.vertex[1].vertex.y, tri_pack.1.vertex[1].vertex.z,);
                println!("    : {},{},{}", tri_pack.1.vertex[2].vertex.x, tri_pack.1.vertex[2].vertex.y, tri_pack.1.vertex[2].vertex.z,);
                println!("Tri2: {},{},{}", tri_pack.2.vertex[0].vertex.x, tri_pack.2.vertex[0].vertex.y, tri_pack.2.vertex[0].vertex.z,);
                println!("    : {},{},{}", tri_pack.2.vertex[1].vertex.x, tri_pack.2.vertex[1].vertex.y, tri_pack.2.vertex[1].vertex.z,);
                println!("    : {},{},{}", tri_pack.2.vertex[2].vertex.x, tri_pack.2.vertex[2].vertex.y, tri_pack.2.vertex[2].vertex.z,);
                println!("Tri3: {},{},{}", tri_pack.3.vertex[0].vertex.x, tri_pack.3.vertex[0].vertex.y, tri_pack.3.vertex[0].vertex.z,);
                println!("    : {},{},{}", tri_pack.3.vertex[1].vertex.x, tri_pack.3.vertex[1].vertex.y, tri_pack.3.vertex[1].vertex.z,);
                println!("    : {},{},{}", tri_pack.3.vertex[2].vertex.x, tri_pack.3.vertex[2].vertex.y, tri_pack.3.vertex[2].vertex.z,);
*/
                item_count += 4;
                println!("CheckPat: Zero04");
            },
            _ => {
                triangle = plm.create_triangle(hpos, check_pattern);
                triangle.decompose(&mut tri_buf);
                item_count += 1;
                println!("CheckPat: Solo");
            }
        }

        idx += INDEX_UNIT;
        c_cnt += (INDEX_UNIT-1);
    }
    
    unsafe{
        let rcv_ptr = rcv_buf as *mut f32;
        let rcv_slice: &mut [f32] = std::slice::from_raw_parts_mut(rcv_ptr, buf_len);
        let loop_size = if tri_buf.len() < buf_len { tri_buf.len() }else{ buf_len };
        println!("LoopSize: {}, TriBuf: {}, BufLen: {}", loop_size, tri_buf.len(), buf_len);
        for x in 0..loop_size{
            rcv_slice[x] = tri_buf[x];
        }
    }

    tri_buf.len()
}

//= generate_tiled_floor ====================================================================
pub fn generate_tiled_floor(leng: f32, tnum: i32, color0: Color<u8>, color1: Color<u8>) -> (Vec<f32>,Vec<f32>,Vec<u8>) {
    let tlg = leng*0.5;
    let unit_pos = [
        -tlg, -2.0, -tlg,
        -tlg, -2.0,  tlg,
         tlg, -2.0,  tlg,
        -tlg, -2.0, -tlg,
         tlg, -2.0,  tlg,
         tlg, -2.0, -tlg,
    ];
    let unit_pos_len = 6;
	let mut offs_x = tlg*((tnum-1) as f32 );
    let mut offs_z = offs_x;
    let tnum1 = (tnum-1) as f32;
    let mut vtx_array: Vec<f32> = Vec::new();
    let mut nor_array: Vec<f32> = Vec::new();
    let mut col_array: Vec<u8> = Vec::new();

	// 黒のプレート
	for clmn in 0..tnum {
		offs_x = tlg*tnum1;
		for row in 0..tnum {
			if ((row+clmn) & 0x01) == 0x01 {
				offs_x -= leng;
				continue;
			}
			for idx in 0..unit_pos_len {
                let ii = idx*3;
                vtx_array.push(unit_pos[ii]+offs_x);
                vtx_array.push(unit_pos[ii+1]);
                vtx_array.push(unit_pos[ii+2]+offs_z);
                nor_array.push(0.0);
                nor_array.push(1.0);
                nor_array.push(0.0);
                col_array.push(color0.r);
                col_array.push(color0.g);
                col_array.push(color0.b);
                col_array.push(color0.a);
            }
			offs_x -= leng;
		}
		offs_z -= leng;
    }
    
    // 白のプレート
    offs_z = tlg*((tnum-1)as f32);
	for clmn in 0..tnum {
		offs_x = tlg*tnum1;
		for row in 0..tnum {
			if ((row+clmn) & 0x01) == 0 {
				offs_x -= leng;
				continue;
			}
			for idx in 0..unit_pos_len {
                let ii = idx*3;
                vtx_array.push(unit_pos[ii]+offs_x);
                vtx_array.push(unit_pos[ii+1]);
                vtx_array.push(unit_pos[ii+2]+offs_z);
                nor_array.push(0.0);
                nor_array.push(1.0);
                nor_array.push(0.0);
                col_array.push(color1.r);
                col_array.push(color1.g);
                col_array.push(color1.b);
                col_array.push(color1.a);
            }
			offs_x -= leng;
		}
		offs_z -= leng;
    }

    (vtx_array, nor_array, col_array)
}
