#import bevy_sprite::mesh2d_view_bindings globals, view
#import bevy_sprite::mesh2d_vertex_output MeshVertexOutput

struct SdfDemoMaterial {
    color: vec4<f32>,
    shape: u32,
    mouse: vec2<f32>
};

@group(1) @binding(0)
var<uniform> material: SdfDemoMaterial;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let coord = center_sdf(mesh.uv, view.viewport.zw);

    let distance = shapes(material.shape, coord);
    
    if (distance >= 0.) {
        let color = mix(vec4(0.,0.,0.,1.), vec4(1.,0.647,0., 1.), distance);
        return color;
    } else {
        let color = mix(vec4(0.,0.,0.,1.), vec4(0.,0.855,1.,1.), abs(distance));
        return color;
    }

}

// center_sdf moves the coordinate system from uv==-0.5..0.5
// to x: -0.7..0.7, y: -0.5..0.5
// This makes it so that the coordinate system is a square from
// -0.5..0.5 in both directions.
//
// This is useful, especially when rendering SDFs that need to be
// proportional regardless of window aspect_ratio
fn center_sdf(uv: vec2f, width_height: vec2f) -> vec2f {
    let viewport_width = width_height.x;
    let viewport_height = width_height.y;
    // one side is probably going to be shorter than the other
    let min_viewport_size = min(viewport_width, viewport_height);
    let max_viewport_size = max(viewport_width, viewport_height);

    // the adjustment by which we need to reposition the longer side
    // for the shader result to still be centered
    let distance_to_push_center = (max_viewport_size - min_viewport_size) / 2.0;
    let center_push = distance_to_push_center / min_viewport_size * 2.0;

    var coord = (uv * vec2(viewport_width, viewport_height) / min_viewport_size * 2.0) - 1.0;
    if viewport_width > viewport_height {
        coord.x -= center_push;
    } else if viewport_width < viewport_height {
        coord.y -= center_push;
    };

    return coord;
}

fn shapes(shape: u32, coord: vec2f) -> f32 {
      switch shape {
        case 2u: {
          return sd_box(coord, vec2f(0.5,0.5));
        }
        case 3u: {
          return sd_equilateral_triangle(coord, 0.5);
        }
        case 4u: {
            return sd_rounded_x(coord, 0.7, 0.1);
        }
        case 5u {
            return sdf_cool_s(coord);
        }
        // Lone default
        default: {
          return sd_circle(coord, vec2f(0.,0.), 0.5);
        }
    }
}

// fn scene(point: Vec2) -> f32 {
//     let one = sd_box(coord, vec2(0.4));
//     let two = sd_box(coord, vec2(0.4));
// }
fn sd_circle(p: vec2f, center: vec2f, radius: f32) -> f32 {
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

fn sd_rounded_x( p: vec2f, w: f32, r: f32 ) -> f32
{
    let p = abs(p);
    return length(p-min(p.x+p.y,w)*0.5) - r;
}

fn sdf_cool_s( p: vec2f ) -> f32
{
    var p = p;
    var six: f32 = 0.;
    if (p.y<0.0) {
        six = -p.x;
    } else {
        six = p.x;
    }
    p.x = abs(p.x);
    p.y = abs(p.y) - 0.2;
    let rex: f32 = p.x - min(round(p.x/0.4),0.4);
    let aby: f32 = abs(p.y - 0.2) - 0.6;
    
    var d: f32 = dot2(vec2(six,-p.y)-clamp(0.5*(six-p.y),0.0,0.2));
    d = min(d,   dot2(vec2(p.x,-aby)-clamp(0.5*(p.x-aby),0.0,0.4)));
    d = min(d,   dot2(vec2(rex,p.y  -clamp(p.y          ,0.0,0.4))));
    
    let s: f32 = 2.0*p.x + aby + abs(aby+0.4) - 0.4;
    return sqrt(d) * sign(s);
}

fn dot2( v: vec2f ) -> f32 { return dot(v,v); }
