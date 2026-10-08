(* OCaml port of the Octane/JetStream raytrace benchmark (raytrace.js).

   The ray tracer code was written by Adam Burmister, modified by Google to
   work as a standalone benchmark, and rewritten with ES6 classes for
   JetStream 3. The class hierarchies (Material, Shape) become records with a
   variant for the subclass-specific part, and dynamic dispatch becomes a
   match. All-float records (vector, color) are stored unboxed by OCaml. *)

external bench_start : unit -> unit = "bench_start"
external bench_end : unit -> unit = "bench_end"

type color = { mutable red : float; mutable green : float; mutable blue : float }

module Color = struct
  let make red green blue = { red; green; blue }

  let add c1 c2 =
    make (c1.red +. c2.red) (c1.green +. c2.green) (c1.blue +. c2.blue)

  let limit c =
    c.red <- (if c.red > 0. then if c.red > 1. then 1. else c.red else 0.);
    c.green <- (if c.green > 0. then if c.green > 1. then 1. else c.green else 0.);
    c.blue <- (if c.blue > 0. then if c.blue > 1. then 1. else c.blue else 0.);
    c

  let add_scalar c1 s = limit (make (c1.red +. s) (c1.green +. s) (c1.blue +. s))

  let multiply c1 c2 =
    make (c1.red *. c2.red) (c1.green *. c2.green) (c1.blue *. c2.blue)

  let multiply_scalar c1 f = make (c1.red *. f) (c1.green *. f) (c1.blue *. f)

  let blend c1 c2 w = add (multiply_scalar c1 (1. -. w)) (multiply_scalar c2 w)

  let brightness c =
    let r = int_of_float (Float.floor (c.red *. 255.)) in
    let g = int_of_float (Float.floor (c.green *. 255.)) in
    let b = int_of_float (Float.floor (c.blue *. 255.)) in
    ((r * 77) + (g * 150) + (b * 29)) asr 8
end

type vector = { x : float; mutable y : float; z : float }

module Vector = struct
  let make x y z = { x; y; z }
  let add v w = make (w.x +. v.x) (w.y +. v.y) (w.z +. v.z)
  let subtract v w = make (v.x -. w.x) (v.y -. w.y) (v.z -. w.z)
  let multiply_scalar v w = make (v.x *. w) (v.y *. w) (v.z *. w)
  let magnitude v = Float.sqrt ((v.x *. v.x) +. (v.y *. v.y) +. (v.z *. v.z))

  let normalize v =
    let m = magnitude v in
    make (v.x /. m) (v.y /. m) (v.z /. m)

  let negate_y v = v.y <- v.y *. -1.

  let cross v w =
    make
      ((-.v.z *. w.y) +. (v.y *. w.z))
      ((v.z *. w.x) -. (v.x *. w.z))
      ((-.v.y *. w.x) +. (v.x *. w.y))

  let dot v w = (v.x *. w.x) +. (v.y *. w.y) +. (v.z *. w.z)
end

type light = { light_position : vector; light_color : color }
type ray = { ray_position : vector; direction : vector }

type material_kind =
  | Solid of color
  | Chessboard of { color_even : color; color_odd : color; density : float }

type material = {
  reflection : float;
  transparency : float;
  gloss : float;
  has_texture : bool;
  kind : material_kind;
}

let solid_material color reflection transparency gloss =
  { reflection; transparency; gloss; has_texture = true; kind = Solid color }

let chessboard_material color_even color_odd reflection transparency gloss density =
  {
    reflection;
    transparency;
    gloss;
    has_texture = true;
    kind = Chessboard { color_even; color_odd; density };
  }

let wrap_up t =
  let t = Float.rem t 2. in
  let t = if t < -1. then t +. 2. else t in
  if t >= 1. then t -. 2. else t

let get_color m u v =
  match m.kind with
  | Solid color -> color
  | Chessboard { color_even; color_odd; density } ->
      let t = wrap_up (u *. density) *. wrap_up (v *. density) in
      if t < 0. then color_even else color_odd

type shape_kind = Sphere of float | Plane of float

type shape = { position : vector; material : material; shape_kind : shape_kind }

type intersection_info = {
  mutable is_hit : bool;
  mutable hit_count : int;
  mutable shape : shape option;
  mutable info_position : vector;
  mutable normal : vector;
  mutable color : color;
  mutable distance : float;
}

(* Stands in for the JS `null` that `position` and `normal` start out as. *)
let null_vector = Vector.make 0. 0. 0.
let default_color = Color.make 0. 0. 0.

let new_intersection_info () =
  {
    is_hit = false;
    hit_count = 0;
    shape = None;
    info_position = null_vector;
    normal = null_vector;
    color = default_color;
    distance = 0.;
  }

