import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import simplifile

pub fn main() -> Nil {
  case simplifile.read("../input/02.txt") {
    Ok(s) -> {
      io.println("Part 1: " <> part_1(s))
      io.println("Part 2: " <> part_2(s))
    }
    Error(_) -> io.println("could not read file")
  }
}

fn part_1(s: String) -> String {
  string.split(s, ",")
  |> list.fold(0, fn(acc, r) {
    case string.split(r, "-") {
      [l, r] ->
        check_invalid(
          int.parse(l) |> result.unwrap(0),
          int.parse(r) |> result.unwrap(0),
        )
        + acc
      _ -> acc
    }
  })
  |> int.to_string()
}

fn check_invalid(id: Int, max_id: Int) -> Int {
  case id > max_id {
    True -> 0
    False -> {
      let s = int.to_string(id)
      let validity = case string.length(s) {
        len if len % 2 == 0 -> {
          let l = string.slice(s, 0, len / 2)
          let r = string.slice(s, len / 2, len / 2)
          case l == r {
            True -> id
            False -> 0
          }
        }
        _ -> 0
      }
      check_invalid(id + 1, max_id) + validity
    }
  }
}

fn part_2(_s: String) -> String {
  "todo"
}
