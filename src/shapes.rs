use crate::fdw::*;

//= データ定義 ====================================================================
/* 正八胞体 */
const CUBE4_VTX_NUM: usize = 16;
static CUBE4_VTX: [Vec4D; CUBE4_VTX_NUM] = [
    Vec4D{ x:-1.0, y: 1.0, z:1.0, h:1.0 },
    Vec4D{ x: 1.0, y: 1.0, z:1.0, h:1.0 },
    Vec4D{ x:-1.0, y:-1.0, z:1.0, h:1.0 },
    Vec4D{ x: 1.0, y:-1.0, z:1.0, h:1.0 },

    Vec4D{ x: 1.0, y: 1.0, z:-1.0, h:1.0 },
    Vec4D{ x:-1.0, y: 1.0, z:-1.0, h:1.0 },
    Vec4D{ x: 1.0, y:-1.0, z:-1.0, h:1.0 },
    Vec4D{ x:-1.0, y:-1.0, z:-1.0, h:1.0 },

    Vec4D{ x:-1.0, y: 1.0, z:1.0, h:-1.0 },
    Vec4D{ x: 1.0, y: 1.0, z:1.0, h:-1.0 },
    Vec4D{ x:-1.0, y:-1.0, z:1.0, h:-1.0 },
    Vec4D{ x: 1.0, y:-1.0, z:1.0, h:-1.0 },

    Vec4D{ x: 1.0, y: 1.0, z:-1.0, h:-1.0 },
    Vec4D{ x:-1.0, y: 1.0, z:-1.0, h:-1.0 },
    Vec4D{ x: 1.0, y:-1.0, z:-1.0, h:-1.0 },
    Vec4D{ x:-1.0, y:-1.0, z:-1.0, h:-1.0 },
];

const CUBE4_VTX_INDEX_NUM: usize = 20*8;
static CUBE4_VTX_INDEX: [usize; CUBE4_VTX_INDEX_NUM] = [
    0, 1, 2, 5,  1, 2, 3, 6,   4, 5, 6, 1,   5, 6, 7, 2,   1, 2, 5, 6,		// こっち(h=+1)
    8, 9,10,13,  9,10,11,14,  12,13,14, 9,  13,14,15,10,   9,10,13,14,		// あっち(h=-1)
    9, 1,11,12,  1,11, 3, 6,   4,12, 6, 1,  11, 6,12,14,   1,11, 6,12,		// 右(X=+1)
    0, 8, 2, 5,  8, 2,10,15,   5,13,15, 8,  5, 7,15, 2,   2, 8, 5,15,		// 左(X=-1)
    0, 1, 8, 5,  1, 8, 9,12,   5, 4,12, 1,   5,12,13, 8,   1, 8, 5,12,		// 上(Y=+1)
    2,10,11,15,  2, 3,11, 6,  15,14, 6,11,  15, 6, 7, 2,   2,11, 6,15,		// 下(Y=-1)
    0, 1, 2, 8,  1, 2, 3,11,   8, 9,11, 1,   8,11,10, 2,   1, 2, 8,11,		// 手前(Z=+1)
    13,12,15, 5, 12,15,14, 6,   5, 4, 6,12,   5, 6, 7,15,  12,15, 5, 6		// 奥(Z=-1)
];

const CUBE4_NOR_NUM: usize = 8;
static CUBE4_NOR: [Vec4D; CUBE4_NOR_NUM] = [
    Vec4D{ x: 1.0, y: 0.0, z: 0.0, h: 0.0 },    // x+1
    Vec4D{ x:-1.0, y: 0.0, z: 0.0, h: 0.0 },    // x-1
    Vec4D{ x: 0.0, y: 1.0, z: 0.0, h: 0.0 },    // y+1
    Vec4D{ x: 0.0, y:-1.0, z: 0.0, h: 0.0 },    // y-1
    Vec4D{ x: 0.0, y: 0.0, z: 1.0, h: 0.0 },    // z+1
    Vec4D{ x: 0.0, y: 0.0, z:-1.0, h: 0.0 },    // z-1
    Vec4D{ x: 0.0, y: 0.0, z: 0.0, h: 1.0 },    // h+1
    Vec4D{ x: 0.0, y: 0.0, z: 0.0, h:-1.0 },    // h-1
];

static CUBE4_NOR_INDEX: [usize; CUBE4_VTX_INDEX_NUM] = [
    6, 6, 6, 6,  6, 6, 6, 6,  6, 6, 6, 6,  6, 6, 6, 6,  6, 6, 6, 6, 	// こっち(h=+1)
    7, 7, 7, 7,  7, 7, 7, 7,  7, 7, 7, 7,  7, 7, 7, 7,  7, 7, 7, 7, 	// あっち(h=-1)
    0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,		// 右(X=+1)
    1, 1, 1, 1,  1, 1, 1, 1,  1, 1, 1, 1,  1, 1, 1, 1,  1, 1, 1, 1,		// 左(X=-1)
    2, 2, 2, 2,  2, 2, 2, 2,  2, 2, 2, 2,  2, 2, 2, 2,  2, 2, 2, 2,		// 上(Y=+1)
    3, 3, 3, 3,  3, 3, 3, 3,  3, 3, 3, 3,  3, 3, 3, 3,  3, 3, 3, 3,		// 下(Y=-1)
    4, 4, 4, 4,  4, 4, 4, 4,  4, 4, 4, 4,  4, 4, 4, 4,  4, 4, 4, 4, 	// 手前(Z=+1)
    5, 5, 5, 5,  5, 5, 5, 5,  5, 5, 5, 5,  5, 5, 5, 5,  5, 5, 5, 5,		// 奥(Z=-1)
];

