use std::io;

fn main() {
    /* point 1 */
    println!("Enter all points as x,y ( no spaces and no '()' )");
    let mut point1 = String::new();
    io::stdin().read_line(&mut point1).expect("Failed to read line error");
    let point1 = point1.trim();
    let parts1: Vec<_> = point1.split(',').collect();

    let x1 = parts1.get(0);
    let x1: i32 = x1.expect("REASON").parse().unwrap();
    let y1 = parts1.get(1);
    let y1: i32 = y1.expect("REASON").parse().unwrap();

    /* Point 2 */
    println!("Enter all points as x,y ( no spaces and no '()' )");
    let mut point2 = String::new();
    io::stdin().read_line(&mut point2).expect("Failed to read line error");
    let point2 = point2.trim();
    let parts2: Vec<_> = point2.split(',').collect();

    let x2 = parts2.get(0);
    let x2: i32 = x2.expect("REASON").parse().unwrap();
    let y2 = parts2.get(1);
    let y2: i32 = y2.expect("REASON").parse().unwrap();

    /* Point 3 */
    println!("Enter all points as x,y ( no spaces and no '()' )");
    let mut point3 = String::new();
    io::stdin().read_line(&mut point3).expect("Failed to read line error");
    let point3 = point3.trim();
    let parts3: Vec<_> = point3.split(',').collect();

    let x3 = parts3.get(0);
    let x3: i32 = x3.expect("REASON").parse().unwrap();
    let y3 = parts3.get(1);
    let y3: i32 = y3.expect("REASON").parse().unwrap();

    translate(x1,x2,x3,y1,y2,y3);
    reflect(x1,x2,x3,y1,y2,y3);
    rotate(y1,y2,y3);

}

fn rotate(y1: i32, y2: i32, y3: i32) {
    let x1 = y1;
    let y1 = -1*x1;

    let x2 = y2;
    let y2 = -1*x2;

    let x3 = y3;
    let y3 = -1*x3;

    println!("Rotated Triagnle ({},{}), ({},{}), ({},{})\n", x1,y1,x2,y2,x3,y3)
}

fn reflect(x1: i32, x2: i32, x3: i32, y1: i32, y2: i32, y3: i32) {
    println!("Enter 'x' to reflect over x-axis or y to reflect over 'y' axis");

    let mut reflected = String::new();
    io::stdin().read_line(&mut reflected).expect("Failed to read line error");
    let reflected = reflected.trim();

    if reflected=="x" {
        let x1 = x1*-1;
        let x2 = x2*-1;
        let x3 = x3*-1;
        println!("Reflected Triangle ({},{}), ({},{}), ({},{})\n", x1,y1,x2,y2,x3,y3);
    } else if reflected=="y" {
        let y1 = y1*-1;
        let y2 = y2*-1;
        let y3 = y3*-1;
        println!("Reflected Triangle ({},{}), ({},{}), ({},{})\n", x1,y1,x2,y2,x3,y3);
    } else {
        println!("Invalid axis\n");
    }
}

fn translate(x1: i32, x2: i32, x3: i32, y1: i32, y2: i32, y3: i32) {
    println!("Enter how much to change x");
    let mut d_x = String::new();
    io::stdin().read_line(&mut d_x).expect("Failed to read line error");
    let d_x = d_x.trim();
    let d_x: i32 = d_x.parse().expect("Invalid Input");

    println!("Enter how much to change y");
    let mut d_y = String::new();
    io::stdin().read_line(&mut d_y).expect("Failed to read line error");
    let d_y = d_y.trim();
    let d_y: i32 = d_y.parse().expect("Invalid Input");

    let x1 = x1+d_x;
    let y1 = y1+d_y;

    let x2 = x2+d_x;
    let y2 = y2+d_y;

    let x3 = x3+d_x;
    let y3 = y3+d_y;

    println!("Translated Triangle ({},{}), ({},{}), ({},{})", x1,y1,x2,y2,x3,y3);
}