let split_whitespace s = s 
  |> String.trim 
  |> String.split_on_char ' ' 
  |> List.filter ((<>) "")

let extract_pair s =
  match split_whitespace s with
  | [first; last] -> (first, last)
  | _ -> failwith "bad format"

let count (list : int list) : int = List.fold_left (fun acc _ -> acc + 1) 0 list 
let find_all (y : int) (xs : int list) : int list = List.filter (fun x -> x = y) xs

let parse (list : string list) : int =
  let (left, right) = List.split (List.map extract_pair list) in

  (* Parse to int *)
  let left = List.map int_of_string left in
  let right = List.map int_of_string right in

  (* Find the similarities between left and right *)
  let count_all x list = count (find_all x list) in
  let scores = List.map (fun lhs -> lhs * count_all lhs right) left in

  (* Finally, sum up all the scores *)
  List.fold_left (+) 0 scores
