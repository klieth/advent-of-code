-- ==== Lifted from idris2's standard library...these were extremely useful in debugging this.
-- import Debug.Trace
-- 
-- traceValBy : (a -> String) -> a -> a
-- traceValBy f v = trace (f v) v
-- 
-- traceVal : Show a => a -> a
-- traceVal = traceValBy show

import Data.List

countPriority : Char -> Nat
countPriority c = case (elem c ['a'..'z'], elem c ['A'..'Z']) of
  (True, False) => minus (cast $ ord c) 96
  (False, True) => minus (cast $ ord c) 38 -- minus 64 for the starting ascii code, plus 26 to put in range 27..

Foldable (Either a) where
  foldr _ init (Left _) = init
  foldr func init (Right r) = func r init

findDuplicate : List Char -> Either String Char
findDuplicate c = findDuplicate' $ splitAt (divNat (length c) 2) c where
  findDuplicate' : (List Char, List Char) -> Either String Char
  findDuplicate' (xs, ys) with (intersect xs ys)
    | (x :: []) = Right x
    | xs' with (nub xs')
      | (x :: []) = Right x
      | xs'' = Left $ "must be exactly 1 duplicate item, found: " ++ (show xs'')

part1 : List String -> Either String Nat
part1 xs = liftA sum $ sequence $ map ((map countPriority) . findDuplicate . unpack) xs

part2 : List String -> Either String Nat
part2 xs = liftA sum $ sequence $ map ((map countPriority) . findCommonItem) $ chunks xs where
  chunks : List String -> List (List Char, List Char, List Char)
  chunks [] = []
  chunks (x :: y :: z :: xs) = (unpack x, unpack y, unpack z) :: chunks xs
  chunks _ = ?unreachable -- input is always guaranteed to have lines % 3 == 0
  findCommonItem : (List Char, List Char, List Char) -> Either String Char
  findCommonItem (xs, ys, zs) = case (nub $ intersect xs $ intersect ys zs) of
                                  (a :: []) => Right a
                                  bs => Left $ "must be exactly 1 shared item, found: " ++ (show bs)

main : IO ()
main = do
  [_, arg] <- getArgs
    | [] => putStrLn "Unreachable"
    | [_] => putStrLn "No arguments passed"
    | _ => putStrLn "Too many arguments"
  Right content <- readFile arg | Left err => putStrLn (show err)
  putStrLn $ show $ part1 $ lines $ trim content
  putStrLn $ show $ part2 $ lines $ trim content
