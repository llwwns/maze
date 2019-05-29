use rand::prelude::*;
use rand_xoshiro::Xoshiro256Plus;
use union_find::UnionFind;
use bitvec::prelude::*;
use clap::{Arg, App};
use std::io::{stdout, Write, BufWriter};

// left wall of cell : cell * 2
// upper wall of cell : cell * 2 + 1

fn main() {
  let matches = App::new("Maze generator")
    .version("0.1")
    .arg(Arg::with_name("size").short("n").takes_value(true))
    .get_matches();
  let size: usize = matches.value_of("size").unwrap_or("1").parse().unwrap_or(1);
  let mut r = Xoshiro256Plus::from_rng(thread_rng()).unwrap();
  let csize = size + 1;
  let mut num = size * size;
  let max_wall = (size + 1) * (size + 1) * 2;
  let mut maze = BitVec::<BigEndian, u64>::with_capacity(max_wall);
  maze.resize(max_wall, true);
  let mut uf = UnionFind::new(csize * csize);
  let mut walls: Vec<_> = (0..max_wall - 2).collect();
  walls.shuffle(&mut r);
  while num > 1 {
    let wall = walls.pop().unwrap();
    let a = wall >> 1;
    let is_vectial = (wall & 1) == 0;
    if a % csize == size || a / csize == size || is_vectial && (a % csize == 0) || !is_vectial && (a / csize == 0) {
      continue;
    }
    let b = if wall & 1 == 1 {
      a - csize
    } else {
      a - 1
    };
    if uf.is_same(a, b) {
      continue
    }
    uf.unite(a, b);
    maze.set(wall, false);
    num -= 1;
  }
  let out = stdout();
  let mut out = BufWriter::new(out.lock());
  write!(out, "╔").unwrap();
  for j in 0..(size-1) {
    write!(out, "════").unwrap();
    let cell = j + 1;
    let wall = cell * 2;
    if maze[wall] {
      write!(out, "╤").unwrap();
    } else {
      write!(out, "═").unwrap();
    }
  }
  writeln!(out, "════╗").unwrap();
  for i in 0..(size-1) {
    write!(out, "║").unwrap();
    for j in 0..(size-1) {
      let cell = i * csize + j + 1;
      let wall = cell * 2;
      //write!(out, "#{}#", wall).unwrap();
      if maze[wall] {
        write!(out, "    │").unwrap();
      } else {
        write!(out, "     ").unwrap();
      }
    }
    writeln!(out, "    ║").unwrap();
    let cell = (i + 1) * csize;
    let wall = cell * 2 + 1;
    if maze[wall] {
      write!(out, "╟").unwrap();
    } else {
      write!(out, "║").unwrap();
    }
    for j in 0..(size-1) {
      let cell = (i + 1) * csize + j;
      let wall_l = cell * 2 + 1;
      //write!(out, "#{}#", wall).unwrap();
      if maze[wall_l] {
        write!(out, "────").unwrap();
      } else {
        write!(out, "    ").unwrap();
      }
      let c2 = cell + 1;
      let c3 = c2 - csize;
      let wall_r =c2 * 2 + 1;
      let wall_u = c3 * 2;
      let wall_d = c2 * 2;
      write!(out, "{}", match (maze[wall_l], maze[wall_r], maze[wall_u], maze[wall_d]) {
        (false , false , false , false) => ' ' ,
        (true  , false , false , false) => '╴' ,
        (false , true  , false , false) => '╶' ,
        (true  , true  , false , false) => '─' ,
        (false , false , true  , false) => '╵' ,
        (true  , false , true  , false) => '┘' ,
        (false , true  , true  , false) => '└' ,
        (true  , true  , true  , false) => '┴' ,
        (false , false , false , true) => '╷' ,
        (true  , false , false , true) => '┐' ,
        (false , true  , false , true) => '┌' ,
        (true  , true  , false , true) => '┬' ,
        (false , false , true  , true) => '│' ,
        (true  , false , true  , true) => '┤' ,
        (false , true  , true  , true) => '├' ,
        (true  , true  , true  , true) => '┼' ,
      }).unwrap();
    }
    let cell = (i + 1) * csize + size - 1;
    let wall = cell * 2 + 1;
    //write!(out, "#{}#", wall).unwrap();
    if maze[wall] {
      writeln!(out, "────╢").unwrap();
    } else {
      writeln!(out, "    ║").unwrap();
      }
  }
  write!(out, "║").unwrap();
  for j in 0..(size-1) {
      let cell = (size - 1) * csize + j + 1;
      let wall = cell * 2;
      //write!(out, "#{}#", wall).unwrap();
    if maze[wall] {
      write!(out, "    │").unwrap();
    } else {
      write!(out, "     ").unwrap();
    }
  }
  writeln!(out, "    ║").unwrap();

  write!(out, "╚").unwrap();
  for j in 0..(size-1) {
    let cell = (size - 1) * csize + j + 1;
    let wall = cell * 2;
    write!(out, "════").unwrap();
    if maze[wall] {
      write!(out, "╧").unwrap();
    } else {
      write!(out, "═").unwrap();
    }
  }
  writeln!(out, "════╝").unwrap();
}
