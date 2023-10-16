#import bevy_sprite::mesh2d_view_bindings globals, view
#import bevy_sprite::mesh2d_vertex_output MeshVertexOutput

struct SdfDemoMaterial {
    color: vec4<f32>,
    mouse: vec2<f32>
};

@group(1) @binding(0)
var<uniform> material: SdfDemoMaterial;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let viewport_width = view.viewport.z;
    let viewport_height = view.viewport.w;


    let coord = ((mesh.uv * 2.0) - 1.0);
    let mouse_coord = ((material.mouse.xy * 2.0) - 1.0);

    // let distance_from_circle = length(coord) - 0.1;
    let rounded = (sin(globals.time * 2.0) + 1.0) / 4.0;
    let rounded_2 = (sin(globals.time * 0.25) + 1.0) / 4.0;
    let rounded_3 = (sin(globals.time * 4.0) + 1.0) / 4.0;
    let rounded_4 = (sin(globals.time * 1.0) + 1.0) / 4.0;
    let distance_from_circle = rounded_box(
        coord,
        vec2(0.6, 0.6),
        vec4(rounded, rounded_2, rounded_3, rounded_4)
    );

    // let frag_dist_to_scene = sd_box(coord, vec2(0.4));
    // let mouse_dist_to_scene = sd_box(mouse_coord, vec2(0.4));

    let frag_dist_to_scene = sd_triangle(coord, vec2(-0.5, -0.5), vec2(-0.5, 0.5), vec2(0.5, -0.5));
    let mouse_dist_to_scene = sd_triangle(mouse_coord, vec2(-0.5, -0.5), vec2(-0.5, 0.5), vec2(0.5, -0.5));

    let position = sin(globals.time);

// return vec4(abs(mouse_dist_to_scene), 0.0, 0.0,1.0);

if (!(
    frag_dist_to_scene > mouse_dist_to_scene - 0.005
  && frag_dist_to_scene < mouse_dist_to_scene + 0.005
    )){
        let output = step(0.00001, frag_dist_to_scene);
        return vec4(vec3(output), 1.0);
    } else {
        // green
        return vec4(0.,1.,0.,1.);
    };

    // if (!(scene < mouse_scene && scene > mouse_scene - 0.01)) {
    //     let output = step(0.00001, scene);
    //     return vec4(vec3(output), 1.0);
    // } else {
    //     // green
    //     return vec4(0.,1.,0.,1.);
    // };
}

// fn scene(point: Vec2) -> f32 {
//     let one = sd_box(coord, vec2(0.4));
//     let two = sd_box(coord, vec2(0.4));
// }
fn circle(p: vec2f, center: vec2f, radius: f32) -> f32 {
    return length(p-center) - radius;
}

fn rounded_box(p: vec2f, b: vec2f, r: vec4f) -> f32 {
    var new_r = r;
    if (p.x <= 0.0) {
        new_r.x = r.z;
        new_r.y = r.w;
    };
    if (p.y <= 0.0) {
        new_r.x = r.y;
    };
    let q = abs(p)-b+new_r.x;
    return min(max(q.x,q.y),0.0) + length(max(q,vec2(0.0))) - new_r.x;
}

// fn sdArc( in vec2 p, in vec2 sc, in float ra, float rb ) -> f32
// {
//     // sc is the sin/cos of the arc's aperture
//     p.x = abs(p.x);
//     return ((sc.y*p.x>sc.x*p.y) ? length(p-sc*ra) : 
//                                   abs(length(p)-ra)) - rb;
// }

fn sd_box( p: vec2f, b: vec2f ) -> f32
{
    let d = abs(p) - b;
    return length(max(d,vec2(0.0))) + min(max(d.x,d.y),0.0);
}

fn sd_equilateral_triangle( point: vec2f, r: f32 ) -> f32
{
    var p = point;
    let k: f32 = sqrt(3.0);
    p.x = abs(p.x) - r;
    p.y = p.y + r/k;
    if( p.x+k*p.y>0.0 ) {
        p = vec2(p.x-k*p.y,-k*p.x-p.y)/2.0;
    }
    p.x -= clamp( p.x, -2.0*r, 0.0 );
    return -length(p)*sign(p.y);
}

fn smin( a: f32, b: f32, k: f32 ) -> f32
{
    let h = max(k-abs(a-b),0.0);
    return min(a, b) - h*h*0.25/k;
}

fn sd_triangle( p: vec2f, p0: vec2f, p1: vec2f, p2: vec2f ) -> f32
{
    let e0: vec2f = p1-p0;
    let e1: vec2f = p2-p1;
    let e2: vec2f = p0-p2;
    let v0: vec2f = p -p0;
    let v1: vec2f = p -p1;
    let v2: vec2f = p -p2;
    let pq0: vec2f = v0 - e0*clamp( dot(v0,e0)/dot(e0,e0), 0.0, 1.0 );
    let pq1: vec2f = v1 - e1*clamp( dot(v1,e1)/dot(e1,e1), 0.0, 1.0 );
    let pq2: vec2f = v2 - e2*clamp( dot(v2,e2)/dot(e2,e2), 0.0, 1.0 );
    let s: f32 = sign( e0.x*e2.y - e0.y*e2.x );
    let d: vec2f = min(min(vec2(dot(pq0,pq0), s*(v0.x*e0.y-v0.y*e0.x)),
                     vec2(dot(pq1,pq1), s*(v1.x*e1.y-v1.y*e1.x))),
                     vec2(dot(pq2,pq2), s*(v2.x*e2.y-v2.y*e2.x)));
    return -sqrt(d.x)*sign(d.y);
}