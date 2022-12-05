import Debug.Trace

data RPS : Type where
  Rock : RPS
  Paper : RPS
  Scissors : RPS

Show RPS where
  show Rock = "Rock"
  show Paper = "Paper"
  show Scissors = "Scissors"

countPoints : (RPS, RPS) -> Nat
countPoints (a, b) = (handPoints b) + (resultPoints a b) where
  -- Each hand played gets a point value
  handPoints Rock = 1
  handPoints Paper = 2
  handPoints Scissors = 3
  -- Loss is 0, Draw is 3, Win is 6
  -- win is relative to player 2
  resultPoints Rock Paper = 6
  resultPoints Paper Scissors = 6
  resultPoints Scissors Rock = 6
  resultPoints Rock Rock = 3
  resultPoints Paper Paper = 3
  resultPoints Scissors Scissors = 3
  resultPoints Rock Scissors = 0
  resultPoints Scissors Paper = 0
  resultPoints Paper Rock = 0

parsePart1 : String -> Either String (List (RPS, RPS))
parsePart1 "" = Left "no lines to parse"
parsePart1 s = sequence $ map parseLine $ lines $ trim s where
  parseOpponent : String -> Either String RPS
  parseOpponent o = case o of
                       "A" => Right Rock
                       "B" => Right Paper
                       "C" => Right Scissors
                       _ => Left $ "Expected A, B, or C. Got: " ++ o
  parsePlayer : String -> Either String RPS
  parsePlayer p = case p of
                       "X" => Right Rock
                       "Y" => Right Paper
                       "Z" => Right Scissors
                       _ => Left $ "Expected X, Y, or Z. Got: " ++ p
  parseLine' : String -> String -> Either String (RPS, RPS)
  parseLine' opponent player = case parseOpponent opponent of
                                    Right opponent' => case parsePlayer player of
                                                            Right player' => Right (opponent', player')
                                                            Left err => Left err
                                    Left err => Left err
  parseLine s = case words s of
                     [] => Left "empty line"
                     (x :: y :: []) => parseLine' x y
                     _ => Left "wrong number of characters on line"

parsePart2 : String -> Either String (List (RPS, RPS))
parsePart2 "" = Left "no lines to parse"
parsePart2 s = sequence $ map parseLine $ lines $ trim s where
  parseOpponent : String -> Either String RPS
  parseOpponent "A" = Right Rock
  parseOpponent "B" = Right Paper
  parseOpponent "C" = Right Scissors
  parseOpponent o = Left $ "Expected A, B, or C. Got: " ++ o
  parsePlayer : RPS -> String -> Either String RPS
  parsePlayer Rock "X" = Right Scissors
  parsePlayer Rock "Y" = Right Rock
  parsePlayer Rock "Z" = Right Paper
  parsePlayer Paper "X" = Right Rock
  parsePlayer Paper "Y" = Right Paper
  parsePlayer Paper "Z" = Right Scissors
  parsePlayer Scissors "X" = Right Paper
  parsePlayer Scissors "Y" = Right Scissors
  parsePlayer Scissors "Z" = Right Rock
  parsePlayer _ r = Left $ "Expected X, Y, or Z. Got: " ++ r
  parseLine' : String -> String -> Either String (RPS, RPS)
  parseLine' opponent desiredResult = case parseOpponent opponent of
                                           Right opponent' => case parsePlayer opponent' desiredResult of
                                                                   Right player' => Right (opponent', player')
                                                                   Left err => Left err
                                           Left err => Left err
  parseLine s = case words s of
                     [] => Left "empty line"
                     (x :: y :: []) => parseLine' x y
                     _ => Left "wrong number of characters on line"

runGame : List (RPS, RPS) -> Nat
--runGame xs = sum $ map (\x => trace ("calculating for " ++ (show x) ++ " " ++ (show $ countPoints x)) $ countPoints x) xs
runGame xs = sum $ map countPoints xs

main : IO ()
main = do
  [_, arg] <- getArgs
    | [] => putStrLn "Unreachable"
    | [_] => putStrLn "No arguments passed"
    | _ => putStrLn "Too many arguments"
  Right content <- readFile arg | Left err => putStrLn (show err)
  case parsePart1 content of
       Left err => putStrLn err
       Right rounds => putStrLn $ show $ runGame rounds
  case parsePart2 content of
       Left err => putStrLn err
       Right rounds => putStrLn $ show $ runGame rounds
