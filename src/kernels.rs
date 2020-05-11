use crate::descriptors::{KernelConstructor::{self,*},SKernelConstructor};
use crate::descriptors::ConstructorTypes::*;
use std::collections::HashMap;
use crate::functions::{Needed::{self,*},SNeeded};
use serde::{Serialize,Deserialize};

#[derive(Clone,Debug)]
pub struct Kernel<'a> { //TODO use one SC for each &'a str
    pub name: &'a str,
    pub args: Vec<KernelConstructor<'a>>,
    pub src: &'a str,
    pub needed: Vec<Needed<'a>>,
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct SKernel {
    pub name: String,
    pub args: Vec<SKernelConstructor>,
    pub src: String,
    pub needed: Vec<SNeeded>,
}

impl<'a> From<&Kernel<'a>> for SKernel {
    fn from(f: &Kernel<'a>) -> Self {
        SKernel {
            name: f.name.into(),
            args: f.args.iter().map(|i| i.into()).collect(),
            src: f.src.into(),
            needed: f.needed.iter().map(|i| i.into()).collect(),
        }
    }
}

macro_rules! random {
    (philox4x32_10) => {
        "    uint key[2] = {x>>32,x};
    const uint l = 4;
    const uint M = 0xD2511F53;
    const uint M2 = 0xCD9E8D57;
    for(int i = 0;i<10;i++){
        uint hi0 = mul_hi(M,src[x*l+0]);
        uint lo0 = M * src[x*l+0];
        uint hi1 = mul_hi(M,src[x*l+2]);
        uint lo1 = M2 * src[x*l+2];
        src[x*l+0] = hi1^key[1]^src[x*l+3];
        src[x*l+2] = hi0^key[0]^src[x*l+1];
        src[x*l+1] = lo0;
        src[x*l+3] = lo1;
        key[0] += 0x9E3779B9;
        key[1] += 0xBB67AE85;
    }
"

    };
    (philox2x64_10) => { 
        "    ulong key = x;
    const uint l = 2;
    const ulong M = 0xD2B74407B1CE6E93;
    for(int i = 0;i<10;i++){
        ulong hi = mul_hi(M,src[x*l+0]);
        ulong lo = M * src[x*l+0];
        src[x*l+0] = hi^key^src[x*l+1];
        src[x*l+1] = lo;
        key += 0x9E3779B97F4A7C15;
    }
"
    };
    (philox4x64_10) => {
        "    ulong key[2] = {0,x};
    const uint l = 4;
    const ulong M = 0xD2B74407B1CE6E93;
    const ulong M2 = 0xCA5A826395121157;
    for(int i = 0;i<10;i++){
        ulong hi0 = mul_hi(M,src[x*l+0]);
        ulong lo0 = M * src[x*l+0];
        ulong hi1 = mul_hi(M,src[x*l+2]);
        ulong lo1 = M2 * src[x*l+2];
        src[x*l+0] = hi1^key[1]^src[x*l+3];
        src[x*l+2] = hi0^key[0]^src[x*l+1];
        src[x*l+1] = lo0;
        src[x*l+3] = lo1;
        key[0] += 0x9E3779B97F4A7C15;
        key[1] += 0xBB67AE8584CAA73B;
    }
"
    };
    ($name:ident, $more:literal) => {
        concat!(random!($name),$more)
    };
}

