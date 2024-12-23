let _print_int_list list = 
  Printf.printf "[";
  List.iter (Printf.printf "%d ") list;
  Printf.printf "]\n"

let _print_string_list list =
  Printf.printf "[ ";
  List.iter (Printf.printf "\"%s\" ") list;
  Printf.printf "]\n"

let try_read input_channel = try Some(input_line input_channel) with End_of_file -> None

let rec _read_lines input_channel lines =
  match try_read input_channel with
  | Some(l) -> _read_lines input_channel (l :: lines)
  | None -> close_in input_channel; List.rev lines

let read_lines filename =
  let input_channel = open_in filename in
  _read_lines input_channel []

let () = 
  let lines = read_lines "input.txt" in
  let sum = Part.B.parse lines in
  Printf.printf "%d " sum