let intersect shape ray =
  let info = new_intersection_info () in
  info.shape <- Some shape;
  match shape.shape_kind with
  | Sphere radius ->
      let dst = Vector.subtract ray.ray_position shape.position in
      let b = Vector.dot dst ray.direction in
      let c = Vector.dot dst dst -. (radius *. radius) in
      let d = (b *. b) -. c in
      if d > 0. then begin
        info.is_hit <- true;
        info.distance <- -.b -. Float.sqrt d;
        info.info_position <-
          Vector.add ray.ray_position (Vector.multiply_scalar ray.direction info.distance);
        info.normal <- Vector.normalize (Vector.subtract info.info_position shape.position);
        info.color <- get_color shape.material 0. 0.
      end
      else info.is_hit <- false;
      info
  | Plane d ->
      let vd = Vector.dot shape.position ray.direction in
      if vd = 0. then info
      else
        let t = -.(Vector.dot shape.position ray.ray_position +. d) /. vd in
        if t <= 0. then info
        else begin
          info.is_hit <- true;
          info.info_position <-
            Vector.add ray.ray_position (Vector.multiply_scalar ray.direction t);
          info.normal <- shape.position;
          info.distance <- t;
          if shape.material.has_texture then begin
            let p = shape.position in
            let vu = Vector.make p.y p.z (-.p.x) in
            let vv = Vector.cross vu p in
            let u = Vector.dot info.info_position vu in
            let v = Vector.dot info.info_position vv in
            info.color <- get_color shape.material u v
          end
          else info.color <- get_color shape.material 0. 0.;
          info
        end

type camera = { cam_position : vector; look_at : vector; up : vector; equator : vector; screen : vector }

let make_camera position look_at up =
  {
    cam_position = position;
    look_at;
    up;
    equator = Vector.cross (Vector.normalize look_at) up;
    screen = Vector.add position look_at;
  }

let get_ray camera vx vy =
  let pos =
    Vector.subtract camera.screen
      (Vector.subtract
         (Vector.multiply_scalar camera.equator vx)
         (Vector.multiply_scalar camera.up vy))
  in
  Vector.negate_y pos;
  let dir = Vector.subtract pos camera.cam_position in
  { ray_position = pos; direction = Vector.normalize dir }

type background = { bg_color : color; ambience : float }

type scene = { camera : camera; background : background; shapes : shape array; lights : light array }

type options = {
  canvas_height : float;
  canvas_width : float;
  render_diffuse : bool;
  render_shadows : bool;
  render_highlights : bool;
  render_reflections : bool;
  ray_depth : int;
}

type engine = { mutable check_number : int; mutable pixel_checksum : int; options : options }

let make_engine ~canvas_width ~canvas_height ~pixel_width ~pixel_height ~render_diffuse
    ~render_shadows ~render_highlights ~render_reflections ~ray_depth =
  {
    check_number = 0;
    pixel_checksum = 0;
    options =
      {
        canvas_height = canvas_height /. pixel_height;
        canvas_width = canvas_width /. pixel_width;
        render_diffuse;
        render_shadows;
        render_highlights;
        render_reflections;
        ray_depth;
      };
  }