const CUBE4_COL_NUM: usize = 8;
static CUBE4_COL: [Color<u8>; CUBE4_COL_NUM] = [
    Color{ r:0xff, g:0x80, b:0x80, a:0xff },        // R
    Color{ r:0x80, g:0xff, b:0x80, a:0xff },        // G
    Color{ r:0x80, g:0x80, b:0xff, a:0xff },        // B
    Color{ r:0xff, g:0xff, b:0x80, a:0xff },        // Y
    Color{ r:0x80, g:0xff, b:0xff, a:0xff },        // C
    Color{ r:0xff, g:0x80, b:0xff, a:0xff },        // M
    Color{ r:0xff, g:0xff, b:0xff, a:0xff },        // W
    Color{ r:0x80, g:0x80, b:0x80, a:0xff },        // K
];

static CUBE4_COL_INDEX: [usize; CUBE4_VTX_INDEX_NUM] = [
    0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,  0, 0, 0, 0,		// R
    1, 1, 1, 1,  1, 1, 1, 1,  1, 1, 1, 1,  1, 1, 1, 1,  1, 1, 1, 1,		// G
    2, 2, 2, 2,  2, 2, 2, 2,  2, 2, 2, 2,  2, 2, 2, 2,  2, 2, 2, 2,		// B
    3, 3, 3, 3,  3, 3, 3, 3,  3, 3, 3, 3,  3, 3, 3, 3,  3, 3, 3, 3,		// Y
    4, 4, 4, 4,  4, 4, 4, 4,  4, 4, 4, 4,  4, 4, 4, 4,  4, 4, 4, 4, 	// C
    5, 5, 5, 5,  5, 5, 5, 5,  5, 5, 5, 5,  5, 5, 5, 5,  5, 5, 5, 5,		// M
    6, 6, 6, 6,  6, 6, 6, 6,  6, 6, 6, 6,  6, 6, 6, 6,  6, 6, 6, 6, 	// W
    7, 7, 7, 7,  7, 7, 7, 7,  7, 7, 7, 7,  7, 7, 7, 7,  7, 7, 7, 7, 	// K
/*
    0,1,2,3, 4,5,6,7, 0,1,2,3, 4,5,6,7, 0,1,2,3,
    0,1,2,3, 4,5,6,7, 0,1,2,3, 4,5,6,7, 0,1,2,3,
    0,1,2,3, 4,5,6,7, 0,1,2,3, 4,5,6,7, 0,1,2,3,
    0,1,2,3, 4,5,6,7, 0,1,2,3, 4,5,6,7, 0,1,2,3,
    0,1,2,3, 4,5,6,7, 0,1,2,3, 4,5,6,7, 0,1,2,3,
    0,1,2,3, 4,5,6,7, 0,1,2,3, 4,5,6,7, 0,1,2,3,
    0,1,2,3, 4,5,6,7, 0,1,2,3, 4,5,6,7, 0,1,2,3,
    0,1,2,3, 4,5,6,7, 0,1,2,3, 4,5,6,7, 0,1,2,3,
*/
];


//= 関数定義 ====================================================================
pub fn generate_tesseract(scale: f32) -> Vec<TriPylam> {
    generate_tesseract_base(
        &CUBE4_VTX, &CUBE4_VTX_INDEX,
        &CUBE4_NOR, &CUBE4_NOR_INDEX,
        &CUBE4_COL, &CUBE4_COL_INDEX,
        CUBE4_VTX_INDEX_NUM,
        scale
    )
}

fn generate_tesseract_base(
    pos_v: &[Vec4D], pos_i: &[usize],
    nor_v: &[Vec4D], nor_i: &[usize],
    col_v: &[Color::<u8>], col_i: &[usize],
    i_num: usize,
    scale: f32
) -> Vec<TriPylam> {
    let mut pylam_array = Vec::<TriPylam>::new();
    let plm_num = i_num/4;

    for plm_idx in 0..plm_num {
        let idx = plm_idx*4;
        let v0 = Vertex4D{ vertex: pos_v[pos_i[idx  ]], normal: nor_v[nor_i[idx  ]], color: col_v[col_i[idx  ]] };
        let v1 = Vertex4D{ vertex: pos_v[pos_i[idx+1]], normal: nor_v[nor_i[idx+1]], color: col_v[col_i[idx+1]] };
        let v2 = Vertex4D{ vertex: pos_v[pos_i[idx+2]], normal: nor_v[nor_i[idx+2]], color: col_v[col_i[idx+2]] };
        let v3 = Vertex4D{ vertex: pos_v[pos_i[idx+3]], normal: nor_v[nor_i[idx+3]], color: col_v[col_i[idx+3]] };
        let nor = nor_v[nor_i[idx]];
        let pylam = TriPylam::new( &v0, &v1, &v2, &v3, &nor );
        let pylam = pylam.scale(scale);

        pylam_array.push(pylam);
    }

    pylam_array
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
