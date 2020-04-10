use gpgpu::Handler;
use gpgpu::descriptors::{BufferConstructor::*,KernelArg::*,Type::*};
use gpgpu::{Dim,DimDir::*};
use std::time::{SystemTime, UNIX_EPOCH};
use gpgpu::algorithms::moments_to_cumulants;

fn main() -> gpgpu::Result<()> {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
    let len = 1<<17;
    let n = 1<<11;
    let nmom = 4;
    let mut gpu = Handler::builder()?
        .add_buffer("src", Len(U64(time),len))
        .add_buffer("num", Len(F64(0.0),len))
        .add_buffer("tmp", Len(F64(0.0),len))
        .add_buffer("sum", Len(F64(0.0),len))
        .add_buffer("dstsum", Len(F64(0.0),len))
        .add_buffer("dst", Len(F64(0.0),nmom))
        .load_kernel_named("philox2x64_10_normal","noise")
        .load_kernel_named("philox4x64_10_normal","noise4")
        .load_kernel_named("philox4x32_10_normal","noise32")
        .load_algorithm("moments")
        .build()?;

    gpu.set_arg("noise",&[Buffer("src"),BufArg("num","dst")])?;
    gpu.set_arg("noise4",&[Buffer("src"),BufArg("num","dst")])?;
    gpu.set_arg("noise32",&[Buffer("src"),BufArg("num","dst")])?;

    println!("Generating 10^{:.2} random numbers and computing the meam:", (((len*n) as f64).ln()/10f64.ln()));

    println!("\nphilox2x64_10_normal");
    let start = SystemTime::now();
    let mut moms = vec![0.0;nmom];
    for _ in 0..n {
        gpu.run("noise",Dim::D1(len/2))?;
        gpu.run_algorithm("moments",Dim::D1(len),&[X],&["num","tmp","sum","dstsum","dst"],Some(&(nmom as u32)))?;
        moms = gpu.get::<f64>("dst")?.iter().enumerate().map(|(i,v)| moms[i]+v).collect();
    }
    moms = moms.iter().map(|v| v/n as f64).collect();
    println!("{:?}", moments_to_cumulants(&moms));
    println!("{}", SystemTime::now().duration_since(start).unwrap().as_millis());

    println!("\nphilox4x64_10_normal");
    let start = SystemTime::now();
    moms = vec![0.0;nmom];
    for _ in 0..n {
        gpu.run("noise4",Dim::D1(len/4))?;
        gpu.run_algorithm("moments",Dim::D1(len),&[X],&["num","tmp","sum","dstsum","dst"],Some(&(nmom as u32)))?;
        moms = gpu.get::<f64>("dst")?.iter().enumerate().map(|(i,v)| moms[i]+v).collect();
    }
    moms = moms.iter().map(|v| v/n as f64).collect();
    println!("{:?}", moments_to_cumulants(&moms));
    println!("{}", SystemTime::now().duration_since(start).unwrap().as_millis());

    println!("\nphilox4x32_10_normal");
    let start = SystemTime::now();
    moms = vec![0.0;nmom];
    for _ in 0..n {
        gpu.run("noise32",Dim::D1(len/2))?;
        gpu.run_algorithm("moments",Dim::D1(len),&[X],&["num","tmp","sum","dstsum","dst"],Some(&(nmom as u32)))?;
        moms = gpu.get::<f64>("dst")?.iter().enumerate().map(|(i,v)| moms[i]+v).collect();
    }
    moms = moms.iter().map(|v| v/n as f64).collect();
    println!("{:?}", moments_to_cumulants(&moms));
    println!("{}", SystemTime::now().duration_since(start).unwrap().as_millis());

    Ok(())
}