(* Beyond the JS's diagonal-only `checkNumber`, also checksum every pixel. *)
let set_pixel engine x y color =
  let b = Color.brightness color in
  engine.pixel_checksum <- ((engine.pixel_checksum * 31) + b) land 0x3fffffff;
  if x = y then engine.check_number <- engine.check_number + b

(* JS compares shapes by identity, `null` included. *)
let same_shape a b =
  match (a, b) with Some a, Some b -> a == b | None, None -> true | _ -> false

let test_intersection ray scene exclude =
  let hit_count = ref 0 in
  let best = ref (new_intersection_info ()) in
  !best.distance <- 2000.;
  for i = 0 to Array.length scene.shapes - 1 do
    let shape = scene.shapes.(i) in
    if not (same_shape (Some shape) exclude) then begin
      let info = intersect shape ray in
      if info.is_hit && info.distance >= 0. && info.distance < !best.distance then begin
        best := info;
        incr hit_count
      end
    end
  done;
  !best.hit_count <- !hit_count;
  !best

let get_reflection_ray p n v =
  let c1 = -.Vector.dot n v in
  let r1 = Vector.add (Vector.multiply_scalar n (2. *. c1)) v in
  { ray_position = p; direction = r1 }

let shape_of info = match info.shape with Some s -> s | None -> assert false

let rec ray_trace engine info ray scene depth =
  let opts = engine.options in
  let info_shape = shape_of info in
  let material = info_shape.material in
  (* Calc ambient *)
  let color = ref (Color.multiply_scalar info.color scene.background.ambience) in
  let shininess = 10. ** (material.gloss +. 1.) in
  for i = 0 to Array.length scene.lights - 1 do
    let light = scene.lights.(i) in
    (* Calc diffuse lighting *)
    let v = Vector.normalize (Vector.subtract light.light_position info.info_position) in
    if opts.render_diffuse then begin
      let l = Vector.dot v info.normal in
      if l > 0. then
        color :=
          Color.add !color
            (Color.multiply info.color (Color.multiply_scalar light.light_color l))
    end;
    (* The greater the depth the more accurate the colours, but this is
       exponentially (!) expensive. *)
    if depth <= opts.ray_depth then begin
      (* calculate reflection ray *)
      if opts.render_reflections && material.reflection > 0. then begin
        let reflection_ray = get_reflection_ray info.info_position info.normal ray.direction in
        let refl = test_intersection reflection_ray scene info.shape in
        if refl.is_hit && refl.distance > 0. then
          refl.color <- ray_trace engine refl reflection_ray scene (depth + 1)
        else refl.color <- scene.background.bg_color;
        color := Color.blend !color refl.color material.reflection
      end
    end;
    (* Render shadows and highlights *)
    let shadow_info = ref (new_intersection_info ()) in
    if opts.render_shadows then begin
      let shadow_ray = { ray_position = info.info_position; direction = v } in
      shadow_info := test_intersection shadow_ray scene info.shape;
      if !shadow_info.is_hit && not (same_shape !shadow_info.shape info.shape) then begin
        let va = Color.multiply_scalar !color 0.5 in
        let db = 0.5 *. ((shape_of !shadow_info).material.transparency ** 0.5) in
        color := Color.add_scalar va db
      end
    end;
    (* Phong specular highlights *)
    if opts.render_highlights && (not !shadow_info.is_hit) && material.gloss > 0. then begin
      let lv = Vector.normalize (Vector.subtract info_shape.position light.light_position) in
      let e = Vector.normalize (Vector.subtract scene.camera.cam_position info_shape.position) in
      let h = Vector.normalize (Vector.subtract e lv) in
      let gloss_weight = Float.max (Vector.dot info.normal h) 0. ** shininess in
      color := Color.add (Color.multiply_scalar light.light_color gloss_weight) !color
    end
  done;
  Color.limit !color

let get_pixel_color engine ray scene =
  let info = test_intersection ray scene None in
  if info.is_hit then ray_trace engine info ray scene 0 else scene.background.bg_color

exception Scene_rendered_incorrectly

let render engine scene =
  let opts = engine.options in
  let width = int_of_float (Float.ceil opts.canvas_width) in
  let height = int_of_float (Float.ceil opts.canvas_height) in
  for x = 0 to width - 1 do
    for y = 0 to height - 1 do
      let xp = (float_of_int x *. 1. /. opts.canvas_width *. 2.) -. 1. in
      let yp = (float_of_int y *. 1. /. opts.canvas_height *. 2.) -. 1. in
      let ray = get_ray scene.camera xp yp in
      let color = get_pixel_color engine ray scene in
      set_pixel engine x y color
    done
  done;
  if engine.check_number <> 2321 then raise Scene_rendered_incorrectly

let render_scene () =
  let camera =
    make_camera (Vector.make 0. 0. (-15.)) (Vector.make (-0.2) 0. 5.) (Vector.make 0. 1. 0.)
  in
  let background = { bg_color = Color.make 0.5 0.5 0.5; ambience = 0.4 } in
  let shapes =
    [|
      {
        position = Vector.make (-1.5) 1.5 2.;
        material = solid_material (Color.make 0. 0.5 0.5) 0.3 0. 2.;
        shape_kind = Sphere 1.5;
      };
      {
        position = Vector.make 1. 0.25 1.;
        material = solid_material (Color.make 0.9 0.9 0.9) 0.1 0. 1.5;
        shape_kind = Sphere 0.5;
      };
      {
        position = Vector.normalize (Vector.make 0.1 0.9 (-0.5));
        material =
          chessboard_material (Color.make 1. 1. 1.) (Color.make 0. 0. 0.) 0.2 0. 1. 0.7;
        shape_kind = Plane 1.2;
      };
    |]
  in
  let lights =
    [|
      { light_position = Vector.make 5. 10. (-1.); light_color = Color.make 0.8 0.8 0.8 };
      { light_position = Vector.make (-3.) 5. (-15.); light_color = Color.make 0.8 0.8 0.8 };
    |]
  in
  let scene = { camera; background; shapes; lights } in
  let raytracer =
    make_engine ~canvas_width:100. ~canvas_height:100. ~pixel_width:5. ~pixel_height:5.
      ~render_diffuse:true ~render_highlights:true ~render_shadows:true
      ~render_reflections:true ~ray_depth:2
  in
  render raytracer scene;
  raytracer

(* Fallback for when ./default.input is missing; keep in sync with it. *)
let default_renders = 24

(* Plain channels: In_channel.with_open_* goes through Fun.protect, which
   links in Printexc and more than doubles the code size. *)
let read_renders () =
  match open_in "./default.input" with
  | exception Sys_error _ -> default_renders
  | ic ->
      let line = try input_line ic with End_of_file -> "" in
      close_in ic;
      Option.value (int_of_string_opt (String.trim line)) ~default:default_renders

let () =
  let renders = read_renders () in
  bench_start ();
  let check = ref 0 and checksum = ref 0 in
  for _ = 1 to renders do
    let e = render_scene () in
    check := !check + e.check_number;
    checksum := (!checksum * 31 + e.pixel_checksum) land 0x3fffffff
  done;
  bench_end ();
  print_string
    ("renders: " ^ string_of_int renders ^ "\ncheck number: " ^ string_of_int !check
   ^ "\npixel checksum: " ^ string_of_int !checksum ^ "\n")
