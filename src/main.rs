//use rand::prelude::*;

fn smoothstep(start: f64, end : f64, x : f64) -> f64 {
    start + ( 
        (end-start) * (6.0*x.powi(5)-15.0*x.powi(4)+10.0*x.powi(3)) as f64
    )
}
fn main () {
    /*for x in 0..16 {
        println!("{}",smoothstep(1.0,10.0,(x as f64 / 16.0)));
    }*/
    const XLEN : usize = 4;
    const YLEN : usize = 3;
    const RESOLUTION : usize = 12; //picks of noise per block
    let mut gradients = [[[0f64;2]; XLEN]; YLEN]; //hardcoded 4x3
    //const resolutionsq : i32 = 144;

    for x in &mut gradients {
        for y in x {
            let r: f64 = rand::random::<f64>() * 2f64 * std::f64::consts::PI;
            y[0] = r.cos();
            y[1] = r.sin();
            //println!("{}",y[0].to_string());
        }
    }
    //todo - don't hardcode array size

    //let mut noisevals = [[0f64;RESOLUTION*XLEN];RESOLUTION*YLEN];

    for y in 0..(RESOLUTION*(YLEN-1)) { 
        let yc = (y / RESOLUTION); //x coordinate
        //println!("y: {}",yc);
        let mut s = "".to_string();
        for x in 0..(RESOLUTION*(XLEN-1)) { //no overflow kludge
            let xc = (x / RESOLUTION); //y coordinate
            //println!("x: {}",xc);
            let grads = [ //upleft, upright, downleft, downright
                gradients[yc][xc],
                gradients[yc+1][xc],
                gradients[yc][xc+1], //count y from top down
                gradients[yc+1][xc+1]
            ];
            let dx = (x as f64 / RESOLUTION as f64) % 1f64;
            let dy = (y as f64 / RESOLUTION as f64) % 1f64;
            let dists = [
                [dx,dy], //UL
                [1.0-dx,dy], //UR
                [dx,1.0-dy], //DL
                [1.0-dx,1.0-dy], //DL
            ];
            let mut dotproducts = [0f64;4];

            for i in 0..dotproducts.len() {
                dotproducts[i] = 
                    dists[i][0] * grads[i][0]
                    + dists[i][1] * grads[i][1];
            }

            //this bit guided from adrianb.io, thanks
            //get the noise values along the x-lines, 
            //between up left/right and down left/right,
            //then interpolate between them at the appropriate y

            let noiseval = smoothstep(
                smoothstep(dotproducts[0],dotproducts[1],dx),
                smoothstep(dotproducts[2],dotproducts[3],dx),
                dy
            );
            //println!("{}",noiseval);

            //match is gonna be deprecated on floats, so if time
            //unknown issue: why am i not getting over 0.5 in either direction?
            if noiseval < -0.25 {
                s.push_str(".");
            } else if noiseval >= -0.25 && noiseval < 0.0 {
                s.push_str(",");
            } else if noiseval >= 0.0 && noiseval < 0.25 {
                s.push_str(";");
            } else if noiseval >= 0.25 {
                s.push_str("#");
            }

        }
        println!("{}",s);
        s = "".to_string()

    }
    
}