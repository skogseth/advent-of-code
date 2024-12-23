let split_whitespace s = s 
  |> String.trim 
  |> String.split_on_char ' ' 
  |> List.filter ((<>) "")

let extract_pair s =
  match split_whitespace s with
  | [first; last] -> (first, last)
  | _ -> failwith "bad format"

let parse (list : string list) : int =
  let (left, right) = List.split (List.map extract_pair list) in

  (* Parse to int *)
  let left = List.map int_of_string left in
  let right = List.map int_of_string right in

  (* Sort lists *)
  let left = List.sort compare left in
  let right = List.sort compare right in

  (* Find the difference between left and right *)
  let differences = List.combine left right |> List.map (fun (x, y) -> abs(y - x)) in

  (* Finally, sum up all the differences *)
  List.fold_left (+) 0 differences
