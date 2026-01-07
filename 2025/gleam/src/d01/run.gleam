import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import simplifile

const starting_pos = 50

pub fn main() -> Nil {
  case simplifile.read("../input/01.txt") {
    Ok(s) -> {
      io.println("Part 1: " <> part_1(s))
      io.println("Part 2: " <> part_2(s))
    }
    Error(_) -> io.println("could not read file")
  }
}

fn str_to_int(s: String) -> Int {
  int.parse(s)
  |> result.unwrap(0)
}

fn part_1(s: String) -> String {
  let lines = string.split(s, on: "\n")
  let #(_final_pos, final_count) =
    list.fold(lines, #(starting_pos, 0), fn(acc, l) {
      let #(pos, count) = acc
      let new_pos = case l {
        "R" <> rest -> { pos + str_to_int(rest) + 100 } % 100
        "L" <> rest -> { pos - str_to_int(rest) + 100 } % 100
        _ -> 50
      }
      case new_pos {
        0 -> #(new_pos, count + 1)
        _ -> #(new_pos, count)
      }
    })
  int.to_string(final_count)
}

fn part_2(s: String) -> String {
  let lines = string.split(s, on: "\n")
  let #(_final_pos, final_count) =
    list.fold(lines, #(starting_pos, 0), fn(acc, l) {
      let #(pos, count) = acc
      case l {
        "R" <> rest -> move(pos, 1, count, str_to_int(rest))
        "L" <> rest -> move(pos, -1, count, str_to_int(rest))
        _ -> acc
      }
    })
  int.to_string(final_count)
}

fn move(pos: Int, magnitude: Int, count: Int, i: Int) -> #(Int, Int) {
  case i > 0 {
    False -> #(pos, count)
    True -> {
      let new_count = {
        case pos {
          0 | 100 -> count + 1
          _ -> count
        }
      }
      move({ pos + magnitude + 100 } % 100, magnitude, new_count, i - 1)
    }
  }
}