pub fn kernels() -> HashMap<&'static str,Kernel<'static>> {
    vec![
        // *************************************** RANDOM ***************************************
        Kernel {
            name: "philox2x64_10",
            args: vec![KCBuffer("src",CU64)],
            src: random!(philox2x64_10),
            needed: vec![],
        },
        Kernel {
            name: "philox2x64_10_unit",
            args: vec![KCBuffer("src",CU64),KCBuffer("dst",CF64)],
            src: random!(philox2x64_10,"    for(uint i = 0;i<l;i++)
        dst[x*l+i] = (double)(src[x*l+i]>>11)/(1l << 53);"
            ),
            needed: vec![],
        },
        Kernel {
            name: "philox2x64_10_normal",
            args: vec![KCBuffer("src",CU64),KCBuffer("dst",CF64)],
            src: random!(philox2x64_10,"    for(uint i = 0;i<l;i+=2) {
        double u1 = (double)(src[x*l+i]>>11)/(1l << 53);
        double u2 = (double)(src[x*l+i+1]>>11)/(1l << 53);
        dst[x*l+i] = sqrt(-2*log(u1))*cos(2*M_PI*u2);
        dst[x*l+i+1] = sqrt(-2*log(u1))*sin(2*M_PI*u2);
    }"
            ),
            needed: vec![],
        },
        Kernel {
            name: "philox4x64_10",
            args: vec![KCBuffer("src",CU64)],
            src: random!(philox4x64_10),
            needed: vec![],
        },
        Kernel {
            name: "philox4x64_10_unit",
            args: vec![KCBuffer("src",CU64),KCBuffer("dst",CF64)],
            src: random!(philox4x64_10,"    for(uint i = 0;i<l;i++)
        dst[x*l+i] = (double)(src[x*l+i]>>11)/(1l << 53);"
            ),
            needed: vec![],
        },
        Kernel {
            name: "philox4x64_10_normal",
            args: vec![KCBuffer("src",CU64),KCBuffer("dst",CF64)],
            src: random!(philox4x64_10,"    for(uint i = 0;i<l;i+=2) {
        double u1 = (double)(src[x*l+i]>>11)/(1l << 53);
        double u2 = (double)(src[x*l+i+1]>>11)/(1l << 53);
        dst[x*l+i] = sqrt(-2*log(u1))*cos(2*M_PI*u2);
        dst[x*l+i+1] = sqrt(-2*log(u1))*sin(2*M_PI*u2);
    }"
            ),
            needed: vec![],
        },
        Kernel {
            name: "philox4x32_10",
            args: vec![KCBuffer("src",CU32)],
            src: random!(philox4x32_10),
            needed: vec![],
        },
        Kernel {
            name: "philox4x32_10_unit",
            args: vec![KCBuffer("src",CU32),KCBuffer("dst",CF64)],
            src: random!(philox4x32_10,"    const uint l2 = l/2;
    for(uint i = 0;i<l2;i++)
        dst[x*l2+i] = (double)(((((ulong)src[x*l+i*2])<<32)+src[x*l+i*2+1])>>11)/(1l << 53);"
            ),
            needed: vec![],
        },
        Kernel {
            name: "philox4x32_10_normal",
            args: vec![KCBuffer("src",CU32),KCBuffer("dst",CF64)],
            src: random!(philox4x32_10,"    const uint l2 = l/2;
    for(uint i = 0;i<l2;i+=2) {
        double u1 = (double)(((((ulong)src[x*l+i])<<32)+src[x*l+i+1])>>11)/(1l << 53);
        double u2 = (double)(((((ulong)src[x*l+i+2])<<32)+src[x*l+i+3])>>11)/(1l << 53);
        dst[x*l2+i] = sqrt(-2*log(u1))*cos(2*M_PI*u2);
        dst[x*l2+i+1] = sqrt(-2*log(u1))*sin(2*M_PI*u2);
    }"
            ),
            needed: vec![],
        },

        // ******************************************************************************

        Kernel {
            name: "plus",
            args: vec![KCBuffer("a",CF64),KCBuffer("b",CF64),KCBuffer("dst",CF64)],
            src: "    dst[x] = a[x]+b[x];",
            needed: vec![],
        },
        Kernel {
            name: "minus",
            args: vec![KCBuffer("a",CF64),KCBuffer("b",CF64),KCBuffer("dst",CF64)],
            src: "    dst[x] = a[x]-b[x];",
            needed: vec![],
        },
        Kernel {
            name: "times",
            args: vec![KCBuffer("a",CF64),KCBuffer("b",CF64),KCBuffer("dst",CF64)],
            src: "    dst[x] = a[x]*b[x];",
            needed: vec![],
        },
        Kernel {
            name: "divides",
            args: vec![KCBuffer("a",CF64),KCBuffer("b",CF64),KCBuffer("dst",CF64)],
            src: "    dst[x] = a[x]/b[x];",
            needed: vec![],
        },
        Kernel {
            name: "cplus",
            args: vec![KCBuffer("src",CF64),KCParam("c",CF64),KCBuffer("dst",CF64)],
            src: "    dst[x] = src[x]+c;",
            needed: vec![],
        },
        Kernel {
            name: "cminus",
            args: vec![KCBuffer("src",CF64),KCParam("c",CF64),KCBuffer("dst",CF64)],
            src: "    dst[x] = src[x]-c;",
            needed: vec![],
        },
        Kernel {
            name: "ctimes",
            args: vec![KCBuffer("src",CF64),KCParam("c",CF64),KCBuffer("dst",CF64)],
            src: "    dst[x] = src[x]*c;",
            needed: vec![],
        },
        Kernel {
            name: "cdivides",
            args: vec![KCBuffer("src",CF64),KCParam("c",CF64),KCBuffer("dst",CF64)],
            src: "    dst[x] = src[x]/c;",
            needed: vec![],
        },
        Kernel {
            name: "move",
            args: vec![KCBuffer("src",CF64),KCBuffer("dst",CF64),KCParam("size",CU32_4),KCParam("offset",CU32)],
            src: "    dst[x+x_size*(y+y_size*z) + offset] = src[x+size.x*(y+size.y*z)];",
            needed: vec![],
        },
        Kernel {
            name: "smove",
            args: vec![KCBuffer("src",CF64),KCBuffer("dst",CF64),KCParam("size",CU32_4),KCParam("offset",CU32)],
            src: "    dst[x+size.x*(y+size.y*z) + offset] = src[x+x_size*(y+y_size*z)];",
            needed: vec![],
        },
        Kernel {
            name: "dmove",
            args: vec![KCBuffer("src",CF64),KCBuffer("dst",CF64),KCParam("size",CU32_4),KCParam("dst_size",CU32_4)],
            src: "    dst[dst_size.x*(x+dst_size.y*(y+dst_size.z*z)) + dst_size.w] = src[x+size.x*(y+size.y*z)];",
            needed: vec![],
        },
        Kernel {
            name: "rdmove",
            args: vec![KCBuffer("src",CF64),KCBuffer("dst",CF64),KCParam("size",CU32_4),KCParam("dst_size",CU32_4)],
            src: "    dst[x+dst_size.y*(y+dst_size.z*(z+dst_size.x*dst_size.w))] = src[x+size.x*(y+size.y*z)];",
            needed: vec![],
        },
        Kernel {
            name: "complex_from_real",
            args: vec![KCBuffer("src",CF64),KCBuffer("dst",CF64_2)],
            src: "    dst[x] = (double2)(src[x],0);",
            needed: vec![],
        },
        Kernel {
            name: "complex_from_image",
            args: vec![KCBuffer("src",CF64),KCBuffer("dst",CF64_2)],
            src: "    dst[x] = (double2)(0,src[x]);",
            needed: vec![],
        },
        Kernel {
            name: "real_from_complex",
            args: vec![KCBuffer("src",CF64_2),KCBuffer("dst",CF64)],
            src: "    dst[x] = src[x].x;",
            needed: vec![],
        },
        Kernel {
            name: "image_from_complex",
            args: vec![KCBuffer("src",CF64_2),KCBuffer("dst",CF64)],
            src: "    dst[x] = src[x].y;",
            needed: vec![],
        },
        Kernel {
            name: "kc_sqrmod",
            args: vec![KCBuffer("src",CF64_2),KCBuffer("dst",CF64)],
            src: "    dst[x] = c_sqrmod(src[x]);",
            needed: vec![FuncName("c_sqrmod".into())],
        },
        Kernel {
            name: "kc_mod",
            args: vec![KCBuffer("src",CF64_2),KCBuffer("dst",CF64)],
            src: "    dst[x] = c_mod(src[x]);",
            needed: vec![FuncName("c_mod".into())],
        },
        Kernel {
            name: "kc_times",
            args: vec![KCBuffer("a",CF64_2),KCBuffer("b",CF64_2),KCBuffer("dst",CF64_2)],
            src: "    dst[x] = c_times(a[x],b[x]);",
            needed: vec![FuncName("c_times".into())],
        },
        Kernel {
            name: "kc_times_conj",
            args: vec![KCBuffer("a",CF64_2),KCBuffer("b",CF64_2),KCBuffer("dst",CF64_2)],
            src: "    dst[x] = c_times_conj(a[x],b[x]);",
            needed: vec![FuncName("c_times_conj".into())],
        },
        Kernel {
            name: "kc_divides",
            args: vec![KCBuffer("a",CF64_2),KCBuffer("b",CF64_2),KCBuffer("dst",CF64_2)],
            src: "    dst[x] = c_divides(a[x],b[x]);",
            needed: vec![FuncName("c_divides".into())],
        },
        ].into_iter().map(|k| (k.name,k)).collect()
}
